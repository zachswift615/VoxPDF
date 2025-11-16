use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use crate::models::Paragraph;
use crate::pdf::PDFDocument;
use crate::extraction::{extract_word_positions, detect_paragraphs, reassemble_hyphenated_words};

pub struct StreamingExtractor {
    tx: Sender<ExtractionEvent>,
    rx: Receiver<ExtractionEvent>,
}

pub enum ExtractionEvent {
    PageComplete { page: u32, paragraphs: Vec<Paragraph> },
    Error { page: u32, error: String },
    Complete,
}

impl StreamingExtractor {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }

    pub fn extract_async(&self, doc_path: String, start_page: u32, end_page: u32) {
        let tx = self.tx.clone();

        thread::spawn(move || {
            let doc = match PDFDocument::open(&doc_path) {
                Ok(d) => d,
                Err(e) => {
                    tx.send(ExtractionEvent::Error {
                        page: 0,
                        error: e.to_string()
                    }).ok();
                    return;
                }
            };

            for page in start_page..=end_page {
                match extract_word_positions(&doc, page) {
                    Ok(words) => {
                        let mut paragraphs = detect_paragraphs(words);
                        paragraphs = reassemble_hyphenated_words(paragraphs);
                        tx.send(ExtractionEvent::PageComplete { page, paragraphs }).ok();
                    }
                    Err(e) => {
                        tx.send(ExtractionEvent::Error {
                            page,
                            error: e.to_string()
                        }).ok();
                    }
                }
            }

            tx.send(ExtractionEvent::Complete).ok();
        });
    }

    pub fn receive(&self) -> Option<ExtractionEvent> {
        self.rx.recv().ok()
    }
}
