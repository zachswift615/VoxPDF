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

use voxpdf_core::cache::ExtractionCache;
use voxpdf_core::models::{Word, Rect};

#[test]
fn test_cache_operations() {
    let cache = ExtractionCache::new();
    let word = Word::new("test".to_string(), Rect::new(0.0, 0.0, 10.0, 10.0), 0, 12.0);

    // Test empty cache
    assert!(cache.get_words("doc.pdf", 0).is_none());

    // Test set and get
    cache.set_words("doc.pdf".to_string(), 0, vec![word.clone()]);
    let cached = cache.get_words("doc.pdf", 0).unwrap();
    assert_eq!(cached.len(), 1);
    assert_eq!(cached[0].text, "test");

    // Test clear
    cache.clear();
    assert!(cache.get_words("doc.pdf", 0).is_none());
}
