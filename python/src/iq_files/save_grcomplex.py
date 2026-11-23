import numpy as np

def save_grcomplex(filename: str, signal: np.ndarray) -> bool:
    """
    Guarda una señal compleja en formato GNU Radio complex float32 (complex64).
    """
    signal = np.asarray(signal, dtype=np.complex64)
    signal.tofile(filename)
    return True

if __name__ == '__main__':
    sig = np.array([0.5 + 0.5j, -0.7 - 0.2j], dtype=np.complex64)
    save_grcomplex('test_grcomplex.iq', sig)
    print("Guardado test_grcomplex.iq con éxito.")
