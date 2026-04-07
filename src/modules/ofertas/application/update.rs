use crate::modules::{
    oferta_niveles::domain::{
        oferta_niveles::OfertaNiveles, oferta_niveles_repository::OfertaNivelesRepository,
    },
    ofertas::domain::{
        dtos::update_dto::UpdateOfertaDto, oferta::Oferta, repository::OfertaRepository,
    },
};

pub struct Update<T: OfertaRepository, K: OfertaNivelesRepository> {
    pub repository: T,
    pub niveles_repository: K,
}

impl<T: OfertaRepository, K: OfertaNivelesRepository> Update<T, K> {
    pub fn new(repository: T, niveles_repository: K) -> Self {
        Self {
            repository,
            niveles_repository,
        }
    }

    pub async fn execute(&self, oferta_dto: UpdateOfertaDto) -> Result<(), String> {
        self.repository
            .with_transaction(async move |tx| {
                let oferta: Oferta = oferta_dto.clone().into();

                let update_result = self.repository.update(oferta.clone(), tx).await;
                if update_result.is_err() {
                    return Err(update_result.err().unwrap());
                }
                // elimina todos los nives si no hay niveles
                if oferta_dto.niveles_data.is_empty() {
                    let delete_result = self
                        .niveles_repository
                        .remove_by_id_oferta(oferta.id, tx)
                        .await;
                    if delete_result.is_err() {
                        return Err(delete_result.err().unwrap());
                    }

                } else {
                    // elimina que ya no estan en la lista
                    let delete_result = self
                        .niveles_repository
                        .remove_by_nivel_academico(oferta.id, oferta_dto.niveles_data.clone(), tx)
                        .await;
                    if delete_result.is_err() {
                        return Err(delete_result.err().unwrap());
                    }

                    let niveles = oferta_dto
                        .niveles_data
                        .into_iter()
                        .map(|x| OfertaNiveles {
                            id: 0,
                            id_oferta: oferta.id,
                            id_nivel_academico: x,
                        })
                        .collect::<Vec<OfertaNiveles>>();

                    // inserta solo los nuevos
                    let create_result = self.niveles_repository.create_multiple(niveles, tx).await;
                    if create_result.is_err() {
                        return Err(create_result.err().unwrap());
                    }
                }
                Ok(())
            })
            .await
    }
}
