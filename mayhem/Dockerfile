# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Dependencies
RUN apt update && apt install -y build-essential cmake

## Remove this when current nightly can build the fuzz harness (no std::error::Backtrace error)
RUN rustup install nightly-2022-07-28

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN mkdir /testsuite && \
    sh -c "find . -name '*.parquet' -exec cp {} /testsuite/ \;"

RUN echo building instrumented harnesses && \
    bash -c "pushd common/meta/api/fuzz && cargo +nightly-2022-07-28 -Z sparse-registry fuzz build && popd" && \
    mv common/meta/api/fuzz/target/x86_64-unknown-linux-gnu/release/roundtrip /roundtrip && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd common/meta/api/fuzz && cargo +nightly-2022-07-28 -Z sparse-registry build --release && popd" && \
    mv common/meta/api/fuzz/target/release/roundtrip /roundtrip_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /roundtrip /roundtrip_no_inst /
COPY --from=builder /testsuite/  /testsuite/
