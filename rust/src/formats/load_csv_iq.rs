use std::fs::File;
use std::io::{Read, Result as IoResult};
use std::path::Path;

/// Lee muestras complejas I/Q desde un archivo de texto plano CSV (columnas I, Q).
pub fn load_csv_iq(path: &Path) -> IoResult<Vec<(f32, f32)>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(load_csv_iq_from_str(&content))
}

/// Decodifica muestras complejas I/Q desde un string en formato CSV.
pub fn load_csv_iq_from_str(text: &str) -> Vec<(f32, f32)> {
    let mut samples = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('%') {
            continue;
        }
        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() >= 2 {
            if let (Ok(i), Ok(q)) = (parts[0].trim().parse::<f32>(), parts[1].trim().parse::<f32>()) {
                samples.push((i, q));
            }
        }
    }
    samples
}
