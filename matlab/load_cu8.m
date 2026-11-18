function iq = load_cu8(filename)
% LOAD_CU8 Lee muestras I/Q en formato RAW CU8 (uint8 sin signo con cero en 127.5).
%
% ¿CUÁNDO USAR ESTE FORMATO?
%   - Hardware típico: Receptores RTL-SDR (DVB-T Blog V3/V4 basados en chip RTL2832U).
%   - Software emisor: Grabaciones de la utilidad 'rtl_sdr' o streams de 'rtl_tcp'.
%   - Ejemplo de comando típico:
%       rtl_sdr -f 433.92M -s 2048000 -g 30 captura.cu8
%
% ESPECIFICACIONES TÉCNICAS:
%   - Tipo primitivo: uint8 (0 a 255).
%   - Entrelazado: [I0, Q0, I1, Q1, ...]
%   - Punto cero: 127.5
%   - Normalización a double [-1.0, 1.0]: (double(val) - 127.5) / 127.5

fid = fopen(filename, 'rb');
if fid == -1
    error('No se pudo abrir el archivo: %s', filename);
end

raw = fread(fid, 'uint8=>double');
fclose(fid);

if mod(length(raw), 2) ~= 0
    raw = raw(1:end-1);
end

% Desplazar cero y normalizar
raw = (raw - 127.5) / 127.5;

i_samples = raw(1:2:end);
q_samples = raw(2:2:end);
iq = complex(i_samples, q_samples);
end
