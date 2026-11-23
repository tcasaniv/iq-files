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
use std::io::Read;
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

pub fn load_cs12(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    load_cs12_from_file(path)
}


