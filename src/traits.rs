use std::collections::HashMap;

pub trait TextAnalyzer {
    fn count_word_frequency(&self) -> HashMap<String, usize>;
    fn count_words(&self, word: String) -> u32;
    fn search_word(&self, word: String) -> bool;
}

impl TextAnalyzer for crate::LocalTextAnalyzer {
    fn count_word_frequency(&self) -> HashMap<String, usize> {
        crate::functions::count_word_frequency(&self.combined_file_path)
    }

    fn count_words(&self, _word: String) -> u32 {
        todo!()
    }

    fn search_word(&self, word: String) -> bool {
        crate::functions::search_word_in_file(&self.combined_file_path, &word).unwrap()
    }
}
