#[derive(Debug)]
pub enum Tmc5160Error<T> {
    SpiError(T),
}

pub type Result<T, E> = core::result::Result<T, Tmc5160Error<E>>;
