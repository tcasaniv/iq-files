"""
iq_files: Módulo unificado para lectura y guardado de archivos de señales I/Q complejas.
"""

import numpy as np
from .load_sc16q11 import load_sc16q11
from .save_sc16q11 import save_sc16q11
from .load_sc16full import load_sc16full
from .save_sc16full import save_sc16full
from .load_grcomplex import load_grcomplex
from .save_grcomplex import save_grcomplex
from .load_int8 import load_int8
from .save_int8 import save_int8
from .load_wav_iq import load_wav_iq
from .save_wav_iq import save_wav_iq
from .load_csv_iq import load_csv_iq
from .save_csv_iq import save_csv_iq

__all__ = [
    'read_iq',
    'write_iq',
    'load_sc16q11',
    'save_sc16q11',
    'load_sc16full',
    'save_sc16full',
    'load_grcomplex',
    'save_grcomplex',
    'load_int8',
    'save_int8',
    'load_wav_iq',
    'save_wav_iq',
    'load_csv_iq',
    'save_csv_iq',
]

def read_iq(filename: str, format: str = 'sc16q11') -> tuple[np.ndarray, int]:
    """
    Lee un archivo de señal I/Q en el formato especificado.
    Formatos disponibles: 'sc16q11', 'sc16full', 'grcomplex', 'float32', 'int8', 'wav_iq', 'csv_iq'.
    Retorna (signal, sample_rate).
    """
    fmt = format.lower()
    if fmt == 'sc16q11':
        return load_sc16q11(filename), 288000
    elif fmt == 'sc16full':
        return load_sc16full(filename), 288000
    elif fmt in ('grcomplex', 'float32'):
        return load_grcomplex(filename), 1000000
    elif fmt == 'int8':
        return load_int8(filename), 2000000
    elif fmt == 'wav_iq':
        return load_wav_iq(filename)
    elif fmt == 'csv_iq':
        return load_csv_iq(filename), 288000
    else:
        raise ValueError(f"Formato no reconocido: {format}")

def write_iq(filename: str, signal: np.ndarray, sample_rate: int = 288000, format: str = 'sc16q11') -> bool:
    """
    Guarda un array complejo en un archivo de señal I/Q en el formato especificado.
    """
    fmt = format.lower()
    if fmt == 'sc16q11':
        return save_sc16q11(filename, signal)
    elif fmt == 'sc16full':
        return save_sc16full(filename, signal)
    elif fmt in ('grcomplex', 'float32'):
        return save_grcomplex(filename, signal)
    elif fmt == 'int8':
        return save_int8(filename, signal)
    elif fmt == 'wav_iq':
        return save_wav_iq(filename, signal, sample_rate=sample_rate)
    elif fmt == 'csv_iq':
        return save_csv_iq(filename, signal)
    else:
        raise ValueError(f"Formato no reconocido: {format}")
