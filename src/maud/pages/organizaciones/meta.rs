use std::collections::HashMap;

pub fn meta() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // General
        (
            "description",
            "Revisa el directorio de entidades del Estado peruano que ofrecen prácticas pre y profesionales. Consulta cuántas convocatorias tiene cada organización y accede a sus oportunidades.",
        ),
        (
            "keywords",
            "entidades públicas con prácticas Perú, ministerios con prácticas preprofesionales, organismos del Estado prácticas, directorio organizaciones sector público Perú, instituciones gobierno prácticas",
        ),
        ("robots", "index, follow"),
        ("author", "Practicasperupro"),
        ("canonical", "https://www.practicasperupro.com"),
        // Open Graph
        ("og:title", "Lista de departamentos en el Perú"),
        ("og:type", "website"),
        ("og:site_name", "Practicas Pre y Profesionales Perú"),
        (
            "og:description",
            "Revisa el directorio de entidades del Estado peruano que ofrecen prácticas pre y profesionales. Consulta cuántas convocatorias tiene cada organización y accede a sus oportunidades.",
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
