use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

/// Lee muestras complejas I/Q desde un archivo binario RAW Float32 / GNU Radio complex (`grcomplex`).
/// Formato IEEE 754 de 32 bits Little-Endian entrelazado [I0, Q0, I1, Q1, ...].
pub fn load_grcomplex(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(load_grcomplex_from_memory(&buffer))
}

/// Decodifica muestras complejas I/Q desde un búfer en memoria en formato RAW Float32.
pub fn load_grcomplex_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let mut samples = Vec::with_capacity(bytes.len() / 8);
    for chunk in bytes.chunks_exact(8) {
        let i = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let q = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
        samples.push((i, q));
    }
    samples
}

/// Alias explícito para lectura desde archivo en disco.
pub fn load_grcomplex_from_file(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    load_grcomplex(path)
}

