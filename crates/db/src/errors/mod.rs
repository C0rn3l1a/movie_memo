pub mod already_exists;

use sqlx::Error as SqlxError;

use self::already_exists::AlreadyExistsError;

pub enum OperationError<T> {
    AlreadyExists(AlreadyExistsError<T>),
    Sqlx(SqlxError),
}

impl<T> From<SqlxError> for OperationError<T> {
    fn from(e: SqlxError) -> Self {
        OperationError::<T>::Sqlx(e)
    }
}

impl<T> From<AlreadyExistsError<T>> for OperationError<T> {
    fn from(e: AlreadyExistsError<T>) -> Self {
        OperationError::<T>::AlreadyExists(e)
    }
}