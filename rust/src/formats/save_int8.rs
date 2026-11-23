use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::Path;

/// Guarda muestras complejas I/Q en un archivo binario RAW Int8 (HackRF / signed 8-bit).
/// Rango de normalización: [-1.0, 1.0] mapeado a enteros [-128, 127].
pub fn save_int8(path: &Path, samples: &[(f32, f32)]) -> IoResult<()> {
    let bytes = save_int8_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Codifica muestras complejas I/Q en un búfer en memoria en formato RAW Int8.
pub fn save_int8_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(samples.len() * 2);
    for &(i, q) in samples {
        let i_val = (i * 127.0).clamp(-128.0, 127.0) as i8;
        let q_val = (q * 127.0).clamp(-128.0, 127.0) as i8;
        buf.push(i_val as u8);
        buf.push(q_val as u8);
    }
    buf
}

/// Alias explícito para guardado a archivo en disco.
pub fn save_int8_to_file(path: &Path, samples: &[(f32, f32)]) -> IoResult<()> {
    save_int8(path, samples)
}

