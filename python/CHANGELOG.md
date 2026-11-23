# Changelog (iq-files Python)

Todos los cambios notables en este paquete serán documentados en este archivo.

## [0.1.0] - 2026-11-22

### Añadido
- **Publicación Oficial en PyPI**: Empaquetado estándar *src-layout* bajo PEP 517/621 (`pip install iq-files`).
- **Instalación desde GitHub**: Compatibilidad total con instalación directa mediante `pip install git+...#subdirectory=python`.
- **Estructura Modular Híbrida**:
  - API unificada en `iq_files.read_iq` y `iq_files.write_iq`.
  - Submódulos independientes `load_*` y `save_*` disponibles tanto en el paquete como en `standalone/`.
- **Formatos Soportados**:
  - bladeRF SC16 Q11 (`load_sc16q11`, `save_sc16q11`).
  - SC16 Full-Scale (`load_sc16full`, `save_sc16full`).
  - GNU Radio Float32 / CF32 (`load_grcomplex`, `save_grcomplex`).
  - HackRF Int8 (`load_int8`, `save_int8`).
  - RTL-SDR CU8 uint8 (`load_cu8`, `save_cu8`).
  - LimeSDR / bladeRF CS12 packed (`load_cs12`, `save_cs12`).
  - WAV I/Q Estéreo 16-bit PCM (`load_wav_iq`, `save_wav_iq`).
  - CSV Texto Plano I/Q (`load_csv_iq`, `save_csv_iq`).
- **Tipado PEP 561**: Inclusión de marcador `py.typed` para soporte de mypy, pyright y autocompletado en IDEs.
- **Batería de Pruebas**: Tests unitarios con `unittest` cubriendo todos los formatos de datos.
