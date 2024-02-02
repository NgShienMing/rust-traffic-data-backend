# Rust backend

Exploration on using Rust to develop backend, for a detailed guide, please go to [Rust CRUD API Example with Axum and PostgreSQL](https://codevoweb.com/rust-crud-api-example-with-axum-and-postgresql/)

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

## Project structure

- When the project is created, the basic folder structure looks like this
    ```
    project/
    ├── src/
    │   └── main.rs
    ├── .gitignore
    ├── Cargo.lock
    └── Cargo.toml
    ```
- `main.rs` is the entry point of the project which contains the main function that initializes and starts the server.
- This project uses the MVC architecture, which looks like
    ```
    project/
    ├── src/
    │   ├── controllers/
    │   │   ├── mod.rs
    │   │   └── // all the controllers
    │   ├── models/
    │   │   ├── mod.rs
    │   │   └── // all the models
    │   ├── routes/
    │   │   ├── mod.rs
    │   │   └── // all the routes
    │   └── main.rs
    ├── target/
    │   └── debug/
    │       └── // built executable
    ├── .gitignore
    ├── Cargo.lock
    └── Cargo.toml
    ```
- `mod.rs` is a file analogous to `__init__.py`, it initializes the directory as a sub-module and contains the mod statements to 'stitch' the Rust files of the module together.
- Rust is a compiled language, hence you are required to compile and build the project into an executable before running it and the built files are contains in a `target` directory.
- For more information, please look at [how to organize a Rust project](https://rust-classes.com/chapter_4_3.html#:~:text=To%20do%20this%2C%20you%20create,files%20of%20the%20module%20together.&text=The%20layout%20you%20choose%2C%20depends,project%20you're%20working%20on)

## How to start

- To build the project, run `cargo build`
- To run the project, run `cargo run`
- Everytime you make a change in the code, it is required to rebuild and rerun the project to see the changes, you can automate this by running `cargo watch -q -c -w src/ -x run`, ensure that you have cargo watch installed before running this command.