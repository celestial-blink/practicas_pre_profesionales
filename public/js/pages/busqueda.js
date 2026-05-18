const query_params = new URLSearchParams(window.location.search);

// escribe en params el valor del select de departamento
const set_params = (key = '', value) => {
    const allow_keys = ['departamento', 'modalidad', 'nivel_academico', 'organizacion'];
    if (allow_keys.includes(key)) {
        query_params.set(key, value);
    }
}

const query_params_set_init = () => {
    const query_params_elements = document.querySelectorAll('[data-ref="query_params"]');
    //TODO:  rehacer la parte de seleccionar organizaciones
    query_params_elements.forEach(element => {
        if (element.type === 'checkbox') {
            element.checked = query_params.get(element.name) === element.value;
        }
    });
};

const handle_search_focus = (event) => {
    const search_list = event.target.nextElementSibling;
    search_list.classList.replace('hidden', 'flex');
}

let timeout_search_org;
const handle_search = (event) => {
    clearTimeout(timeout_search_org);
    timeout_search_org = setTimeout(() => {
        // limpia los caracteres especiales como tildes u pongo todo en minusculas
        const value = event.target.value.normalize("NFD").replace(/[\u0300-\u036f]/g, "").toLowerCase();
        const search_pattern = new RegExp(value, 'i');
        const search_list = event.target.nextElementSibling;
        search_list.innerHTML = '';
        // usa organizaciones, es una variable global
        const filter = organizaciones.filter(organizacion => {
            return search_pattern
                .test(organizacion.nombre_comercial
                    .normalize("NFD")
                    .replace(/[\u0300-\u036f]/g, "")
                    .toLowerCase())
        }).slice(0, 10);
        filter.forEach(organizacion => {
            const li = document.createElement('li');
            li.innerHTML = `
                <button type="button" class="flex items-center cursor-pointer text-slate-400 hover:text-white transition w-full p-1" data-id="${organizacion.id}">
                    ${organizacion.nombre_comercial}
                </button>
            `;
            search_list.appendChild(li);
        });
    }, 5e2);
}

// solo si org_selected_container no tiene mas de 3 items
const handle_set_item = (event) => {
    const search_list = event.target.closest('[data-id="input_search_customized"]');
    const org_selected_container = search_list.querySelector('[data-selected="selected_container"]');
    const input_hidden_element = search_list.querySelector('input[type="hidden"]');
    if (org_selected_container.children.length >= 3) {
        event.preventDefault();
        return;
    }
    search_list.classList.replace('flex', 'hidden');
    const prepare_item = document.createElement('span');
    prepare_item.className = 'flex gap-1 text-sm items-center bg-rose-950 px-2 rounded-full text-rose-200 group-hover:text-white transition';
    prepare_item.innerHTML = `
        ${event.target.textContent}
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path d="M18 6l-12 12" />
            <path d="M6 6l12 12" />
        </svg>
    `;

    org_selected_container.appendChild(prepare_item);
    const input_hidden_array_value = JSON.parse(input_hidden_element?.value || '[]');
    input_hidden_array_value.push(event.target.dataset.id);
    input_hidden_element.value = JSON.stringify(input_hidden_array_value);

    // elimina el item seleccionado de la lista
    event.target.remove();
}

const handle_unset_item = (event) => {
    const search_list = event.target.closest('[data-id="input_search_customized"]');
    const span_child_element = event.target.closest('span');
    if (span_child_element) {
        const org_selected_container = event.currentTarget;
        org_selected_container.removeChild(span_child_element);
        const input_hidden_element = search_list.querySelector('input[type="hidden"]');
        const input_hidden_array_value = JSON.parse(input_hidden_element?.value || '[]');
        input_hidden_array_value.splice(input_hidden_array_value.indexOf(span_child_element.dataset.id), 1);
        input_hidden_element.value = JSON.stringify(input_hidden_array_value);
    }
}

// quita el foco del input de busqueda de organizaciones
document.addEventListener('click', (event) => {
    const input_search_customized = document.querySelectorAll('[data-id="input_search_customized"]');
    input_search_customized.forEach(input_search_customized => {
        if (!input_search_customized.contains(event.target)) {
            const selected_container = input_search_customized.querySelector('[data-menu="search_list"]')
            selected_container.classList.replace('flex', 'hidden');
        }
    });
});

// document.addEventListener('DOMContentLoaded', () => {
//     query_params_set_init();
// });
