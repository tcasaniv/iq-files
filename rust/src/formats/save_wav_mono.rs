use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

/// Guarda el canal I de muestras complejas en un archivo WAV mono (1 canal).
pub fn save_wav_mono(path: &Path, samples: &[(f32, f32)], sample_rate: u32) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = save_wav_mono_to_memory(samples, sample_rate)?;
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

/// Codifica el canal I en un búfer en memoria WAV mono (1 canal).
pub fn save_wav_mono_to_memory(samples: &[(f32, f32)], sample_rate: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut buf = Vec::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut writer = WavWriter::new(cursor, spec)?;
        for &(i, _) in samples {
            let i_s = (i * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer.write_sample(i_s)?;
        }
        writer.finalize()?;
    }
    Ok(buf)
}
