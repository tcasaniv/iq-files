import numpy as np
from scipy.io import wavfile

def save_wav_iq(filename: str, signal: np.ndarray, sample_rate: int = 48000) -> bool:
    """
    Guarda una señal compleja normalizada en un archivo WAV de 2 canales PCM (16 bits).
    Canal 1 (L) = I, Canal 2 (R) = Q.
    """
    signal = np.asarray(signal, dtype=np.complex64)
    i_scaled = np.clip(np.round(signal.real * 32767.0), -32768, 32767).astype(np.int16)
    q_scaled = np.clip(np.round(signal.imag * 32767.0), -32768, 32767).astype(np.int16)

    data = np.column_stack([i_scaled, q_scaled])
    wavfile.write(filename, sample_rate, data)
    return True

if __name__ == '__main__':
    sig = np.array([0.5 + 0.5j, -0.5 - 0.5j], dtype=np.complex64)
    save_wav_iq('test_wav.wav', sig, 48000)
    print("Guardado test_wav.wav con éxito.")
