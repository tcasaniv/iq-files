function [ret] = save_cs12(filename, signal)
% SAVE_CS12 Guarda una señal compleja normalizada en formato RAW CS12 Packed
%           (12 bits con signo empaquetados, 3 bytes por muestra compleja).
%
%   [RET] = save_cs12(FILENAME, SIGNAL)

    if ~isvector(signal)
        error('Las muestras I/Q deben ser un vector.');
    end

    signal = signal(:);
    n = length(signal);

    i_val = round(real(signal) * 2047.0);
    i_val(i_val > 2047)  = 2047;
    i_val(i_val < -2048) = -2048;

    q_val = round(imag(signal) * 2047.0);
    q_val(q_val > 2047)  = 2047;
    q_val(q_val < -2048) = -2048;

    % Mapear a enteros de 12 bits sin signo (0 a 4095)
    i_u12 = uint32(mod(i_val + 4096, 4096));
    q_u12 = uint32(mod(q_val + 4096, 4096));

    b0 = uint8(bitand(i_u12, 255));
    b1 = uint8(bitshift(bitand(i_u12, 3840), -8) + bitshift(bitand(q_u12, 15), 4));
    b2 = uint8(bitshift(bitand(q_u12, 4080), -4));

    bytes = zeros(3 * n, 1, 'uint8');
    bytes(1:3:end) = b0;
    bytes(2:3:end) = b1;
    bytes(3:3:end) = b2;

    [fid, err_msg] = fopen(filename, 'wb');
    if fid == -1
        error(err_msg);
    end

    count = fwrite(fid, bytes, 'uint8');
    fclose(fid);

    ret = (count == 3 * n);
end
