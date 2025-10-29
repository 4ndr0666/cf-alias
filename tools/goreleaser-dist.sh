#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="target/release"
DIST_DIR="dist/cf-alias_linux_amd64_v1"

echo "[cf-alias] preparing Arch-native release artifacts"

mkdir -p "${DIST_DIR}"

# Copy binary built natively for Arch (x86_64-linux-gnu)
cp "${TARGET_DIR}/cf-alias" "${DIST_DIR}/cf-alias"

# Copy metadata for distribution consistency
if [[ -f "LICENSE" ]]; then
  cp LICENSE "${DIST_DIR}/"
fi
if [[ -f "THIRDPARTY.json" ]]; then
  cp THIRDPARTY.json "${DIST_DIR}/"
fi

echo "[cf-alias] release package ready in ${DIST_DIR}"
