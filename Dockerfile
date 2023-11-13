# First stage: build the Rust project
# Use the official Rust image for the build stage
FROM rust:latest as builder

# Instead of creating a new project, we're going to copy in our existing one
# Copy our entire project and set it as the working directory
COPY . /usr/src/retcd-server
WORKDIR /usr/src/retcd-server/cmd/retcd-server

# Build our application
RUN cargo build --release

# Second stage: setup the runtime environment
# Start from an empty container
FROM scratch

# Copy the build artifact from the build stage
COPY --from=builder /usr/src/retcd-server/cmd/retcd-server/target/release/retcd-server .

# Set the CMD to your binary
CMD ["./retcd-server"]
