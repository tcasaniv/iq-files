import numpy as np

def load_int8(filename: str) -> np.ndarray:
    """
    Lee un archivo binario en formato RAW signed 8-bit (HackRF / RTL-SDR).
    Retorna un array de números complejos normalizados a [-1.0, 1.0].
    """
    raw = np.fromfile(filename, dtype=np.int8)
    i = raw[0::2].astype(np.float32) / 128.0
    q = raw[1::2].astype(np.float32) / 128.0
    return i + 1j * q

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data = load_int8(sys.argv[1])
        print(f"Cargadas {len(data)} muestras complejas.")
