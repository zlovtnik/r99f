# Build stage
FROM --platform=$BUILDPLATFORM rust:1.83 as builder

ARG TARGETARCH

# Install trunk from pre-built binary (select arch based on target)
RUN case "$TARGETARCH" in \
      amd64) TRUNK_ARCH="x86_64-unknown-linux-gnu" ;; \
      arm64) TRUNK_ARCH="aarch64-unknown-linux-gnu" ;; \
      *) echo "Unsupported arch: $TARGETARCH" && exit 1 ;; \
    esac && \
    curl -fsSL "https://github.com/trunk-rs/trunk/releases/download/v0.20.3/trunk-${TRUNK_ARCH}.tar.gz" \
    | tar -xzf - -C /usr/local/bin

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy manifests
COPY Cargo.toml Trunk.toml ./

# Create target directory (required by Trunk.toml watch ignore)
RUN mkdir -p target

# Create dummy source to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN trunk build --release || true
RUN rm -rf src

# Copy actual source code
COPY src ./src
COPY index.html ./
COPY styles.css ./

# Build the application
# wasm-opt is disabled in Trunk.toml (no aarch64-linux binary available)
RUN trunk build --release

# Runtime stage - nginx to serve static files
FROM nginx:alpine

# Copy built assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
