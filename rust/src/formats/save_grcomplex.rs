use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::Path;

/// Guarda muestras complejas I/Q en un archivo binario RAW Float32 / GNU Radio complex (`grcomplex`).
/// Formato IEEE 754 de 32 bits Little-Endian entrelazado [I0, Q0, I1, Q1, ...].
pub fn save_grcomplex(path: &Path, samples: &[(f32, f32)]) -> IoResult<()> {
    let bytes = save_grcomplex_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Codifica muestras complejas I/Q en un búfer en memoria en formato RAW Float32.
pub fn save_grcomplex_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(samples.len() * 8);
    for &(i, q) in samples {
        buf.extend_from_slice(&i.to_le_bytes());
        buf.extend_from_slice(&q.to_le_bytes());
    }
    buf
}
