function [signal, signal_i, signal_q] = load_cs12(filename)
% LOAD_CS12 Lee una señal compleja desde un archivo binario RAW CS12 Packed
%           (12 bits con signo empaquetados, 3 bytes por muestra compleja).
%
%   ¿CUÁNDO USAR ESTE FORMATO?
%     - Hardware típico: LimeSDR (LMS7002M) y bladeRF en modo 12-bit packed.
%     - Rango de enteros: [-2048, 2047] normalizados a [-1.0, 1.0].
%
%   [SIGNAL, SIGNAL_I, SIGNAL_Q] = load_cs12(FILENAME)

    [fid, err_msg] = fopen(filename, 'rb');
    if fid == -1
        error(err_msg);
    end

    raw = fread(fid, 'uint8=>uint32');
    fclose(fid);

    n_samples = floor(length(raw) / 3);
    if n_samples == 0
        signal = [];
        signal_i = [];
        signal_q = [];
        return;
    end

    raw = raw(1:n_samples * 3);
    b0 = raw(1:3:end);
    b1 = raw(2:3:end);
    b2 = raw(3:3:end);

    i_raw = bitshift(bitand(b1, 15), 8) + b0;
    q_raw = bitshift(b2, 4) + bitshift(b1, -4);

    % Extensión de signo de 12 bits a entero con signo
    i_raw = double(i_raw);
    q_raw = double(q_raw);
    i_raw(i_raw >= 2048) = i_raw(i_raw >= 2048) - 4096;
    q_raw(q_raw >= 2048) = q_raw(q_raw >= 2048) - 4096;

    signal_i = i_raw / 2048.0;
    signal_q = q_raw / 2048.0;
    signal = signal_i + 1j * signal_q;
end
