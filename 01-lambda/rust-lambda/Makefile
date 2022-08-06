clean:
	cargo clean && \
	rm -rf lambda.zip
	
release:
	docker run \
	-v "${PWD}":/usr/src/code -w /usr/src/code lambda \
	/root/.cargo/bin/cargo lambda build --release --target x86_64-unknown-linux-gnu

release-local:
	cargo lambda build --release --target x86_64-unknown-linux-gnu

zip:
	zip -j lambda.zip target/lambda/rust-lambda/bootstrap	

package: release zip

package-local: release-local zip
	