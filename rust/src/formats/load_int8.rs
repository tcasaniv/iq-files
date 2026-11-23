use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

/// Lee muestras complejas I/Q desde un archivo binario RAW Int8 (HackRF / signed 8-bit).
/// Rango de enteros: [-128, 127] normalizados a [-1.0, 1.0].
pub fn load_int8(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(load_int8_from_memory(&buffer))
}

/// Decodifica muestras complejas I/Q desde un búfer en memoria en formato RAW Int8.
pub fn load_int8_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let mut samples = Vec::with_capacity(bytes.len() / 2);
    for chunk in bytes.chunks_exact(2) {
        let i = (chunk[0] as i8 as f32) / 128.0;
        let q = (chunk[1] as i8 as f32) / 128.0;
        samples.push((i, q));
    }
    samples
}

/// Alias explícito para lectura desde archivo en disco.
pub fn load_int8_from_file(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    load_int8(path)
}

