pub use core::RequestClient;
mod core {
    use derive_getters::Getters;
    use derive_more::AsRef;
    use serde::Serialize;
    use url::Url;

    #[derive(Serialize)]
    pub struct AuthorizedRequest<T> {
        api_key: String,
        #[serde(flatten)]
        inner: T,
    }

    #[derive(Debug, Getters, AsRef)]
    pub struct RequestClient {
        pub(crate) url: Url,
        pub(crate) username: String,
        pub(crate) password: String,
        #[as_ref]
        inner: reqwest::Client,
    }

    impl RequestClient {
        pub(crate) fn new(url: Url, username: impl Into<String>, password: impl Into<String>) -> Self {
            Self {
                url,
                username: username.into(),
                password: password.into(),
                inner: reqwest::Client::new(),
            }
        }

        pub(crate) fn authorize_request<T>(&self, inner: T) -> AuthorizedRequest<T> {
            AuthorizedRequest {
                api_key: self.get_api_key(),
                inner,
            }
        }

        pub(crate) fn get_api_key(&self) -> String {
            format!("{}:{}", self.username, self.password)
        }

        pub(crate) fn set_authorized_query_params<T: Serialize>(&self, url: &mut Url, params: T) {
            let authorized_request_params = self.authorize_request(params);
            let query_string = serde_qs::to_string(&authorized_request_params).expect("failed to serialize query parameters");
            url.set_query(Some(&query_string));
        }
    }

    impl Default for RequestClient {
        fn default() -> Self {
            Self {
                url: "http://localhost:8088/".parse().expect("failed to parse url"),
                username: "asterisk".to_string(),
                password: "asterisk".to_string(),
                inner: reqwest::Client::new(),
            }
        }
    }
}
