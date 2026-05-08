const params = new URLSearchParams(window.location.search);

// escribe en params el valor del select de departamento
const set_params = (key = '', value) => {
    const allow_keys = ['departamento', 'modalidad', 'nivel_academico', 'organizacion'];
    if (allow_keys.includes(key)) {
        params.set(key, value);
    }
}
