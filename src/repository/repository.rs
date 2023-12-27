use sqlx::error::Error;


// TODO: verificar a lib async_trait, pois a linguagem não tem suporte nativo
// pub trait Repository<T> {
//    async fn save(&mut self, payload: T) -> Result<(), Error>;

pub trait Repository<T> {
    fn save(&mut self, payload: T) -> Result<(), Error>;
    fn get_by_id(&self, id: i32) -> Option<T>;
    fn remove(&mut self, id: i32) -> Result<(), Error>;
    fn patch(&mut self, id: i32, payload: T) -> Result<(), Error>;
}
