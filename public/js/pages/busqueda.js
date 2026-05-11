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
    query_params_elements.forEach(element => {
        if (element.type === 'checkbox') {
            element.checked = query_params.get(element.name) === element.value;
        }
    });
};

const handle_search_org_focus = () => {
    const search_org_list = document.getElementById('search_org_list');
    search_org_list.classList.replace('hidden', 'flex');
}

// solo si org_selected_container no tiene mas de 3 items
const handle_set_org_item = (event) => {
    const org_selected_container = document.getElementById('org_selected_container');
    if (org_selected_container.children.length >= 3) {
        event.preventDefault();
        return;
    }
    const search_org_list = document.getElementById('search_org_list');
    search_org_list.classList.replace('flex', 'hidden');
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
    // elimina el item seleccionado de la lista
    event.target.remove();
}

const handle_unset_org_item = (event) => {
    const span_child_element = event.target.closest('span');
    const org_selected_container = document.getElementById('org_selected_container');
    org_selected_container.removeChild(span_child_element);
}

// quita el foco del input de busqueda de organizaciones
document.addEventListener('click', (event) => {
    const search_org_container = document.getElementById('search_org_container');
    if (!search_org_container.contains(event.target)) {
        const search_org_list = document.getElementById('search_org_list');
        search_org_list.classList.replace('flex', 'hidden');
    }
});

// document.addEventListener('DOMContentLoaded', () => {
//     query_params_set_init();
// });
