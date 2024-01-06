use uuid::Uuid;
use sqlx::error::Error;
use async_trait::async_trait;


// Na proxima versão de RUST talvez seja incluido suporte a traits assincronas
// remover a lib async_trait caso a linguagem forneça suporte nativo
#[async_trait]
pub trait Repository<T> {
    async fn insert(&self, payload: T) -> Result<T, Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<T>, Error>;
    async fn delete(&self, id: Uuid) -> Result<u64, Error>;
    async fn patch(&self, id: Uuid, payload: T) -> Result<(), Error>;
}
