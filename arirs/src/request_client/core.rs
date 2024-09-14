use std::collections::HashMap;

use derive_getters::Getters;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::*;

#[derive(Debug, Getters)]
pub struct RequestClient {
    url: Url,
    bearer: String,
    inner: reqwest::Client,
}

#[derive(Serialize)]
struct AuthorizedRequest<'a, T> {
    api_key: &'a str,
    #[serde(flatten)]
    inner: T,
}

impl RequestClient {
    pub(crate) fn new(url: Url, username: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        Self {
            url,
            bearer: format!("{}:{}", username.as_ref(), password.as_ref()),
            inner: reqwest::Client::new(),
        }
    }

    pub(crate) async fn authorized_get<T: Serialize, R: DeserializeOwned>(&self, path: impl AsRef<[&str]>, params: T) -> Result<R> {
        let url = self.authorized_url(path, params)?;
        let response = self.inner.get(url).send().await?.json().await?;
        Ok(response)
    }

    pub(crate) async fn authorized_post<T: Serialize>(&self, path: impl AsRef<[&str]>, params: T) -> Result<()> {
        let url = self.authorized_url(path, params)?;
        self.inner.post(url).send().await?;
        Ok(())
    }

    pub(crate) async fn authorized_post_json_response<T: Serialize, R: DeserializeOwned>(
        &self,
        path: impl AsRef<[&str]>,
        params: T,
    ) -> Result<R> {
        let url = self.authorized_url(path, params)?;
        let response = self.inner.post(url).send().await?.json().await?;
        Ok(response)
    }

    pub(crate) async fn authorized_post_variables<T: Serialize, R: DeserializeOwned>(
        &self,
        path: impl AsRef<[&str]>,
        params: T,
        variables: &HashMap<&str, &str>,
    ) -> Result<R> {
        let url = self.authorized_url(path, params)?;
        let response = self
            .inner
            .post(url)
            .json(&serde_json::json!({
                "variables": variables
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub(crate) async fn authorized_delete<T: Serialize>(&self, path: impl AsRef<[&str]>, params: T) -> Result<()> {
        let url = self.authorized_url(path, params)?;
        self.inner.delete(url).send().await?;
        Ok(())
    }

    fn authorized_url<'a, T: Serialize>(&self, path: impl AsRef<[&'a str]>, params: T) -> Result<Url> {
        let mut url = self.url().join(&path.as_ref().join("/"))?;
        self.set_authorized_query_params(&mut url, params);
        Ok(url)
    }

    pub(crate) fn set_authorized_query_params<T: Serialize>(&self, url: &mut Url, params: T) {
        let authorized_request_params = AuthorizedRequest {
            api_key: &self.bearer,
            inner: params,
        };

        let query_string = serde_qs::to_string(&authorized_request_params).expect("failed to serialize query parameters");
        url.set_query(Some(&query_string));
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new(
            "http://localhost:8088/".parse().expect("failed to parse url"),
            "asterisk",
            "asterisk",
        )
    }
}
