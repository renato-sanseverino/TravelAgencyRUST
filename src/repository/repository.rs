use sqlx::error::Error;


pub trait Repository<T> {
    fn save(&mut self, payload: T) -> Result<(), Error>;
    fn get_by_id(&self, id: i32) -> Option<T>;
    fn remove(&mut self, id: i32) -> Result<(), Error>;
    fn patch(&mut self, id: i32, payload: T) -> Result<(), Error>;
}
