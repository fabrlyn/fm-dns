build:
	cargo build --release

pack: build
	upx --best --lzma target/release/fm-dns-cli

install: pack
	mv target/release/fm-dns-cli target/release/fm-dns
	sudo mv target/release/fm-dns /usr/local/bin/