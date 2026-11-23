# Changelog (iq-files Rust)

## [0.1.0] - 2026-11-23

### Añadido
- Primera versión del crate `iq-files`.
- API unificada `read_iq_file`, `write_iq_file`, `read_iq_memory`, `write_iq_memory`.
- Módulos físicos independientes `load_*` y `save_*` para los 16 formatos I/Q soportados (`cf16`, `cf64`, `cs12`, `cs24`, `cs32`, `csv_iq`, `cu16`, `cu8`, `grcomplex`, `int8`, `sc16full`, `sc16q11`, `sigmf`, `wav_iq`, `wav_mono`, `wav_rf64`).
- Carpeta de ejemplos `examples/` con `basic_conversion.rs` ejecutable vía `cargo run --example basic_conversion`.
- Metadatos descriptivos de formatos (`IqMetadata`).

