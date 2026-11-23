//! # save_cs12
//!
//! Guarda muestras I/Q en formato CS12 Packed (3 bytes por par) para LimeSDR y bladeRF.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Codifica muestras I/Q a CS12 packed (3 bytes por par).
pub fn save_cs12_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 3);
    for &(i, q) in samples {
        let i_val = (i * 2047.0).clamp(-2048.0, 2047.0).round() as i16;
        let q_val = (q * 2047.0).clamp(-2048.0, 2047.0).round() as i16;

        let i_u12 = (i_val as u16) & 0x0FFF;
        let q_u12 = (q_val as u16) & 0x0FFF;

        let b0 = (i_u12 & 0x00FF) as u8;
        let b1 = (((i_u12 >> 8) & 0x0F) | ((q_u12 & 0x0F) << 4)) as u8;
        let b2 = ((q_u12 >> 4) & 0xFF) as u8;

        bytes.push(b0);
        bytes.push(b1);
        bytes.push(b2);
    }
    bytes
}

/// Guarda muestras I/Q en formato CS12 packed a un archivo en disco.
pub fn save_cs12_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cs12_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato CS12 packed a un archivo en disco.
pub fn save_cs12(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    save_cs12_to_file(path, samples)
}
