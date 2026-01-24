use anyhow::Result;
use gloo_net::http::{Request, RequestBuilder, Response};

use serde_lite::{Deserialize, Serialize};

use super::cookies::get_cookie;

pub trait NetResponsive {
    async fn post_json<T, B>(self, body: B) -> Result<(Option<T>, Response)>
    where
        T: Deserialize,
        B: Serialize;

    async fn get_response<T>(self) -> Result<(Option<T>, Response)>
    where
        T: Deserialize;

    fn authenticate(self) -> Self;
}

impl NetResponsive for RequestBuilder {
    async fn post_json<T, B>(self, body: B) -> Result<(Option<T>, Response)>
    where
        T: Deserialize,
        B: Serialize,
    {
        let result = self.authenticate().json_lite(&body)?.send().await?;

        Ok((result.json_lite::<T>().await.ok(), result))
    }

    async fn get_response<T>(self) -> Result<(Option<T>, Response)>
    where
        T: Deserialize,
    {
        let result = self.authenticate().send().await?;

        Ok((result.json_lite::<T>().await.ok(), result))
    }

    fn authenticate(self) -> Self {
        self.header(
            "Authorization",
            &("Bearer ".to_string() + &get_cookie("auth-token").unwrap_or_default()),
        )
    }
}

pub trait JsonLiteSerialize {
    fn json_lite<T: Serialize + ?Sized>(self, value: &T) -> Result<Request>;
}

impl JsonLiteSerialize for gloo_net::http::RequestBuilder {
    /// Sets the body and Content-Type header to json using serde-lite
    fn json_lite<T: Serialize + ?Sized>(self, value: &T) -> Result<Request> {
        let json = serde_json::to_string(&value.serialize()?)?;
        Ok(self.header("Content-Type", "application/json").body(json)?)
    }
}

pub trait JsonLiteDeserialize {
    async fn json_lite<T: Deserialize>(&self) -> Result<T>;
}

impl JsonLiteDeserialize for gloo_net::http::Response {
    /// Gets the body from json using serde-lite
    /// If the returned response is a blank string
    /// The text response will be coalesced to "{}"
    async fn json_lite<T: Deserialize>(&self) -> Result<T> {
        // blank return result equals crash

        let json_value = serde_json::from_str(&self.text().await?)?;

        // Convert JSON value to Intermediate, then deserialize with serde-lite
        Ok(Deserialize::deserialize(&json_value)?)
    }
}
