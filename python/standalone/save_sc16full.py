import numpy as np

def save_sc16full(filename: str, signal: np.ndarray) -> bool:
    """
    Guarda una señal compleja normalizada en formato RAW SC16 escala completa (int16 estándar).
    Rango de normalización: [-1.0, 1.0] mapeado a enteros [-32768, 32767].
    """
    signal = np.asarray(signal, dtype=np.complex64)
    i_scaled = np.clip(np.round(signal.real * 32767.0), -32768, 32767).astype(np.int16)
    q_scaled = np.clip(np.round(signal.imag * 32767.0), -32768, 32767).astype(np.int16)

    interleaved = np.empty(len(signal) * 2, dtype=np.int16)
    interleaved[0::2] = i_scaled
    interleaved[1::2] = q_scaled

    interleaved.tofile(filename)
    return True

if __name__ == '__main__':
    sig = np.array([0.707 - 0.707j, -0.5 + 0.5j], dtype=np.complex64)
    save_sc16full('test_sc16full.bin', sig)
    print("Guardado test_sc16full.bin con éxito.")
