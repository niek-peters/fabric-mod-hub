pub fn is_constraint_err(err: &rusqlite::Error) -> bool {
    match err {
        rusqlite::Error::SqliteFailure(error, _) => {
            error.code == rusqlite::ErrorCode::ConstraintViolation
        }
        _ => false,
    }
}
