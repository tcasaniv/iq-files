%% Demostración básica de uso de iq-files en MATLAB / GNU Octave
%
% Este script muestra cómo:
% 1. Sintetizar un tono I/Q analítico (48 kHz, tono de 1 kHz).
% 2. Exportar e importar en formato bladeRF SC16 Q11.
% 3. Exportar e importar en formato RTL-SDR CU8.
% 4. Exportar e importar en formato LimeSDR / bladeRF CS12 Packed.
% 5. Exportar e importar en formato WAV I/Q estéreo.
% 6. Utilizar la función unificadora iq_files('write', ...) y iq_files('read', ...).

clear; clc;

% Asegurar que la carpeta raíz de matlab esté en el PATH
addpath('..');

fprintf('=== Demostración de iq-files en MATLAB / Octave ===\n\n');

%% 1. Sintetizar tono complejo I/Q
fs = 48000;              % 48 kSps
duration = 0.1;          % 100 ms
f_tone = 1000;           % Tono a 1000 Hz
t = 0:(1/fs):(duration - 1/fs);
signal = 0.75 * exp(1j * 2 * pi * f_tone * t);

fprintf('1. Señal sintetizada: %d muestras complejas a %d Hz.\n', length(signal), fs);

%% 2. Guardar y leer usando funciones individuales (bladeRF SC16 Q11)
file_bladerf = fullfile(tempdir, 'demo_bladerf.sc16q11');
save_sc16q11(file_bladerf, signal);
loaded_bladerf = load_sc16q11(file_bladerf);
fprintf('2. bladeRF SC16 Q11: %d muestras leídas exitosamente.\n', length(loaded_bladerf));
delete(file_bladerf);

%% 3. Guardar y leer usando funciones individuales (RTL-SDR CU8)
file_rtlsdr = fullfile(tempdir, 'demo_rtlsdr.cu8');
save_cu8(file_rtlsdr, signal);
loaded_rtlsdr = load_cu8(file_rtlsdr);
fprintf('3. RTL-SDR CU8: %d muestras leídas exitosamente.\n', length(loaded_rtlsdr));
delete(file_rtlsdr);

%% 4. Guardar y leer usando funciones individuales (LimeSDR CS12 Packed)
file_cs12 = fullfile(tempdir, 'demo_cs12.cs12');
save_cs12(file_cs12, signal);
loaded_cs12 = load_cs12(file_cs12);
fprintf('4. LimeSDR CS12: %d muestras leídas exitosamente.\n', length(loaded_cs12));
delete(file_cs12);

%% 5. Guardar y leer usando la función unificada iq_files
file_wav = fullfile(tempdir, 'demo_stereo.wav');
iq_files('write', file_wav, signal, fs, 'wav_iq');
[loaded_wav, fs_read] = iq_files('read', file_wav, 'wav_iq');
fprintf('5. WAV I/Q unificado: %d muestras a %d Hz.\n', length(loaded_wav), fs_read);
delete(file_wav);

%% 6. Verificación de precisión
max_diff = max(abs(signal(:) - loaded_wav(:)));
fprintf('6. Error residual WAV vs Original: %.6f\n', max_diff);

fprintf('\n¡Demostración completada con éxito!\n');
