use crate::error::{Result, VoxPDFError};
use flate2::read::ZlibDecoder;
use lopdf::Object;
use std::io::Read;

/// Decode PDF content stream, handling ASCII85+Flate compression.
///
/// This function works around a lopdf 0.32 bug where ASCII85+Flate filter
/// combinations (common in ReportLab PDFs) fail to decompress. We manually
/// decode ASCII85 first, then apply Flate decompression.
///
/// Falls back to lopdf's native decompression for other filter types.
pub(crate) fn decode_content_stream(doc: &lopdf::Document, contents: &Object) -> Result<String> {
    match contents {
        Object::Reference(ref_id) => {
            let stream_obj = doc.get_object(*ref_id)?;
            if let Ok(stream) = stream_obj.as_stream() {
                let raw_data = &stream.content;

                // Check the filter type
                if let Ok(filter) = stream.dict.get(b"Filter") {
                    if let Object::Array(filters) = filter {
                        // Check if it's ASCII85 + Flate
                        let has_ascii85 = filters.iter().any(|f| {
                            if let Ok(name) = f.as_name_str() {
                                name == "ASCII85Decode"
                            } else {
                                false
                            }
                        });

                        let has_flate = filters.iter().any(|f| {
                            if let Ok(name) = f.as_name_str() {
                                name == "FlateDecode"
                            } else {
                                false
                            }
                        });

                        if has_ascii85 && has_flate {
                            // Manual decode: ASCII85 -> Flate
                            let ascii85_str = String::from_utf8_lossy(raw_data);

                            // Add start marker if needed
                            let to_decode = if ascii85_str.contains("~>") {
                                format!("<~{}", ascii85_str)
                            } else {
                                format!("<~{}~>", ascii85_str)
                            };

                            let decoded_ascii85 = ascii85::decode(&to_decode).map_err(|e| {
                                VoxPDFError::ExtractionError(format!(
                                    "ASCII85 decode error: {:?}",
                                    e
                                ))
                            })?;

                            let mut decoder = ZlibDecoder::new(&decoded_ascii85[..]);
                            let mut decompressed = Vec::new();
                            decoder.read_to_end(&mut decompressed).map_err(|e| {
                                VoxPDFError::ExtractionError(format!("Flate decode error: {:?}", e))
                            })?;

                            return Ok(String::from_utf8_lossy(&decompressed).to_string());
                        }
                    }
                }

                // Fall back to lopdf's decompression
                match stream.decompressed_content() {
                    Ok(data) => Ok(String::from_utf8_lossy(&data).to_string()),
                    Err(e) => Err(VoxPDFError::ExtractionError(format!(
                        "Failed to decompress stream: {:?}",
                        e
                    ))),
                }
            } else {
                Err(VoxPDFError::ExtractionError(
                    "Content is not a stream".to_string(),
                ))
            }
        }
        _ => Err(VoxPDFError::ExtractionError(
            "Unexpected Contents type".to_string(),
        )),
    }
}
