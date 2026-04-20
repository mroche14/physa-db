#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RELEASE_PLZ="${RELEASE_PLZ:-release-plz}"
TMP_WORKTREE="$(mktemp -d /tmp/physa-release-plz.XXXXXX)"
TMP_BRANCH="tmp/release-plz-dry-run-$(date +%s)-$$"

cleanup() {
  git -C "$ROOT" worktree remove "$TMP_WORKTREE" --force >/dev/null 2>&1 || true
  git -C "$ROOT" branch -D "$TMP_BRANCH" >/dev/null 2>&1 || true
}
trap cleanup EXIT

git -C "$ROOT" worktree add -b "$TMP_BRANCH" "$TMP_WORKTREE" HEAD
git -C "$TMP_WORKTREE" branch --set-upstream-to=origin/main "$TMP_BRANCH"
if ! git -C "$ROOT" diff --quiet HEAD -- . ":(exclude)private/**"; then
  git -C "$ROOT" diff --binary HEAD -- . ":(exclude)private/**" |
    git -C "$TMP_WORKTREE" apply --allow-empty
fi

cd "$TMP_WORKTREE"
"$RELEASE_PLZ" update --manifest-path Cargo.toml --allow-dirty

echo
echo "Disposable release-plz package/version/changelog changes in the temporary worktree:"
git status --short -- Cargo.toml Cargo.lock crates xtask
