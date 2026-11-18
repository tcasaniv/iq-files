function [signal, signal_i, signal_q] = load_csv_iq(filename)
% LOAD_CSV_IQ Lee una señal compleja desde un archivo CSV con columnas I, Q.
%
%   [SIGNAL, SIGNAL_I, SIGNAL_Q] = load_csv_iq(FILENAME)

    raw = readmatrix(filename);
    signal_i = raw(:, 1);
    signal_q = raw(:, 2);
    signal = signal_i + 1j .* signal_q;
end
