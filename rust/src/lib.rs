pub mod formats;
pub mod metadata;

pub use formats::*;
pub use metadata::*;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Alias para compatibilidad hacia atrás con el código existente.
pub type IqExportFormat = IqFormat;

/// Lee muestras I/Q desde un archivo en el disco según el formato especificado.
pub fn read_iq_file(
    path: &Path,
    format: IqFormat,
) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    match format {
        IqFormat::SigMf => formats::load_sigmf_from_file(path),
        _ => {
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            read_iq_memory(&buffer, format)
        }
    }
}

/// Guarda muestras I/Q a un archivo en el disco según el formato y tasa de muestreo especificados.
pub fn write_iq_file(
    path: &Path,
    samples: &[(f32, f32)],
    sample_rate: u32,
    format: IqFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        IqFormat::SigMf => formats::save_sigmf_to_file(path, samples, sample_rate, None),
        _ => {
            let bytes = write_iq_memory(samples, sample_rate, format)?;
            let mut file = File::create(path)?;
            file.write_all(&bytes)?;
            Ok(())
        }
    }
}

/// Decodifica muestras I/Q directamente desde un búfer en memoria.
pub fn read_iq_memory(
    bytes: &[u8],
    format: IqFormat,
) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    match format {
        IqFormat::WavIq16 => formats::load_wav_iq_from_memory(bytes),
        IqFormat::WavRf64 => formats::load_wav_rf64_from_memory(bytes),
        IqFormat::SigMf => {
            // Decodificación directa asumiendo cf32_le si viene solo el flujo binario
            let num_samples = bytes.len() / 8;
            let mut samples = Vec::with_capacity(num_samples);
            for chunk in bytes.chunks_exact(8) {
                let i = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                let q = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                samples.push((i, q));
            }
            Ok((samples, 1_000_000))
        }
        IqFormat::RawCu8 => Ok((formats::load_cu8_from_memory(bytes), 2_048_000)),
        IqFormat::RawCu16 => Ok((formats::load_cu16_from_memory(bytes), 1_000_000)),
        IqFormat::RawInt8 => Ok((formats::load_int8_from_memory(bytes), 2_000_000)),
        IqFormat::RawCs12Packed => Ok((formats::load_cs12_from_memory(bytes), 2_000_000)),
        IqFormat::RawSc16Q11 => Ok((formats::load_sc16q11_from_memory(bytes), 288_000)),
        IqFormat::RawSc16Full => Ok((formats::load_sc16full_from_memory(bytes), 288_000)),
        IqFormat::RawCs24 => Ok((formats::load_cs24_from_memory(bytes), 192_000)),
        IqFormat::RawCs32 => Ok((formats::load_cs32_from_memory(bytes), 1_000_000)),
        IqFormat::RawCf16 => Ok((formats::load_cf16_from_memory(bytes), 1_000_000)),
        IqFormat::RawFloat32 => Ok((formats::load_grcomplex_from_memory(bytes), 1_000_000)),
        IqFormat::RawCf64 => Ok((formats::load_cf64_from_memory(bytes), 1_000_000)),
        IqFormat::WavAudioMono => formats::load_wav_mono_from_memory(bytes),
        IqFormat::CsvIq => {
            let text = std::str::from_utf8(bytes)?;
            Ok((formats::load_csv_iq_from_str(text), 288_000))
        }
    }
}

/// Codifica muestras I/Q a bytes en memoria en el formato solicitado.
pub fn write_iq_memory(
    samples: &[(f32, f32)],
    sample_rate: u32,
    format: IqFormat,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match format {
        IqFormat::WavIq16 => formats::save_wav_iq_to_memory(samples, sample_rate),
        IqFormat::WavRf64 => formats::save_wav_rf64_to_memory(samples, sample_rate),
        IqFormat::SigMf => {
            // Serializa el stream de datos cf32_le
            let mut bytes = Vec::with_capacity(samples.len() * 8);
            for &(i, q) in samples {
                bytes.extend_from_slice(&i.to_le_bytes());
                bytes.extend_from_slice(&q.to_le_bytes());
            }
            Ok(bytes)
        }
        IqFormat::RawCu8 => Ok(formats::save_cu8_to_memory(samples)),
        IqFormat::RawCu16 => Ok(formats::save_cu16_to_memory(samples)),
        IqFormat::RawInt8 => Ok(formats::save_int8_to_memory(samples)),
        IqFormat::RawCs12Packed => Ok(formats::save_cs12_to_memory(samples)),
        IqFormat::RawSc16Q11 => Ok(formats::save_sc16q11_to_memory(samples)),
        IqFormat::RawSc16Full => Ok(formats::save_sc16full_to_memory(samples)),
        IqFormat::RawCs24 => Ok(formats::save_cs24_to_memory(samples)),
        IqFormat::RawCs32 => Ok(formats::save_cs32_to_memory(samples)),
        IqFormat::RawCf16 => Ok(formats::save_cf16_to_memory(samples)),
        IqFormat::RawFloat32 => Ok(formats::save_grcomplex_to_memory(samples)),
        IqFormat::RawCf64 => Ok(formats::save_cf64_to_memory(samples)),
        IqFormat::WavAudioMono => formats::save_wav_mono_to_memory(samples, sample_rate),
        IqFormat::CsvIq => Ok(formats::save_csv_iq_to_string(samples).into_bytes()),
    }
}

/// Función alias para compatibilidad directa con Signals SDR.
pub fn export_iq_file(
    path: &Path,
    samples: &[(f32, f32)],
    sample_rate: u32,
    format: IqFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    write_iq_file(path, samples, sample_rate, format)
}

/// Función alias para compatibilidad directa con Signals SDR.
pub fn export_iq_to_memory(
    samples: &[(f32, f32)],
    sample_rate: u32,
    format: IqFormat,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    write_iq_memory(samples, sample_rate, format)
}

/// Función alias para compatibilidad directa con Signals SDR.
pub fn read_iq_from_memory(
    bytes: &[u8],
    format: IqFormat,
) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    read_iq_memory(bytes, format)
}
