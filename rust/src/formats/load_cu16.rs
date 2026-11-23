//! # load_cu16 / save_cu16
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware típico**: ADCs industriales y sistemas de adquisición embebidos sin signo (Unsigned 16-bit).
//! - **Punto cero**: `32768.0`. Rango numérico `[0, 65535]`.
//! - **Bytes por muestra**: 4 bytes (2 bytes I, 2 bytes Q, Little-Endian).

use std::fs::File;
use std::io::Read;
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

pub fn load_cu16(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    load_cu16_from_file(path)
}


