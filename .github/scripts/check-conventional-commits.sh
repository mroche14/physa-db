#!/usr/bin/env bash
set -euo pipefail

readonly conventional_re='^(feat|fix|perf|refactor|docs|test|bench|build|ci|chore)(\([a-z0-9][a-z0-9-]*\))?!?: .+'
readonly revert_re='^revert: .+'
readonly merge_re='^Merge pull request #[0-9]+'

usage() {
  cat >&2 <<'USAGE'
Usage:
  check-conventional-commits.sh --title "feat(scope): subject"
  check-conventional-commits.sh --subjects <file|->
USAGE
}

valid_subject() {
  local subject="$1"

  [[ "$subject" =~ $conventional_re ]] ||
    [[ "$subject" =~ $revert_re ]] ||
    [[ "$subject" =~ $merge_re ]]
}

check_one() {
  local label="$1"
  local subject="$2"

  if valid_subject "$subject"; then
    return 0
  fi

  printf 'Invalid conventional commit %s: %s\n' "$label" "$subject" >&2
  printf 'Expected: type(optional-scope): subject, with type in feat|fix|perf|refactor|docs|test|bench|build|ci|chore.\n' >&2
  return 1
}

check_subjects() {
  local input="$1"
  local failures=0
  local line_no=0
  local subject

  while IFS= read -r subject || [[ -n "$subject" ]]; do
    line_no=$((line_no + 1))
    [[ -z "$subject" ]] && continue
    check_one "#$line_no" "$subject" || failures=$((failures + 1))
  done <"$input"

  if [[ "$failures" -gt 0 ]]; then
    printf 'Found %s invalid commit subject(s).\n' "$failures" >&2
    return 1
  fi
}

case "${1:-}" in
  --title)
    if [[ "$#" -ne 2 ]]; then
      usage
      exit 2
    fi
    check_one "title" "$2"
    ;;
  --subjects)
    if [[ "$#" -ne 2 ]]; then
      usage
      exit 2
    fi
    if [[ "$2" == "-" ]]; then
      tmp="$(mktemp)"
      trap 'rm -f "$tmp"' EXIT
      cat >"$tmp"
      check_subjects "$tmp"
    else
      check_subjects "$2"
    fi
    ;;
  *)
    usage
    exit 2
    ;;
esac
