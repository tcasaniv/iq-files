import numpy as np

def save_cf64(filename: str, signal: np.ndarray) -> bool:
    """
    Guarda una señal compleja en formato RAW Float64 / Double precision complex (complex128).
    """
    signal = np.asarray(signal, dtype=np.complex128)
    signal.tofile(filename)
    return True

if __name__ == '__main__':
    sig = np.array([0.5 + 0.5j, -0.7 - 0.2j], dtype=np.complex128)
    save_cf64('test_cf64.iq', sig)
    print("Guardado test_cf64.iq con éxito.")
