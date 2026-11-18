# iq-files (Rust)

Crate para la lectura, escritura, normalización y conversión eficiente de archivos de señales I/Q complejas para Radio Definida por Software (SDR).

## Uso como Crate (crates.io)

Agrega la dependencia a tu `Cargo.toml`:

```toml
[dependencies]
iq-files = "0.1.0"
```

### Ejemplo de API Unificada

```rust
use iq_files::{read_iq_file, write_iq_file, IqFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Leer muestras I/Q en formato bladeRF SC16 Q11
    let (samples, sample_rate) = read_iq_file("signal.bin", IqFormat::RawSc16Q11)?;
    println!("Leídas {} muestras a {} Hz", samples.len(), sample_rate);

    // Guardar en formato GNU Radio float32
    write_iq_file("signal.iq", &samples, sample_rate, IqFormat::RawFloat32)?;
    Ok(())
}
```

## Uso como Archivo Individual Autocontenido

Si no deseas agregar el crate completo como dependencia, puedes copiar cualquiera de los archivos específicos de `src/formats/` directamente a tu proyecto:
- `load_sc16q11.rs` / `save_sc16q11.rs`: para señales de bladeRF.
- `load_grcomplex.rs` / `save_grcomplex.rs`: para señales de GNU Radio.
- `load_sc16full.rs` / `save_sc16full.rs`: para formato de 16 bits a escala completa.
- `load_int8.rs` / `save_int8.rs`: para HackRF y RTL-SDR.
- `load_wav_iq.rs` / `save_wav_iq.rs`: para archivos WAV estéreo I/Q (SDR#, SDR++).
- `load_csv_iq.rs` / `save_csv_iq.rs`: para archivos de texto plano CSV.
