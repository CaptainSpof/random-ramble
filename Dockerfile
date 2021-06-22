# select build image
FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new random-ramble
WORKDIR /random-ramble

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./CargoCacheDeps.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY core core
COPY cli cli

COPY ./Cargo.toml ./Cargo.toml

# build for release
RUN rm ./target/release/deps/random_ramble*
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /random-ramble/target/release/rr /usr/bin/rr

ENV RR_ADJS_PATH=/dict/adjectives/

ENV RR_THEMES_PATH=/dict/themes/

# set the startup command to run your binary
CMD ["./rr"]
