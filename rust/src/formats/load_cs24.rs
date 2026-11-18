//! # load_cs24 / save_cs24
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware típico**: Receptores SDR de muy alto rango dinámico (HF / VLF), digitalizadores de instrumentación científica y transceptores de audio I/Q.
//! - **Especificaciones Técnicas**:
//!   - 24 bits por componente con signo (3 bytes por componente, 6 bytes por muestra compleja).
//!   - Rango numérico: `[-8388608, 8388607]`.
//!   - Little-Endian.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Lee muestras I/Q en formato RAW CS24 (int24 Little-Endian) desde memoria.
pub fn load_cs24_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let num_samples = bytes.len() / 6;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(6) {
        let mut i_raw = (chunk[0] as i32) | ((chunk[1] as i32) << 8) | ((chunk[2] as i32) << 16);
        let mut q_raw = (chunk[3] as i32) | ((chunk[4] as i32) << 8) | ((chunk[5] as i32) << 16);

        // Sign extension de 24 bits a 32 bits
        if (i_raw & 0x0080_0000) != 0 {
            i_raw |= !0x00FF_FFFF;
        }
        if (q_raw & 0x0080_0000) != 0 {
            q_raw |= !0x00FF_FFFF;
        }

        let i = (i_raw as f32) / 8_388_608.0;
        let q = (q_raw as f32) / 8_388_608.0;
        samples.push((i, q));
    }
    samples
}

/// Lee muestras I/Q en formato RAW CS24 desde archivo.
pub fn load_cs24_from_file(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(load_cs24_from_memory(&bytes))
}

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

/// Guarda muestras I/Q en formato RAW CS24 a archivo.
pub fn save_cs24_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cs24_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}
