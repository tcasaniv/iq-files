use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

/// Lee muestras complejas I/Q desde un archivo binario RAW SC16 Full Scale (int16 estándar).
/// Rango de enteros: [-32768, 32767] normalizados a [-1.0, 1.0].
pub fn load_sc16full(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(load_sc16full_from_memory(&buffer))
}

/// Decodifica muestras complejas I/Q desde un búfer en memoria en formato RAW SC16 Full Scale.
pub fn load_sc16full_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let mut samples = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        let i = i16::from_le_bytes([chunk[0], chunk[1]]) as f32 / 32768.0;
        let q = i16::from_le_bytes([chunk[2], chunk[3]]) as f32 / 32768.0;
        samples.push((i, q));
    }
    samples
}
