use clap::Parser;

mod cli;
mod functions;
mod tests;
mod traits;

use cli::*;

use traits::TextAnalyzer;

#[derive(Clone)]
struct LocalTextAnalyzer {
    combined_file_path: String,
}

impl LocalTextAnalyzer {
    fn new(combined_file_path: String) -> Self {
        Self { combined_file_path }
    }

    async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        functions::load_text_files_from_s3(&self.combined_file_path).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Starting text analyzer");
    let text_analyzer = LocalTextAnalyzer::new(args.file);

    if args.load {
        println!("Fetching all files from aws combined file!");
        text_analyzer.load().await?;
    }

    if let Some(query) = args.query {
        use cli::Query::*;
        match query {
            Search { word } => {
                let result = text_analyzer.search_word(word);
                println!("Result {:?}", result);
            }
            Count { word: _ } => {
                todo!()
            }
            FrequencyMap => {
                let result = text_analyzer.count_word_frequency();
                println!("Result {:?}", result);
            }
        }
    }

    Ok(())
}
