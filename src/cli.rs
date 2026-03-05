use crate::GameMode;
use clap::Parser;

/// a minimalist terminal typing tutor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// timer: fixed time, variable words | words: fixed words, variable time
    #[arg(short, long, value_enum, default_value_t = GameMode::Timer)]
    pub mode: GameMode,

    /// time limit in seconds or total word count, depending on mode
    #[arg(short, long, default_value_t = 30)]
    pub quantity: usize,

    /// predefined sets (small, medium, long), a local file path, or a url
    #[arg(short, long, default_value = "medium")]
    pub dictionary: String,

    /// restrict dictionary to include words containing only these characters
    #[arg(short, long)]
    pub include: Option<String>,

    /// exclude these chars from dictionary
    #[arg(short, long)]
    pub exclude: Option<String>,
    
    /// number of upcoming words to display in the queue
    #[arg(short, long, default_value_t = 2)]
    pub word_preview: usize,

    /// advance to the next word automatically without pressing space
    #[arg(short, long, default_value_t = false)]
    pub auto_advance: bool,

    /// exclude words shorter than this length
    #[arg(short = 's', long, default_value_t = 2)]
    pub min_word_size: usize,

    /// generate random strings using only the characters provided (overrides dictionary parameter)
    #[arg(short, long)]
    pub practice: Option<String>,

    /// will make the first letter of a word upper-case (random)
    #[arg(short, long, default_value_t = false)]
    pub upper_case: bool,
}
