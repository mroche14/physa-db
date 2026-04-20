#!/usr/bin/env bash
# bench-package-results.sh — package nightly bench output into a time-series JSON
# to be published to gh-pages:bench-history/.
#
# Usage:
#   bench-package-results.sh <output_dir>
#
# Reads:
#   <output_dir>/hardware.env   (key=value pairs from bench-nightly.yml)
#   target/iai-run.txt          (captured stdout of `just bench-iai`)
#   target/criterion/           (cargo-criterion raw output, if present)
#
# Writes:
#   <output_dir>/publish/<YYYY-MM-DD>-<short-sha>.json
#   <output_dir>/publish/latest.json   (copy for dashboard convenience)

set -euo pipefail

OUT="${1:?usage: $0 <output_dir>}"
PUBLISH="$OUT/publish"
mkdir -p "$PUBLISH"

# Load hardware context
if [ -f "$OUT/hardware.env" ]; then
  # shellcheck disable=SC1090
  set -a; . "$OUT/hardware.env"; set +a
fi

SHA="${commit:-${GITHUB_SHA:-$(git rev-parse HEAD 2>/dev/null || echo unknown)}}"
SHORT_SHA="${SHA:0:7}"
DATE_UTC="${date:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"
DAY_UTC=$(date -u +%Y-%m-%d -d "$DATE_UTC" 2>/dev/null || date -u +%Y-%m-%d)

# Collect iai instruction counts into a temp dir, then fold into JSON
IAI_TMP=$(mktemp -d)
trap 'rm -rf "$IAI_TMP"' EXIT
bash "$(dirname "$0")/bench-collect-iai.sh" "$IAI_TMP" "${IAI_SRC:-target/iai-run.txt}" || true

# Build iai JSON: { bench_name: instruction_count, ... }
iai_json="{}"
if compgen -G "$IAI_TMP"/*.ir >/dev/null; then
  iai_json=$(
    for f in "$IAI_TMP"/*.ir; do
      k=$(basename "$f" .ir)
      v=$(tr -d '[:space:]' < "$f")
      printf '{"%s":%s}\n' "$k" "${v:-null}"
    done | jq -s 'add // {}'
  )
fi

# Criterion is best-effort — we only record that results exist; full parse lives in the dashboard later.
criterion_json='null'
if [ -d target/criterion ]; then
  estimates_count=$(find target/criterion -name estimates.json 2>/dev/null | wc -l)
  criterion_json=$(jq -n --argjson n "$estimates_count" '{targets_with_estimates: $n}')
fi

jq -n \
  --arg schema "physa-db.bench-history.v1" \
  --arg commit "$SHA" \
  --arg ref "${ref:-unknown}" \
  --arg date "$DATE_UTC" \
  --arg kernel "${kernel:-unknown}" \
  --arg cpu "${cpu:-unknown}" \
  --argjson cores "${cores:-0}" \
  --argjson mem_kb "${mem_kb:-0}" \
  --arg rustc "${rustc:-unknown}" \
  --argjson iai "$iai_json" \
  --argjson criterion "$criterion_json" \
  '{
    schema: $schema,
    commit: $commit,
    ref: $ref,
    date: $date,
    hardware: {
      kernel: $kernel,
      cpu: $cpu,
      cores: $cores,
      mem_kb: $mem_kb,
      rustc: $rustc
    },
    iai: $iai,
    criterion: $criterion
  }' > "$PUBLISH/${DAY_UTC}-${SHORT_SHA}.json"

cp "$PUBLISH/${DAY_UTC}-${SHORT_SHA}.json" "$PUBLISH/latest.json"

echo "bench-package-results: wrote $PUBLISH/${DAY_UTC}-${SHORT_SHA}.json"
ls -la "$PUBLISH/"
