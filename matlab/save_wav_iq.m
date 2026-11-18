function [ret] = save_wav_iq(filename, signal, sample_rate)
% SAVE_WAV_IQ Escribe una señal compleja a un archivo WAV de 2 canales PCM (16 bits).
%             Canal 1 (L): En fase (I)
%             Canal 2 (R): Cuadratura (Q)
%
%   [RET] = save_wav_iq(FILENAME, SIGNAL, SAMPLE_RATE)

    if nargin < 3
        sample_rate = 48000;
    end

    data = [real(signal(:)), imag(signal(:))];
    audiowrite(filename, data, sample_rate, 'BitsPerSample', 16);
    ret = 1;
end
