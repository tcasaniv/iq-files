function [ret] = save_int8(filename, signal)
% SAVE_INT8 Escribe una señal compleja normalizada a un archivo binario
%           en formato RAW signed 8-bit (HackRF / RTL-SDR).
%
%   [RET] = save_int8(FILENAME, SIGNAL)

    [f, err_msg] = fopen(filename, 'w');
    if f == -1
        error(err_msg);
        ret = 0;
        return;
    end

    sig_i = round(real(signal) .* 127.0);
    sig_i(sig_i > 127)  = 127;
    sig_i(sig_i < -128) = -128;

    sig_q = round(imag(signal) .* 127.0);
    sig_q(sig_q > 127)  = 127;
    sig_q(sig_q < -128) = -128;

    sig_len = 2 * length(sig_i);
    sig_out = zeros(1, sig_len, 'int8');
    sig_out(1:2:sig_len - 1) = int8(sig_i);
    sig_out(2:2:sig_len)     = int8(sig_q);

    count = fwrite(f, sig_out, 'int8');
    fclose(f);

    ret = (count == sig_len);
end
