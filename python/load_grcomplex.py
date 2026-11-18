import numpy as np

def load_grcomplex(filename: str) -> np.ndarray:
    """
    Lee un archivo binario en formato GNU Radio complex float32 (complex64).
    Retorna un array de números complejos numpy.complex64.
    """
    return np.fromfile(filename, dtype=np.complex64)

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data = load_grcomplex(sys.argv[1])
        print(f"Cargadas {len(data)} muestras complejas.")
