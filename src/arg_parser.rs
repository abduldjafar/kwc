use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub filename: String,

    #[arg(short, long, default_value_t = false)]
    pub word_count: bool,

    #[arg(short, long, default_value_t = false)]
    pub map_count: bool,

    #[arg(short, long, default_value_t = false)]
    pub line_count: bool,

    #[arg(short, long, default_value_t = 100000)]
    pub chunk_size: i32,

    #[arg(short, long, default_value_t = 5)]
    pub thread: i32,
}
