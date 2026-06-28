use std::collections::HashMap;

pub fn meta() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // General
        (
            "description",
            "Conoce cómo recopilamos, usamos y protegemos tu información personal en nuestro portal de prácticas pre y profesionales en el sector público del Perú.",
        ),
        (
            "keywords",
            "política de privacidad portal prácticas, protección de datos prácticas Perú, privacidad convocatorias sector público, tratamiento datos personales practicantes",
        ),
        ("robots", "index, follow"),
        ("author", "Practicasperupro"),
        ("canonical", "https://www.practicasperupro.com"),
        // Open Graph
        ("og:title", "Politicas de Privacidad - Prácticas Perú Pro"),
        ("og:type", "website"),
        ("og:site_name", "Practicas Pre y Profesionales Perú"),
        (
            "og:description",
            "Conoce cómo recopilamos, usamos y protegemos tu información personal en nuestro portal de prácticas pre y profesionales en el sector público del Perú.",
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
