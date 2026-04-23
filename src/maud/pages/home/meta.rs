use std::collections::HashMap;

pub fn meta() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // General
        ("description", "Encuentra practicas pre y profesionales en Peru"),
        ("keywords", "practicas, pre, profesionales, peru"),
        ("robots", "index, follow"),
        ("author", "Practicasperupro"),
        ("canonical", "https://www.practicasperupro.com"),
        // Open Graph
        ("og:title", "Practicas Pre y Profesionales Peru"),
        ("og:type", "website"),
        ("og:site_name", "Practicas Pre y Profesionales Peru"),
        ("og:description", "Encuentra practicas pre y profesionales en Peru"),
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
