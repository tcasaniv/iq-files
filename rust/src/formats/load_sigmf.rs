//! # load_sigmf / save_sigmf
//!
//! ## ¿Cuándo usar este formato?
//! - **Ecosistema**: Es el **estándar abierto de referencia** para la comunidad SDR moderna (GNU Radio, Inspectrum, SigMF GUI, repositorios de datasets de RF).
//! - **Estructura**:
//!   - Archivo binario de datos: `<nombre>.sigmf-data` (típicamente `cf32_le` o `cs16_le`).
//!   - Archivo de metadatos JSON: `<nombre>.sigmf-meta` conteniendo `global` (`core:datatype`, `core:sample_rate`, `core:version`) y `captures` (`core:sample_start`, `core:frequency`, `core:datetime`).

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Lee el par SigMF (.sigmf-data y .sigmf-meta si existe) dado el path a cualquiera de los dos o el base.
pub fn load_sigmf_from_file(data_path: &Path) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    // Resolver path a .sigmf-data
    let actual_data_path = if data_path.extension().and_then(|e| e.to_str()) == Some("sigmf-meta") {
        data_path.with_extension("sigmf-data")
    } else {
        data_path.to_path_buf()
    };

    let meta_path = actual_data_path.with_extension("sigmf-meta");

    let mut sample_rate = 1_000_000u32;
    if meta_path.exists() {
        if let Ok(meta_str) = std::fs::read_to_string(&meta_path) {
            // Extracción sencilla de "core:sample_rate": N sin forzar librerías JSON pesadas
            if let Some(pos) = meta_str.find("\"core:sample_rate\"") {
                let rest = &meta_str[pos..];
                if let Some(colon_pos) = rest.find(':') {
                    let num_str = rest[colon_pos + 1..]
                        .chars()
                        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ' ')
                        .collect::<String>();
                    if let Ok(sr) = num_str.trim().parse::<f64>() {
                        sample_rate = sr as u32;
                    }
                }
            }
        }
    }

    let mut file = File::open(&actual_data_path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Por defecto en SigMF estándar se asume cf32_le (8 bytes por muestra I/Q)
    let num_samples = bytes.len() / 8;
    let mut samples = Vec::with_capacity(num_samples);
    for chunk in bytes.chunks_exact(8) {
        let i = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let q = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
        samples.push((i, q));
    }

    Ok((samples, sample_rate))
}

pub fn load_sigmf(data_path: &Path) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    load_sigmf_from_file(data_path)
}


