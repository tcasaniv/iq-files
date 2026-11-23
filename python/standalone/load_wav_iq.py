import numpy as np
from scipy.io import wavfile

def load_wav_iq(filename: str) -> tuple[np.ndarray, int]:
    """
    Lee un archivo WAV de 2 canales PCM (16 bits) como señal I/Q.
    Canal 1 = I, Canal 2 = Q.
    Retorna (signal, sample_rate).
    """
    sample_rate, data = wavfile.read(filename)
    if data.ndim == 1:
        i = data.astype(np.float32) / 32768.0
        q = np.zeros_like(i)
    else:
        i = data[:, 0].astype(np.float32) / 32768.0
        q = data[:, 1].astype(np.float32) / 32768.0
    return (i + 1j * q), int(sample_rate)

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data, fs = load_wav_iq(sys.argv[1])
        print(f"Cargadas {len(data)} muestras a {fs} Hz.")
