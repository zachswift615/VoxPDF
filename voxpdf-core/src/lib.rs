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

//! VoxPDF Core - PDF text extraction optimized for text-to-speech

pub mod cache;
pub mod error;
pub mod extraction;
pub mod ffi;
pub mod memory_pool;
pub mod models;
pub mod pdf;

pub use error::{Result, VoxPDFError};
pub use models::{Paragraph, Rect, TocEntry, Word};
pub use pdf::PDFDocument;
