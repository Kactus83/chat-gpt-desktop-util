################               ################
################  BUILD STAGE  #################
################               ################

FROM rust:1.66.1

# Install GTK 4 and build essentials
RUN apt-get update && apt-get install -y libgtk-4-dev build-essential

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release


################                ################
################  DEPLOY STAGE  #################
################                ################

# Set the deployment stage
FROM gcr.io/distroless/cc-debian11

# Set the working directory to /app
WORKDIR /app

# Copy the binary from the build stage
COPY --from=build /app/target/release/chat-gpt-desktop-util .

# Start the application
CMD ["./chat-gpt-desktop-util"]
