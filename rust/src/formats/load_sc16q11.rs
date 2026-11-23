use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

/// Lee muestras complejas I/Q desde un archivo binario RAW SC16 Q11 (bladeRF).
/// Rango de enteros: [-2048, 2047] normalizados a [-1.0, 1.0].
pub fn load_sc16q11(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(load_sc16q11_from_memory(&buffer))
}

/// Decodifica muestras complejas I/Q desde un búfer en memoria en formato RAW SC16 Q11.
pub fn load_sc16q11_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let mut samples = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks_exact(4) {
        let i = i16::from_le_bytes([chunk[0], chunk[1]]) as f32 / 2048.0;
        let q = i16::from_le_bytes([chunk[2], chunk[3]]) as f32 / 2048.0;
        samples.push((i, q));
    }
    samples
}

/// Alias explícito para lectura desde archivo en disco.
pub fn load_sc16q11_from_file(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    load_sc16q11(path)
}

