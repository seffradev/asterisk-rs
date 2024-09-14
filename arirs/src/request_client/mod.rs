pub use core::RequestClient;
mod core {
    use derive_getters::Getters;
    use serde::Serialize;
    use url::Url;

    #[derive(Serialize)]
    pub struct AuthorizedRequest<T> {
        api_key: String,
        #[serde(flatten)]
        inner: T,
    }

    #[derive(Debug, Getters)]
    pub struct RequestClient {
        pub(crate) url: Url,
        pub(crate) username: String,
        pub(crate) password: String,
    }

    impl RequestClient {
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
}
