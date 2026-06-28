use std::collections::HashMap;

pub fn meta() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // General
        (
            "description",
            "Lee los términos y condiciones de uso de nuestro portal de convocatorias de prácticas pre y profesionales en entidades públicas del Perú antes de registrarte o postular.",
        ),
        (
            "keywords",
            "términos y condiciones portal prácticas, condiciones de uso convocatorias Perú, aviso legal prácticas sector público, términos de servicio practicantes Perú",
        ),
        ("robots", "index, follow"),
        ("author", "Practicasperupro"),
        ("canonical", "https://www.practicasperupro.com"),
        // Open Graph
        ("og:title", "Términos y Condiciones - Prácticas Perú Pro"),
        ("og:type", "website"),
        ("og:site_name", "Practicas Pre y Profesionales Perú"),
        (
            "og:description",
            "Lee los términos y condiciones de uso de nuestro portal de convocatorias de prácticas pre y profesionales en entidades públicas del Perú antes de registrarte o postular.",
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
