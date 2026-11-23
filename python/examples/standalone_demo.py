#!/usr/bin/env python3
"""
Demostración de uso de módulos individuales independientes (standalone) sin instalar el paquete.

Permite copiar únicamente los archivos `load_*.py` o `save_*.py` requeridos
a cualquier script sin dependencias externas fuera de NumPy/SciPy.
"""

import os
import sys
import tempfile
import numpy as np

# Añadir la carpeta standalone al path de Python
standalone_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'standalone'))
sys.path.insert(0, standalone_dir)

# Importación directa de archivos individuales
import load_cu8
import save_cu8
import load_sc16q11
import save_sc16q11

def main():
    print("=== Demostración Standalone (archivos sueltos) ===")

    signal = np.array([0.5 + 0.5j, -0.5 - 0.5j, 0.9 - 0.1j], dtype=np.complex64)
    tmp_path = os.path.join(tempfile.gettempdir(), "test_standalone.cu8")

    # Guardar usando save_cu8 independiente
    save_cu8.save_cu8(tmp_path, signal)
    # Leer usando load_cu8 independiente
    recovered = load_cu8.load_cu8(tmp_path)

    print(f"Señal original : {signal}")
    print(f"Señal leída    : {recovered}")
    print("Demostración standalone completada con éxito.")

    if os.path.exists(tmp_path):
        os.remove(tmp_path)

if __name__ == '__main__':
    main()
