#!/usr/bin/env python3
"""
Demostración básica de uso del paquete `iq_files` en Python.

Ejecutar:
    python quickstart.py
"""

import os
import tempfile
import numpy as np
import iq_files
from iq_files import (
    read_iq, write_iq,
    load_sc16q11, save_sc16q11,
    load_cu8, save_cu8,
    load_cs12, save_cs12,
    load_wav_iq, save_wav_iq,
)

def main():
    print("=== Demostración de iq-files en Python ===")
    print(f"Versión de iq-files: {iq_files.__version__}\n")

    # 1. Sintetizar tono complejo I/Q (48 kHz, tono analítico a 1000 Hz)
    sample_rate = 48000
    duration = 0.1
    t = np.arange(0, duration, 1.0 / sample_rate)
    freq_hz = 1000.0
    signal = (0.75 * np.exp(1j * 2.0 * np.pi * freq_hz * t)).astype(np.complex64)
    print(f"1. Señal sintetizada: {len(signal)} muestras complejas a {sample_rate} Sps.")

    out_dir = os.path.join(tempfile.gettempdir(), "iq_files_py_demo")
    os.makedirs(out_dir, exist_ok=True)

    # 2. bladeRF SC16 Q11
    path_bladerf = os.path.join(out_dir, "demo_bladerf.sc16q11")
    save_sc16q11(path_bladerf, signal)
    loaded_bladerf = load_sc16q11(path_bladerf)
    print(f"2. bladeRF SC16 Q11: guardado y leído exitosamente ({len(loaded_bladerf)} muestras).")

    # 3. RTL-SDR CU8 (uint8 sin signo con punto cero en 127.5)
    path_rtlsdr = os.path.join(out_dir, "demo_rtlsdr.cu8")
    save_cu8(path_rtlsdr, signal)
    loaded_rtlsdr = load_cu8(path_rtlsdr)
    print(f"3. RTL-SDR CU8: guardado y leído exitosamente ({len(loaded_rtlsdr)} muestras).")

    # 4. LimeSDR CS12 Packed (3 bytes por muestra compleja)
    path_cs12 = os.path.join(out_dir, "demo_limesdr.cs12")
    save_cs12(path_cs12, signal)
    loaded_cs12 = load_cs12(path_cs12)
    print(f"4. LimeSDR CS12 Packed: guardado y leído exitosamente ({len(loaded_cs12)} muestras).")

    # 5. API Unificada: WAV I/Q Estéreo
    path_wav = os.path.join(out_dir, "demo_stereo.wav")
    write_iq(path_wav, signal, sample_rate=sample_rate, format='wav_iq')
    loaded_wav, fs_read = read_iq(path_wav, format='wav_iq')
    print(f"5. WAV I/Q Estéreo (unificado): guardado y leído exitosamente ({len(loaded_wav)} muestras a {fs_read} Hz).")

    # 6. Comprobación de precisión
    residual = np.max(np.abs(signal - loaded_wav))
    print(f"6. Error residual WAV vs Original: {residual:.6f}")

    print("\n¡Demostración completada con éxito!")

if __name__ == '__main__':
    main()
