# iq-files

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/iq-files.svg)](https://crates.io/crates/iq-files)
[![PyPI version](https://img.shields.io/pypi/v/iq-files.svg)](https://pypi.org/project/iq-files/)

Monorepo con herramientas, bibliotecas y funciones independientes para leer, escribir y convertir archivos de señales I/Q (en fase y cuadratura) en múltiples lenguajes (**Rust**, **Python**, **MATLAB / GNU Octave**).

Permite utilizar la biblioteca unificada completa a través de gestores de paquetes oficiales (`cargo`, `pip`), instalación directa desde GitHub o simplemente copiando un único archivo de función independiente (`load_*.m`, `save_*.py`, etc.) directamente en tu proyecto local sin dependencias externas.

---

## Ecosistemas y Lenguajes Soportados

- **[`rust/`](rust/)**: Crate oficial para Rust ([`iq-files` en crates.io](https://crates.io/crates/iq-files)). Lectura/escritura de alto rendimiento, normalización de muestras y API unificada o funciones individuales `load_*` / `save_*`. Incluye ejemplos en [`rust/examples/`](rust/examples/).
- **[`python/`](python/)**: Paquete para Python ([`iq-files` en PyPI](https://pypi.org/project/iq-files/)), instalable vía `pip install iq-files`, desde GitHub, o mediante scripts autocontenidos en `standalone/`. Soporte completo con NumPy y SciPy. Incluye ejemplos en [`python/examples/`](python/examples/).
- **[`matlab/`](matlab/)**: Funciones independientes para MATLAB y GNU Octave (`load_sc16q11.m`, `save_sc16q11.m`, `load_cu8.m`, `load_cs12.m`, etc.) y función integradora `iq_files.m`. Incluye ejemplos en [`matlab/examples/`](matlab/examples/).

---

## Formatos Soportados

| Formato | Identificador | Tipo de Dato | Tamaño/Muestra | Rango / Escala | Hardware / Software Típico |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **RAW SC16 Q11** | `sc16q11` | `int16_t` LE | 4 bytes (2B I + 2B Q) | $[-2048, 2047]$ ($\div 2048$) | Nuand bladeRF-cli, libbladeRF |
| **RAW SC16 Estándar** | `sc16full` | `int16_t` LE | 4 bytes (2B I + 2B Q) | $[-32768, 32767]$ ($\div 32768$) | SDRangel, URH, USRP, PlutoSDR |
| **RAW Float32** | `grcomplex` / `float32` | `float` (IEEE 754) | 8 bytes (4B I + 4B Q) | $[-1.0, 1.0]$ | GNU Radio (`complex64`), SigMF, Gqrx |
| **RAW Int8** | `int8` | `int8_t` | 2 bytes (1B I + 1B Q) | $[-128, 127]$ ($\div 128$) | HackRF (`hackrf_transfer`), RTL-SDR |
| **RAW CU8 (uint8)** | `cu8` | `uint8_t` | 2 bytes (1B I + 1B Q) | $[0, 255]$ (cero en 127.5) | RTL-SDR (`rtl_sdr`, `rtl_tcp`) |
| **RAW CS12 Packed** | `cs12` | 12-bit empacado | 3 bytes por par (I, Q) | $[-2048, 2047]$ | LimeSDR, bladeRF 12-bit |
| **WAV I/Q Estéreo** | `wav_iq` | Contenedor WAV 16-bit | 4 bytes PCM (Ch1: I, Ch2: Q) | $[-32768, 32767]$ | SDR#, SDR++, HDSDR, Audacity |
| **WAV Audio Mono** | `wav_mono` | Contenedor WAV 16-bit | 2 bytes PCM (Mono) | $[-32768, 32767]$ | Reproductores de audio, subportadoras |
| **WAV RF64** | `wav_rf64` | Contenedor WAV 64-bit | Cabecera RF64 / ds64 | Terabytes ($> 4\text{ GB}$) | SDR#, SDR Console v3, HDSDR |
| **CSV I/Q** | `csv_iq` | Texto plano (ASCII) | Variable (I, Q por línea) | $[-1.0, 1.0]$ | Análisis numérico, hojas de cálculo |

---

## Licencia

Distribuido bajo la licencia **GNU General Public License v3.0 o posterior (GPL-3.0-or-later)**. Consulte el archivo [LICENSE](LICENSE) para más detalles.

Autor: **Terry Joel Casani Valencia**  
Repositorio: [https://github.com/tcasaniv/iq-files](https://github.com/tcasaniv/iq-files)
