use tracing_log::log::error;

use crate::modules::{
    oferta_niveles::domain::{
        oferta_niveles::OfertaNiveles, oferta_niveles_repository::OfertaNivelesRepository,
    },
    ofertas::domain::{
        dtos::create_dto::CreateOfertaDto, oferta::Oferta, repository::OfertaRepository,
    },
};

pub struct Create<T: OfertaRepository, K: OfertaNivelesRepository> {
    pub repository: T,
    pub niveles_repository: K,
}

impl<T: OfertaRepository, K: OfertaNivelesRepository> Create<T, K> {
    pub fn new(repository: T, niveles_repository: K) -> Self {
        Self {
            repository,
            niveles_repository,
        }
    }

    pub async fn execute(&self, oferta_dto: CreateOfertaDto) -> Result<(), String> {
        self.repository
            .with_transaction(async move |tx| {
                let oferta: Oferta = oferta_dto.clone().into();
                let id = self.repository.create_with_niveles(oferta, tx).await;
                if id.is_ok() {
                    let id_oferta = id.clone().unwrap();
                    let niveles = oferta_dto
                        .niveles_data
                        .into_iter()
                        .map(|x| OfertaNiveles {
                            id: 0,
                            id_oferta,
                            id_nivel_academico: x,
                        })
                        .collect::<Vec<OfertaNiveles>>();
                    self.niveles_repository.create_multiple(niveles, tx).await
                } else {
                    let error = id.err().unwrap();
                    error!("Error al crear la oferta --- {}", &error);
                    Err("Error al crear la oferta ".to_string() + &error)
                }
            })
            .await
    }
}
