# Create chef image that installs cargo chef
FROM rust:latest as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:latest AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Copy everything in current app
FROM rust:latest as builder 
COPY . /app

# Set the working directory to the app folder that we just copied everything in to.
WORKDIR /app

COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Build the image using release tag
RUN cargo build --release

# Pull down small debian image hosted on google container repo
FROM gcr.io/distroless/cc-debian11 AS runtime

# Copy release build from builder image
COPY --from=builder /app/target/release/rust-auth /app/rust-auth
WORKDIR /app

CMD ["./Desk1Datamart"]
