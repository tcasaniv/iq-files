function [signal, sample_rate, signal_i, signal_q] = load_wav_iq(filename)
% LOAD_WAV_IQ Lee una señal compleja desde un archivo WAV de 2 canales PCM (16 bits).
%
%   [SIGNAL, SAMPLE_RATE, SIGNAL_I, SIGNAL_Q] = load_wav_iq(FILENAME)

    [data, sample_rate] = audioread(filename);
    if size(data, 2) < 2
        % Si es mono, duplicar o asumir Q=0
        signal_i = data(:, 1);
        signal_q = zeros(size(signal_i));
    else
        signal_i = data(:, 1);
        signal_q = data(:, 2);
    end
    signal = signal_i + 1j .* signal_q;
end
