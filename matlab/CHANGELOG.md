# Changelog (iq-files MATLAB / Octave)

## [0.1.0] - 2026-11-23

### Añadido
- Primera versión del módulo de MATLAB / GNU Octave.
- Funciones independientes `load_*` y `save_*` para `sc16q11`, `sc16full`, `grcomplex`, `int8`, `cu8`, `cs12`, `wav_iq`, `csv_iq`.
- Soporte para muestras empaquetadas LimeSDR / bladeRF CS12 de 12 bits (`load_cs12.m`, `save_cs12.m`).
- Soporte para muestras sin signo RTL-SDR CU8 con punto cero en 127.5 (`load_cu8.m`, `save_cu8.m`).
- Función unificadora `iq_files.m` con soporte para todos los formatos.
- Carpeta de ejemplos dedicados `examples/` con `example_basic_io.m` y documentación.
- Batería de pruebas unitarias `test_matlab_iq.m`.

