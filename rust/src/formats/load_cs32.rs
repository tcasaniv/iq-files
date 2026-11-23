//! # load_cs32 / save_cs32
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware / Ecosistema típico**: Procesamiento interno DSP en FPGAs (Xilinx / Intel FPGA), etapas post-filtro DDC/DUC en chips SDR avanzados o pipelines numéricos de enteros de alta resolución.
//! - **Especificaciones Técnicas**:
//!   - 32 bits por componente con signo (`int32_t` / `i32` Little-Endian).
//!   - 8 bytes por muestra compleja.
//!   - Rango numérico: `[-2147483648, 2147483647]`.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Lee muestras I/Q en formato RAW CS32 (int32 Little-Endian) desde memoria.
pub fn load_cs32_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let num_samples = bytes.len() / 8;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(8) {
        let i_raw = i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let q_raw = i32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
        let i = (i_raw as f64 / 2_147_483_648.0) as f32;
        let q = (q_raw as f64 / 2_147_483_648.0) as f32;
        samples.push((i, q));
    }
    samples
}

/// Lee muestras I/Q en formato RAW CS32 desde archivo.
pub fn load_cs32_from_file(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(load_cs32_from_memory(&bytes))
}

pub fn load_cs32(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    load_cs32_from_file(path)
}


