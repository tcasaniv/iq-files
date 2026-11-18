function [varargout] = iq_files(action, filename, varargin)
% IQ_FILES Función unificadora para leer y escribir señales I/Q en MATLAB/Octave.
%
%   USO PARA GUARDAR:
%     iq_files('write', filename, signal, sample_rate, format_type)
%     iq_files('save',  filename, signal, sample_rate, format_type)
%
%   USO PARA LEER:
%     [signal, fs] = iq_files('read', filename, format_type)
%     [signal, fs] = iq_files('load', filename, format_type)
%
%   FORMATOS SOPORTADOS:
%     'sc16q11'   - bladeRF SC16 Q11 (int16 normalizado a [-2048, 2047])
%     'sc16full'  - RAW SC16 escala completa (int16 normalizado a [-32768, 32767])
%     'grcomplex' - GNU Radio complex float32
%     'float32'   - Alias de grcomplex
%     'int8'      - HackRF / RTL-SDR signed 8-bit
%     'wav_iq'    - Archivo WAV I/Q estéreo (16 bits)
%     'csv_iq'    - Texto plano CSV con columnas I, Q

    action = lower(action);

    if strcmp(action, 'write') || strcmp(action, 'save')
        if nargin < 3
            error('Debe proporcionar la señal a guardar.');
        end
        signal = varargin{1};
        sample_rate = 288000;
        format_type = 'sc16q11';

        if nargin >= 4 && ~isempty(varargin{2})
            sample_rate = varargin{2};
        end
        if nargin >= 5 && ~isempty(varargin{3})
            format_type = lower(varargin{3});
        end

        switch format_type
            case 'sc16q11'
                ret = save_sc16q11(filename, signal);
            case 'sc16full'
                ret = save_sc16full(filename, signal);
            case {'grcomplex', 'float32'}
                ret = save_grcomplex(filename, signal);
            case 'int8'
                ret = save_int8(filename, signal);
            case 'wav_iq'
                ret = save_wav_iq(filename, signal, sample_rate);
            case 'csv_iq'
                ret = save_csv_iq(filename, signal);
            otherwise
                error(['Formato no reconocido: ', format_type]);
        end
        varargout{1} = ret;

    elseif strcmp(action, 'read') || strcmp(action, 'load')
        format_type = 'sc16q11';
        if nargin >= 3 && ~isempty(varargin{1})
            format_type = lower(varargin{1});
        end

        sample_rate = 288000;
        switch format_type
            case 'sc16q11'
                signal = load_sc16q11(filename);
            case 'sc16full'
                signal = load_sc16full(filename);
            case {'grcomplex', 'float32'}
                signal = load_grcomplex(filename);
            case 'int8'
                signal = load_int8(filename);
            case 'wav_iq'
                [signal, sample_rate] = load_wav_iq(filename);
            case 'csv_iq'
                signal = load_csv_iq(filename);
            otherwise
                error(['Formato no reconocido: ', format_type]);
        end
        varargout{1} = signal;
        if nargout >= 2
            varargout{2} = sample_rate;
        end
    else
        error(['Acción desconocida: ', action]);
    end
end
