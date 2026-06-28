use std::collections::HashMap;

pub fn meta() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // General
        (
            "description",
            "Busca prácticas en el sector público según tu departamento. Lima, Arequipa, Cusco y más: consulta cuántas convocatorias hay en cada región y accede directamente a ellas.",
        ),
        (
            "keywords",
            "prácticas por departamento Perú, prácticas Lima sector público, prácticas Arequipa entidades públicas, prácticas por región Perú, convocatorias por departamento gobierno",
        ),
        ("robots", "index, follow"),
        ("author", "Practicasperupro"),
        ("canonical", "https://www.practicasperupro.com"),
        // Open Graph
        ("og:title", "Lista de organizaciones"),
        ("og:type", "website"),
        ("og:site_name", "Practicas Pre y Profesionales Perú"),
        (
            "og:description",
            "Busca prácticas en el sector público según tu departamento. Lima, Arequipa, Cusco y más: consulta cuántas convocatorias hay en cada región y accede directamente a ellas.",
        ),
        ("og:url", "https://www.practicasperupro.com"),
        (
            "og:image",
            "https://www.practicasperupro.com/images/practicas-pre-profesionales-peru.jpg",
        ),
        ("og:locale", "es_PE"),
        // Fb
        ("fb:app_id", ""),
    ])
}
