function [ret] = save_sc16full(filename, signal)
% SAVE_SC16FULL Escribe una señal compleja normalizada a un archivo binario
%               en formato RAW SC16 escala completa (int16 Little-Endian).
%
%   [RET] = save_sc16full(FILENAME, SIGNAL)

    [f, err_msg] = fopen(filename, 'w', 'ieee-le');
    if f == -1
        error(err_msg);
        ret = 0;
        return;
    end

    sig_i = round(real(signal) .* 32767.0);
    sig_i(sig_i > 32767)  = 32767;
    sig_i(sig_i < -32768) = -32768;

    sig_q = round(imag(signal) .* 32767.0);
    sig_q(sig_q > 32767)  = 32767;
    sig_q(sig_q < -32768) = -32768;

    sig_len = 2 * length(sig_i);
    sig_out = zeros(1, sig_len, 'int16');
    sig_out(1:2:sig_len - 1) = int16(sig_i);
    sig_out(2:2:sig_len)     = int16(sig_q);

    count = fwrite(f, sig_out, 'int16');
    fclose(f);

    ret = (count == sig_len);
end
