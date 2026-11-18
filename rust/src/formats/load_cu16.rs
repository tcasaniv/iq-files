//! # load_cu16 / save_cu16
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware típico**: ADCs industriales y sistemas de adquisición embebidos sin signo (Unsigned 16-bit).
//! - **Punto cero**: `32768.0`. Rango numérico `[0, 65535]`.
//! - **Bytes por muestra**: 4 bytes (2 bytes I, 2 bytes Q, Little-Endian).

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Lee muestras I/Q en formato RAW CU16 (uint16 LE con cero en 32768) desde memoria.
pub fn load_cu16_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let num_samples = bytes.len() / 4;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(4) {
        let i_raw = u16::from_le_bytes([chunk[0], chunk[1]]);
        let q_raw = u16::from_le_bytes([chunk[2], chunk[3]]);
        let i = ((i_raw as f32) - 32768.0) / 32768.0;
        let q = ((q_raw as f32) - 32768.0) / 32768.0;
        samples.push((i, q));
    }
    samples
}

/// Lee muestras I/Q en formato RAW CU16 desde un archivo.
pub fn load_cu16_from_file(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(load_cu16_from_memory(&bytes))
}

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

/// Guarda muestras I/Q en formato RAW CU16 a un archivo.
pub fn save_cu16_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cu16_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}
