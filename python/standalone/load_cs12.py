"""
load_cs12 / save_cs12: Muestras I/Q RAW Enteros de 12 bits con signo empaquetados (CS12 Packed).

¿CUÁNDO USAR ESTE FORMATO?
--------------------------
- Hardware típico: Dispositivos LimeSDR (LMS7002M) y bladeRF en modo 12-bit packed.
- Motivo: Ahorra un 25% de ancho de banda USB empaquetando dos enteros de 12 bits en 3 bytes.

ESPECIFICACIONES TÉCNICAS:
--------------------------
- Bytes por muestra compleja: 3 bytes por par (I, Q).
- Rango: [-2048, 2047]. Cero en 0.0.
"""

import numpy as np

def load_cs12_from_bytes(raw_bytes: bytes) -> np.ndarray:
    """Decodifica un buffer de bytes CS12 packed a complex64."""
    num_samples = len(raw_bytes) // 3
    if num_samples == 0:
        return np.array([], dtype=np.complex64)

    raw = np.frombuffer(raw_bytes[:num_samples * 3], dtype=np.uint8)
    b0 = raw[0::3].astype(np.int32)
    b1 = raw[1::3].astype(np.int32)
    b2 = raw[2::3].astype(np.int32)

    i_raw = b0 | ((b1 & 0x0F) << 8)
    q_raw = ((b1 >> 4) & 0x0F) | (b2 << 4)

    # Sign extension de 12 bits a int32
    i_raw = np.where(i_raw >= 2048, i_raw - 4096, i_raw)
    q_raw = np.where(q_raw >= 2048, q_raw - 4096, q_raw)

    return (i_raw.astype(np.float32) / 2048.0) + 1j * (q_raw.astype(np.float32) / 2048.0)

def load_cs12(filepath: str) -> np.ndarray:
    with open(filepath, 'rb') as f:
        return load_cs12_from_bytes(f.read())

def save_cs12_to_bytes(iq_samples: np.ndarray) -> bytes:
    i_val = np.clip(np.round(np.real(iq_samples) * 2047.0), -2048, 2047).astype(np.int32)
    q_val = np.clip(np.round(np.imag(iq_samples) * 2047.0), -2048, 2047).astype(np.int32)

    i_u12 = i_val & 0x0FFF
    q_u12 = q_val & 0x0FFF

    b0 = (i_u12 & 0x00FF).astype(np.uint8)
    b1 = (((i_u12 >> 8) & 0x0F) | ((q_u12 & 0x0F) << 4)).astype(np.uint8)
    b2 = ((q_u12 >> 4) & 0xFF).astype(np.uint8)

    interleaved = np.empty(len(iq_samples) * 3, dtype=np.uint8)
    interleaved[0::3] = b0
    interleaved[1::3] = b1
    interleaved[2::3] = b2
    return interleaved.tobytes()

def save_cs12(filepath: str, iq_samples: np.ndarray) -> None:
    with open(filepath, 'wb') as f:
        f.write(save_cs12_to_bytes(iq_samples))
