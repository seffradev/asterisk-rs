use std::collections::HashMap;

use derive_getters::Getters;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::*;

#[derive(Serialize)]
pub struct AuthorizedRequest<T> {
    api_key: String,
    #[serde(flatten)]
    inner: T,
}

#[derive(Debug, Getters)]
pub struct RequestClient {
    url: Url,
    username: String,
    password: String,
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

    pub(crate) fn get_api_key(&self) -> String {
        format!("{}:{}", self.username, self.password)
    }

    pub(crate) fn set_authorized_query_params<T: Serialize>(&self, url: &mut Url, params: T) {
        let authorized_request_params = AuthorizedRequest {
            api_key: self.get_api_key(),
            inner: params,
        };

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
