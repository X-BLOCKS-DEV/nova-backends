pub mod user;

use crate::persistence::postgres::Db;
use std::marker::PhantomData;

pub struct DatabaseRepositoryImpl<T> {
    db: Db,
    _marker: PhantomData<T>,
}

impl<T> DatabaseRepositoryImpl<T> {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            _marker: PhantomData,
        }
    }
}