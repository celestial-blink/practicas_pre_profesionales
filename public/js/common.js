const handle_add_bg_title = (id_element = '', id_focus_element = '') => {
    const element = document.getElementById(id_element);
    const focus_element = document.getElementById(id_focus_element);

    if (!element || !focus_element) return;

    new IntersectionObserver(([entry]) => {
        if (entry.isIntersecting || entry.boundingClientRect.top > 0) {
            element.classList.remove('bg-dark-card');
        } else {
            element.classList.add('bg-dark-card');
        }
    }, { root: null }).observe(focus_element);
}

document.addEventListener('DOMContentLoaded', () => {
    handle_add_bg_title('ultimas_covocatorias', 'convocatoria_0'); // para ultimas convocatorias del inicio "/"
});


const handle_open_dialog = (id_dialog = '', open = false, prevent_background_scroll = true) => {
    const dialog_element = document.getElementById(id_dialog);

    if (open) {
        dialog_element.showModal();
        if (prevent_background_scroll) {
            document.body.classList.add("overflow-hidden");
        }
    } else {
        dialog_element.close();
        document.body.classList.remove("overflow-hidden");
    }
}

const handle_close_header_dialog = (event, id_dialog = '') => {
    const { tagName, dataset } = event.target;
    if (tagName === 'DIALOG' && dataset?.evref === 'handle_close_header_dialog') {
        const confirm = window.confirm('Quieres cerrar la ventana?');
        if (!confirm) {
            return;
        }
        handle_open_dialog(id_dialog, false);
    }
}

// se usa en eventos onkeydown
const handle_prevent_submit_on_key_enter = (event) => {
    const { tagName } = event.target;
    if (tagName === "INPUT" && event.keyCode === 13) event.preventDefault();
}
