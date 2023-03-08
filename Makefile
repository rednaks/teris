web:
	git checkout gh-pages
	git merge master
	cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/teris.wasm www/teris.wasm
	git add www/teris.wasm
	git commit -m "release $(date +%Y.%m.%d.%H.%M)"
