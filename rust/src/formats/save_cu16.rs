//! # save_cu16
//!
//! Guarda muestras I/Q en formato RAW Enteros de 16 bits sin signo (CU16).

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Codifica muestras I/Q a RAW CU16 (uint16 LE con cero en 32768).
pub fn save_cu16_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 4);
    for &(i, q) in samples {
        let i_u16 = ((i * 32768.0) + 32768.0).clamp(0.0, 65535.0) as u16;
        let q_u16 = ((q * 32768.0) + 32768.0).clamp(0.0, 65535.0) as u16;
        bytes.extend_from_slice(&i_u16.to_le_bytes());
        bytes.extend_from_slice(&q_u16.to_le_bytes());
    }
    bytes
}

/// Guarda muestras I/Q en formato RAW CU16 a un archivo en disco.
pub fn save_cu16_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cu16_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato RAW CU16 a un archivo en disco.
pub fn save_cu16(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    save_cu16_to_file(path, samples)
}
