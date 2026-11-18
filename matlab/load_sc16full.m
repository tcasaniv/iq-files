function [signal, signal_i, signal_q] = load_sc16full(filename)
% LOAD_SC16FULL Lee una señal compleja desde un archivo binario
%               en formato RAW SC16 escala completa (int16 Little-Endian).
%
%   [SIGNAL, SIGNAL_I, SIGNAL_Q] = load_sc16full(FILENAME)

    [f, err_msg] = fopen(filename, 'r', 'ieee-le');
    if f == -1
        error(err_msg);
    end

    raw = fread(f, [2, Inf], 'int16');
    fclose(f);

    signal_i = raw(1, :) ./ 32768.0;
    signal_q = raw(2, :) ./ 32768.0;
    signal = signal_i + 1j .* signal_q;
end
