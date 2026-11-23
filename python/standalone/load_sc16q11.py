import numpy as np

def load_sc16q11(filename: str) -> np.ndarray:
    """
    Lee un archivo binario en formato bladeRF SC16 Q11.
    Retorna un array de números complejos con rango normalizado [-1.0, 1.0].
    """
    raw = np.fromfile(filename, dtype=np.int16)
    i = raw[0::2].astype(np.float32) / 2048.0
    q = raw[1::2].astype(np.float32) / 2048.0
    return i + 1j * q

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data = load_sc16q11(sys.argv[1])
        print(f"Cargadas {len(data)} muestras complejas.")
