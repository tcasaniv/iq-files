function [signal, signal_i, signal_q] = load_grcomplex(filename)
% LOAD_GRCOMPLEX Lee una señal compleja desde un archivo binario
%                en formato GNU Radio complex float32 (Little-Endian).
%
%   [SIGNAL, SIGNAL_I, SIGNAL_Q] = load_grcomplex(FILENAME)

    [f, err_msg] = fopen(filename, 'r', 'ieee-le');
    if f == -1
        error(err_msg);
    end

    raw = fread(f, [2, Inf], 'float32');
    fclose(f);

    signal_i = raw(1, :);
    signal_q = raw(2, :);
    signal = signal_i + 1j .* signal_q;
end
