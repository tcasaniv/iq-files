#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Formato de codificación binaria o contenedor para muestras complejas I/Q.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IqFormat {
    // --- Contenedores Estructurados ---
    /// Archivo WAV de 2 canales I/Q (16-bit PCM, estándar RIFF <= 4 GB) compatible con SDR#, SDR++, Audacity y Gqrx
    WavIq16,
    /// Archivo WAV RF64 de 64 bits (EBU Tech 3306) para grabaciones I/Q mayores a 4 GB (SDR#, SDR Console, HDSDR)
    WavRf64,
    /// Contenedor estándar abierto SigMF (.sigmf-meta JSON + .sigmf-data binario)
    SigMf,
    /// Archivo WAV de audio mono (16-bit PCM) para señales con subportadora audible
    WavAudioMono,
    /// Archivo de texto plano CSV con pares (I, Q) por línea
    CsvIq,

    // --- Binarios Crudos (Raw I/Q) - Enteros Sin Signo ---
    /// Entero sin signo de 8 bits (CU8 / uint8, rango [0, 255], cero en 127.5). Nativo de RTL-SDR (rtl_sdr, rtl_tcp)
    RawCu8,
    /// Entero sin signo de 16 bits (CU16 / uint16, rango [0, 65535], cero en 32768.0). ADCs industriales
    RawCu16,

    // --- Binarios Crudos (Raw I/Q) - Enteros Con Signo ---
    /// Entero con signo de 8 bits (CS8 / int8, rango [-128, 127], cero en 0.0). Nativo de HackRF One (hackrf_transfer)
    RawInt8,
    /// Entero con signo de 12 bits empaquetado (CS12 / 3 bytes por muestra I/Q). Ahorro de ancho de banda en LimeSDR y bladeRF
    RawCs12Packed,
    /// Entero con signo de 16 bits escalado Q11 (SC16Q11, rango [-2048, 2047]). Nativo de Nuand bladeRF
    RawSc16Q11,
    /// Entero con signo de 16 bits escala completa (CS16 / SC16, rango [-32768, 32767]). Estándar en USRP, PlutoSDR, Airspy, SDRplay
    RawSc16Full,
    /// Entero con signo de 24 bits (CS24 / 6 bytes por muestra I/Q). SDRs de alto rango dinámico y VLF
    RawCs24,
    /// Entero con signo de 32 bits (CS32 / 8 bytes por muestra I/Q). Salida interna de DSP / FPGAs
    RawCs32,

    // --- Binarios Crudos (Raw I/Q) - Punto Flotante ---
    /// Flotante de media precisión IEEE 754 de 16 bits (CF16 / 4 bytes por muestra). Redes neuronales / GPU / ML
    RawCf16,
    /// Flotante de precisión simple IEEE 754 de 32 bits (CF32 / .cfile / gr_complex, 8 bytes por muestra). Estándar GNU Radio, Gqrx, MATLAB
    RawFloat32,
    /// Flotante de doble precisión IEEE 754 de 64 bits (CF64 / complex128, 16 bytes por muestra). Radioastronomía y simulaciones
    RawCf64,
}

impl IqFormat {
    /// Categoría general del formato para agrupaciones en menús y asistentes.
    pub fn category(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 | IqFormat::WavRf64 | IqFormat::SigMf | IqFormat::WavAudioMono | IqFormat::CsvIq => "Contenedores con Metadatos",
            IqFormat::RawCu8 | IqFormat::RawCu16 => "RAW Enteros Sin Signo (Unsigned)",
            IqFormat::RawInt8 | IqFormat::RawCs12Packed | IqFormat::RawSc16Q11 | IqFormat::RawSc16Full | IqFormat::RawCs24 | IqFormat::RawCs32 => "RAW Enteros Con Signo (Signed)",
            IqFormat::RawCf16 | IqFormat::RawFloat32 | IqFormat::RawCf64 => "RAW Punto Flotante (Float)",
        }
    }

    /// Nombre contextual y claro para desplegables de interfaz de usuario.
    pub fn short_name(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "WAV I/Q Estéreo (SDR#, SDR++, Gqrx)",
            IqFormat::WavRf64 => "WAV RF64 (>4 GB - SDR#, SDR Console)",
            IqFormat::SigMf => "SigMF Dataset (.sigmf-data + .sigmf-meta)",
            IqFormat::WavAudioMono => "WAV Audio Mono (Subportadora audible)",
            IqFormat::CsvIq => "CSV I/Q (Texto plano ASCII)",
            IqFormat::RawCu8 => "RAW CU8 (RTL-SDR: rtl_sdr / rtl_tcp)",
            IqFormat::RawCu16 => "RAW CU16 (uint16 Industrial)",
            IqFormat::RawInt8 => "RAW CS8 / Int8 (HackRF: hackrf_transfer)",
            IqFormat::RawCs12Packed => "RAW CS12 Packed (LimeSDR / bladeRF 12-bit)",
            IqFormat::RawSc16Q11 => "RAW SC16 Q11 (Nuand bladeRF nativo)",
            IqFormat::RawSc16Full => "RAW CS16 (USRP, PlutoSDR, Airspy, SDRplay)",
            IqFormat::RawCs24 => "RAW CS24 (24-bit Alto Rango Dinámico)",
            IqFormat::RawCs32 => "RAW CS32 (32-bit DSP / FPGA)",
            IqFormat::RawCf16 => "RAW CF16 (Half-precision Float16)",
            IqFormat::RawFloat32 => "RAW CF32 (GNU Radio File Sink / .cfile)",
            IqFormat::RawCf64 => "RAW CF64 (Double Float64 / Radioastronomía)",
        }
    }

    /// Nombre completo formal o técnico anterior para compatibilidad.
    pub fn name(&self) -> &'static str {
        self.short_name()
    }

    pub fn label(&self) -> &'static str {
        self.short_name()
    }

    /// Extensión de archivo sugerida por defecto.
    pub fn default_extension(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 | IqFormat::WavAudioMono => "wav",
            IqFormat::WavRf64 => "rf64",
            IqFormat::SigMf => "sigmf-data",
            IqFormat::CsvIq => "csv",
            IqFormat::RawCu8 => "cu8",
            IqFormat::RawCu16 => "u16",
            IqFormat::RawInt8 => "raw",
            IqFormat::RawCs12Packed => "cs12",
            IqFormat::RawSc16Q11 | IqFormat::RawSc16Full => "bin",
            IqFormat::RawCs24 => "cs24",
            IqFormat::RawCs32 => "cs32",
            IqFormat::RawCf16 => "cf16",
            IqFormat::RawFloat32 => "iq",
            IqFormat::RawCf64 => "cf64",
        }
    }

    /// Tipo de dato elemental subyacente.
    pub fn data_type_desc(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "int16_t PCM en contenedor RIFF/WAV estándar",
            IqFormat::WavRf64 => "int16_t PCM en contenedor RF64 con chunk ds64",
            IqFormat::SigMf => "cf32_le / cs16_le con cabecera externa JSON",
            IqFormat::WavAudioMono => "int16_t PCM mono en contenedor RIFF/WAV",
            IqFormat::CsvIq => "ASCII float pares (I, Q) por renglón",
            IqFormat::RawCu8 => "uint8_t sin signo (unsigned char)",
            IqFormat::RawCu16 => "uint16_t Little-Endian sin signo",
            IqFormat::RawInt8 => "int8_t con signo (signed char)",
            IqFormat::RawCs12Packed => "int12 empaquetado (3 bytes por par I/Q)",
            IqFormat::RawSc16Q11 => "int16_t Little-Endian (Punto fijo Q11)",
            IqFormat::RawSc16Full => "int16_t Little-Endian (Escala completa)",
            IqFormat::RawCs24 => "int24 Little-Endian de 3 bytes por componente",
            IqFormat::RawCs32 => "int32_t Little-Endian con signo",
            IqFormat::RawCf16 => "float16 IEEE 754 Half-Precision Little-Endian",
            IqFormat::RawFloat32 => "float32 IEEE 754 Single-Precision Little-Endian",
            IqFormat::RawCf64 => "float64 IEEE 754 Double-Precision Little-Endian",
        }
    }

    /// Número de bytes por muestra compleja (par I/Q).
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            IqFormat::WavIq16 => 4,
            IqFormat::WavRf64 => 4,
            IqFormat::SigMf => 8,
            IqFormat::WavAudioMono => 2,
            IqFormat::CsvIq => 16,
            IqFormat::RawCu8 => 2,
            IqFormat::RawCu16 => 4,
            IqFormat::RawInt8 => 2,
            IqFormat::RawCs12Packed => 3,
            IqFormat::RawSc16Q11 => 4,
            IqFormat::RawSc16Full => 4,
            IqFormat::RawCs24 => 6,
            IqFormat::RawCs32 => 8,
            IqFormat::RawCf16 => 4,
            IqFormat::RawFloat32 => 8,
            IqFormat::RawCf64 => 16,
        }
    }

    /// Estructura de entrelazado de componentes I y Q.
    pub fn interleaving_desc(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 | IqFormat::WavRf64 => "2 Canales: Canal 1 (L) = I, Canal 2 (R) = Q",
            IqFormat::SigMf => "Entrelazado continuo: [I0, Q0, I1, Q1, ...]",
            IqFormat::WavAudioMono => "1 Canal mono (I = audio, Q = 0.0)",
            IqFormat::CsvIq => "Dos columnas separadas por coma: 'I, Q'",
            IqFormat::RawCu8 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (uint8)",
            IqFormat::RawCu16 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (uint16)",
            IqFormat::RawInt8 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (int8)",
            IqFormat::RawCs12Packed => "Empaquetado bit a bit continuo: [I0[11:0], Q0[11:0]] en 3 bytes",
            IqFormat::RawSc16Q11 | IqFormat::RawSc16Full => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (int16)",
            IqFormat::RawCs24 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (int24)",
            IqFormat::RawCs32 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (int32)",
            IqFormat::RawCf16 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (float16)",
            IqFormat::RawFloat32 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (float32)",
            IqFormat::RawCf64 => "Entrelazado continuo: [I0, Q0, I1, Q1, ...] (float64)",
        }
    }

    /// Rango dinámico y factor de normalización matemática al rango [-1.0, 1.0].
    pub fn dynamic_range_desc(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 | IqFormat::WavRf64 => "Enteros [-32768, 32767] -> Dividir entre 32768.0",
            IqFormat::SigMf => "Float normalizado [-1.0, 1.0] (o según tipo en metadatos)",
            IqFormat::WavAudioMono => "Enteros [-32768, 32767] -> Dividir entre 32768.0",
            IqFormat::CsvIq => "Valores normalizados [-1.0, 1.0] en texto plano",
            IqFormat::RawCu8 => "Enteros sin signo [0, 255] -> Restar 127.5 y dividir entre 127.5 (Cero en 127.5)",
            IqFormat::RawCu16 => "Enteros sin signo [0, 65535] -> Restar 32768.0 y dividir entre 32768.0 (Cero en 32768)",
            IqFormat::RawInt8 => "Enteros con signo [-128, 127] -> Dividir entre 128.0 (Cero en 0.0)",
            IqFormat::RawCs12Packed => "Enteros con signo [-2048, 2047] -> Dividir entre 2048.0 (12 bits empaquetados)",
            IqFormat::RawSc16Q11 => "Enteros [-2048, 2047] -> Dividir entre 2048.0 (Punto fijo bladeRF Q11)",
            IqFormat::RawSc16Full => "Enteros [-32768, 32767] -> Dividir entre 32768.0",
            IqFormat::RawCs24 => "Enteros [-8388608, 8388607] -> Dividir entre 8388608.0 (24-bit)",
            IqFormat::RawCs32 => "Enteros [-2147483648, 2147483647] -> Dividir entre 2147483648.0 (32-bit)",
            IqFormat::RawCf16 => "Continuo [-1.0, 1.0] IEEE 754 half precision",
            IqFormat::RawFloat32 => "Continuo [-1.0, 1.0] IEEE 754 float32 estándar GNU Radio",
            IqFormat::RawCf64 => "Continuo [-1.0, 1.0] IEEE 754 double precision float64",
        }
    }

    /// Punto cero de referencia en la codificación física.
    pub fn zero_point_desc(&self) -> &'static str {
        match self {
            IqFormat::RawCu8 => "127.5 (Unsigned uint8)",
            IqFormat::RawCu16 => "32768.0 (Unsigned uint16)",
            _ => "0.0 (Bipolar / Signed / Float)",
        }
    }

    /// Nombre base del archivo de función independiente en el monorepo iq-files.
    pub fn standalone_base_name(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "wav_iq",
            IqFormat::WavRf64 => "wav_rf64",
            IqFormat::SigMf => "sigmf",
            IqFormat::WavAudioMono => "wav_mono",
            IqFormat::CsvIq => "csv_iq",
            IqFormat::RawCu8 => "cu8",
            IqFormat::RawCu16 => "cu16",
            IqFormat::RawInt8 => "int8",
            IqFormat::RawCs12Packed => "cs12",
            IqFormat::RawSc16Q11 => "sc16q11",
            IqFormat::RawSc16Full => "sc16full",
            IqFormat::RawCs24 => "cs24",
            IqFormat::RawCs32 => "cs32",
            IqFormat::RawCf16 => "cf16",
            IqFormat::RawFloat32 => "grcomplex",
            IqFormat::RawCf64 => "cf64",
        }
    }

    /// Tupla con nombres de funciones standalone (load_<nombre>, save_<nombre>).
    pub fn standalone_function_names(&self) -> (&'static str, &'static str) {
        match self {
            IqFormat::WavIq16 => ("load_wav_iq", "save_wav_iq"),
            IqFormat::WavRf64 => ("load_wav_rf64", "save_wav_rf64"),
            IqFormat::SigMf => ("load_sigmf", "save_sigmf"),
            IqFormat::WavAudioMono => ("load_wav_mono", "save_wav_mono"),
            IqFormat::CsvIq => ("load_csv_iq", "save_csv_iq"),
            IqFormat::RawCu8 => ("load_cu8", "save_cu8"),
            IqFormat::RawCu16 => ("load_cu16", "save_cu16"),
            IqFormat::RawInt8 => ("load_int8", "save_int8"),
            IqFormat::RawCs12Packed => ("load_cs12", "save_cs12"),
            IqFormat::RawSc16Q11 => ("load_sc16q11", "save_sc16q11"),
            IqFormat::RawSc16Full => ("load_sc16full", "save_sc16full"),
            IqFormat::RawCs24 => ("load_cs24", "save_cs24"),
            IqFormat::RawCs32 => ("load_cs32", "save_cs32"),
            IqFormat::RawCf16 => ("load_cf16", "save_cf16"),
            IqFormat::RawFloat32 => ("load_grcomplex", "save_grcomplex"),
            IqFormat::RawCf64 => ("load_cf64", "save_cf64"),
        }
    }

    /// Hardware o ecosistema primario que suele originar o consumir este formato.
    pub fn primary_hardware_ecosystem(&self) -> &'static str {
        match self {
            IqFormat::WavIq16 => "SDR#, SDR++, Gqrx, Audacity, HDSDR",
            IqFormat::WavRf64 => "SDR#, SDR Console v3 (Grabaciones gigantes > 4 GB)",
            IqFormat::SigMf => "Comunidad SDR, GNU Radio, inspectrum, SigMF Project",
            IqFormat::WavAudioMono => "Dispositivos de audio, receptores SSB/VLF mono",
            IqFormat::CsvIq => "Análisis en MATLAB / Excel / Python",
            IqFormat::RawCu8 => "RTL-SDR (DVB-T Blog V3/V4, rtl_sdr, rtl_tcp)",
            IqFormat::RawCu16 => "ADCs industriales y digitalizadores embebidos",
            IqFormat::RawInt8 => "Great Scott Gadgets HackRF One (hackrf_transfer)",
            IqFormat::RawCs12Packed => "LimeSDR (LimeSuite), Nuand bladeRF (Modo packed USB)",
            IqFormat::RawSc16Q11 => "Nuand bladeRF (bladeRF-cli, libbladeRF)",
            IqFormat::RawSc16Full => "Ettus USRP, ADALM-Pluto (PlutoSDR), Airspy, SDRplay",
            IqFormat::RawCs24 => "Receptores de audio/VLF y SDRs de alto rango dinámico",
            IqFormat::RawCs32 => "Procesamiento interno DSP en FPGAs y SoCs",
            IqFormat::RawCf16 => "Inferencia Machine Learning / PyTorch / TensorRT",
            IqFormat::RawFloat32 => "GNU Radio (File Sink complex), Gqrx, SDR#, MATLAB",
            IqFormat::RawCf64 => "Radioastronomía, simulaciones de alta precisión",
        }
    }

    /// Comando CLI o bloque típico con el que se suele capturar la señal.
    pub fn typical_capture_command(&self) -> Option<&'static str> {
        match self {
            IqFormat::RawCu8 => Some("rtl_sdr -f 433.92M -s 2048000 -g 30 captura.cu8"),
            IqFormat::RawInt8 => Some("hackrf_transfer -r captura.raw -f 433920000 -s 2000000 -a 0 -x 20"),
            IqFormat::RawSc16Q11 => Some("bladeRF-cli -e 'set frequency rx 433.92M; set samplerate rx 2.88M; rx start file=captura.bin format=bin samples=2880000'"),
            IqFormat::RawFloat32 => Some("GNU Radio: Bloque 'File Sink' con Input Type = Complex"),
            IqFormat::WavIq16 => Some("SDR# o SDR++: Panel 'Record' seleccionando 'Baseband I/Q (16-bit PCM)'"),
            _ => None,
        }
    }

    /// Software y ecosistemas SDR comúnmente compatibles.
    pub fn compatible_software(&self) -> &'static [&'static str] {
        match self {
            IqFormat::WavIq16 => &["SDR#", "SDR++", "Gqrx", "Audacity", "HDSDR", "MATLAB", "Python"],
            IqFormat::WavRf64 => &["SDR#", "SDR Console", "HDSDR", "Audacity", "MATLAB", "Python"],
            IqFormat::SigMf => &["GNU Radio", "Inspectrum", "SigMF GUI", "MATLAB", "Python"],
            IqFormat::WavAudioMono => &["Reproductores multimedia", "Audacity", "MATLAB", "Python"],
            IqFormat::CsvIq => &["Excel / Hojas de cálculo", "MATLAB", "Python", "Scripts de análisis"],
            IqFormat::RawCu8 => &["rtl_sdr", "rtl_tcp", "GNU Radio (UChar to Float)", "MATLAB", "Python"],
            IqFormat::RawCu16 => &["Sistemas industriales", "MATLAB", "Python"],
            IqFormat::RawInt8 => &["hackrf_transfer", "libhackrf", "GNU Radio (Byte to Float)", "MATLAB", "Python"],
            IqFormat::RawCs12Packed => &["LimeSuiteGUI", "bladeRF-cli", "MATLAB", "Python"],
            IqFormat::RawSc16Q11 => &["bladeRF-cli", "libbladeRF", "MATLAB", "Python", "GNU Radio"],
            IqFormat::RawSc16Full => &["UHD (USRP)", "IIO Oscilloscope (Pluto)", "Airspy", "MATLAB", "Python", "GNU Radio"],
            IqFormat::RawCs24 => &["SDRs de audio", "MATLAB", "Python"],
            IqFormat::RawCs32 => &["FPGA pipelines", "GNU Radio (Int to Float)", "MATLAB", "Python"],
            IqFormat::RawCf16 => &["PyTorch", "TensorFlow", "CuPy", "MATLAB", "Python"],
            IqFormat::RawFloat32 => &["GNU Radio (File Source)", "Gqrx", "SDR#", "MATLAB", "Python"],
            IqFormat::RawCf64 => &["Radioastronomía", "SciPy", "MATLAB", "Python"],
        }
    }

    /// Lista completa de todos los formatos disponibles para iteración en UI.
    pub fn all() -> &'static [IqFormat] {
        &[
            IqFormat::WavIq16,
            IqFormat::WavRf64,
            IqFormat::SigMf,
            IqFormat::RawCu8,
            IqFormat::RawInt8,
            IqFormat::RawSc16Full,
            IqFormat::RawSc16Q11,
            IqFormat::RawCs12Packed,
            IqFormat::RawFloat32,
            IqFormat::RawCf16,
            IqFormat::RawCf64,
            IqFormat::RawCu16,
            IqFormat::RawCs24,
            IqFormat::RawCs32,
            IqFormat::WavAudioMono,
            IqFormat::CsvIq,
        ]
    }
}
