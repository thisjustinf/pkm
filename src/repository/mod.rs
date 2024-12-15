pub mod note_repository;

pub trait Repository<T, U> {
    fn get_all();
    fn get_by_id<U>(id: U);
    fn create();
    fn update<U>(id: U);
    fn delete<U>(id: U);
}
