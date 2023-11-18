pub mod book_adapter {
    use crate::{
        blockfrost_adapter::blockfrost_adapter::get_asset_details,
        error_wrapper::error_wrapper::ErrorWrapper, utils::utils::fetch_get_json,
    };
    use serde::Deserialize;
    use std::fmt::Debug;

    const BOOK_BASE_PATH: &str = "https://api.book.io/api/v0/";

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct Collection {
        collection_id: String,
        description: String,
        blockchain: String,
        network: String,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct CollectionResponse {
        #[serde(rename = "type")]
        type_: String,
        data: Vec<Collection>,
    }

    #[derive(Debug, Clone)]
    pub struct BookInfo {
        pub policy_id: String,
        pub asset_id: String,
        pub name: String,
        pub variation: i32,
        pub img_url: String,
    }

    pub async fn policy_is_book(policy_id: &str) -> Result<bool, ErrorWrapper> {
        let url = &(BOOK_BASE_PATH.to_string() + "collections");
        let collection_response = fetch_get_json::<CollectionResponse>(url).await?;

        Ok(collection_response
            .data
            .iter()
            .any(|collection| collection.collection_id == policy_id))
    }

    pub async fn get_book_info(asset_id: &str) -> Result<BookInfo, ErrorWrapper> {
        let asset_details = get_asset_details(asset_id).await?;

        let name = asset_details
            .onchain_metadata
            .as_ref()
            .unwrap()
            .get("attributes")
            .unwrap()
            .get("Cover Theme")
            .unwrap()
            .to_string()
            .trim_matches('\"')
            .to_string();

        let variation = asset_details
            .onchain_metadata
            .as_ref()
            .unwrap()
            .get("attributes")
            .unwrap()
            .get("Variation")
            .unwrap()
            .to_string()
            .trim_matches('\"')
            .parse::<i32>()
            .unwrap();

        let img_url = asset_details
            .onchain_metadata
            .unwrap()
            .get("files")
            .unwrap()
            .get(0)
            .unwrap()
            .get("src")
            .unwrap()
            .to_string()
            .replace("\"", "");

        Ok(BookInfo {
            policy_id: asset_details.policy_id,
            asset_id: asset_id.to_string(),
            name,
            variation,
            img_url,
        })
    }
}
