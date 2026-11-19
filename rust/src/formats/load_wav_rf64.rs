//! # load_wav_rf64 / save_wav_rf64
//!
//! ## ¿Cuándo usar este formato?
//! - **Ecosistema**: Grabaciones continuas de muy larga duración o ancho de banda elevado en **SDR#**, **SDR Console v3** y **HDSDR** que superan el límite de 4 GB de la cabecera estándar RIFF de 32 bits.
//! - **Especificación**: Estándar EBU Tech 3306 (RF64). Utiliza el FourCC `'RF64'` en lugar de `'RIFF'`, y define una tabla `'ds64'` con tamaños de archivo y de bloque de 64 bits.

/// Lee muestras I/Q desde un archivo WAV RF64 o WAV tradicional compatible.
pub fn load_wav_rf64_from_memory(bytes: &[u8]) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    if bytes.len() < 36 {
        return Err("Búfer demasiado corto para cabecera WAV/RF64".into());
    }

    // Comprobar FourCC ("RIFF" o "RF64")
    let is_rf64 = &bytes[0..4] == b"RF64";
    let is_riff = &bytes[0..4] == b"RIFF";
    if !is_rf64 && !is_riff {
        return Err("No es un archivo WAV ni RF64 válido".into());
    }

    if &bytes[8..12] != b"WAVE" {
        return Err("Formato no es WAVE".into());
    }

    let mut pos = 12;
    let mut sample_rate = 288_000u32;
    let mut channels = 2u16;
    let mut bits_per_sample = 16u16;
    let mut data_start = None;
    let mut data_len = None;

    while pos + 8 <= bytes.len() {
        let chunk_id = &bytes[pos..pos + 4];
        let chunk_size = u32::from_le_bytes([bytes[pos + 4], bytes[pos + 5], bytes[pos + 6], bytes[pos + 7]]) as usize;
        let chunk_data_start = pos + 8;

        if chunk_id == b"fmt " && chunk_data_start + 14 <= bytes.len() {
            channels = u16::from_le_bytes([bytes[chunk_data_start + 2], bytes[chunk_data_start + 3]]);
            sample_rate = u32::from_le_bytes([
                bytes[chunk_data_start + 4],
                bytes[chunk_data_start + 5],
                bytes[chunk_data_start + 6],
                bytes[chunk_data_start + 7],
            ]);
            bits_per_sample = u16::from_le_bytes([bytes[chunk_data_start + 14], bytes[chunk_data_start + 15]]);
        } else if chunk_id == b"data" {
            data_start = Some(chunk_data_start);
            data_len = Some(if chunk_size == 0xFFFF_FFFF || chunk_size > bytes.len() - chunk_data_start {
                bytes.len() - chunk_data_start
            } else {
                chunk_size
            });
            break;
        }

        pos = chunk_data_start + chunk_size;
    }

    let start = data_start.ok_or("Chunk 'data' no encontrado en WAV/RF64")?;
    let len = data_len.unwrap_or(bytes.len() - start);
    let pcm_bytes = &bytes[start..start + len];

    let mut samples = Vec::new();
    if bits_per_sample == 16 && channels == 2 {
        let num_samples = pcm_bytes.len() / 4;
        samples.reserve(num_samples);
        for chunk in pcm_bytes.chunks_exact(4) {
            let i = i16::from_le_bytes([chunk[0], chunk[1]]) as f32 / 32768.0;
            let q = i16::from_le_bytes([chunk[2], chunk[3]]) as f32 / 32768.0;
            samples.push((i, q));
        }
    } else if bits_per_sample == 32 && channels == 2 {
        // Soporte para Float32 WAV I/Q
        let num_samples = pcm_bytes.len() / 8;
        samples.reserve(num_samples);
        for chunk in pcm_bytes.chunks_exact(8) {
            let i = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let q = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
            samples.push((i, q));
        }
    } else {
        return Err(format!("Formato no soportado en WAV/RF64: {} channels, {} bits", channels, bits_per_sample).into());
    }

    Ok((samples, sample_rate))
}

/// Guarda muestras I/Q en un contenedor RF64 con chunk `ds64` para soporte de grabaciones arbitrariamente grandes.
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
