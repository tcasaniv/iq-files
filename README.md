# iq-files

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/iq-files.svg)](https://crates.io/crates/iq-files)
[![PyPI version](https://img.shields.io/pypi/v/iq-files.svg)](https://pypi.org/project/iq-files/)

Monorepo con herramientas, bibliotecas y funciones independientes para leer, escribir y convertir archivos de señales I/Q (en fase y cuadratura) en múltiples lenguajes (**Rust**, **Python**, **MATLAB / GNU Octave**).

Permite utilizar la biblioteca unificada completa a través de gestores de paquetes (`cargo`, `pip`) o simplemente copiar un único archivo de función independiente (`load_*.m`, `save_*.py`, etc.) directamente en tu proyecto local sin dependencias externas.

---

## Estructura del Repositorio

- **[`rust/`](rust/)**: Crate para Rust (`iq-files` en crates.io). Proporciona lectura y escritura de alta velocidad, normalización de muestras y API unificada o funciones individuales `load_*` / `save_*`.
- **[`python/`](python/)**: Paquete para Python (`iq-files` en PyPI) y scripts independientes en `standalone/`. Soporte completo con NumPy.
- **[`matlab/`](matlab/)**: Funciones independientes para MATLAB y GNU Octave (`load_sc16q11.m`, `save_sc16q11.m`, etc.) y función integradora `iq_files.m`.

---

## Formatos Soportados

| Formato | Identificador | Tipo de Dato | Tamaño/Muestra | Rango / Escala | Software Compatible |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **RAW SC16 Q11** | `sc16q11` | `int16_t` LE | 4 bytes (2B I + 2B Q) | $[-2048, 2047]$ ($\div 2048$) | bladeRF-cli, libbladeRF |
| **RAW SC16 Estándar** | `sc16full` | `int16_t` LE | 4 bytes (2B I + 2B Q) | $[-32768, 32767]$ ($\div 32768$) | SDRangel, URH, GNU Radio (Short) |
| **RAW Float32** | `grcomplex` / `float32` | `float` (IEEE 754) | 8 bytes (4B I + 4B Q) | $[-1.0, 1.0]$ | GNU Radio (`complex64`), SigMF, Inspectrum |
| **RAW Int8** | `int8` | `int8_t` | 2 bytes (1B I + 1B Q) | $[-128, 127]$ ($\div 128$) | HackRF (`hackrf_transfer`), RTL-SDR |
| **WAV I/Q Estéreo** | `wav_iq` | Contenedor WAV 16-bit | 4 bytes PCM (Ch1: I, Ch2: Q) | $[-32768, 32767]$ | SDR#, SDR++, Gqrx, Audacity |
| **WAV Audio Mono** | `wav_mono` | Contenedor WAV 16-bit | 2 bytes PCM (Mono) | $[-32768, 32767]$ | Reproductores de audio, subportadoras |
| **CSV I/Q** | `csv_iq` | Texto plano (ASCII) | Variable (I, Q por línea) | $[-1.0, 1.0]$ | Análisis numérico, hojas de cálculo |

---

## Licencia

Distribuido bajo la licencia **GNU General Public License v3.0 o posterior (GPL-3.0-or-later)**. Consulte el archivo [LICENSE](LICENSE) para más detalles.

Autor: **Terry Joel Casani Valencia**
Repositorio: [https://github.com/tcasaniv/iq-files](https://github.com/tcasaniv/iq-files)
