use hound::WavReader;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

/// Lee muestras complejas I/Q desde un archivo WAV de 2 canales PCM (16 bits).
/// Retorna `(samples, sample_rate)`.
pub fn load_wav_iq(path: &Path) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    load_wav_iq_from_memory(&buffer)
}

/// Decodifica muestras complejas I/Q desde un búfer en memoria WAV de 2 canales.
pub fn load_wav_iq_from_memory(bytes: &[u8]) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(bytes);
    let mut reader = WavReader::new(cursor)?;
    let sample_rate = reader.spec().sample_rate;
    let mut samples = Vec::new();
    let mut i_val = 0.0f32;
    let mut is_i = true;

    for sample in reader.samples::<i16>() {
        let s = (sample? as f32) / 32768.0;
        if is_i {
            i_val = s;
            is_i = false;
        } else {
            samples.push((i_val, s));
            is_i = true;
        }
    }
    Ok((samples, sample_rate))
}

/// Alias explícito para lectura desde archivo en disco.
pub fn load_wav_iq_from_file(path: &Path) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    load_wav_iq(path)
}

