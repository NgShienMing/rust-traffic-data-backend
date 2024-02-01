# Rust backend

Exploration on using Rust to develop backend

## Step by step guide on how to create a Rust backend

## Create a new project

- When create a git repository on remote server, try not to initialize repo with README
- After the remote repository is created, copy its url
- Setup command
    ```sh
    cargo new rust-backend
    cd rust-backend
    git remote add origin <remote-repo-url>
    git pull
    git add .
    git commit -m "initial commit"
    git branch -M main
    git push -u origin main
    ```

## Adding dependencies
- Dependencies used in the project is listed under the dependencies section in [Cargo.toml](./Cargo.toml)
- Dependencies can be added to the project using `cargo add <dependency-name>`
- Dependencies in this project
    ```sh
    cargo add axum
    cargo add tokio -F full
    cargo add tower-http -F "cors"
    cargo add serde_json
    cargo add serde -F derive
    cargo add chrono -F serde
    cargo add dotenv
    cargo add uuid -F "serde v4"
    cargo add sqlx -F "runtime-async-std-native-tls postgres chrono uuid"

    # HotReload
    cargo install cargo-watch

    # SQLX-CLI
    cargo install sqlx-cli
    ```
- If you encounter an error that is related to openssl-sys v0.9.99 on the last two installation command, try running `sudo apt-get install libssl-dev`