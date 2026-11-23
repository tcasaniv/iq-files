import numpy as np

def load_csv_iq(filename: str) -> np.ndarray:
    """
    Lee un archivo CSV con columnas I, Q como señal compleja.
    """
    data = np.loadtxt(filename, delimiter=',', comments='#')
    return data[:, 0].astype(np.float32) + 1j * data[:, 1].astype(np.float32)

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data = load_csv_iq(sys.argv[1])
        print(f"Cargadas {len(data)} muestras complejas.")
