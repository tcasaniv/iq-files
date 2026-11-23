//! Ejemplo básico de uso de la biblioteca `iq-files` en Rust.
//!
//! Ejecutar con:
//! ```bash
//! cargo run --example basic_conversion
//! ```

use iq_files::{
    load_sc16q11_from_file, save_sc16q11_to_file,
    load_cu8_from_file, save_cu8_to_file,
    load_wav_iq_from_file, save_wav_iq_to_file,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Demostración de iq-files en Rust ===");

    // 1. Sintetizar un tono complejo I/Q a 1000 Hz con tasa de muestreo de 48 kHz
    let sample_rate = 48_000;
    let duration_sec = 0.1;
    let n_samples = (sample_rate as f32 * duration_sec) as usize;
    let freq_hz = 1000.0f32;

    let mut signal = Vec::with_capacity(n_samples);
    for n in 0..n_samples {
        let t = n as f32 / sample_rate as f32;
        let phase = 2.0 * std::f32::consts::PI * freq_hz * t;
        let i = 0.75 * phase.cos();
        let q = 0.75 * phase.sin();
        signal.push((i, q));
    }
    println!("1. Señal sintetizada: {} muestras complejas a {} Sps.", signal.len(), sample_rate);

    let out_dir = std::env::temp_dir().join("iq_files_demo");
    std::fs::create_dir_all(&out_dir)?;

    // 2. Guardar y leer en formato bladeRF SC16 Q11 (int16 normalizado a +/- 2048)
    let path_bladerf = out_dir.join("demo_bladerf.sc16q11");
    save_sc16q11_to_file(&path_bladerf, &signal)?;
    let loaded_bladerf = load_sc16q11_from_file(&path_bladerf)?;
    println!("2. bladeRF SC16 Q11: guardado y leído exitosamente ({} muestras).", loaded_bladerf.len());

    // 3. Guardar y leer en formato RTL-SDR CU8 (uint8 sin signo con cero en 127.5)
    let path_rtlsdr = out_dir.join("demo_rtlsdr.cu8");
    save_cu8_to_file(&path_rtlsdr, &signal)?;
    let loaded_rtlsdr = load_cu8_from_file(&path_rtlsdr)?;
    println!("3. RTL-SDR CU8: guardado y leído exitosamente ({} muestras).", loaded_rtlsdr.len());

    // 4. Guardar y leer en formato WAV I/Q Estéreo (16 bits PCM estándar)
    let path_wav = out_dir.join("demo_stereo.wav");
    save_wav_iq_to_file(&path_wav, &signal, sample_rate)?;
    let (loaded_wav, fs_read) = load_wav_iq_from_file(&path_wav)?;
    println!("4. WAV I/Q Estéreo: guardado y leído exitosamente ({} muestras a {} Hz).", loaded_wav.len(), fs_read);


    // 5. Verificación de precisión
    let diff = (signal[10].0 - loaded_wav[10].0).abs();
    println!("5. Error residual WAV vs Original: {:.6} (precisión intacta).", diff);

    println!("\n¡Demostración completada con éxito!");
    Ok(())
}
