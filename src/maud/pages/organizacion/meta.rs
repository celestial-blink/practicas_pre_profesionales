use std::collections::HashMap;

pub fn meta() -> HashMap<String, String> {
    HashMap::from([
        // General
        (
            "description".to_owned(),
            "Busca prácticas en el sector público según tu departamento. Lima, Arequipa, Cusco y más: consulta cuántas convocatorias hay en cada región y accede directamente a ellas.".to_owned(),
        ),
        (
            "keywords".to_owned(),
            "prácticas por departamento Perú, prácticas Lima sector público, prácticas Arequipa entidades públicas, prácticas por región Perú, convocatorias por departamento gobierno".to_owned(),
        ),
        ("robots".to_owned(), "index, follow".to_owned()),
        ("author".to_owned(), "Practicasperupro".to_owned()),
        ("canonical".to_owned(), "https://www.practicasperupro.com".to_owned()),
        // Open Graph
        ("og:type".to_owned(), "website".to_owned()),
        ("og:site_name".to_owned(), "Practicas Pre y Profesionales Perú".to_owned()),
        (
            "og:description".to_owned(),
            "Busca prácticas en el sector público según tu departamento. Lima, Arequipa, Cusco y más: consulta cuántas convocatorias hay en cada región y accede directamente a ellas.".to_owned(),
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
