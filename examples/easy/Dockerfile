FROM amazonlinux:2.0.20200722.0

LABEL url="https://github.com/chiefbiiko/crab-lambda" \
  version="0.1.0" \
  title="crab-lambda-function-build-image" \
  description="Docker image for building a lambda for the crab-lambda runtime" \
  maintainer="Noah Anabiik Schwarz" \
  license="MIT"
# -C target-feature=-crt-static -C prefer-dynamic
ENV RUST_VERSION=1.45.2 \
  RUST_BACKTRACE=FULL \
  PATH="/root/.cargo/bin:$PATH" \
  CC_x86_64_unknown_linux_gnu=clang \
  CXX_x86_64_unknown_linux_gnu=clang++ \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang++ \
  RUSTFLAGS="-C link-arg=-lpthread -C link-arg=-lm --crate-type=dylib" \
  BUNDLE_ZIP_FILE_NAME=example.zip \
  LIBRARY_NAME=libcrab_lambda_easy_example.so

RUN yum install -y \
  glibc-headers-2.26-35.amzn2.0.2.x86_64 \
  glibc-devel-2.26-35.amzn2.0.2.x86_64 \
  clang-7.0.1-1.amzn2.0.2.x86_64 \
  clang-libs-7.0.1-1.amzn2.0.2.x86_64 \
  clang-devel-7.0.1-1.amzn2.0.2.x86_64 \
  cmake3-3.13.1-1.amzn2.x86_64 \
  make-1:3.82-23.amzn2.x86_64 \
  ncurses-compat-libs-6.0-8.20170212.amzn2.1.3.x86_64 \
  ncurses-devel-6.0-8.20170212.amzn2.1.3.x86_64 \
  openssl-devel-1:1.0.2k-19.amzn2.1.1.x86_64 \
  zip-3.0-11.amzn2.0.2.x86_64 && \
  ln -s /usr/bin/cmake3 /usr/bin/cmake && \
  rm -rf /var/cache/yum && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain $RUST_VERSION --profile minimal -y

VOLUME /home
WORKDIR /home

COPY Cargo.toml /home/
COPY src /home/src/

CMD ["sh", "-c", "cargo build --release && cp ./target/release/$LIBRARY_NAME /tmp/example.so && zip -mj $BUNDLE_ZIP_FILE_NAME /tmp/example.so"]
