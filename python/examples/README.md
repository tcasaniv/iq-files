# Ejemplos de uso en Python

Esta carpeta contiene ejemplos prácticos y reproducibles del uso de `iq-files` en Python.

## Estructura

- `quickstart.py`: Demuestra el flujo de trabajo estándar utilizando el paquete instalado (`pip install iq-files`) o desde el código fuente del proyecto (`src-layout`). Cubre la síntesis de señales de prueba, conversión entre múltiples formatos SDR (`SC16 Q11`, `CU8`, `CS12`, `WAV`), y el uso de la API unificada `read_iq` / `write_iq`.
- `standalone_demo.py`: Demuestra el uso de los módulos independientes de la carpeta `standalone/`, ideal para proyectos que requieran copiar únicamente uno o dos archivos `.py` sin empaquetado formal.

## Ejecución

### 1. Con el paquete instalado
```bash
python quickstart.py
```

### 2. Modo directo desde el repositorio
```bash
PYTHONPATH=../src python quickstart.py
python standalone_demo.py
```
