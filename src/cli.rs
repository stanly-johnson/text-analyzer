use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to CSV file of dot contributions with no header
    #[clap(long)]
    pub file: String,

    /// Should load file from aws s3
    #[clap(long)]
    pub load: bool,

    #[clap(subcommand)]
    pub query: Option<Query>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Query {
    // search for given word
    Search { word: String },
    // count occurences of given word
    Count { word: String },
    // create word frequency map
    FrequencyMap,
}
