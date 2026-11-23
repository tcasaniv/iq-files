//! # save_cf64
//!
//! Guarda muestras I/Q en formato RAW Punto Flotante de Doble Precisión (Double-precision float64).

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Codifica muestras I/Q a formato RAW CF64 (double precision float64).
pub fn save_cf64_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 16);
    for &(i, q) in samples {
        let i_f64 = i as f64;
        let q_f64 = q as f64;
        bytes.extend_from_slice(&i_f64.to_le_bytes());
        bytes.extend_from_slice(&q_f64.to_le_bytes());
    }
    bytes
}

/// Guarda muestras I/Q en formato RAW CF64 a archivo en disco.
pub fn save_cf64_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cf64_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato RAW CF64 a archivo en disco.
pub fn save_cf64(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    save_cf64_to_file(path, samples)
}
