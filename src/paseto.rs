use paseto_v4::LocalKey;

pub fn local_key() -> Result<LocalKey, actix_web::Error> {
    let raw = std::env::var("PASETO_SECRET_KEY")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Missing PASETO_SECRET_KEY"))?;
    let bytes: [u8; 32] = hex::decode(raw)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid PASETO_SECRET_KEY hex"))?
        .try_into()
        .map_err(|_| actix_web::error::ErrorInternalServerError("PASETO_SECRET_KEY must be 32 bytes"))?;
    Ok(LocalKey::from(bytes))
}
