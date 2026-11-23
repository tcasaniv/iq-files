use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::Path;

/// Guarda muestras complejas I/Q en un archivo de texto plano CSV (columnas I, Q).
pub fn save_csv_iq(path: &Path, samples: &[(f32, f32)]) -> IoResult<()> {
    let text = save_csv_iq_to_string(samples);
    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

/// Alias explícito para guardado a archivo en disco.
pub fn save_csv_iq_to_file(path: &Path, samples: &[(f32, f32)]) -> IoResult<()> {
    save_csv_iq(path, samples)
}


/// Codifica muestras complejas I/Q en un string en formato CSV.
pub fn save_csv_iq_to_string(samples: &[(f32, f32)]) -> String {
    let mut out = String::with_capacity(samples.len() * 24);
    out.push_str("# I, Q\n");
    for &(i, q) in samples {
        use std::fmt::Write;
        let _ = writeln!(&mut out, "{:.7}, {:.7}", i, q);
    }
    out
}
