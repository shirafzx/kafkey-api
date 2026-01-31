use crate::api::axum_http::response_utils::error_response;
use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: serde::de::DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|e| {
            error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_JSON",
                &e.to_string(),
                None,
            )
            .into_response()
        })?;

        if let Err(errors) = data.validate() {
            let error_details = errors
                .field_errors()
                .iter()
                .map(|(field, errs)| crate::application::dtos::ApiErrorDetail {
                    field: field.to_string(),
                    reason: errs
                        .iter()
                        .map(|e| e.code.clone().into_owned())
                        .collect::<Vec<_>>()
                        .join(", "),
                })
                .collect::<Vec<_>>();

            return Err(error_response(
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                "Input validation failed",
                Some(error_details),
            )
            .into_response());
        }

        Ok(Self(data))
    }
}
