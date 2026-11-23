# iq-files (MATLAB / GNU Octave)

Colección de funciones independientes y modulares para leer y guardar archivos de señales I/Q complejas en MATLAB y GNU Octave.

## Opciones de Uso

Tienes dos formas de utilizar estas funciones en tu proyecto:

### 1. Copiar una Función Individual (Sin Dependencias)
Si únicamente necesitas trabajar con un formato específico (por ejemplo, generar señales para bladeRF o GNU Radio), simplemente copia el archivo `.m` correspondiente a tu carpeta de trabajo:
- `load_sc16q11.m` / `save_sc16q11.m`: bladeRF SC16 Q11 (int16 normalizado a $\pm 2048$).
- `load_grcomplex.m` / `save_grcomplex.m`: GNU Radio float32 complex (`complex64`).
- `load_sc16full.m` / `save_sc16full.m`: int16 Little-Endian escala completa ($\pm 32768$).
- `load_int8.m` / `save_int8.m`: HackRF / RTL-SDR signed 8-bit ($\pm 128$).
- `load_cu8.m` / `save_cu8.m`: RTL-SDR unsigned 8-bit con cero en 127.5.
- `load_cs12.m` / `save_cs12.m`: LimeSDR / bladeRF 12-bit packed (3 bytes por par).
- `load_wav_iq.m` / `save_wav_iq.m`: WAV I/Q estéreo de 16 bits PCM.
- `load_csv_iq.m` / `save_csv_iq.m`: Texto plano CSV con columnas I, Q.

### 2. Usar la Función Unificadora (`iq_files.m`)
Si trabajas con múltiples formatos, copia `iq_files.m` y las funciones `load_*` / `save_*` a tu carpeta o agrégalas al path de MATLAB (`addpath('ruta/a/iq-files/matlab')`):

```matlab
% Guardar señal compleja
iq = 0.7 * exp(1j * 2 * pi * 1000 * (0:1/48000:1));
iq_files('write', 'senial.bin', iq, 48000, 'sc16q11');

% Leer señal
[samples, fs] = iq_files('read', 'senial.bin', 'sc16q11');
```

---

## Carpeta de Ejemplos

Encuentra scripts interactivos y listos para ejecutar en la carpeta [`examples/`](examples/):
- `examples/example_basic_io.m`: Sintetiza un tono analítico puro, lo serializa en múltiples formatos (bladeRF, RTL-SDR, LimeSDR, WAV), lo lee de vuelta y verifica la precisión.


## Compatibilidad
Compatible con **MATLAB R2016b o posterior** y **GNU Octave 6.0 o posterior**.
No requiere Toolboxes adicionales.
