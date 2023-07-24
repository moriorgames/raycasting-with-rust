# Raycasting engine with Rust

This project is a Game with combination of Rust and a Raycasting engine.

## Quick Start

Here is how you can get the project up and running:

```
# Clone the repository
$ git clone <repo_url>

# Change into project directory
$ cd raycasting-with-rust

# Run the project
$ cargo run

# Test the project
$ cargo test
```

## Testing and Code Coverage

You can run the test suite and generate a code coverage report with these commands:

```
# Install the code coverage tool
$ cargo install cargo-tarpaulin

# In order to process code coverage without struggling with Arm64 arch I have created a workaround with docker
# Run the code coverage tool
$ docker-compose build
$ docker-compose up -d
$ docker logs -f coverage
$ docker cp coverage:/app/target/debug/tarpaulin/tarpaulin-report.html .

# Open the code coverage report on Google Chrome
$ open -a "Google Chrome" ./tarpaulin-report.html

# Open the code coverage report on Google Chrome
$ open -a "Google Chrome" ./target/debug/tarpaulin/tarpaulin-report.html

# Test a single file
$ cargo test --package raycasting_with_rust --bin raycasting_with_rust -- player::player::tests --nocapture

# Test a single integration test
$ cargo test --package raycasting_with_rust --test integration_sample_test -- tests --nocapture
```

## Tooling

This utility command can be used to print file contents for exploration:

```
$ find . -type f -exec printf '### START OF FILE ###\n%s\n' {} \; -exec cat {} \; -exec printf '### END OF FILE ###\n' \;
```

Modules

```  
$ cargo install cargo-modules
$ cargo modules generate tree
$ cargo modules generate tree -types
$ cargo modules generate tree --types --lib
$ cargo modules generate tree --types --bin raycasting_with_rust
```
