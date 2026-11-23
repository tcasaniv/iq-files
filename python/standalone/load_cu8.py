"""
load_cu8 / save_cu8: Muestras I/Q RAW Enteros de 8 bits sin signo (CU8 / uint8).

¿CUÁNDO USAR ESTE FORMATO?
--------------------------
- Hardware típico: Receptores RTL-SDR (DVB-T Blog V3/V4 basados en el chip RTL2832U).
- Software emisor: Grabaciones de utilidades como 'rtl_sdr', streams de 'rtl_tcp'.
- Comando típico de captura:
    rtl_sdr -f 433.92M -s 2048000 -g 30 captura.cu8

ESPECIFICACIONES TÉCNICAS:
--------------------------
- Tipo primitivo: uint8 (unsigned 8-bit, 0 a 255).
- Entrelazado: [I0, Q0, I1, Q1, ...].
- Bytes por muestra compleja: 2 bytes.
- Punto cero: 127.5.
- Normalización: val_float = (val_u8 - 127.5) / 127.5
"""

import numpy as np

def load_cu8_from_bytes(raw_bytes: bytes) -> np.ndarray:
    """Decodifica un buffer de bytes CU8 a un array complejo complex64 normalizado en [-1.0, 1.0]."""
    raw = np.frombuffer(raw_bytes, dtype=np.uint8)
    if len(raw) % 2 != 0:
        raw = raw[:len(raw) - 1]
    raw = (raw.astype(np.float32) - 127.5) / 127.5
    return raw[0::2] + 1j * raw[1::2]

def load_cu8(filepath: str) -> np.ndarray:
    """Lee un archivo RAW CU8 de RTL-SDR y retorna muestras complex64."""
    with open(filepath, 'rb') as f:
        return load_cu8_from_bytes(f.read())

def save_cu8_to_bytes(iq_samples: np.ndarray) -> bytes:
    """Codifica un array complejo a bytes en formato RAW CU8 (uint8 sin signo con cero en 127.5)."""
    i_part = np.clip(np.real(iq_samples) * 127.5 + 127.5, 0, 255).astype(np.uint8)
    q_part = np.clip(np.imag(iq_samples) * 127.5 + 127.5, 0, 255).astype(np.uint8)
    interleaved = np.empty(len(iq_samples) * 2, dtype=np.uint8)
    interleaved[0::2] = i_part
    interleaved[1::2] = q_part
    return interleaved.tobytes()

def save_cu8(filepath: str, iq_samples: np.ndarray) -> None:
    """Guarda muestras complejas en un archivo RAW CU8 compatible con RTL-SDR."""
    with open(filepath, 'wb') as f:
        f.write(save_cu8_to_bytes(iq_samples))
