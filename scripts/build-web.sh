#!/usr/bin/env bash
# Build for wasm and run wasm-bindgen so the output is browser-ready.
# Usage: ./scripts/build-web.sh [--serve]
set -euo pipefail

cd "$(dirname "$0")/.."

PROFILE=release-web
TARGET=wasm32-unknown-unknown
OUT_DIR=web
WASM_BIN=target/${TARGET}/${PROFILE}/blockvy.wasm

echo ">> cargo build --profile ${PROFILE} --target ${TARGET}"
cargo build --profile "${PROFILE}" --target "${TARGET}"

echo ">> wasm-bindgen → ${OUT_DIR}/"
mkdir -p "${OUT_DIR}"
wasm-bindgen --no-typescript --target web \
    --out-dir "${OUT_DIR}" \
    --out-name blockvy \
    "${WASM_BIN}"

# Mirror the assets folder next to index.html so the browser's AssetServer
# can fetch samples via /assets/... — a missing assets dir just skips this
# step (useful for dev builds with no audio).
if [[ -d assets ]]; then
    echo ">> assets → ${OUT_DIR}/assets/"
    rm -rf "${OUT_DIR}/assets"
    cp -r assets "${OUT_DIR}/assets"
fi

echo ">> done — open ${OUT_DIR}/index.html via a local web server"

if [[ "${1:-}" == "--serve" ]]; then
    PORT="${PORT:-8000}"
    echo ">> serving http://localhost:${PORT}"
    python3 -m http.server --directory "${OUT_DIR}" "${PORT}"
fi
