import numpy as np
from scipy.io import wavfile

def load_wav_mono(filename: str) -> tuple[np.ndarray, int]:
    """
    Lee un archivo WAV mono (1 canal PCM 16 bits).
    Mapea el canal de audio real a la componente I y fija la componente Q en 0.0.
    Retorna (signal, sample_rate).
    """
    sample_rate, data = wavfile.read(filename)
    if data.ndim > 1:
        data = data[:, 0]
    i = data.astype(np.float32) / 32768.0
    q = np.zeros_like(i)
    return (i + 1j * q), int(sample_rate)

if __name__ == '__main__':
    import sys
    if len(sys.argv) > 1:
        data, fs = load_wav_mono(sys.argv[1])
        print(f"Cargadas {len(data)} muestras mono a {fs} Hz.")
