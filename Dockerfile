FROM amazonlinux:2.0.20200722.0

LABEL url="https://github.com/chiefbiiko/crab-lambda" \
  version="0.1.0" \
  title="crab-lambda-runtime-build-image" \
  description="Docker image for building the crab-lambda runtime" \
  maintainer="Noah Anabiik Schwarz" \
  license="MIT"

ENV RUST_VERSION=1.45.2 \
  RUST_BACKTRACE=FULL \
  PATH="/root/.cargo/bin:$PATH" \
  CC_x86_64_unknown_linux_gnu=clang \
  CXX_x86_64_unknown_linux_gnu=clang++ \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang++ \
  RUNTIME_ZIP_FILE_NAME=runtime.zip

RUN yum install -y \
  tree \
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
  curl -fsSL https://sh.rustup.rs | sh -s -- --default-toolchain $RUST_VERSION --profile minimal -y && \
  yum clean all -y && \
  rm -rf /var/cache/yum

VOLUME /home

WORKDIR /home

COPY Cargo.toml /home/
COPY src /home/src/

CMD ["sh", "-c", "cargo build --release && tree ./target && cp ./target/release/crab_lambda_runtime /tmp/bootstrap && zip -mj ./$RUNTIME_ZIP_FILE_NAME /tmp/bootstrap"]
