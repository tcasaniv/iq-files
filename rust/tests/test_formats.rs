use iq_files::*;

#[test]
fn test_roundtrip_sc16q11() {
    let original = vec![(0.5f32, -0.25f32), (0.0f32, 0.75f32)];
    let bytes = save_sc16q11_to_memory(&original);
    let decoded = load_sc16q11_from_memory(&bytes);
    assert_eq!(decoded.len(), original.len());
    for ((orig_i, orig_q), (dec_i, dec_q)) in original.iter().zip(decoded.iter()) {
        assert!((orig_i - dec_i).abs() < 0.002);
        assert!((orig_q - dec_q).abs() < 0.002);
    }
}

#[test]
fn test_roundtrip_cu8() {
    let original = vec![(0.5f32, -0.5f32), (0.0f32, 1.0f32)];
    let bytes = save_cu8_to_memory(&original);
    let decoded = load_cu8_from_memory(&bytes);
    assert_eq!(decoded.len(), original.len());
    for ((orig_i, orig_q), (dec_i, dec_q)) in original.iter().zip(decoded.iter()) {
        assert!((orig_i - dec_i).abs() < 0.02);
        assert!((orig_q - dec_q).abs() < 0.02);
    }
}

#[test]
fn test_roundtrip_cs12() {
    let original = vec![(0.25f32, -0.75f32), (0.5f32, 0.0f32)];
    let bytes = save_cs12_to_memory(&original);
    let decoded = load_cs12_from_memory(&bytes);
    assert_eq!(decoded.len(), original.len());
    for ((orig_i, orig_q), (dec_i, dec_q)) in original.iter().zip(decoded.iter()) {
        assert!((orig_i - dec_i).abs() < 0.005);
        assert!((orig_q - dec_q).abs() < 0.005);
    }
}

#[test]
fn test_roundtrip_cf16() {
    let original = vec![(0.123f32, -0.456f32), (0.875f32, 0.0f32)];
    let bytes = save_cf16_to_memory(&original);
    let decoded = load_cf16_from_memory(&bytes);
    assert_eq!(decoded.len(), original.len());
    for ((orig_i, orig_q), (dec_i, dec_q)) in original.iter().zip(decoded.iter()) {
        assert!((orig_i - dec_i).abs() < 0.002);
        assert!((orig_q - dec_q).abs() < 0.002);
    }
}

#[test]
fn test_roundtrip_wav_rf64() {
    let original = vec![(0.3f32, -0.6f32), (-0.1f32, 0.9f32)];
    let bytes = save_wav_rf64_to_memory(&original, 1_000_000).expect("save rf64");
    let (decoded, sr) = load_wav_rf64_from_memory(&bytes).expect("load rf64");
    assert_eq!(sr, 1_000_000);
    assert_eq!(decoded.len(), original.len());
    for ((orig_i, orig_q), (dec_i, dec_q)) in original.iter().zip(decoded.iter()) {
        assert!((orig_i - dec_i).abs() < 0.001);
        assert!((orig_q - dec_q).abs() < 0.001);
    }
}

#[test]
fn test_roundtrip_grcomplex() {
    let original = vec![(0.123456f32, -0.987654f32)];
    let bytes = save_grcomplex_to_memory(&original);
    let decoded = load_grcomplex_from_memory(&bytes);
    assert_eq!(decoded.len(), original.len());
    assert!((decoded[0].0 - original[0].0).abs() < 1e-6);
    assert!((decoded[0].1 - original[0].1).abs() < 1e-6);
}
