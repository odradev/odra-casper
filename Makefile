prepare:
	sudo apt install wabt
	rustup target add wasm32-unknown-unknown

test:
	cd shared && cargo test
	cd backend && cargo test
	cd codegen && cargo test

build-test-env:
	cd test_env && cargo build --release

clippy:
	cd backend && cargo clippy --target wasm32-unknown-unknown -- -D warnings
	cd codegen && cargo clippy --all-targets -- -D warnings
	cd shared && cargo clippy --all-targets -- -D warnings
	cd test_env && cargo clippy --all-targets -- -D warnings
	cd test_env/getter_proxy && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd backend && cargo fmt -- --check
	cd codegen && cargo fmt -- --check
	cd shared && cargo fmt -- --check
	cd test_env && cargo fmt -- --check
	cd test_env/getter_proxy && cargo fmt -- --check

lint: clippy
	cd backend && cargo fmt
	cd codegen && cargo fmt
	cd shared && cargo fmt
	cd test_env && cargo fmt
	cd test_env/getter_proxy && cargo fmt

clean:
	cd backend && cargo clean
	cd codegen && cargo clean
	cd shared && cargo clean
	cd test_env && cargo clean
	cd test_env/getter_proxy && cargo clean

docs-backend:
	cd backend && cargo doc --lib --no-deps --open

docs-shared:
	cd shared && cargo doc --lib --no-deps --open

docs-test-env:
	cd test_env && cargo doc --lib --no-deps --open

docs-codegen:
	cd codegen && cargo doc --lib --no-deps --open
