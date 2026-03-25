use maud::{html, Markup};
use time::OffsetDateTime;

use crate::maud::components::convocatoria_section::convocatoria_section;
use crate::maud::components::convocatoria_item::ConvocatoriaItem;

pub fn right_home() -> Markup {

    let convocatorias: Vec<ConvocatoriaItem> = vec![
        ConvocatoriaItem {
            id: 1,
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            id_organizacion: 1,
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: OffsetDateTime::now_utc(),
            carreras: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
            texto: "Practicas Pre Profesionales".to_string(),
            finalizan_todos: true,
        },
        ConvocatoriaItem {
            id: 2,
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            id_organizacion: 1,
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: OffsetDateTime::now_utc(),
            carreras: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
            texto: "Practicas Pre Profesionales".to_string(),
            finalizan_todos: true,
        },
        ConvocatoriaItem {
            id: 3,
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            id_organizacion: 1,
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: OffsetDateTime::from_unix_timestamp(OffsetDateTime::now_utc().unix_timestamp() + 172800).unwrap(), // fecha 2 dias en el futuro
            carreras: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
            texto: "Practicas Pre Profesionales".to_string(),
            finalizan_todos: true,
        },
        ConvocatoriaItem {
            id: 4,
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            id_organizacion: 1,
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: OffsetDateTime::now_utc(),
            carreras: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
            texto: "Practicas Pre Profesionales".to_string(),
            finalizan_todos: true,
        },
        ConvocatoriaItem {
            id: 5,
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            id_organizacion: 1,
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: OffsetDateTime::now_utc(),
            carreras: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
            texto: "Practicas Pre Profesionales".to_string(),
            finalizan_todos: true,
        },
    ];



    html! {
        div class="flex flex-col gap-4" {
            (convocatoria_section(convocatorias, "Últimas convocatorias"))
        }
    }
}
