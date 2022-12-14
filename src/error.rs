use inquire::InquireError;
use thiserror::Error;

pub type ClapIntResult<T> = core::result::Result<T, ClapIntError>;

#[derive(Error, Debug)]
pub enum ClapIntError {
    #[error("{0}")]
    Generic(String),

    #[error("{0}")]
    Inquire(#[from] InquireError),

    #[error("{0}")]
    Clap(#[from] clap::Error),

    #[error("clap-interactive supplied these args: {args:?}\n{clap_error}")]
    WrapClap {
        args: Vec<String>,
        clap_error: clap::Error,
    },
}
