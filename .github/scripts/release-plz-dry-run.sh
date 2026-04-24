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
# Bake any dirty diff into a disposable commit in the worktree, rather than
# leaving it unstaged. release-plz would otherwise need --allow-dirty and
# stash internally, and refs/stash is a global ref shared across worktrees —
# so its stash would survive the worktree removal and accumulate in the
# caller's repo. With a clean worktree, release-plz never stashes.
# The "chore(release):" prefix is `skip = true` in release-plz.toml so the
# commit does not pollute the previewed CHANGELOG.
if ! git -C "$ROOT" diff --quiet HEAD -- . ":(exclude)private/**"; then
  git -C "$ROOT" diff --binary HEAD -- . ":(exclude)private/**" |
    git -C "$TMP_WORKTREE" apply --allow-empty
  git -C "$TMP_WORKTREE" add --all
  git -C "$TMP_WORKTREE" \
    -c user.name=physa-dry-run \
    -c user.email=physa-dry-run@users.noreply.github.com \
    commit --allow-empty -m "chore(release): dry-run preview (disposable)"
fi

cd "$TMP_WORKTREE"
"$RELEASE_PLZ" update --manifest-path Cargo.toml

echo
echo "Disposable release-plz package/version/changelog changes in the temporary worktree:"
git status --short -- Cargo.toml Cargo.lock crates xtask
