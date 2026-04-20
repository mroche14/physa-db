#!/usr/bin/env bash
# bench-collect-iai.sh — capture iai-callgrind instruction counts into per-bench files.
#
# Usage:
#   bench-collect-iai.sh <dest_dir>
#
# Reads `target/iai-run.txt` (captured stdout of `just bench-iai`) and writes
# one file per bench: <dest_dir>/<bench_path>.ir containing just the
# instruction count as a single integer.
#
# Parsing rather than JSON because the iai-callgrind JSON schema has shifted
# between versions; the human-readable output is the stable contract.

set -euo pipefail

DEST="${1:?usage: $0 <dest_dir>}"
SRC="${2:-target/iai-run.txt}"

mkdir -p "$DEST"

if [ ! -f "$SRC" ]; then
  echo "bench-collect-iai: no iai output at $SRC — nothing to collect" >&2
  exit 0
fi

# awk extracts: non-indented "foo::bar::baz" header, then the first integer
# before the `|` on the next line starting with "  Instructions:".
awk '
  /^[A-Za-z_][A-Za-z0-9_]*(::[A-Za-z0-9_]+)+[[:space:]]*$/ {
    gsub(/[[:space:]]+$/, "")
    current = $0
    next
  }
  /^[[:space:]]+Instructions:/ {
    line = $0
    sub(/^[[:space:]]+Instructions:[[:space:]]+/, "", line)
    split(line, arr, "|")
    gsub(/[^0-9]/, "", arr[1])
    if (current != "" && arr[1] != "") {
      print current "=" arr[1]
    }
  }
' "$SRC" | while IFS='=' read -r key ir; do
  [ -z "$key" ] && continue
  safe=$(echo "$key" | tr -c 'A-Za-z0-9_:-' '_')
  echo "$ir" > "$DEST/${safe}.ir"
done

count=$(ls -1 "$DEST"/*.ir 2>/dev/null | wc -l)
echo "bench-collect-iai: collected $count benches into $DEST"
if [ "$count" -eq 0 ]; then
  echo "WARNING: no benches matched. Dumping $SRC head:" >&2
  head -n 50 "$SRC" >&2 || true
fi
