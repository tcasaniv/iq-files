# iq-files (Python)

Biblioteca y scripts modulares para leer y guardar archivos de señales I/Q complejas para Radio Definida por Software (SDR) en Python con NumPy y SciPy.

## Formas de Instalación y Uso

### 1. Instalación Oficial desde PyPI
La forma estándar y recomendada para usar en cualquier proyecto o entorno virtual (`venv`, `conda`, `poetry`):

```bash
pip install iq-files
```

### 2. Instalación Directa desde el Repositorio de GitHub
Si deseas instalar directamente la última versión desde el repositorio oficial:

```bash
pip install "iq-files @ git+https://github.com/tcasaniv/iq-files.git#subdirectory=python"
```

O clonando el repositorio localmente en modo editable (para desarrollo):
```bash
git clone https://github.com/tcasaniv/iq-files.git
cd iq-files/python
pip install -e .
```

---

### 3. Copiar Scripts Individuales (Modo Standalone sin Paquete)
Si no deseas instalar ningún paquete y solo necesitas dar soporte a un formato específico, puedes copiar directamente el archivo individual que necesites desde la carpeta [`standalone/`](standalone/) (o desde `src/iq_files/`) hacia tu proyecto local:

| Formato SDR | Lectura | Escritura | Hardware / Software Típico |
| :--- | :--- | :--- | :--- |
| **bladeRF SC16 Q11** | `load_sc16q11.py` | `save_sc16q11.py` | Nuand bladeRF-cli, libbladeRF ($\pm 2048$) |
| **RAW SC16 Estándar** | `load_sc16full.py` | `save_sc16full.py` | SDRangel, URH, USRP, PlutoSDR ($\pm 32768$) |
| **GNU Radio Float32** | `load_grcomplex.py` | `save_grcomplex.py` | GNU Radio (`complex64`), SigMF, Gqrx |
| **HackRF / RTL-SDR Int8** | `load_int8.py` | `save_int8.py` | HackRF (`hackrf_transfer`) signed 8-bit |
| **RTL-SDR CU8 (uint8)** | `load_cu8.py` | `save_cu8.py` | RTL-SDR (`rtl_sdr`, `rtl_tcp`) unsigned 8-bit |
| **LimeSDR / bladeRF CS12** | `load_cs12.py` | `save_cs12.py` | LimeSDR, bladeRF 12-bit packed (3 bytes/muestra) |
| **WAV I/Q Estéreo 16-bit** | `load_wav_iq.py` | `save_wav_iq.py` | SDR#, SDR++, HDSDR, Audacity |
| **CSV Texto Plano I/Q** | `load_csv_iq.py` | `save_csv_iq.py` | Hojas de cálculo, MATLAB, depuración visual |

---

## Ejemplos de Uso

### Uso de la API Unificada (`read_iq` / `write_iq`)

```python
import iq_files
import numpy as np

# 1. Generar tono complejo de prueba a 48 kHz
fs = 48000
t = np.arange(0, 1.0, 1/fs)
signal = 0.7 * np.exp(1j * 2 * np.pi * 1000 * t).astype(np.complex64)

# 2. Guardar en formato bladeRF SC16 Q11
iq_files.write_iq('captura_bladerf.bin', signal, format='sc16q11')

# 3. Guardar en formato RTL-SDR CU8
iq_files.write_iq('captura_rtlsdr.cu8', signal, format='cu8')

# 4. Guardar en formato WAV I/Q Estéreo
iq_files.write_iq('captura_audio.wav', signal, sample_rate=fs, format='wav_iq')

# 5. Leer señal en formato GNU Radio float32
muestras, tasa = iq_files.read_iq('captura.iq', format='grcomplex')
print(f"Cargadas {len(muestras)} muestras complejas a {tasa} Sps.")
```

### Importación Directa de Funciones Específicas

```python
from iq_files import load_cu8, save_cu8, load_sc16q11

# Leer directamente archivo RTL-SDR
samples = load_cu8("rtl_capture.cu8")

# Guardar en formato bladeRF
save_sc16q11("bladerf_out.bin", samples)
```

---

## Ejecución de Pruebas Unitarias

```bash
cd python
py -m unittest discover tests
```

---

## Licencia

Distribuido bajo la licencia **GNU General Public License v3.0 o posterior (GPL-3.0-or-later)**.
Autor: **Terry Joel Casani Valencia**
Repositorio: [https://github.com/tcasaniv/iq-files](https://github.com/tcasaniv/iq-files)
