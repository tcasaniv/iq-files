//! # save_cs32
//!
//! Guarda muestras I/Q en formato RAW Enteros de 32 bits con signo (CS32).

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Codifica muestras I/Q a formato RAW CS32 (int32 Little-Endian).
pub fn save_cs32_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 8);
    for &(i, q) in samples {
        let i_val = (i as f64 * 2_147_483_647.0).clamp(-2_147_483_648.0, 2_147_483_647.0).round() as i32;
        let q_val = (q as f64 * 2_147_483_647.0).clamp(-2_147_483_648.0, 2_147_483_647.0).round() as i32;
        bytes.extend_from_slice(&i_val.to_le_bytes());
        bytes.extend_from_slice(&q_val.to_le_bytes());
    }
    bytes
}

/// Guarda muestras I/Q en formato RAW CS32 a archivo en disco.
pub fn save_cs32_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cs32_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato RAW CS32 a archivo en disco.
pub fn save_cs32(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    save_cs32_to_file(path, samples)
}
