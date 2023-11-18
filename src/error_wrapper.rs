pub mod error_wrapper {
    #[derive(Debug)]
    pub enum ErrorWrapper {
        Reqwest(reqwest::Error),
        Blockfrost(blockfrost::Error),
        IO(std::io::Error),
    }

    impl From<reqwest::Error> for ErrorWrapper {
        fn from(err: reqwest::Error) -> ErrorWrapper {
            ErrorWrapper::Reqwest(err)
        }
    }

    impl From<blockfrost::Error> for ErrorWrapper {
        fn from(err: blockfrost::Error) -> ErrorWrapper {
            ErrorWrapper::Blockfrost(err)
        }
    }

    impl From<std::io::Error> for ErrorWrapper {
        fn from(err: std::io::Error) -> ErrorWrapper {
            ErrorWrapper::IO(err)
        }
    }
}
