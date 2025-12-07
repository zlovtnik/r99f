# Build stage
FROM rust:1.75 as builder

# Install trunk and wasm target
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy manifests
COPY Cargo.toml Trunk.toml ./

# Create dummy source to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN trunk build --release || true
RUN rm -rf src

# Copy actual source code
COPY src ./src
COPY index.html ./
COPY styles.css ./

# Build the application
RUN trunk build --release

# Runtime stage - nginx to serve static files
FROM nginx:alpine

# Copy built assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
