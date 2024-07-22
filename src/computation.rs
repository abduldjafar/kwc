use std::{collections::HashMap, sync::{Arc, Mutex}};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn process_count_every_word(chunk: &Vec<String>, arc_var: Arc<Mutex<usize>>) {
    let local_count: usize = chunk.par_iter()
        .map(|data| count_every_word(data))
        .sum();

    let mut global_count = arc_var.lock().unwrap();
    *global_count += local_count;
}

fn count_every_word(chunk: &str) -> usize {
    chunk.split_whitespace().count()
}


pub fn word_count_in_map(chunk: Vec<String>, arc_var: Arc<Mutex<HashMap<String, i32>>>) {
    let local_counts: HashMap<String, i32> = chunk.par_iter()
        .map(|data| word_mapping(data))
        .reduce(HashMap::new, |mut acc, counts| {
            for (word, count) in counts {
                *acc.entry(word).or_insert(0) += count;
            }
            acc
        });

    // Merge local counts into the global hashmap
    let mut global_counts = arc_var.lock().unwrap();
    for (word, count) in local_counts {
        *global_counts.entry(word).or_insert(0) += count;
    }
}

fn word_mapping(chunk: &str) -> HashMap<String, i32> {
    let mut counts = HashMap::new();
    for word in chunk.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

pub fn count_line(chunk: &Vec<String>, arc_var: Arc<Mutex<usize>>) {
    let local_count: usize = chunk.par_iter()
        .count();

    let mut global_count = arc_var.lock().unwrap();
    *global_count += local_count;
}