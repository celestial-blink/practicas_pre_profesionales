use maud::{Markup, html};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::maud::components::convocatoria_item::{ConvocatoriaItem, convocatoria_item};
use crate::maud::components::convocatoria_section::convocatoria_section;

pub fn right_home() -> Markup {
    let convocatorias: Vec<ConvocatoriaItem> = vec![
        ConvocatoriaItem {
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            alias: "irtp".to_string(),
            alias_org: "irtp".to_string(),
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: PrimitiveDateTime::new(OffsetDateTime::now_utc().date(), OffsetDateTime::now_utc().time()),
            formacion: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
        },
        ConvocatoriaItem {
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            alias: "irtp".to_string(),
            alias_org: "irtp".to_string(),
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: PrimitiveDateTime::new(OffsetDateTime::now_utc().date(), OffsetDateTime::now_utc().time()),
            formacion: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
        },
        ConvocatoriaItem {
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            alias: "irtp".to_string(),
            alias_org: "irtp".to_string(),
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: PrimitiveDateTime::new(OffsetDateTime::now_utc().date(), OffsetDateTime::now_utc().time()),
            formacion: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
        },
        ConvocatoriaItem {
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            alias: "irtp".to_string(),
            alias_org: "irtp".to_string(),
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: PrimitiveDateTime::new(OffsetDateTime::now_utc().date(), OffsetDateTime::now_utc().time()),
            formacion: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
        },
        ConvocatoriaItem {
            titulo: "(Abril 2026) ¿Quieres hacer prácticas en el IRTP? Revisa aquí sus convocatorias vigentes".to_string(),
            alias: "irtp".to_string(),
            alias_org: "irtp".to_string(),
            nombre_org: "IRTP".to_string(),
            logo_org: "https://www.practicas.pe/organizaciones/practicas-MININTER.png".to_string(),
            fin_convocatoria: PrimitiveDateTime::new(OffsetDateTime::now_utc().date(), OffsetDateTime::now_utc().time()),
            formacion: "Ciencias de la comunicación, Comunicación, Comunicación audiovisual, Comunicación social, Marketing, Periodismo".to_string(),
            departamentos: "Lima".to_string(),
        },
    ];

    let convocatorias_markup = html! {
        @for (index, prop) in convocatorias.into_iter().enumerate() {
            (convocatoria_item(prop, index))
            br;
            hr class="border-slate-700";
            br;
        }
    };

    html! {
        div class="flex flex-col gap-4" {
            (convocatoria_section(convocatorias_markup.clone(), "Últimas convocatorias"))
            (convocatoria_section(convocatorias_markup, "Convocatorias por departamento"))
        }
    }
}
