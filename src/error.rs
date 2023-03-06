#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error(transparent)]
    Surreal(#[from] srrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
