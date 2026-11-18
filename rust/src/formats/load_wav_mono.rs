use hound::WavReader;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

/// Lee muestras desde un archivo WAV mono (1 canal).
/// Mapea la muestra real al canal I y asigna 0.0 al canal Q.
pub fn load_wav_mono(path: &Path) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    load_wav_mono_from_memory(&buffer)
}

/// Decodifica muestras desde un búfer en memoria WAV mono (1 canal).
pub fn load_wav_mono_from_memory(bytes: &[u8]) -> Result<(Vec<(f32, f32)>, u32), Box<dyn std::error::Error>> {
    let cursor = Cursor::new(bytes);
    let mut reader = WavReader::new(cursor)?;
    let sample_rate = reader.spec().sample_rate;
    let mut samples = Vec::new();

    for sample in reader.samples::<i16>() {
        let s = (sample? as f32) / 32768.0;
        samples.push((s, 0.0));
    }
    Ok((samples, sample_rate))
}
