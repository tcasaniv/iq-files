//! # save_wav_rf64
//!
//! Guarda muestras I/Q en formato WAV RF64 de 64 bits para archivos de gran capacidad (> 4 GB).

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Guarda muestras I/Q en un contenedor RF64 con chunk ds64 en memoria.
pub fn save_wav_rf64_to_memory(
    samples: &[(f32, f32)],
    sample_rate: u32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data_bytes_len = (samples.len() * 4) as u64;
    // ds64 chunk: 8 bytes id + 4 bytes size (28) + 8 RIFF size + 8 data size + 8 sample count + 4 table len = 36 bytes
    // fmt chunk: 8 + 16 = 24 bytes
    // data chunk: 8 + data_bytes_len
    let total_file_size = 12 + 36 + 24 + 8 + data_bytes_len;

    let mut out = Vec::new();
    // Cabecera RF64
    out.extend_from_slice(b"RF64");
    out.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes()); // Marcador estándar indicando leer ds64
    out.extend_from_slice(b"WAVE");

    // Chunk 'ds64'
    out.extend_from_slice(b"ds64");
    out.extend_from_slice(&28u32.to_le_bytes()); // tamaño de ds64
    out.extend_from_slice(&(total_file_size - 8).to_le_bytes()); // riffSize64
    out.extend_from_slice(&data_bytes_len.to_le_bytes()); // dataSize64
    out.extend_from_slice(&(samples.len() as u64).to_le_bytes()); // sampleCount64
    out.extend_from_slice(&0u32.to_le_bytes()); // tableLength

    // Chunk 'fmt '
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat PCM = 1
    out.extend_from_slice(&2u16.to_le_bytes()); // Channels = 2
    out.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate * 4;
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes()); // BlockAlign = 4
    out.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample = 16

    // Chunk 'data'
    out.extend_from_slice(b"data");
    out.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes()); // Indicador estándar en RF64

    for &(i, q) in samples {
        let i_i16 = (i * 32767.0).clamp(-32768.0, 32767.0).round() as i16;
        let q_i16 = (q * 32767.0).clamp(-32768.0, 32767.0).round() as i16;
        out.extend_from_slice(&i_i16.to_le_bytes());
        out.extend_from_slice(&q_i16.to_le_bytes());
    }

    Ok(out)
}

/// Guarda muestras I/Q en formato WAV RF64 a un archivo en disco.
pub fn save_wav_rf64_to_file(path: &Path, samples: &[(f32, f32)], sample_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = save_wav_rf64_to_memory(samples, sample_rate)?;
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Guarda muestras I/Q en formato WAV RF64 a un archivo en disco.
pub fn save_wav_rf64(path: &Path, samples: &[(f32, f32)], sample_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    save_wav_rf64_to_file(path, samples, sample_rate)
}
