import numpy as np

def save_sc16q11(filename: str, signal: np.ndarray) -> bool:
    """
    Guarda una señal compleja normalizada en un archivo binario en formato bladeRF SC16 Q11.
    Valores saturados y mapeados a enteros de 16 bits en el rango [-2048, 2047].
    """
    signal = np.asarray(signal, dtype=np.complex64)
    i_scaled = np.clip(np.round(signal.real * 2048.0), -2048, 2047).astype(np.int16)
    q_scaled = np.clip(np.round(signal.imag * 2048.0), -2048, 2047).astype(np.int16)

    interleaved = np.empty(len(signal) * 2, dtype=np.int16)
    interleaved[0::2] = i_scaled
    interleaved[1::2] = q_scaled

    interleaved.tofile(filename)
    return True

if __name__ == '__main__':
    # Ejemplo de prueba
    fs = 48000
    t = np.arange(0, 0.1, 1/fs)
    sig = 0.8 * np.exp(1j * 2 * np.pi * 1000 * t)
    save_sc16q11('test_sc16q11.bin', sig)
    print("Guardado test_sc16q11.bin con éxito.")
