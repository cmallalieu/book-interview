mod system_io;
use system_io::system_io::{books_to_write, get_user_inputs, write_bytes_to_fs, InputArgs};

mod book_adapter;
use book_adapter::book_adapter::policy_is_book;

mod blockfrost_adapter;
use blockfrost_adapter::blockfrost_adapter::{get_asset_ids, get_book_infos};

mod img_downloader;
use img_downloader::img_downloader::get_img;

mod utils;

use futures::future::join_all;
use std::iter::zip;
use utils::utils::book_to_file_name;

mod error_wrapper;
use error_wrapper::error_wrapper::ErrorWrapper;

#[tokio::main]
async fn main() -> Result<(), ErrorWrapper> {
    let InputArgs {
        policy_id,
        output_dir,
    } = match get_user_inputs() {
        Some(args) => args,
        None => return Ok(()),
    };

    if !policy_is_book(&policy_id).await? {
        println!("This policy id does not belong to a book");
        return Ok(());
    }

    let asset_ids = get_asset_ids(&policy_id).await?;
    let candidate_book_infos = get_book_infos(&asset_ids).await?;
    let books_to_write = books_to_write(&output_dir, &candidate_book_infos)?;

    let img_futures: Vec<_> = books_to_write
        .iter()
        .map(|info| get_img(&info.img_url))
        .collect();
    let imgs: Vec<_> = join_all(img_futures).await.into_iter().collect();

    let file_names: Vec<String> = books_to_write
        .iter()
        .map(|info| book_to_file_name(info))
        .collect();

    for (img, file_name) in zip(imgs, file_names) {
        let full_path = output_dir.clone() + "/" + &file_name;
        write_bytes_to_fs(&img?, &full_path).await?
    }

    Ok(())
}
