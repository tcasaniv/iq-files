//! # save_sigmf
//!
//! Guarda muestras I/Q y metadatos JSON bajo el estándar SigMF (.sigmf-data + .sigmf-meta).

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Codifica las muestras en el stream binario cf32_le estándar de SigMF.
pub fn save_sigmf_to_memory(samples: &[(f32, f32)]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 8);
    for &(i, q) in samples {
        bytes.extend_from_slice(&i.to_le_bytes());
        bytes.extend_from_slice(&q.to_le_bytes());
    }
    bytes
}

/// Guarda un par SigMF (.sigmf-data binario cf32_le y .sigmf-meta JSON).
pub fn save_sigmf_to_file(
    data_path: &Path,
    samples: &[(f32, f32)],
    sample_rate: u32,
    center_frequency_hz: Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let actual_data_path = if data_path.extension().and_then(|e| e.to_str()) == Some("sigmf-meta") {
        data_path.with_extension("sigmf-data")
    } else {
        data_path.to_path_buf()
    };
    let meta_path = actual_data_path.with_extension("sigmf-meta");

    let bytes = save_sigmf_to_memory(samples);
    let mut data_file = File::create(&actual_data_path)?;
    data_file.write_all(&bytes)?;

    let freq = center_frequency_hz.unwrap_or(433_920_000.0);
    let meta_json = format!(
        r#"{{
    "global": {{
        "core:datatype": "cf32_le",
        "core:sample_rate": {},
        "core:version": "1.0.0",
        "core:description": "Generado por Signals SDR / iq-files",
        "core:recorder": "Signals SDR"
    }},
    "captures": [
        {{
            "core:sample_start": 0,
            "core:frequency": {}
        }}
    ],
    "annotations": []
}}"#,
        sample_rate, freq
    );

    let mut meta_file = File::create(&meta_path)?;
    meta_file.write_all(meta_json.as_bytes())?;

    Ok(())
}

/// Guarda un par SigMF (.sigmf-data binario cf32_le y .sigmf-meta JSON).
pub fn save_sigmf(
    data_path: &Path,
    samples: &[(f32, f32)],
    sample_rate: u32,
    center_frequency_hz: Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    save_sigmf_to_file(data_path, samples, sample_rate, center_frequency_hz)
}
