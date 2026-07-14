#!/usr/bin/env bash
# Vercel's build image ships Rust and Node, but not Trunk or Tailwind.
#
# Two things here are deliberate, and both are about glibc. The build image's
# glibc is older than GLIBC_2.35:
#
#   Trunk    -> the musl tarball. The gnu one needs GLIBC_2.35 and won't start.
#   Tailwind -> the npm package, not the standalone binary. The "musl" binary is
#               dynamically linked against musl libc, which isn't on a glibc
#               image either, so neither prebuilt binary works. Node does.
#
# The two tools Trunk fetches for itself (wasm-bindgen, wasm-opt) are both
# statically linked, so they need no help.
set -euo pipefail

TRUNK_VERSION=0.21.14
TAILWIND_VERSION=4.2.4

export PATH="$PWD/.bin:$PWD/node_modules/.bin:$PATH"
mkdir -p .bin

echo "==> rust"
if ! command -v rustc >/dev/null 2>&1; then
  curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
  export PATH="$HOME/.cargo/bin:$PATH"
fi
rustc --version
rustup target add wasm32-unknown-unknown

echo "==> trunk ${TRUNK_VERSION}"
curl -sSfL "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-musl.tar.gz" \
  | tar -xzf - -C .bin
chmod +x .bin/trunk
trunk --version

echo "==> tailwindcss ${TAILWIND_VERSION}"
npm install --no-save --no-audit --no-fund \
  "@tailwindcss/cli@${TAILWIND_VERSION}" "tailwindcss@${TAILWIND_VERSION}"
tailwindcss --help >/dev/null

echo "==> build"
trunk build --release

echo "==> dist:"
ls -la dist
