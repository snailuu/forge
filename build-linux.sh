#!/bin/bash
set -e

echo "=== Building static Linux x86_64 binary (musl) ==="

docker run --rm \
  --platform linux/amd64 \
  -v "$(pwd)":/workspace \
  -w /workspace \
  messense/rust-musl-cross:x86_64-musl \
  cargo build --release --target x86_64-unknown-linux-musl

echo "✅ Build complete!"
echo "Binary: target/x86_64-unknown-linux-musl/release/forge (static, no GLIBC dependency)"
