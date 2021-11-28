clean:
	cargo clean && \
	rm -rf lambda.zip
	
release:
	docker run \
	--platform linux/arm64 \
	--rm --user "$(id -u)":"$(id -g)" \
	-v "${PWD}":/usr/src/code -w /usr/src/code rust:latest \
	cargo build --release --target "aarch64-unknown-linux-gnu"

zip:
	cp target/aarch64-unknown-linux-gnu/release/rust-lambda ./bootstrap && \
	zip lambda.zip bootstrap && \
	rm bootstrap

package: release zip
	