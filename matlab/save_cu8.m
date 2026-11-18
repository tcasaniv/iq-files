function save_cu8(filename, iq)
% SAVE_CU8 Guarda muestras complejas I/Q en formato RAW CU8 (uint8 con cero en 127.5).
%
% ¿CUÁNDO USAR ESTE FORMATO?
%   - Emulación de capturas RTL-SDR para pruebas de decodificadores ADS-B, ACARS,
%     FM o pipelines que consuman streams de rtl_sdr / rtl_tcp.
%
% ESPECIFICACIONES TÉCNICAS:
%   - Tipo primitivo: uint8
%   - Entrelazado: [I0, Q0, I1, Q1, ...]

if ~isvector(iq)
    error('Las muestras I/Q deben ser un vector.');
end

iq = iq(:);
i_part = max(0, min(255, round(real(iq) * 127.5 + 127.5)));
q_part = max(0, min(255, round(imag(iq) * 127.5 + 127.5)));

interleaved = zeros(2 * length(iq), 1, 'uint8');
interleaved(1:2:end) = uint8(i_part);
interleaved(2:2:end) = uint8(q_part);

fid = fopen(filename, 'wb');
if fid == -1
    error('No se pudo abrir el archivo para escritura: %s', filename);
end

fwrite(fid, interleaved, 'uint8');
fclose(fid);
end
