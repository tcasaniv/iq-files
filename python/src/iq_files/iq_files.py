"""
iq_files: Módulo unificado para lectura y guardado de archivos de señales I/Q complejas.
"""

from typing import Tuple, Optional
import numpy as np

try:
    from .load_sc16q11 import load_sc16q11
    from .save_sc16q11 import save_sc16q11
    from .load_sc16full import load_sc16full
    from .save_sc16full import save_sc16full
    from .load_grcomplex import load_grcomplex
    from .save_grcomplex import save_grcomplex
    from .load_int8 import load_int8
    from .save_int8 import save_int8
    from .load_cu8 import load_cu8, save_cu8
    from .load_cs12 import load_cs12, save_cs12
    from .load_wav_iq import load_wav_iq
    from .save_wav_iq import save_wav_iq
    from .load_csv_iq import load_csv_iq
    from .save_csv_iq import save_csv_iq
    from .load_cf64 import load_cf64
    from .save_cf64 import save_cf64
    from .load_wav_mono import load_wav_mono
    from .save_wav_mono import save_wav_mono
except ImportError:
    from load_sc16q11 import load_sc16q11
    from save_sc16q11 import save_sc16q11
    from load_sc16full import load_sc16full
    from save_sc16full import save_sc16full
    from load_grcomplex import load_grcomplex
    from save_grcomplex import save_grcomplex
    from load_int8 import load_int8
    from save_int8 import save_int8
    from load_cu8 import load_cu8, save_cu8
    from load_cs12 import load_cs12, save_cs12
    from load_wav_iq import load_wav_iq
    from save_wav_iq import save_wav_iq
    from load_csv_iq import load_csv_iq
    from save_csv_iq import save_csv_iq
    from load_cf64 import load_cf64
    from save_cf64 import save_cf64
    from load_wav_mono import load_wav_mono
    from save_wav_mono import save_wav_mono

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
    'load_cu8',
    'save_cu8',
    'load_cs12',
    'save_cs12',
    'load_wav_iq',
    'save_wav_iq',
    'load_csv_iq',
    'save_csv_iq',
    'load_cf64',
    'save_cf64',
    'load_wav_mono',
    'save_wav_mono',
]

def read_iq(filename: str, format: str = 'sc16q11') -> Tuple[np.ndarray, int]:
    """
    Lee un archivo de señal I/Q en el formato especificado.
    Formatos disponibles: 'sc16q11', 'sc16full', 'grcomplex', 'float32', 'int8', 'cu8', 'u8', 'cs12', 'wav_iq', 'csv_iq'.
    Retorna (signal, sample_rate).
    """
    fmt = format.lower()
    if fmt == 'sc16q11':
        return load_sc16q11(filename), 288000
    elif fmt == 'sc16full':
        return load_sc16full(filename), 288000
    elif fmt in ('grcomplex', 'float32', 'cf32'):
        return load_grcomplex(filename), 1000000
    elif fmt in ('int8', 'cs8', 'sc8'):
        return load_int8(filename), 2000000
    elif fmt in ('cu8', 'u8'):
        return load_cu8(filename), 2048000
    elif fmt in ('cs12', 'sc12'):
        return load_cs12(filename), 2000000
    elif fmt in ('wav_iq', 'wav'):
        return load_wav_iq(filename)
    elif fmt in ('csv_iq', 'csv'):
        return load_csv_iq(filename), 48000
    elif fmt in ('cf64', 'float64'):
        return load_cf64(filename), 1000000
    elif fmt in ('wav_mono', 'mono'):
        return load_wav_mono(filename)
    else:
        raise ValueError(f"Formato desconocido '{format}'. Use: sc16q11, sc16full, grcomplex, int8, cu8, cs12, wav_iq, csv_iq, cf64, wav_mono")

def write_iq(filename: str, signal: np.ndarray, sample_rate: int = 288000, format: str = 'sc16q11') -> bool:
    """
    Guarda una señal I/Q en el formato especificado.
    Formatos disponibles: 'sc16q11', 'sc16full', 'grcomplex', 'float32', 'int8', 'cu8', 'u8', 'cs12', 'wav_iq', 'csv_iq'.
    """
    fmt = format.lower()
    if fmt == 'sc16q11':
        return save_sc16q11(filename, signal)
    elif fmt == 'sc16full':
        return save_sc16full(filename, signal)
    elif fmt in ('grcomplex', 'float32', 'cf32'):
        return save_grcomplex(filename, signal)
    elif fmt in ('int8', 'cs8', 'sc8'):
        return save_int8(filename, signal)
    elif fmt in ('cu8', 'u8'):
        save_cu8(filename, signal)
        return True
    elif fmt in ('cs12', 'sc12'):
        save_cs12(filename, signal)
        return True
    elif fmt in ('wav_iq', 'wav'):
        return save_wav_iq(filename, signal, sample_rate)
    elif fmt in ('csv_iq', 'csv'):
        return save_csv_iq(filename, signal)
    elif fmt in ('cf64', 'float64'):
        return save_cf64(filename, signal)
    elif fmt in ('wav_mono', 'mono'):
        return save_wav_mono(filename, signal, sample_rate)
    else:
        raise ValueError(f"Formato desconocido '{format}'. Use: sc16q11, sc16full, grcomplex, int8, cu8, cs12, wav_iq, csv_iq, cf64, wav_mono")
