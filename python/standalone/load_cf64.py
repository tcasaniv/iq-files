import numpy as np

def load_cf64(filename: str) -> np.ndarray:
    """
    Lee un archivo binario en formato RAW Float64 / Double precision complex (complex128).
    Retorna un array de números complejos numpy.complex128.
    """
    return np.fromfile(filename, dtype=np.complex128)

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data = load_cf64(sys.argv[1])
        print(f"Cargadas {len(data)} muestras complejas (Float64).")
