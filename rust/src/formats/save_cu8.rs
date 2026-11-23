//! # save_cu8
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware típico**: Emulación de streams de entrada para **RTL-SDR** y herramientas que esperan capturas `rtl_sdr` o `rtl_tcp`.
//! - **Uso**: Exportar señales sintéticas para probar pipelines o demoduladores diseñados para dongles RTL2832U.
//!
//! ## Especificaciones Técnicas
//! - **Tipo de dato**: `uint8` (`u8`).
//! - **Entrelazado**: `[I0, Q0, I1, Q1, ...]`.
//! - **Conversión desde float [-1.0, 1.0]**: `val_u8 = ((val_f32 * 127.5) + 127.5).clamp(0.0, 255.0) as u8`.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Codifica muestras I/Q en formato RAW CU8 (uint8 sin signo con cero en 127.5).
pub fn save_cu8_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for &(i, q) in samples {
        let i_u8 = ((i * 127.5) + 127.5).clamp(0.0, 255.0) as u8;
        let q_u8 = ((q * 127.5) + 127.5).clamp(0.0, 255.0) as u8;
        bytes.push(i_u8);
        bytes.push(q_u8);
    }
    bytes
}

/// Guarda muestras I/Q en formato RAW CU8 a un archivo en disco.
pub fn save_cu8_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cu8_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato RAW CU8 a un archivo en disco.
pub fn save_cu8(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    save_cu8_to_file(path, samples)
}


