% Script de pruebas de verificación numérica para iq-files en MATLAB/Octave.

disp('Iniciando tests de iq-files MATLAB...');
test_signal = [0.0, 0.5 - 0.5j, 0.99 - 0.99j, -0.7 + 0.3j];

formats = {'sc16q11', 'sc16full', 'grcomplex', 'int8', 'cu8', 'cs12', 'wav_iq', 'csv_iq'};

for i = 1:length(formats)
    fmt = formats{i};
    ext = '.tmp';
    if strcmp(fmt, 'wav_iq')
        ext = '.wav';
    elseif strcmp(fmt, 'csv_iq')
        ext = '.csv';
    end
    fname = ['tmp_test_', fmt, ext];

    % Guardar
    iq_files('write', fname, test_signal, 48000, fmt);

    % Leer
    [loaded, fs] = iq_files('read', fname, fmt);

    assert(length(loaded) == length(test_signal), ['Error de longitud en formato ', fmt]);

    % Tolerancia según resolución
    tol = 0.05;
    if strcmp(fmt, 'grcomplex')
        tol = 1e-5;
    elseif strcmp(fmt, 'sc16full') || strcmp(fmt, 'wav_iq')
        tol = 1e-3;
    end

    diff_val = max(abs(loaded(:) - test_signal(:)));
    assert(diff_val < tol, sprintf('Diferencia excesiva en %s: %f', fmt, diff_val));

    delete(fname);
    fprintf('  Formato %-10s: OK (error max = %e)\n', fmt, diff_val);
end

disp('Todos los tests de MATLAB pasaron exitosamente.');
