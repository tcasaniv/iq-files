//! # save_cs24
//!
//! Guarda muestras I/Q en formato RAW Enteros de 24 bits con signo (CS24).

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Codifica muestras I/Q a formato RAW CS24 (int24 Little-Endian).
pub fn save_cs24_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 6);
    for &(i, q) in samples {
        let i_val = (i * 8_388_607.0).clamp(-8_388_608.0, 8_388_607.0).round() as i32;
        let q_val = (q * 8_388_607.0).clamp(-8_388_608.0, 8_388_607.0).round() as i32;

        let i_u24 = (i_val as u32) & 0x00FF_FFFF;
        let q_u24 = (q_val as u32) & 0x00FF_FFFF;

        bytes.push((i_u24 & 0xFF) as u8);
        bytes.push(((i_u24 >> 8) & 0xFF) as u8);
        bytes.push(((i_u24 >> 16) & 0xFF) as u8);

        bytes.push((q_u24 & 0xFF) as u8);
        bytes.push(((q_u24 >> 8) & 0xFF) as u8);
        bytes.push(((q_u24 >> 16) & 0xFF) as u8);
    }
    bytes
}

/// Guarda muestras I/Q en formato RAW CS24 a archivo en disco.
pub fn save_cs24_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cs24_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato RAW CS24 a archivo en disco.
pub fn save_cs24(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    save_cs24_to_file(path, samples)
}
