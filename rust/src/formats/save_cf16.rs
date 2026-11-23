//! # save_cf16
//!
//! Guarda muestras I/Q en formato RAW Punto Flotante de Media Precisión (Half-precision float16).

use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::formats::load_cf16::f32_to_half;

/// Codifica muestras I/Q a formato RAW CF16 (half-precision float16).
pub fn save_cf16_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 4);
    for &(i, q) in samples {
        let i_h = f32_to_half(i);
        let q_h = f32_to_half(q);
        bytes.extend_from_slice(&i_h.to_le_bytes());
        bytes.extend_from_slice(&q_h.to_le_bytes());
    }
    bytes
}

/// Guarda muestras I/Q en formato RAW CF16 a archivo en disco.
pub fn save_cf16_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cf16_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato RAW CF16 a archivo en disco.
pub fn save_cf16(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    save_cf16_to_file(path, samples)
}
