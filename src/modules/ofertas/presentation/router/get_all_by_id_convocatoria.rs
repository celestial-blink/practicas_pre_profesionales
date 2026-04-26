use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    general_types::State,
    maud::components::convocatoria_item::{ConvocatoriaItem, convocatoria_item},
    modules::ofertas::{
        application::get_all_by_id_convocatoria::GetAllByIdConvocatoria,
        infrastructure::persistence::mariadb_repository::MariaDbRepository,
        presentation::router::dtos::generate_texto_by_convocatoria_dto::GenerateTextoByConvocatoriaDto,
    },
};

#[get("/convocatoria/{id}")]
pub async fn get_all_by_id_convocatoria(
    state: web::Data<State>,
    params: web::Path<i32>,
) -> impl Responder {
    let id = params.into_inner();
    let infrastructure = MariaDbRepository::new(state.db.clone());
    let application = GetAllByIdConvocatoria::new(infrastructure);
    let result = application.execute(id).await;

    match result {
        Ok(ofertas) => {
            let total_fin_convocatoria = ofertas.iter().map(|oferta| oferta.fecha_fin_oferta).len();
            let fin_convocatoria = ofertas
                .iter()
                .map(|oferta| oferta.fecha_fin_oferta)
                .max()
                .unwrap();
            let vacantes: i32 = ofertas.iter().map(|oferta| oferta.vacantes as i32).sum();

            // une todas las carreras y pone en minisculas y mayusculas por palabra, y no debe repetirse las carreras
            let carreras = ofertas
                .iter()
                .map(|oferta| oferta.carreras.to_lowercase())
                .collect::<Vec<String>>()
                .join(",");
            let mut carreras = carreras
                .split(",")
                .map(|carrera| carrera.trim().to_owned())
                .collect::<Vec<String>>();
            // eliminar duplicados
            carreras.sort();
            carreras.dedup();

            let carreras = carreras
                .iter()
                .map(|carrera| {
                    carrera
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>();
            let carreras = carreras.join(", ");

            // igual a carreras
            let departamentos = ofertas
                .iter()
                .map(|oferta| oferta.region.to_lowercase())
                .collect::<Vec<String>>()
                .join(",");
            let mut departamentos = departamentos
                .split(",")
                .map(|departamento| departamento.trim().to_owned())
                .collect::<Vec<String>>();
            // eliminar duplicados
            departamentos.sort();
            departamentos.dedup();

            let departamentos = departamentos
                .iter()
                .map(|departamento| {
                    departamento
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>();
            let departamentos = departamentos.join(", ");

            // obtengo el mayor y menor subvencion
            let max_subvencion = ofertas
                .iter()
                .max_by(|a, b| a.subvencion.cmp(&b.subvencion));
            let min_subvencion = ofertas
                .iter()
                .min_by(|a, b| a.subvencion.cmp(&b.subvencion));

            let subvenciones = match (max_subvencion, min_subvencion) {
                (Some(max), Some(min)) => {
                    if max.subvencion == min.subvencion {
                        format!("S/.{}", max.subvencion)
                    } else {
                        format!("Entre S/.{} - S/.{}", min.subvencion, max.subvencion)
                    }
                }
                _ => String::new(),
            };

            // igual a subvenciones
            let modalidades_ids = ofertas
                .iter()
                .map(|oferta| oferta.modalidad_practicas)
                .collect::<Vec<i8>>();

            let modalidades = if modalidades_ids.contains(&0) && modalidades_ids.contains(&1) {
                "Pre y profesionales".to_string()
            } else if modalidades_ids.contains(&0) {
                "Preprofesionales".to_string()
            } else if modalidades_ids.contains(&1) {
                "Profesionales".to_string()
            } else {
                String::new()
            };

            let niveles_estudios = ofertas
                .iter()
                .map(|oferta| oferta.niveles.clone())
                .collect::<Vec<String>>()
                .join(",");
            let mut niveles_estudios = niveles_estudios
                .split(",")
                .map(|nivel| nivel.trim().to_string())
                .collect::<Vec<String>>();
            niveles_estudios.sort();
            niveles_estudios.dedup();
            let nivel_estudios = niveles_estudios.join(", ");

            let texto = ofertas
                .iter()
                .map(|oferta| {
                    convocatoria_item(
                        ConvocatoriaItem {
                            titulo: oferta.titulo.clone(),
                            alias: oferta.alias.clone(),
                            alias_org: oferta.alias_org.clone(),
                            nombre_org: oferta.nombre_org.clone(),
                            logo_org: oferta.logo_org.clone(),
                            fin_convocatoria: oferta.fecha_fin_oferta,
                            formacion: oferta.formacion.clone(),
                            departamentos: oferta.region.clone(),
                        },
                        oferta.id as usize,
                    )
                    .into_string()
                })
                .collect::<Vec<String>>()
                .join("");

            let result = GenerateTextoByConvocatoriaDto {
                fin_convocatoria,
                vacantes,
                carreras,
                departamentos,
                subvenciones,
                modalidades,
                nivel_estudios,
                texto: Some(texto),
                finalizan_todos: total_fin_convocatoria == 1,
            };

            HttpResponse::Ok().json(result)
        }
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
