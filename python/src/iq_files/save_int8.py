import numpy as np

def save_int8(filename: str, signal: np.ndarray) -> bool:
    """
    Guarda una señal compleja normalizada en formato RAW signed 8-bit (HackRF / RTL-SDR).
    Rango de normalización: [-1.0, 1.0] mapeado a enteros [-128, 127].
    """
    signal = np.asarray(signal, dtype=np.complex64)
    i_scaled = np.clip(np.round(signal.real * 127.0), -128, 127).astype(np.int8)
    q_scaled = np.clip(np.round(signal.imag * 127.0), -128, 127).astype(np.int8)

    interleaved = np.empty(len(signal) * 2, dtype=np.int8)
    interleaved[0::2] = i_scaled
    interleaved[1::2] = q_scaled

    interleaved.tofile(filename)
    return True

if __name__ == '__main__':
    sig = np.array([0.5 + 0.5j, -0.9 - 0.1j], dtype=np.complex64)
    save_int8('test_int8.raw', sig)
    print("Guardado test_int8.raw con éxito.")
