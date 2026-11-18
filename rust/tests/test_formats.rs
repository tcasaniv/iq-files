use iq_files::{read_iq_memory, write_iq_memory, IqFormat};

#[test]
fn test_roundtrip_sc16q11() {
    let original: Vec<(f32, f32)> = vec![
        (0.0, 0.0),
        (0.5, -0.5),
        (0.999, -0.999),
        (-1.0, 1.0),
    ];
    let bytes = write_iq_memory(&original, 288_000, IqFormat::RawSc16Q11).expect("write failed");
    let (decoded, _) = read_iq_memory(&bytes, IqFormat::RawSc16Q11).expect("read failed");

    assert_eq!(original.len(), decoded.len());
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert!((orig.0 - dec.0).abs() < 0.002, "I mismatch: orig={}, dec={}", orig.0, dec.0);
        assert!((orig.1 - dec.1).abs() < 0.002, "Q mismatch: orig={}, dec={}", orig.1, dec.1);
    }
}

#[test]
fn test_roundtrip_sc16full() {
    let original: Vec<(f32, f32)> = vec![
        (0.0, 0.0),
        (0.707, -0.707),
        (-0.5, 0.5),
    ];
    let bytes = write_iq_memory(&original, 288_000, IqFormat::RawSc16Full).expect("write failed");
    let (decoded, _) = read_iq_memory(&bytes, IqFormat::RawSc16Full).expect("read failed");

    assert_eq!(original.len(), decoded.len());
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert!((orig.0 - dec.0).abs() < 0.0001);
        assert!((orig.1 - dec.1).abs() < 0.0001);
    }
}

#[test]
fn test_roundtrip_grcomplex() {
    let original: Vec<(f32, f32)> = vec![
        (0.12345, -0.6789),
        (-0.999, 0.888),
    ];
    let bytes = write_iq_memory(&original, 1_000_000, IqFormat::RawFloat32).expect("write failed");
    let (decoded, sr) = read_iq_memory(&bytes, IqFormat::RawFloat32).expect("read failed");

    assert_eq!(sr, 1_000_000);
    assert_eq!(original.len(), decoded.len());
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert!((orig.0 - dec.0).abs() < 1e-6);
        assert!((orig.1 - dec.1).abs() < 1e-6);
    }
}

#[test]
fn test_roundtrip_int8() {
    let original: Vec<(f32, f32)> = vec![
        (0.0, 0.0),
        (0.5, -0.5),
        (1.0, -1.0),
    ];
    let bytes = write_iq_memory(&original, 2_000_000, IqFormat::RawInt8).expect("write failed");
    let (decoded, _) = read_iq_memory(&bytes, IqFormat::RawInt8).expect("read failed");

    assert_eq!(original.len(), decoded.len());
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert!((orig.0 - dec.0).abs() < 0.02);
        assert!((orig.1 - dec.1).abs() < 0.02);
    }
}

#[test]
fn test_roundtrip_wav_iq() {
    let original: Vec<(f32, f32)> = vec![
        (0.25, -0.75),
        (-0.5, 0.5),
    ];
    let bytes = write_iq_memory(&original, 48_000, IqFormat::WavIq16).expect("write failed");
    let (decoded, sr) = read_iq_memory(&bytes, IqFormat::WavIq16).expect("read failed");

    assert_eq!(sr, 48_000);
    assert_eq!(original.len(), decoded.len());
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert!((orig.0 - dec.0).abs() < 0.0001);
        assert!((orig.1 - dec.1).abs() < 0.0001);
    }
}

#[test]
fn test_roundtrip_csv_iq() {
    let original: Vec<(f32, f32)> = vec![
        (0.1234, -0.5678),
        (0.9999, -0.9999),
    ];
    let bytes = write_iq_memory(&original, 288_000, IqFormat::CsvIq).expect("write failed");
    let (decoded, _) = read_iq_memory(&bytes, IqFormat::CsvIq).expect("read failed");

    assert_eq!(original.len(), decoded.len());
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert!((orig.0 - dec.0).abs() < 0.0001);
        assert!((orig.1 - dec.1).abs() < 0.0001);
    }
}
