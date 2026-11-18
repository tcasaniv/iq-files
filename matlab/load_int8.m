function [signal, signal_i, signal_q] = load_int8(filename)
% LOAD_INT8 Lee una señal compleja desde un archivo binario
%           en formato RAW signed 8-bit (HackRF / RTL-SDR).
%
%   [SIGNAL, SIGNAL_I, SIGNAL_Q] = load_int8(FILENAME)

    [f, err_msg] = fopen(filename, 'r');
    if f == -1
        error(err_msg);
    end

    raw = fread(f, [2, Inf], 'int8');
    fclose(f);

    signal_i = raw(1, :) ./ 128.0;
    signal_q = raw(2, :) ./ 128.0;
    signal = signal_i + 1j .* signal_q;
end
