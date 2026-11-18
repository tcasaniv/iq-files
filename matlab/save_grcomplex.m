function [ret] = save_grcomplex(filename, signal)
% SAVE_GRCOMPLEX Escribe una señal compleja normalizada a un archivo binario
%                en formato GNU Radio complex float32 (Little-Endian).
%
%   [RET] = save_grcomplex(FILENAME, SIGNAL)

    [f, err_msg] = fopen(filename, 'w', 'ieee-le');
    if f == -1
        error(err_msg);
        ret = 0;
        return;
    end

    sig_i = single(real(signal));
    sig_q = single(imag(signal));

    sig_len = 2 * length(sig_i);
    sig_out = zeros(1, sig_len, 'single');
    sig_out(1:2:sig_len - 1) = sig_i;
    sig_out(2:2:sig_len)     = sig_q;

    count = fwrite(f, sig_out, 'float32');
    fclose(f);

    ret = (count == sig_len);
end
