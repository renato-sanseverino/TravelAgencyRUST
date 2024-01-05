use uuid::Uuid;
use sqlx::error::Error;
use async_trait::async_trait;


// Na proxima versão de RUST talvez seja incluido suporte a traits assincronas
// remover a lib async_trait caso a linguagem forneça suporte nativo
#[async_trait]
pub trait Repository<T> {
    async fn insert(&mut self, payload: T) -> Result<(), Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<T>, Error>;
    async fn delete(&mut self, id: Uuid) -> Result<(), Error>;
    async fn patch(&mut self, id: Uuid, payload: T) -> Result<(), Error>;
}
