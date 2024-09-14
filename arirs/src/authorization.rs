use serde::Serialize;
use url::Url;

pub struct Authorization;

#[derive(Serialize)]
struct AuthorizedRequestParams<'a, T> {
    api_key: &'a str,
    #[serde(flatten)]
    inner: T,
}

impl Authorization {
    pub(crate) fn api_key(username: &str, password: &str) -> String {
        format!("{}:{}", username, password)
    }

    pub(crate) fn build_url<'a, T: Serialize>(
        url: &Url,
        path: impl AsRef<[&'a str]>,
        api_key: &str,
        params: T,
    ) -> std::result::Result<Url, url::ParseError> {
        let mut url = url.join(&path.as_ref().join("/"))?;

        Self::add_query_parameters(&mut url, api_key, params);

        Ok(url)
    }

    fn add_query_parameters<T: Serialize>(url: &mut Url, api_key: &str, params: T) {
        let authorized_request_params = AuthorizedRequestParams { api_key, inner: params };

        let query_string = serde_qs::to_string(&authorized_request_params).expect("failed to serialize query parameters");

        url.set_query(Some(&query_string));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn builds_url() {
        let url = "http://localhost:8080/".parse().unwrap();
        let api_key = Authorization::api_key("asterisk", "asterisk");

        let expected = "http://localhost:8080/channel?api_key=asterisk%3Aasterisk&media=sound%3Ahello&lang=en";
        let actual = Authorization::build_url(
            &url,
            ["channel"],
            &api_key,
            PlayMediaParams {
                playback_id: None,
                base_params: PlayMediaBaseParams {
                    media: &["sound:hello"],
                    lang: Some("en"),
                    offset_ms: None,
                    skip_ms: None,
                },
            },
        )
        .unwrap();

        assert_eq!(expected, actual.as_str())
    }

    #[test]
    fn builds_url_with_newtype() {
        let url = "http://localhost:8080/".parse().unwrap();
        let api_key = Authorization::api_key("asterisk", "asterisk");

        let expected = "http://localhost:8080/channel?api_key=asterisk%3Aasterisk";
        let actual = Authorization::build_url(&url, ["channel"], &api_key, ()).unwrap();

        assert_eq!(expected, actual.as_str())
    }
}
