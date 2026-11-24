import os
import unittest
import numpy as np
import sys

# Asegurar importación de src para pruebas contra la estructura del paquete
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'src')))

import iq_files
from iq_files import (
    read_iq, write_iq,
    load_sc16q11, save_sc16q11,
    load_sc16full, save_sc16full,
    load_grcomplex, save_grcomplex,
    load_int8, save_int8,
    load_cu8, save_cu8,
    load_cs12, save_cs12,
    load_wav_iq, save_wav_iq,
    load_csv_iq, save_csv_iq,
    load_cf64, save_cf64,
    load_wav_mono, save_wav_mono,
)

class TestPythonIqFiles(unittest.TestCase):
    def setUp(self):
        self.test_signal = np.array([0.0, 0.5 - 0.5j, 0.99 - 0.99j, -0.7 + 0.3j], dtype=np.complex64)
        self.tmp_files = []

    def tearDown(self):
        for f in self.tmp_files:
            if os.path.exists(f):
                try:
                    os.remove(f)
                except OSError:
                    pass

    def test_version_and_metadata(self):
        self.assertEqual(iq_files.__version__, "0.1.1")
        self.assertIn("read_iq", iq_files.__all__)
        self.assertIn("load_cu8", iq_files.__all__)
        self.assertIn("load_cs12", iq_files.__all__)

    def test_sc16q11(self):
        fname = "tmp_sc16q11.bin"
        self.tmp_files.append(fname)
        save_sc16q11(fname, self.test_signal)
        loaded = load_sc16q11(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.002)

        # Probar API unificada read_iq / write_iq
        write_iq(fname, self.test_signal, format='sc16q11')
        loaded_uni, fs = read_iq(fname, format='sc16q11')
        self.assertEqual(fs, 288000)
        np.testing.assert_allclose(loaded_uni, self.test_signal, atol=0.002)

    def test_sc16full(self):
        fname = "tmp_sc16full.bin"
        self.tmp_files.append(fname)
        save_sc16full(fname, self.test_signal)
        loaded = load_sc16full(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.0002)

    def test_grcomplex(self):
        fname = "tmp_grcomplex.iq"
        self.tmp_files.append(fname)
        save_grcomplex(fname, self.test_signal)
        loaded = load_grcomplex(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=1e-6)

    def test_int8(self):
        fname = "tmp_int8.raw"
        self.tmp_files.append(fname)
        save_int8(fname, self.test_signal)
        loaded = load_int8(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.02)

    def test_cu8_rtlsdr(self):
        fname = "tmp_cu8.raw"
        self.tmp_files.append(fname)
        save_cu8(fname, self.test_signal)
        loaded = load_cu8(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.02)

        # Probar API unificada
        write_iq(fname, self.test_signal, format='cu8')
        loaded_uni, fs = read_iq(fname, format='cu8')
        self.assertEqual(fs, 2048000)
        np.testing.assert_allclose(loaded_uni, self.test_signal, atol=0.02)

    def test_cs12_packed(self):
        fname = "tmp_cs12.bin"
        self.tmp_files.append(fname)
        save_cs12(fname, self.test_signal)
        loaded = load_cs12(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.002)

        # Probar API unificada
        write_iq(fname, self.test_signal, format='cs12')
        loaded_uni, fs = read_iq(fname, format='cs12')
        self.assertEqual(fs, 2000000)
        np.testing.assert_allclose(loaded_uni, self.test_signal, atol=0.002)

    def test_wav_iq(self):
        fname = "tmp_wav.wav"
        self.tmp_files.append(fname)
        save_wav_iq(fname, self.test_signal, 48000)
        loaded, fs = load_wav_iq(fname)
        self.assertEqual(fs, 48000)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.0002)

    def test_csv_iq(self):
        fname = "tmp_csv.csv"
        self.tmp_files.append(fname)
        save_csv_iq(fname, self.test_signal)
        loaded = load_csv_iq(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=0.0002)

    def test_cf64(self):
        fname = "tmp_cf64.iq"
        self.tmp_files.append(fname)
        save_cf64(fname, self.test_signal)
        loaded = load_cf64(fname)
        np.testing.assert_allclose(loaded, self.test_signal, atol=1e-12)

    def test_wav_mono(self):
        fname = "tmp_mono.wav"
        self.tmp_files.append(fname)
        save_wav_mono(fname, self.test_signal, 48000)
        loaded, fs = load_wav_mono(fname)
        self.assertEqual(fs, 48000)
        np.testing.assert_allclose(loaded.real, self.test_signal.real, atol=0.0002)
        np.testing.assert_allclose(loaded.imag, np.zeros_like(loaded.imag))


if __name__ == '__main__':
    unittest.main()
