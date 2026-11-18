//! # load_cs12 / save_cs12
//!
//! ## ¿Cuándo usar este formato?
//! - **Hardware típico**: Dispositivos **LimeSDR** (LMS7002M) y **bladeRF** en modo empaquetado de 12 bits (*12-bit packed*).
//! - **Motivo de diseño**: Ahorra un 25% de ancho de banda en el bus USB 2.0/3.0 al transmitir 2 componentes de 12 bits en 3 bytes en vez de 4.
//! - **Bytes por muestra compleja**: 3 bytes por par (I, Q).
//! - **Estructura de bytes**:
//!   - Byte 0: `I[7:0]`
//!   - Byte 1: `I[11:8]` (bits 0..3) | `Q[3:0]` (bits 4..7)
//!   - Byte 2: `Q[11:4]`
//! - **Rango numérico**: Signed 12 bits `[-2048, 2047]`. Cero en `0.0`.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Decodifica muestras I/Q en formato CS12 packed (3 bytes por par) desde memoria.
pub fn load_cs12_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let num_samples = bytes.len() / 3;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(3) {
        let b0 = chunk[0] as i16;
        let b1 = chunk[1] as i16;
        let b2 = chunk[2] as i16;

        let mut i_raw = (b0 | ((b1 & 0x0F) << 8)) as i16;
        let mut q_raw = (((b1 >> 4) & 0x0F) | (b2 << 4)) as i16;

        // Sign extension de 12 bits a 16 bits
        if (i_raw & 0x0800) != 0 {
            i_raw |= !0x0FFF;
        }
        if (q_raw & 0x0800) != 0 {
            q_raw |= !0x0FFF;
        }

        let i = (i_raw as f32) / 2048.0;
        let q = (q_raw as f32) / 2048.0;
        samples.push((i, q));
    }
    samples
}

/// Lee muestras I/Q en formato CS12 packed desde un archivo.
pub fn load_cs12_from_file(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(load_cs12_from_memory(&bytes))
}

/// Codifica muestras I/Q a CS12 packed (3 bytes por par).
pub fn save_cs12_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 3);
    for &(i, q) in samples {
        let i_val = (i * 2047.0).clamp(-2048.0, 2047.0).round() as i16;
        let q_val = (q * 2047.0).clamp(-2048.0, 2047.0).round() as i16;

        let i_u12 = (i_val as u16) & 0x0FFF;
        let q_u12 = (q_val as u16) & 0x0FFF;

        let b0 = (i_u12 & 0x00FF) as u8;
        let b1 = (((i_u12 >> 8) & 0x0F) | ((q_u12 & 0x0F) << 4)) as u8;
        let b2 = ((q_u12 >> 4) & 0xFF) as u8;

        bytes.push(b0);
        bytes.push(b1);
        bytes.push(b2);
    }
    bytes
}

/// Guarda muestras I/Q en formato CS12 packed a un archivo.
pub fn save_cs12_to_file(path: &Path, samples: &[(f32, f32)]) -> Result<(), std::io::Error> {
    let bytes = save_cs12_to_memory(samples);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}
