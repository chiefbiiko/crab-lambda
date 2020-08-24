PKG_CONFIG_ALLOW_CROSS=true \
CARGO_BUILD_TARGET=x86_64-unknown-linux-musl \
CC_x86_64_unknown_linux_musl=clang \
CXX_x86_64_unknown_linux_musl=clang++ \
CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=clang++ \
RUSTFLAGS="-C target-feature=-crt-static" \
cargo build --release && \
zip -mj example.zip ./target/x86_64-unknown-linux-musl/release/libcrab_lambda_easy_example.so
