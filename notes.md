# My problems while learning rust

#### 1. If your rust-analyzer extension doesn't work
    > Get to the root directory and enter the command "cargo run". This will create a cargo.toml file required to run the extension.
    > Once created, hover over the extesion below and restart the server.
#### 2. If your main.rs file doesn't compile over some linker errors
    > Enter the below commands serially:
        1. "rustup toolchain install stable-x86_64-pc-windows-gnu"
        2. "rustup default stable-x86_64-pc-windows-gnu"
    > Run the program again.