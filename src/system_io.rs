pub mod system_io {
    use bytes::Bytes;
    use std::{collections::HashSet, env, fs};

    use crate::{
        book_adapter::book_adapter::BookInfo, error_wrapper::error_wrapper::ErrorWrapper,
        utils::utils::book_to_file_name,
    };

    #[derive(Debug)]
    pub struct InputArgs {
        pub policy_id: String,
        pub output_dir: String,
    }

    pub fn get_user_inputs() -> Option<InputArgs> {
        let args: Vec<String> = env::args().collect();

        let policy_id = match args.get(1) {
            Some(policy) => Some(policy.to_string()),
            None => {
                println!("Must provide a policy id as the first argument");
                return None;
            }
        }?;
        let output_dir = match args.get(2) {
            Some(output_dir) => Some(output_dir.to_string()),
            None => {
                println!("Must provide a output directory as the second argument");
                return None;
            }
        }?;

        Some(InputArgs {
            policy_id,
            output_dir,
        })
    }

    pub async fn write_bytes_to_fs(bytes: &Bytes, output_dir: &str) -> Result<(), ErrorWrapper> {
        fs::write(output_dir, bytes)?;
        Ok(())
    }

    pub fn books_to_write(
        output_dir: &str,
        candidate_books: &[BookInfo],
    ) -> Result<Vec<BookInfo>, ErrorWrapper> {
        let output_dir_file_set: HashSet<String> = fs::read_dir(output_dir)?
            .filter_map(Result::ok)
            .filter_map(|dir_entry| {
                let file_path = dir_entry.path();
                if file_path.is_file() {
                    file_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(String::from)
                } else {
                    None
                }
            })
            .collect();

        let books_to_write: Vec<BookInfo> = candidate_books
            .iter()
            .filter(|book| !output_dir_file_set.contains(&book_to_file_name(book)))
            .cloned()
            .collect();

        println!("Books to write: {}", books_to_write.len());
        Ok(books_to_write)
    }
}
