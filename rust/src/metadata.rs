#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Formato de codificación binaria o contenedor para muestras complejas I/Q.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IqFormat {
    /// Archivo WAV de 2 canales I/Q (16-bit PCM) compatible con SDR#, SDR++, Audacity y Gqrx
    WavIq16,
    /// Archivo RAW binario con int16 escalado en formato Q11 ([-2048, 2047]), estándar para bladeRF
    RawSc16Q11,
    /// Archivo RAW binario con int16 a escala completa ([-32768, 32767])
    RawSc16Full,
    /// Archivo RAW binario con signed int8 entrelazado ([-128, 127]), formato nativo para hackrf_transfer
    RawInt8,
    /// Archivo RAW binario con float32 entrelazados [I0, Q0, ...] para GNU Radio o SoapySDR
    RawFloat32,
    /// Archivo WAV de audio mono (16-bit PCM) para señales con subportadora audible
    WavAudioMono,
    /// Archivo de texto plano CSV con pares (I, Q) por línea
    CsvIq,
}

impl IqFormat {
    /// Nombre conciso y limpio para ComboBox y desplegables de UI.
    pub fn short_name(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "WAV I/Q Estéreo (PCM 16-bit)",
            IqFormat::RawSc16Q11 => "RAW SC16 Q11 (bladeRF)",
            IqFormat::RawSc16Full => "RAW SC16 Estándar (Full Scale)",
            IqFormat::RawInt8 => "RAW Int8 (HackRF)",
            IqFormat::RawFloat32 => "RAW Float32 (GNU Radio / SigMF)",
            IqFormat::WavAudioMono => "WAV Audio Mono",
            IqFormat::CsvIq => "CSV I/Q (Texto plano)",
        }
    }

    /// Nombre completo / descriptivo anterior para compatibilidad.
    pub fn name(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "WAV I/Q Estéreo (16-bit PCM - SDR#, SDR++)",
            IqFormat::RawSc16Q11 => "RAW SC16_Q11 (int16 Little-Endian para bladeRF)",
            IqFormat::RawSc16Full => "RAW SC16 Estándar (int16 Little-Endian Full Scale)",
            IqFormat::RawInt8 => "RAW Int8 (signed 8-bit para hackrf_transfer)",
            IqFormat::RawFloat32 => "RAW Float32 (float32 Little-Endian)",
            IqFormat::WavAudioMono => "WAV Audio Mono (Subportadora audible)",
            IqFormat::CsvIq => "CSV I/Q (Columnas I, Q en texto ASCII)",
        }
    }

    pub fn label(&self) -> &'static str {
        self.short_name()
    }

    /// Extensión de archivo sugerida por defecto.
    pub fn default_extension(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "wav",
            IqFormat::RawSc16Q11 => "bin",
            IqFormat::RawSc16Full => "bin",
            IqFormat::RawInt8 => "raw",
            IqFormat::RawFloat32 => "iq",
            IqFormat::WavAudioMono => "wav",
            IqFormat::CsvIq => "csv",
        }
    }

    /// Tipo de dato elemental subyacente.
    pub fn data_type_desc(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "int16_t PCM en contenedor RIFF/WAV",
            IqFormat::RawSc16Q11 => "int16_t Little-Endian (Punto fijo Q11)",
            IqFormat::RawSc16Full => "int16_t Little-Endian (Escala completa)",
            IqFormat::RawInt8 => "int8_t con signo (signed char)",
            IqFormat::RawFloat32 => "float IEEE 754 Little-Endian (32-bit)",
            IqFormat::WavAudioMono => "int16_t PCM mono en contenedor RIFF/WAV",
            IqFormat::CsvIq => "ASCII float de texto plano",
        }
    }

    /// Número de bytes por muestra compleja.
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            IqFormat::WavIq16 => 4,
            IqFormat::RawSc16Q11 => 4,
            IqFormat::RawSc16Full => 4,
            IqFormat::RawInt8 => 2,
            IqFormat::RawFloat32 => 8,
            IqFormat::WavAudioMono => 2,
            IqFormat::CsvIq => 16, // aproximado promedio
        }
    }

    /// Estructura de entrelazado de componentes I y Q.
    pub fn interleaving_desc(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "2 Canales: Canal 1 (L) = I, Canal 2 (R) = Q",
            IqFormat::RawSc16Q11 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...]",
            IqFormat::RawSc16Full => "Entrelazado continuo: [I0, Q0, I1, Q1, ...]",
            IqFormat::RawInt8 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...]",
            IqFormat::RawFloat32 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...]",
            IqFormat::WavAudioMono => "1 Canal mono (I = audio, Q = 0.0)",
            IqFormat::CsvIq => "Dos columnas separadas por coma: 'I, Q'",
        }
    }

    /// Rango dinámico y factor de normalización.
    pub fn dynamic_range_desc(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "Enteros [-32768, 32767] -> Dividir entre 32768.0",
            IqFormat::RawSc16Q11 => "Enteros [-2048, 2047] -> Dividir entre 2048.0 (12-bit Q11 DAC/ADC bladeRF)",
            IqFormat::RawSc16Full => "Enteros [-32768, 32767] -> Dividir entre 32768.0",
            IqFormat::RawInt8 => "Enteros con signo [-128, 127] -> Dividir entre 128.0 (HackRF / RTL-SDR)",
            IqFormat::RawFloat32 => "Continuo [-1.0, 1.0] (Full-scale nominal IEEE 754)",
            IqFormat::WavAudioMono => "Enteros [-32768, 32767] -> Dividir entre 32768.0",
            IqFormat::CsvIq => "Valores normalizados [-1.0, 1.0] en texto plano",
        }
    }

    /// Nombres base de los archivos standalone correspondientes en el monorepo iq-files.
    pub fn standalone_base_name(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "wav_iq",
            IqFormat::RawSc16Q11 => "sc16q11",
            IqFormat::RawSc16Full => "sc16full",
            IqFormat::RawInt8 => "int8",
            IqFormat::RawFloat32 => "grcomplex",
            IqFormat::WavAudioMono => "wav_mono",
            IqFormat::CsvIq => "csv_iq",
        }
    }

    /// Nombres de archivo sugeridos para funciones de lectura y guardado independientes.
    pub fn standalone_function_names(&self) -> (&'static str, &'static str) {
        match self {
            IqFormat::WavIq16 => ("load_wav_iq", "save_wav_iq"),
            IqFormat::RawSc16Q11 => ("load_sc16q11", "save_sc16q11"),
            IqFormat::RawSc16Full => ("load_sc16full", "save_sc16full"),
            IqFormat::RawInt8 => ("load_int8", "save_int8"),
            IqFormat::RawFloat32 => ("load_grcomplex", "save_grcomplex"),
            IqFormat::WavAudioMono => ("load_wav_mono", "save_wav_mono"),
            IqFormat::CsvIq => ("load_csv_iq", "save_csv_iq"),
        }
    }

    /// Software y ecosistemas SDR comúnmente compatibles.
    pub fn compatible_software(&self) -> &'static [&'static str] {
        match self {
            IqFormat::WavIq16 => &["SDR#", "SDR++", "Gqrx", "Audacity", "HDSDR", "MATLAB", "Python"],
            IqFormat::RawSc16Q11 => &["bladeRF-cli", "libbladeRF", "MATLAB", "Python", "GNU Radio"],
            IqFormat::RawSc16Full => &["SDRangel", "URH", "MATLAB", "Python", "GNU Radio (Short)"],
            IqFormat::RawInt8 => &["hackrf_transfer", "libhackrf", "RTL-SDR", "GNU Radio (Byte)", "MATLAB", "Python"],
            IqFormat::RawFloat32 => &["GNU Radio", "SigMF", "Inspectrum", "Gqrx", "MATLAB", "Python"],
            IqFormat::WavAudioMono => &["Reproductores multimedia", "Audacity", "MATLAB", "Python"],
            IqFormat::CsvIq => &["Excel / Hojas de cálculo", "MATLAB", "Python", "Scripts de análisis"],
        }
    }
}
