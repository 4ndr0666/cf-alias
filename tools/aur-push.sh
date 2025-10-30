#!/usr/bin/env bash
# ============================================================================
# AUR Push Automation Script — cf-alias-bin
# Fully deterministic, idempotent, snapshot-safe, and auditable
# ============================================================================

set -euo pipefail

# ----------------------------------------------------------------------------
# I. Static definitions
# ----------------------------------------------------------------------------
PKG_NAME="cf-alias-bin"
DEPLOY_DIR="dist/aur/${PKG_NAME}"
AUR_URL="ssh://aur@aur.archlinux.org/${PKG_NAME}.git"
GPG_KEY="A81FA5D0033F6EACD61808CD426D822F91067068"
TIMESTAMP="$(date +'%F %T')"
LOG_FILE="${HOME}/.automation_audit.log"

# ----------------------------------------------------------------------------
# II. Preflight checks
# ----------------------------------------------------------------------------
if [[ ! -d "${DEPLOY_DIR}" ]]; then
  echo "⚠️  ${DEPLOY_DIR} missing — snapshot mode or publish skipped." >&2
  echo "[${TIMESTAMP}] AUR_PUSH ${PKG_NAME} SKIPPED (no directory)" >> "${LOG_FILE}"
  exit 0
fi

cd "${DEPLOY_DIR}"

# ensure PKGBUILD integrity
if [[ ! -f PKGBUILD ]]; then
  echo "❌ PKGBUILD missing inside ${DEPLOY_DIR}." >&2
  exit 1
fi

# ----------------------------------------------------------------------------
# III. Repository initialization
# ----------------------------------------------------------------------------
if [[ ! -d .git ]]; then
  git init
  git remote add origin "${AUR_URL}"
else
  git remote set-url origin "${AUR_URL}"
fi

git fetch origin master || true
git checkout -B master || git checkout master

git config user.name "4ndr0666"
git config user.email "01_dolor.loftier@icloud.com"

# ----------------------------------------------------------------------------
# IV. Generate metadata and commit
# ----------------------------------------------------------------------------
if command -v makepkg &>/dev/null; then
  makepkg --printsrcinfo > .SRCINFO
else
  echo "⚠️  makepkg not found — skipping .SRCINFO regeneration." >&2
fi

git add --all

# choose signing if key exists
if gpg --list-secret-keys "${GPG_KEY}" &>/dev/null; then
  SIGN_FLAG="-S"
else
  echo "⚠️  GPG key ${GPG_KEY} unavailable — committing unsigned."
  SIGN_FLAG=""
fi

if git diff --cached --quiet; then
  echo "✅ Nothing new to commit."
else
  git commit ${SIGN_FLAG} -m "release: $(date +%Y-%m-%d) ${PKG_NAME}"
fi

# ----------------------------------------------------------------------------
# V. Push to remote
# ----------------------------------------------------------------------------
echo "🚀 Pushing ${PKG_NAME} to AUR..."
git push --force origin master

# ----------------------------------------------------------------------------
# VI. Post-validation
# ----------------------------------------------------------------------------
if git ls-remote --exit-code origin &>/dev/null; then
  echo "✅ AUR push verified."
  echo "[${TIMESTAMP}] AUR_PUSH ${PKG_NAME} OK" >> "${LOG_FILE}"
else
  echo "❌ Remote verification failed."
  echo "[${TIMESTAMP}] AUR_PUSH ${PKG_NAME} FAILED" >> "${LOG_FILE}"
  exit 1
fi

# ----------------------------------------------------------------------------
# VII. Cleanup
# ----------------------------------------------------------------------------
cd - >/dev/null
echo "[${TIMESTAMP}] END AUR_PUSH ${PKG_NAME} complete"
