pub mod NoteRepository;

pub trait Repository<T, U>{
    fn get_all();
    fn get_by_id<U>(id: U) -> Result<T, E>;
    fn create();
    fn update<U>(id: U);
    fn delete<U>(id: U);
}
