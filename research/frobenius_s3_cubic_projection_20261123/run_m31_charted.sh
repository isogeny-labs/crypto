#!/usr/bin/env bash
set -euo pipefail
study_dir=$(cd "$(dirname "$0")" && pwd)
build_dir=$(mktemp -d)
trap 'rm -rf -- "$build_dir"' EXIT
git -C "$study_dir" cat-file blob 88b86018a694340bdc36be007a37b6a3b3debf50 > "$build_dir/upstream.rs"
python3 - "$build_dir/upstream.rs" "$build_dir/field.rs" <<'PY'
from pathlib import Path
import sys
src = Path(sys.argv[1]).read_text()
start = src.index('/// `F_p` arithmetic')
end = src.index('    /// `S₃(u, v, X)` as coefficients', start)
det_start = src.index('/// Sylvester resultant of')
det_end = src.index('/// A dense polynomial in', det_start)
Path(sys.argv[2]).write_text(src[start:end] + '}\n' + src[det_start:det_end])
PY
S3_PRIME_FIELD="$build_dir/field.rs" "${RUSTC:-rustc}" --edition 2021 -O \
    "$study_dir/m31_charted.rs" -o "$build_dir/m31_charted"
"$build_dir/m31_charted" "$@"
