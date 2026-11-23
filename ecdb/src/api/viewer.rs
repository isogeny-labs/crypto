//! The browser viewer: one static page (`web/viewer.html`) that calls the read
//! routes with a token the visitor pastes. It carries no data and no token, so
//! it is served without authentication like `/` and `/openapi.yaml`.

use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::{IntoResponse, Redirect, Response};

pub const PAGE: &str = include_str!("../../web/viewer.html");

/// The page may only talk to this origin and loads nothing from elsewhere.
const CSP: &str = "default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; \
                   connect-src 'self'; img-src 'self' data:; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

pub async fn page() -> Response {
    (
        [
            (header::CONTENT_TYPE, HeaderValue::from_static("text/html; charset=utf-8")),
            (header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CSP)),
            (header::REFERRER_POLICY, HeaderValue::from_static("no-referrer")),
            (header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff")),
        ],
        PAGE,
    )
        .into_response()
}

/// A browser asks for HTML first; agents and curl do not.
pub fn wants_html(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|a| a.split(',').next().is_some_and(|first| first.trim().starts_with("text/html")))
}

pub fn to_viewer() -> Response {
    Redirect::to("/viewer").into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browsers_get_the_viewer_and_agents_do_not() {
        let mut h = HeaderMap::new();
        h.insert(header::ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,*/*;q=0.8"));
        assert!(wants_html(&h));
        h.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
        assert!(!wants_html(&h));
        h.insert(header::ACCEPT, HeaderValue::from_static("*/*"));
        assert!(!wants_html(&h));
        assert!(!wants_html(&HeaderMap::new()));
    }

    #[test]
    fn page_has_no_external_loads() {
        assert!(PAGE.contains("<title>ECDB viewer</title>"));
        assert!(!PAGE.contains("src=\"http") && !PAGE.contains("href=\"http"));
    }
}
