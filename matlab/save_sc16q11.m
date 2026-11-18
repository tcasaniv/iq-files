function [ret] = save_sc16q11(filename, signal)
% SAVE_SC16Q11 Escribe una señal compleja normalizada a un archivo binario
%              en el formato bladeRF "SC16 Q11".
%
%   [RET] = save_sc16q11(FILENAME, SIGNAL)
%
%   RET es 1 en caso de éxito, y 0 en caso de fallo.
%   FILENAME es la ruta del archivo de destino (Little-Endian).
%   SIGNAL es un vector complejo normalizado en el rango [-1.0, 1.0].

    [f, err_msg] = fopen(filename, 'w', 'ieee-le');
    if f == -1
        error(err_msg);
        ret = 0;
        return;
    end

    sig_i = round(real(signal) .* 2048.0);
    sig_i(sig_i > 2047)  = 2047;
    sig_i(sig_i < -2048) = -2048;

    sig_q = round(imag(signal) .* 2048.0);
    sig_q(sig_q > 2047)  = 2047;
    sig_q(sig_q < -2048) = -2048;

    sig_len = 2 * length(sig_i);
    sig_out = zeros(1, sig_len, 'int16');
    sig_out(1:2:sig_len - 1) = int16(sig_i);
    sig_out(2:2:sig_len)     = int16(sig_q);

    count = fwrite(f, sig_out, 'int16');
    fclose(f);

    ret = (count == sig_len);
end
