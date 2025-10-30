#!/usr/bin/env bash
set -euo pipefail

PKG_NAME="cf-alias-bin"
DEPLOY_DIR="dist/aur/${PKG_NAME}"

if [[ ! -d "$DEPLOY_DIR" ]]; then
  echo "ERROR: ${DEPLOY_DIR} missing." >&2
  exit 1
fi

cd "$DEPLOY_DIR"
git init
git config user.name "4ndr0666"
git config user.email "01_dolor.loftier@icloud.com"
git remote add origin ssh://aur@aur.archlinux.org/${PKG_NAME}.git
git fetch origin || true
git checkout -B master
git add --all
git commit -S -m "release: $(date +%Y-%m-%d) ${PKG_NAME}"
git push --force origin master
