FROM rust:latest

RUN apt update && apt upgrade -y
RUN apt-get install -y libasound2-dev
RUN apt-get install -y libudev-dev

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Default command to build and test the project when the container starts
CMD ["sh", "-c", "cargo build --release --verbose && cargo test --release --verbose"]