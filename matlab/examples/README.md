# Ejemplos de uso en MATLAB / GNU Octave

Esta carpeta contiene scripts interactivos para probar y aprender a utilizar las funciones de lectura y escritura I/Q de `iq-files`.

## Ejecución

1. Abra MATLAB o GNU Octave en esta carpeta o en la raíz del repositorio.
2. Asegúrese de que las funciones de `iq-files` estén en la ruta de búsqueda:
   ```matlab
   addpath('..');
   ```
3. Ejecute el script de demostración:
   ```matlab
   example_basic_io
   ```

## Contenido del ejemplo

- **Sintetización**: Generación de un tono analítico complejo I/Q puro $e^{j 2\pi f t}$.
- **I/O Directo**: Uso de `save_sc16q11` / `load_sc16q11`, `save_cu8` / `load_cu8`, `save_cs12` / `load_cs12`.
- **API Unificada**: Uso de `iq_files('write', filename, signal, fs, format)` y `[signal, fs] = iq_files('read', filename, format)`.
- **Verificación**: Comparación de señal recuperada contra señal original.
