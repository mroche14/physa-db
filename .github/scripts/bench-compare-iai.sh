#!/usr/bin/env bash
# bench-compare-iai.sh — compare two sets of .ir files, post PR comment, gate on threshold.
#
# Usage:
#   bench-compare-iai.sh <base_dir> <pr_dir>
#
# Env:
#   REGRESSION_THRESHOLD  Max allowed positive delta in percent (default 2.0)
#   PR_NUMBER             PR to comment on (optional — comment skipped if unset)
#   GH_TOKEN              Required for posting the comment
#
# Exits non-zero if any bench regresses by more than REGRESSION_THRESHOLD.

set -euo pipefail

BASE_DIR="${1:?usage: $0 <base_dir> <pr_dir>}"
PR_DIR="${2:?}"
THRESHOLD="${REGRESSION_THRESHOLD:-2.0}"

COMMENT=$(mktemp)
trap 'rm -f "$COMMENT"' EXIT

{
  echo "## bench-regression — iai instruction counts"
  echo
  echo "| Benchmark | Base | PR | Δ | Verdict |"
  echo "|-----------|-----:|---:|--:|:-------:|"
} > "$COMMENT"

regressed=0
compared=0
new_benches=()

for pr_file in "$PR_DIR"/*.ir; do
  [ -f "$pr_file" ] || continue
  key=$(basename "$pr_file" .ir)
  pr_val=$(tr -d '[:space:]' < "$pr_file")
  base_file="$BASE_DIR/$key.ir"

  if [ -f "$base_file" ]; then
    base_val=$(tr -d '[:space:]' < "$base_file")
    compared=$((compared + 1))
    if [ -n "$base_val" ] && [ "$base_val" != "0" ]; then
      delta=$(awk -v a="$base_val" -v b="$pr_val" 'BEGIN {printf "%+.2f", (b-a)*100/a}')
      if awk -v d="$delta" -v t="$THRESHOLD" 'BEGIN {exit !(d+0 > t+0)}'; then
        verdict="❌"
        regressed=$((regressed + 1))
      else
        verdict="✅"
      fi
    else
      delta="n/a"
      verdict="—"
    fi
    printf "| \`%s\` | %s | %s | %s%% | %s |\n" \
      "$key" "$base_val" "$pr_val" "$delta" "$verdict" >> "$COMMENT"
  else
    new_benches+=("$key")
    printf "| \`%s\` | _new_ | %s | — | 🆕 |\n" "$key" "$pr_val" >> "$COMMENT"
  fi
done

echo >> "$COMMENT"
if [ "$compared" -eq 0 ] && [ "${#new_benches[@]}" -eq 0 ]; then
  echo "_No iai benches captured. Check that \`just bench-iai\` ran and produced output._" >> "$COMMENT"
elif [ "$compared" -eq 0 ]; then
  echo "_No base-branch benches — all benches are new. Regression check skipped this run; baseline will populate on merge._" >> "$COMMENT"
elif [ "$regressed" -gt 0 ]; then
  echo "**Verdict:** ❌ FAIL — $regressed bench(es) regressed by more than +${THRESHOLD}% (AGENTS.md §5)." >> "$COMMENT"
else
  echo "**Verdict:** ✅ PASS (threshold: +${THRESHOLD}%)" >> "$COMMENT"
fi

echo "---"
cat "$COMMENT"
echo "---"

if [ -n "${PR_NUMBER:-}" ] && command -v gh >/dev/null 2>&1; then
  # Try edit-last to keep the conversation tidy; fall back to new comment.
  gh pr comment "$PR_NUMBER" --body-file "$COMMENT" --edit-last \
    || gh pr comment "$PR_NUMBER" --body-file "$COMMENT"
fi

if [ "$regressed" -gt 0 ]; then
  exit 1
fi
