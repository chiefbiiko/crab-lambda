FROM amazonlinux:2.0.20191217.0

LABEL url="https://github.com/chiefbiiko/crab-lambda" \
  version="0.1.0" \
  title="crab-lambda-runtime-build-image" \
  description="Docker image for building the crab-lambda runtime" \
  maintainer="Noah Anabiik Schwarz" \
  license="MIT"

ENV RUST_VERSION=1.41.0 \
  RUST_BACKTRACE=FULL \
  CARGO_MAKE_VERSION=0.27.0 \
  PATH=/root/.cargo/bin:$PATH \
  PKG_CONFIG_ALLOW_CROSS=true \
  CARGO_BUILD_TARGET=x86_64-unknown-linux-musl \
  CC_x86_64_unknown_linux_musl=clang \
  CXX_x86_64_unknown_linux_musl=clang++ \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=clang++ \
  RUNTIME_ZIP_FILE_NAME=runtime.zip

RUN yum install -y \
  clang-7.0.1-1.amzn2.0.2.x86_64 \
  clang-libs-7.0.1-1.amzn2.0.2.x86_64 \
  clang-devel-7.0.1-1.amzn2.0.2.x86_64 \
  cmake3-3.13.1-1.amzn2.x86_64 \
  make-1:3.82-23.amzn2.x86_64 \
  ncurses-compat-libs-6.0-8.20170212.amzn2.1.3.x86_64 \
  ncurses-devel-6.0-8.20170212.amzn2.1.3.x86_64 \
  openssl-devel-1:1.0.2k-19.amzn2.1.1.x86_64 \
  unzip-6.0-20.amzn2.x86_64 \
  zip-3.0-11.amzn2.0.2.x86_64 && \
  ln -s /usr/bin/cmake3 /usr/bin/cmake && \
  rm -rf /var/cache/yum && \
  curl -fsSL https://sh.rustup.rs | sh -s -- --default-toolchain $RUST_VERSION -y && \
  rustup target add x86_64-unknown-linux-musl && \
  temp_file=$(mktemp) && temp_dir=$(mktemp -d) && \
  curl -fsSL https://github.com/sagiegurari/cargo-make/releases/download/$CARGO_MAKE_VERSION/cargo-make-v$CARGO_MAKE_VERSION-x86_64-unknown-linux-musl.zip -o $temp_file && \
  unzip $temp_file -d $temp_dir && \
  mv $temp_dir/cargo-make-v$CARGO_MAKE_VERSION-x86_64-unknown-linux-musl/cargo-make /root/.cargo/bin/cargo-make && \
  rm $temp_file && rm -rf $temp_dir

VOLUME /home

WORKDIR /home

COPY Cargo.toml Makefile.toml /home/
COPY src /home/src/
# copying the easy example stuff to also build it with this lambda-compat image
COPY examples/easy/src /home/examples/easy/src/
COPY examples/easy/Cargo.toml /home/examples/easy/

CMD ["cargo", "make", "runtime"]