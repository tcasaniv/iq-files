# iq-files (Python)

Biblioteca y scripts modulares para leer y guardar archivos de señales I/Q complejas para Radio Definida por Software (SDR) en Python con NumPy.

## Opciones de Uso

### 1. Copiar un Script Individual Autocontenido (Sin Instalación)
Si solo necesitas un formato concreto, puedes copiar el archivo individual que necesites directamente a tu carpeta de trabajo:
- `load_sc16q11.py` / `save_sc16q11.py`: bladeRF SC16 Q11 (int16 normalizado a $\pm 2048$).
- `load_grcomplex.py` / `save_grcomplex.py`: GNU Radio float32 complex (`complex64`).
- `load_sc16full.py` / `save_sc16full.py`: int16 Little-Endian escala completa ($\pm 32768$).
- `load_int8.py` / `save_int8.py`: HackRF / RTL-SDR signed 8-bit ($\pm 128$).
- `load_wav_iq.py` / `save_wav_iq.py`: WAV I/Q estéreo de 16 bits PCM.
- `load_csv_iq.py` / `save_csv_iq.py`: Texto plano CSV con columnas I, Q.

### 2. Instalar la Biblioteca Unificada (`pip install iq-files`) o Copiar `iq_files.py`

```python
import iq_files
import numpy as np

# Generar tono de prueba
fs = 48000
t = np.arange(0, 1.0, 1/fs)
iq = 0.7 * np.exp(1j * 2 * np.pi * 1000 * t).astype(np.complex64)

# Guardar en formato bladeRF
iq_files.write_iq('signal.bin', iq, sample_rate=fs, format='sc16q11')

# Leer señal en formato GNU Radio float32
samples, rate = iq_files.read_iq('signal.iq', format='grcomplex')
```
