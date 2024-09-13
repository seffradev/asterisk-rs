pub use core::RequestClient;
mod core {
    use derive_getters::Getters;
    use url::Url;

    #[derive(Debug, Getters)]
    pub struct RequestClient {
        pub(crate) url: Url,
        pub(crate) username: String,
        pub(crate) password: String,
    }

    impl RequestClient {
        pub(crate) fn get_api_key(&self) -> String {
            format!("{}:{}", self.username, self.password)
        }

        pub(crate) fn add_api_key(&self, url: &mut url::form_urlencoded::Serializer<url::UrlQuery>) {
            url.append_pair("api_key", &self.get_api_key());
        }
    }
}
