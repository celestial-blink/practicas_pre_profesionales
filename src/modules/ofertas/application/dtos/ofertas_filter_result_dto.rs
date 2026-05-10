use serde::Serialize;

use crate::modules::ofertas::domain::oferta::Oferta;

#[derive(Debug, Serialize)]
pub struct OfertasFilterResultDto {
    pub ofertas_activas: Vec<Oferta>,
    pub ofertas_vencidas: Vec<Oferta>,
    pub total_activas: i32, // total de ofertas activas
}
