//! # load_cu8
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware típico**: Receptores **RTL-SDR** (DVB-T dongles basados en Realtek RTL2832U, p. ej. RTL-SDR Blog V3/V4).
//! - **Software emisor**: Utilidades de línea de comandos como `rtl_sdr`, `rtl_tcp` o bloques de captura rápida USB.
//! - **Comando típico de captura**:
//!   ```bash
//!   rtl_sdr -f 433.92M -s 2048000 -g 30 captura.cu8
//!   ```
//!
//! ## Especificaciones Técnicas
//! - **Tipo de dato primitivo**: Enteros de 8 bits sin signo (`uint8_t` / `u8`).
//! - **Entrelazado**: Muestras I y Q continuas: `[I0, Q0, I1, Q1, ...]`.
//! - **Bytes por muestra compleja**: 2 bytes (1 byte para I, 1 byte para Q).
//! - **Punto cero**: `127.5` (el valor 127 representa ~ -0.0039, y 128 representa ~ +0.0039).
//! - **Normalización a float [-1.0, 1.0]**: `val_f32 = ((val_u8 as f32) - 127.5) / 127.5`.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Lee muestras I/Q en formato RAW CU8 (uint8 con cero en 127.5) desde un búfer en memoria.
pub fn load_cu8_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let num_samples = bytes.len() / 2;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(2) {
        let i = ((chunk[0] as f32) - 127.5) / 127.5;
        let q = ((chunk[1] as f32) - 127.5) / 127.5;
        samples.push((i, q));
    }
    samples
}

/// Lee muestras I/Q en formato RAW CU8 desde un archivo en disco.
pub fn load_cu8_from_file(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(load_cu8_from_memory(&bytes))
}

/// Lee muestras I/Q en formato RAW CU8 desde un archivo en disco.
pub fn load_cu8(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    load_cu8_from_file(path)
}


