all:
	pushd server; cargo build --release
	pushd client; yarn install; yarn build

clean:
	pushd server; cargo clean
	pushd client; rm -rf node_modules yarn.lock package-lock.json dist