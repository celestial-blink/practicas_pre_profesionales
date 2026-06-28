use std::collections::HashMap;

pub fn meta() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // General
        (
            "description",
            "Encuentra convocatorias de prácticas pre profesionales y profesionales filtrando por entidad, formacion, departamento y mas",
        ),
        (
            "keywords",
            "practicas pre profesionales, practicas profesionales, convocatorias practicas, practicas entidad Estado, practicas preprofesionales",
        ),
        ("robots", "index, follow"),
        ("author", "Practicasperupro"),
        ("canonical", "https://www.practicasperupro.com"),
        // Open Graph
        ("og:title", "Busqueda de practicas pre y profesionales"),
        ("og:type", "website"),
        ("og:site_name", "Practicas Pre y Profesionales Perú"),
        (
            "og:description",
            "Encuentra convocatorias de prácticas pre profesionales y profesionales filtrando por entidad, formacion, departamento y mas",
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
