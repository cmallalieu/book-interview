# book-interview
Book interview project

Make sure you have rust/cargo installed for this to work, if you dont, install it here:
https://rustup.rs/

# How to use:

## Get a Blockfrost Project Id
Go to https://blockfrost.io, make a free account, and grab a mainnet Project Id

## Configure `blockfrost.toml`
Add your Project Id to the `blockfrost.toml` file in the root directory
```
project_id = "<Add Blockfrost Project Id Here>"
cardano_network = "https://cardano-mainnet.blockfrost.io/api/v0"
```

## Run the program
It should be run using the following command
```
cargo run -- <policy_id> <output_directory>
```
Note that in order for it to work the output directory must already exist but does not need to be empty 
