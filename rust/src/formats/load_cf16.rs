//! # load_cf16 / save_cf16
//!
//! ## ¿Cuándo usar este formato?
//! - **Ecosistema típico**: Entornos de Machine Learning, Deep Learning (PyTorch, TensorFlow, TensorRT), inferencia en GPUs con núcleos Tensor y compresión de capturas RF para almacenamiento intermedio sin pérdida perceptible.
//! - **Especificaciones Técnicas**:
//!   - Flotante IEEE 754 de media precisión (Half-precision float16 / 2 bytes por componente).
//!   - 4 bytes por muestra compleja.
//!   - Rango nominal normalizado `[-1.0, 1.0]`.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Convierte un u16 que representa un IEEE 754 binary16 (half precision) a f32.
fn half_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) & 1) as u32;
    let exp = ((h >> 10) & 0x1F) as u32;
    let mant = (h & 0x3FF) as u32;

    if exp == 0 {
        if mant == 0 {
            // Cero con signo
            f32::from_bits(sign << 31)
        } else {
            // Subnormal
            let mut m = mant;
            let mut e = 0;
            while (m & 0x400) == 0 {
                m <<= 1;
                e += 1;
            }
            m &= 0x3FF;
            let exp_f = 127 - 15 + 1 - e;
            f32::from_bits((sign << 31) | (exp_f << 23) | (m << 13))
        }
    } else if exp == 31 {
        if mant == 0 {
            // Infinito
            f32::from_bits((sign << 31) | (0xFF << 23))
        } else {
            // NaN
            f32::from_bits((sign << 31) | (0xFF << 23) | (mant << 13))
        }
    } else {
        // Normalizado
        let exp_f = exp + 127 - 15;
        f32::from_bits((sign << 31) | (exp_f << 23) | (mant << 13))
    }
}

/// Convierte un f32 al formato IEEE 754 binary16 (half precision u16).
pub fn f32_to_half(val: f32) -> u16 {
    let bits = val.to_bits();
    let sign = ((bits >> 31) & 1) as u16;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let mant = (bits & 0x7F_FFFF) as u32;

    if exp == 0xFF {
        // Inf o NaN
        let h_mant = if mant != 0 { 0x200 } else { 0 };
        return (sign << 15) | 0x7C00 | h_mant;
    }

    let new_exp = exp - 127 + 15;
    if new_exp >= 31 {
        // Overflow a infinito
        return (sign << 15) | 0x7C00;
    }
    if new_exp <= 0 {
        // Subnormal o bajo flujo
        if new_exp < -10 {
            return sign << 15;
        }
        let full_mant = mant | 0x80_0000;
        let shift = (14 - new_exp) as u32;
        let h_mant = (full_mant >> shift) as u16;
        return (sign << 15) | h_mant;
    }

    let h_mant = (mant >> 13) as u16;
    (sign << 15) | ((new_exp as u16) << 10) | h_mant
}

/// Lee muestras I/Q en formato RAW CF16 (half-precision float16) desde memoria.
pub fn load_cf16_from_memory(bytes: &[u8]) -> Vec<(f32, f32)> {
    let num_samples = bytes.len() / 4;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(4) {
        let i_bits = u16::from_le_bytes([chunk[0], chunk[1]]);
        let q_bits = u16::from_le_bytes([chunk[2], chunk[3]]);
        let i = half_to_f32(i_bits);
        let q = half_to_f32(q_bits);
        samples.push((i, q));
    }
    samples
}

/// Lee muestras I/Q en formato RAW CF16 desde archivo.
pub fn load_cf16_from_file(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(load_cf16_from_memory(&bytes))
}

pub fn load_cf16(path: &Path) -> Result<Vec<(f32, f32)>, std::io::Error> {
    load_cf16_from_file(path)
}


