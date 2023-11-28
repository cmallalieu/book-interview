pub mod utils {
    use bytes::Bytes;
    use reqwest::{self};
    use serde::de::DeserializeOwned;
    use std::{fmt::Debug, time::Duration};

    use crate::book_adapter::book_adapter::BookInfo;
    use crate::error_wrapper::error_wrapper::ErrorWrapper;

    pub async fn fetch_get_json<T: DeserializeOwned + Debug>(url: &str) -> Result<T, ErrorWrapper> {
        let client = reqwest::Client::new();
        let json = client
            .get(url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(json)
    }

    pub async fn fetch_get_bytes(url: &str) -> Result<Bytes, ErrorWrapper> {
        let client = reqwest::Client::new();
        let bytes = client
            .get(url)
            .timeout(Duration::from_secs(200))
            .send()
            .await?
            .bytes()
            .await?;
        Ok(bytes)
    }

    pub fn book_to_file_name(book_info: &BookInfo) -> String {
        book_info.name.clone() + "_cover_" + &book_info.variation.to_string() + ".png"
    }
}
