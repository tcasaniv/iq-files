# Changelog

Todos los cambios notables en este proyecto serán documentados en este archivo.

El formato se basa en [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/)
y este proyecto se adhiere a [Semantic Versioning](https://semver.org/lang/es/).

---

## [0.1.0] - 2026-11-18

### Añadido (Features)
- Lanzamiento inicial del monorepo `iq-files` multilingüe (Rust, Python, MATLAB/GNU Octave).
- Implementación de módulos independientes `load_*` y `save_*` para cada formato I/Q:
  - RAW SC16 Q11 (bladeRF).
  - RAW SC16 Full Scale (int16 estándar).
  - RAW Float32 / GNU Radio complex (`grcomplex`).
  - RAW Int8 (HackRF / signed 8-bit).
  - WAV I/Q estéreo de 16 bits PCM.
  - WAV audio mono de 16 bits PCM.
  - CSV I/Q de texto plano.
- Implementación de bibliotecas unificadas integradoras (`lib.rs` en Rust, `iq_files.py` en Python, `iq_files.m` en MATLAB).
- Baterías de pruebas unitarias y de validación cruzada para todos los lenguajes.
