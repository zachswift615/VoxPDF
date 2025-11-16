// VoxPDF - Cross-platform PDF text extraction for text-to-speech
// Copyright (C) 2025 VoxPDF Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Caching layer for extraction results to avoid re-extraction

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::models::{Word, Paragraph};

pub struct ExtractionCache {
    word_cache: Arc<RwLock<HashMap<(String, u32), Vec<Word>>>>, // (doc_path, page) -> words
    paragraph_cache: Arc<RwLock<HashMap<(String, u32), Vec<Paragraph>>>>,
}

impl ExtractionCache {
    pub fn new() -> Self {
        Self {
            word_cache: Arc::new(RwLock::new(HashMap::new())),
            paragraph_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_words(&self, doc_path: &str, page: u32) -> Option<Vec<Word>> {
        self.word_cache
            .read()
            .ok()?
            .get(&(doc_path.to_string(), page))
            .cloned()
    }

    pub fn set_words(&self, doc_path: String, page: u32, words: Vec<Word>) {
        if let Ok(mut cache) = self.word_cache.write() {
            cache.insert((doc_path, page), words);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut cache) = self.word_cache.write() {
            cache.clear();
        }
        if let Ok(mut cache) = self.paragraph_cache.write() {
            cache.clear();
        }
    }
}
