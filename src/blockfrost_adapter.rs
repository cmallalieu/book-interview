pub mod blockfrost_adapter {
    use crate::{
        book_adapter::book_adapter::{get_book_info, BookInfo},
        error_wrapper::error_wrapper::ErrorWrapper,
    };
    use futures::future::join_all;
    use std::collections::HashSet;

    use blockfrost::{load, AssetDetails, BlockFrostApi};

    pub async fn get_asset_ids(policy_id: &str) -> Result<Vec<String>, ErrorWrapper> {
        let assets = init_blockfrost()?.assets_policy_by_id(policy_id).await?;
        let asset_ids: Vec<String> = assets.into_iter().map(|x| x.asset).collect();
        Ok(asset_ids)
    }

    pub async fn get_book_infos(asset_ids: &Vec<String>) -> Result<Vec<BookInfo>, ErrorWrapper> {
        let mut book_infos: Vec<BookInfo> = Vec::new();
        let mut unique_covers = HashSet::<(String, i32)>::new();

        'outer: for asset_id_chunk in asset_ids.chunks(30) {
            let book_info_futures: Vec<_> = asset_id_chunk
                .iter()
                .map(|asset_id| get_book_info(asset_id))
                .collect();
            let book_info_chunk: Result<Vec<BookInfo>, ErrorWrapper> =
                join_all(book_info_futures).await.into_iter().collect();

            for book_info in book_info_chunk? {
                let cover_info = (book_info.policy_id.to_owned(), book_info.variation);

                if unique_covers.insert(cover_info) {
                    book_infos.push(book_info);
                    if book_infos.len() == 10 {
                        break 'outer;
                    }
                }
            }
        }

        Ok(book_infos)
    }

    pub async fn get_asset_details(asset_id: &str) -> Result<AssetDetails, ErrorWrapper> {
        let asset_details = init_blockfrost()?.assets_by_id(asset_id).await?;
        Ok(asset_details)
    }

    fn init_blockfrost() -> Result<BlockFrostApi, ErrorWrapper> {
        let configurations = load::configurations_from_env()?;
        let project_id = configurations["project_id"].as_str().unwrap();
        let api = BlockFrostApi::new(project_id, Default::default());
        Ok(api)
    }
}
