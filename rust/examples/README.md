# Ejemplos de uso en Rust

Esta carpeta contiene ejemplos ejecutables con Cargo que ilustran la decodificación y serialización de señales complejas I/Q.

## Ejecución

Para ejecutar el ejemplo básico de conversión:

```bash
cargo run --example basic_conversion
```

## Contenido del ejemplo (`basic_conversion.rs`)

1. **Síntesis**: Generación de un tono analítico en cuadratura a 1000 Hz con tasa de 48 kSps.
2. **bladeRF (`sc16q11`)**: Guardado y lectura de enteros de 16 bits normalizados a $[-2048, 2047]$.
3. **RTL-SDR (`cu8`)**: Guardado y lectura de enteros de 8 bits sin signo con cero en $127.5$.
4. **Audio / SDR WAV (`wav_iq`)**: Guardado y lectura en formato RIFF WAV PCM estéreo de 16 bits.
5. **Verificación de precisión**: Cálculo del error residual numérico entre la señal sintetizada y la recuperada.
