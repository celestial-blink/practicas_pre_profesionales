use std::collections::HashMap;

pub fn meta() -> HashMap<String, String> {
    HashMap::from([
        // General
        (
            "description".to_owned(),
            "Explora todas las ofertas de prácticas pre y profesionales en el sector público peruano organizadas por perfil. Encuentra la oportunidad que se ajusta a tu formación y aplica hoy.".to_owned(),
        ),
        (
            "keywords".to_owned(),
            "ofertas de prácticas sector público, prácticas por perfil Perú, bolsa de prácticas Estado peruano, prácticas pre profesionales por carrera, oportunidades prácticas gobierno Peru".to_owned(),
        ),
        ("robots".to_owned(), "index, follow".to_owned()),
        ("author".to_owned(), "Practicasperupro".to_owned()),
        ("canonical".to_owned(), "https://www.practicasperupro.com".to_owned()),
        // Open Graph
        ("og:type".to_owned(), "website".to_owned()),
        ("og:site_name".to_owned(), "Practicas Pre y Profesionales Perú".to_owned()),
        (
            "og:description".to_owned(),
            "Explora todas las ofertas de prácticas pre y profesionales en el sector público peruano organizadas por perfil. Encuentra la oportunidad que se ajusta a tu formación y aplica hoy.".to_owned(),
        ),
        ("og:url".to_owned(), "https://www.practicasperupro.com".to_owned()),
        (
            "og:image".to_owned(),
            "https://www.practicasperupro.com/images/practicas-pre-profesionales-peru.jpg".to_owned(),
        ),
        ("og:locale".to_owned(), "es_PE".to_owned()),
        // Fb
        ("fb:app_id".to_owned(), "".to_owned()),
    ])
}
