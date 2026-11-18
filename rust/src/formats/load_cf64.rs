//! # load_cf64 / save_cf64
//!
//! ## ¿Cuándo usar este formato?
//! - **Ecosistema típico**: Radioastronomía (capturas de radiotelescopios, interferometría VLBI), simulaciones numéricas de altísima fidelidad y procesamiento matemático avanzado con `complex128` en Python (NumPy) o `double` en MATLAB.
//! - **Especificaciones Técnicas**:
//!   - Flotante IEEE 754 de doble precisión (64 bits por componente / 8 bytes por componente).
//!   - 16 bytes por muestra compleja.
//!   - Rango nominal normalizado `[-1.0, 1.0]`.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Lee muestras I/Q en formato RAW CF64 (double precision float64) desde memoria.
pub fn load_cf64_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let num_samples = bytes.len() / 16;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(16) {
        let i_raw = f64::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ]);
        let q_raw = f64::from_le_bytes([
            chunk[8], chunk[9], chunk[10], chunk[11], chunk[12], chunk[13], chunk[14], chunk[15],
        ]);
        samples.push((i_raw as f32, q_raw as f32));
    }
    samples
}

/// Lee muestras I/Q en formato RAW CF64 desde archivo.
pub fn load_cf64_from_file(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(load_cf64_from_memory(&bytes))
}

/// Codifica muestras I/Q a formato RAW CF64 (double precision float64).
pub fn save_cf64_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 16);
    for &(i, q) in samples {
        let i_f64 = i as f64;
        let q_f64 = q as f64;
        bytes.extend_from_slice(&i_f64.to_le_bytes());
        bytes.extend_from_slice(&q_f64.to_le_bytes());
    }
    bytes
}

/// Guarda muestras I/Q en formato RAW CF64 a archivo.
pub fn save_cf64_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cf64_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}
