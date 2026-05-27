const query_params = new URLSearchParams(window.location.search);
// query params keys = search, id_organizacion[], modalidad_practicas, id_region, niveles[]

let proxy_values = {};

query_params.forEach((value, key) => {
    if (query_params.get(key) === '') return;
    proxy_values[key] = value;
});

const query_params_proxy = new Proxy(proxy_values, {
    set: (target, key, value) => {
        target[key] = value;
        query_params.set(key, value);
        return true;
    },
    get: (target, key) => {
        if (key === 'search') {
            return target[key] ?? '';
        } else if (key === 'id_region') {
            return target[key] ?? '';
        } else if (key === 'modalidad_practicas') {
            return target[key] ?? '';
        } else if (key === 'id_organizacion[]') {
            return target[key] ?? [];
        } else if (key === 'niveles[]') {
            return target[key] ?? [];
        }
        return target[key] ?? '';
    }
});

const query_params_set_init = () => {
    const query_params_elements = document.querySelectorAll('[data-ref="query_params"]');
    const selected_container = document.querySelectorAll('[data-selected="selected_container"]');

    query_params_elements.forEach(element => {
        if (element.name === 'search') {
            element.value = query_params_proxy.search;
        } else if (element.name === 'id_organizacion[]') {
            query_params_proxy.id_organizacion.forEach(id_organizacion => {
                const target_org = organizaciones.find(organizacion => organizacion.id == id_organizacion);
                selected_container.forEach(container => {
                    const prepare_item = document.createElement('label');
                    const prepare_checkbox = document.createElement('input');
                    prepare_checkbox.type = 'checkbox';
                    prepare_checkbox.name = 'id_organizacion[]';
                    prepare_checkbox.classList.add('hidden');
                    prepare_checkbox.value = id_organizacion;
                    prepare_checkbox.checked = true;
                    prepare_item.className = 'flex gap-1 text-sm items-center bg-rose-950 px-2 rounded-full text-rose-200 group-hover:text-white transition';
                    prepare_item.innerHTML = `
                    ${prepare_checkbox.outerHTML}
                    ${target_org?.nombre_comercial ?? ''}
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
                            <path d="M18 6l-12 12" />
                            <path d="M6 6l12 12" />
                        </svg>
                    `;
                    container.appendChild(prepare_item);
                })
            });
        } else if (element.name === 'modalidad_practicas') {
            element.value = query_params_proxy.modalidad_practicas;
        } else if (element.name === 'id_region') {
            element.value = query_params_proxy.id_region;
        } else if (element.name === 'niveles[]') {
            element.checked = query_params_proxy.niveles.includes(element.value);
        }
    });
};

const handle_search_focus = (event) => {
    const search_list = event.target.nextElementSibling;
    search_list.classList.replace('hidden', 'flex');
}

let timeout_search_org;
const handle_search = (event) => {
    const org_selected_container = event.target.closest('[data-id="input_search_customized"]');
    clearTimeout(timeout_search_org);
    timeout_search_org = setTimeout(() => {
        let checkbox_selected = org_selected_container.querySelectorAll('[data-selected="selected_container"] input[type="checkbox"]');
        checkbox_selected = [...checkbox_selected].map(checkbox => checkbox.value);
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
                    .toLowerCase()) && !checkbox_selected.includes(organizacion.id)
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
    if (org_selected_container.children.length >= 3) {
        event.preventDefault();
        return;
    }
    search_list.classList.replace('flex', 'hidden');
    const prepare_item = document.createElement('label');
    const prepare_checkbox = document.createElement('input');
    prepare_checkbox.type = 'checkbox';
    prepare_checkbox.name = 'id_organizacion[]';
    prepare_checkbox.classList.add('hidden');
    prepare_checkbox.value = event.target.dataset.id;
    prepare_checkbox.checked = true;
    prepare_item.className = 'flex gap-1 text-sm items-center bg-rose-950 px-2 rounded-full text-rose-200 group-hover:text-white transition';
    prepare_item.innerHTML = `
        ${prepare_checkbox.outerHTML}
        ${event.target.textContent}
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <path d="M18 6l-12 12" />
            <path d="M6 6l12 12" />
        </svg>
    `;

    org_selected_container.appendChild(prepare_item);

    // elimina el item seleccionado de la lista
    event.target.remove();
}

const handle_unset_item = (event) => {
    const label_child_element = event.target.closest('label');
    if (label_child_element) {
        const org_selected_container = event.currentTarget;
        org_selected_container.removeChild(label_child_element);
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

document.addEventListener('DOMContentLoaded', () => {
    query_params_set_init();
});
