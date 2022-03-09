clean:
	cargo clean && \
	rm -rf lambda.zip
	
release:
	docker run \
	--platform linux/arm64 \
	--rm --user "$(id -u)":"$(id -g)" \
	-v "${PWD}":/usr/src/code -w /usr/src/code rust:latest \
	cargo lambda build --release --target x86_64-unknown-linux-gnu

zip:
	zip -j lambda.zip target/lambda/rust-lambda/bootstrap	

package: release zip
	