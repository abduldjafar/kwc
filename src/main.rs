use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;
use std::sync::mpsc::{self};
use std::sync::{ Arc, Mutex};
use std::thread;
use std::time::Instant;
use clap::Parser;
use kwc::arg_parser;
use kwc::computation::{count_line, process_count_every_word, word_count_in_map};

fn main() -> io::Result<()> {
    let args = arg_parser::Args::parse();

    let file_path = args.filename;
    let num_threads = args.thread;
    let word_count = args.word_count;
    let line_count = args.line_count;
    let map_count = args.map_count;
    let chunk_size: usize = args.chunk_size.try_into().unwrap();

    if !word_count && !line_count && !map_count {
        println!("please run kwc --help for more information");
        exit(0);
    }

    let shared_word_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let shared_line: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let shared_map_count: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));

    // Start the timer
    let start = Instant::now();

    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let (worker_tx, worker_rx): (mpsc::Sender<Vec<String>>, mpsc::Receiver<Vec<String>>) = mpsc::channel();
    let worker_rx = Arc::new(Mutex::new(worker_rx));
    // Spawn worker threads
    let mut handles = vec![];
    for _ in 0..num_threads {
        let worker_rx = Arc::clone(&worker_rx);
        let shared_word_count = Arc::clone(&shared_word_count);
        let shared_line = Arc::clone(&shared_line);
        let shared_map_count = Arc::clone(&shared_map_count);

        let handle = thread::spawn(move || {
            while let Ok(chunk) = {
                let rx = worker_rx.lock().unwrap();
                rx.recv()
            }  {
                if word_count {
                    process_count_every_word(&chunk, Arc::clone(&shared_word_count));
                }
                if line_count {
                    count_line(&chunk, Arc::clone(&shared_line));

                }
                if map_count {
                    word_count_in_map(chunk, Arc::clone(&shared_map_count));
                }
            }
        });
        handles.push(handle);
    }

    // Start the timer for pushing data to worker threads
    let start_push_to_worker = Instant::now();
    let mut chunk: Vec<String> = Vec::with_capacity(chunk_size);

    for line in reader.lines() {
        let line = line?;
        chunk.push(line);

        if chunk.len() == chunk_size {
            worker_tx.send(chunk).unwrap();
            chunk = Vec::with_capacity(chunk_size);
        }
    }

    let duration_pusher = start_push_to_worker.elapsed();
    println!("Duration to push data to workers: {:?}", duration_pusher);

    // Send remaining lines if any
    if !chunk.is_empty() {
        worker_tx.send(chunk).unwrap();
    }

    // Drop the sender to signal workers to exit
    drop(worker_tx);

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Stop the timer
    let duration = start.elapsed();

    if word_count {
        let final_word_count = *shared_word_count.lock().unwrap();
        println!("Total word count: {:?}", final_word_count);
    }

    if line_count {
        let final_line_count = *shared_line.lock().unwrap();
        println!("Total line count: {:?}", final_line_count);
    }

    if map_count {
        let final_map_count = shared_map_count.lock().unwrap().clone();
        println!("Total word count in map: {:?}", final_map_count);
    }

    println!("Time taken: {:?}", duration);

    Ok(())
}