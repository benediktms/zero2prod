use actix_web::{error::ErrorInternalServerError, HttpResponse};
use reqwest::header::LOCATION;
use std::fmt::{Debug, Display};

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }

    Ok(())
}

pub fn e500<T>(e: T) -> actix_web::Error
where
    T: Debug + Display + 'static,
{
    ErrorInternalServerError(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}
