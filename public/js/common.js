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
