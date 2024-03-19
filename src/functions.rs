use async_fs::File as AsyncFsFile;
use futures_lite::io::AsyncWriteExt;
use reqwest::Client;
use std::collections::HashMap;
use std::fs::File;

use std::io::{Read, Seek, SeekFrom};

use std::sync::{Arc, Mutex};
use std::thread;

// naive approach based on local system
const BLOCK_SIZE: usize = 16_777_216; //16M
const THREADS: usize = 10; // m1 max cores

pub fn count_word_frequency(file_path: &str) -> HashMap<String, usize> {
    let metadata = std::fs::metadata(file_path).expect("File not found!");
    let length = metadata.len() as usize;
    let chunks: usize = ((length + THREADS - 1) / THREADS) as usize;

    let word_frequency = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..THREADS {
        let word_frequency = Arc::clone(&word_frequency);
        let chunk_start = i * chunks;
        let chunk_end = std::cmp::min((i + 1) * chunks, length);
        let file_path = file_path.to_owned();
        let handle = thread::spawn(move || {
            let mut thread_file = File::open(&file_path).expect("Unable to open file");
            let mut contents = vec![0_u8; BLOCK_SIZE];
            let mut read_total: usize = 0;

            thread_file
                .seek(SeekFrom::Start(chunk_start as u64))
                .expect("Couldn't seek to position in file");

            while read_total < chunk_end - chunk_start {
                let bytes_to_read = std::cmp::min(BLOCK_SIZE, chunk_end - chunk_start - read_total);
                let read_length = thread_file
                    .read(&mut contents[..bytes_to_read])
                    .expect("Couldn't read file");
                let chunk_text = String::from_utf8_lossy(&contents[..read_length]);
                let local_frequency = count_word_frequency_inner(&chunk_text);
                let mut global_frequency = word_frequency.lock().unwrap();
                for (word, count) in local_frequency {
                    *global_frequency.entry(word).or_insert(0) += count;
                }
                read_total += read_length;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Arc::try_unwrap(word_frequency)
        .unwrap()
        .into_inner()
        .unwrap()
}

fn count_word_frequency_inner(text: &str) -> HashMap<String, usize> {
    let mut word_frequency = HashMap::new();
    for word in text.split_whitespace() {
        *word_frequency.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    word_frequency
}

pub fn search_word_in_file(
    file_path: &str,
    word_to_find: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let word_to_find = word_to_find.to_lowercase();
    let file = File::open(file_path)?;
    let length = file.metadata()?.len() as usize;

    let mut found = false;
    let mut handles: Vec<std::thread::JoinHandle<Result<(), std::io::Error>>> = vec![];

    let chunks: usize = (length + THREADS - 1) / THREADS;

    for i in 0..THREADS {
        let chunk_start = i * chunks;
        let chunk_end = std::cmp::min((i + 1) * chunks, length);
        let file_path = file_path.to_owned();
        let word_to_find_clone = word_to_find.clone();

        let handle = thread::spawn(move || {
            let mut thread_file = File::open(&file_path)?;
            let mut contents = vec![0_u8; BLOCK_SIZE];
            let mut read_total: usize = 0;

            thread_file
                .seek(SeekFrom::Start(chunk_start as u64))
                .expect("Couldn't seek to position in file");

            while read_total < chunk_end - chunk_start {
                let bytes_to_read = std::cmp::min(BLOCK_SIZE, chunk_end - chunk_start - read_total);
                let read_length = thread_file
                    .read(&mut contents[..bytes_to_read])
                    .expect("Couldn't read file");
                let chunk_text = String::from_utf8_lossy(&contents[..read_length]);
                if chunk_text.contains(&word_to_find_clone) {
                    return Ok(());
                }
                read_total += read_length;
            }

            Ok(())
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(err) = handle.join().unwrap() {
            return Err(err.into());
        } else {
            found = true;
            break;
        }
    }

    Ok(found)
}

pub async fn load_text_files_from_s3(
    combined_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = AsyncFsFile::create(combined_file_path).await?;
    for i in 1..=1000 {
        let file_path = format!(
            "https://diffusion-corpus.s3.eu-west-2.amazonaws.com/{}.txt",
            i
        );
        let content = fetch_file_content(&file_path).await?;
        file.write_all(content.as_ref()).await?;
    }
    file.flush().await?;
    Ok(())
}

pub async fn fetch_file_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        Ok(body)
    } else {
        Err("Failed to fetch URL".into())
    }
}
