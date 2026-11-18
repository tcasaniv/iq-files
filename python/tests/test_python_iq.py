import os
import unittest
import numpy as np
import sys

# Asegurar importación del directorio raíz de python
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

import load_sc16q11, save_sc16q11
import load_sc16full, save_sc16full
import load_grcomplex, save_grcomplex
import load_int8, save_int8
import load_wav_iq, save_wav_iq
import load_csv_iq, save_csv_iq

class TestPythonIqFiles(unittest.TestCase):
    def setUp(self):
        self.test_signal = np.array([0.0, 0.5 - 0.5j, 0.99 - 0.99j, -0.7 + 0.3j], dtype=np.complex64)
        self.tmp_files = []

    def tearDown(self):
        for f in self.tmp_files:
            if os.path.exists(f):
                os.remove(f)

    def test_sc16q11(self):
        fname = "tmp_sc16q11.bin"
        self.tmp_files.append(fname)
        save_sc16q11.save_sc16q11(fname, self.test_signal)
        loaded = load_sc16q11.load_sc16q11(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.002)

    def test_sc16full(self):
        fname = "tmp_sc16full.bin"
        self.tmp_files.append(fname)
        save_sc16full.save_sc16full(fname, self.test_signal)
        loaded = load_sc16full.load_sc16full(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.0002)

    def test_grcomplex(self):
        fname = "tmp_grcomplex.iq"
        self.tmp_files.append(fname)
        save_grcomplex.save_grcomplex(fname, self.test_signal)
        loaded = load_grcomplex.load_grcomplex(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=1e-6)

    def test_int8(self):
        fname = "tmp_int8.raw"
        self.tmp_files.append(fname)
        save_int8.save_int8(fname, self.test_signal)
        loaded = load_int8.load_int8(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.02)

    def test_wav_iq(self):
        fname = "tmp_wav.wav"
        self.tmp_files.append(fname)
        save_wav_iq.save_wav_iq(fname, self.test_signal, 48000)
        loaded, fs = load_wav_iq.load_wav_iq(fname)
        self.assertEqual(fs, 48000)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.0002)

    def test_csv_iq(self):
        fname = "tmp_csv.csv"
        self.tmp_files.append(fname)
        save_csv_iq.save_csv_iq(fname, self.test_signal)
        loaded = load_csv_iq.load_csv_iq(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.0002)

if __name__ == '__main__':
    unittest.main()
