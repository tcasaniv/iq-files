import numpy as np

def save_csv_iq(filename: str, signal: np.ndarray) -> bool:
    """
    Guarda muestras complejas en formato CSV con columnas I, Q.
    """
    signal = np.asarray(signal, dtype=np.complex64)
    data = np.column_stack([signal.real, signal.imag])
    np.savetxt(filename, data, delimiter=', ', fmt='%.7f', header='I, Q', comments='# ')
    return True

if __name__ == '__main__':
    sig = np.array([0.1 + 0.2j, -0.3 + 0.4j], dtype=np.complex64)
    save_csv_iq('test_csv.csv', sig)
    print("Guardado test_csv.csv con éxito.")
