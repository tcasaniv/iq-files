function [signal, signal_i, signal_q] = load_sc16q11(filename)
% LOAD_SC16Q11 Lee una señal compleja desde un archivo binario
%              en el formato bladeRF "SC16 Q11".
%
%   [SIGNAL, SIGNAL_I, SIGNAL_Q] = load_sc16q11(FILENAME)
%
%   SIGNAL es el vector complejo normalizado en el rango [-1.0, 1.0].
%   SIGNAL_I y SIGNAL_Q son las componentes en fase y cuadratura opcionales.

    [f, err_msg] = fopen(filename, 'r', 'ieee-le');
    if f == -1
        error(err_msg);
    end

    raw = fread(f, [2, Inf], 'int16');
    fclose(f);

    signal_i = raw(1, :) ./ 2048.0;
    signal_q = raw(2, :) ./ 2048.0;
    signal = signal_i + 1j .* signal_q;
end
