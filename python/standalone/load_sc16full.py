import numpy as np

def load_sc16full(filename: str) -> np.ndarray:
    """
    Lee un archivo binario en formato RAW SC16 escala completa (int16 estándar).
    Retorna un array de números complejos normalizados a [-1.0, 1.0].
    """
    raw = np.fromfile(filename, dtype=np.int16)
    i = raw[0::2].astype(np.float32) / 32768.0
    q = raw[1::2].astype(np.float32) / 32768.0
    return i + 1j * q

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data = load_sc16full(sys.argv[1])
        print(f"Cargadas {len(data)} muestras complejas.")
