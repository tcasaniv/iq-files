use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::Path;

/// Guarda muestras complejas I/Q en un archivo binario RAW SC16 Full Scale (int16 estándar).
/// Rango de normalización: [-1.0, 1.0] mapeado a enteros [-32768, 32767].
pub fn save_sc16full(path: &Path, samples: &[(f32, f32)]) -> IoResult<()> {
    let bytes = save_sc16full_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Codifica muestras complejas I/Q en un búfer en memoria en formato RAW SC16 Full Scale.
pub fn save_sc16full_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(samples.len() * 4);
    for &(i, q) in samples {
        let i_val = (i * 32767.0).clamp(-32768.0, 32767.0) as i16;
        let q_val = (q * 32767.0).clamp(-32768.0, 32767.0) as i16;
        buf.extend_from_slice(&i_val.to_le_bytes());
        buf.extend_from_slice(&q_val.to_le_bytes());
    }
    buf
}

/// Alias explícito para guardado a archivo en disco.
pub fn save_sc16full_to_file(path: &Path, samples: &[(f32, f32)]) -> IoResult<()> {
    save_sc16full(path, samples)
}

