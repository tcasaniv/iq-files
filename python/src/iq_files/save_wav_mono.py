import numpy as np
from scipy.io import wavfile

def save_wav_mono(filename: str, signal: np.ndarray, sample_rate: int = 48000) -> bool:
    """
    Guarda la parte real de una señal en un archivo WAV mono (1 canal PCM 16 bits).
    """
    signal = np.asarray(signal)
    real_part = np.real(signal)
    scaled = np.clip(np.round(real_part * 32767.0), -32768, 32767).astype(np.int16)
    wavfile.write(filename, sample_rate, scaled)
    return True

if __name__ == '__main__':
    sig = np.array([0.5, -0.5, 0.2, -0.8], dtype=np.float32)
    save_wav_mono('test_mono.wav', sig, 48000)
    print("Guardado test_mono.wav con éxito.")
