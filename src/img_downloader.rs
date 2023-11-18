pub mod img_downloader {
    use bytes::Bytes;

    use crate::{error_wrapper::error_wrapper::ErrorWrapper, utils::utils::fetch_get_bytes};

    pub async fn get_img(ipfs: &str) -> Result<Bytes, ErrorWrapper> {
        let http_url = ipfs.replace("ipfs://", "https://ipfs.io/ipfs/");
        let raw_bytes = fetch_get_bytes(&http_url).await?;

        Ok(raw_bytes)
    }
}
