function [ret] = save_csv_iq(filename, signal)
% SAVE_CSV_IQ Guarda muestras complejas I/Q en formato CSV (columnas I, Q).
%
%   [RET] = save_csv_iq(FILENAME, SIGNAL)

    data = [real(signal(:)), imag(signal(:))];
    writematrix(data, filename);
    ret = 1;
end
