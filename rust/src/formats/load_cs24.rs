//! # load_cs24 / save_cs24
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware típico**: Receptores SDR de muy alto rango dinámico (HF / VLF), digitalizadores de instrumentación científica y transceptores de audio I/Q.
//! - **Especificaciones Técnicas**:
//!   - 24 bits por componente con signo (3 bytes por componente, 6 bytes por muestra compleja).
//!   - Rango numérico: `[-8388608, 8388607]`.
//!   - Little-Endian.

use std::fs::File;
use std::io::Read;
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

pub fn load_cs24(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    load_cs24_from_file(path)
}


