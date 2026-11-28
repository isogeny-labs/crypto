#!/usr/bin/env bash
set -euo pipefail
study_dir=$(cd "$(dirname "$0")" && pwd)
build_dir=$(mktemp -d)
trap 'rm -rf -- "$build_dir"' EXIT
revision=73d564e1379871a772a0c3464541d3b4751bfc5b
git -C "$study_dir" show "$revision:src/cryptanalysis/semaev_decomp.rs" > "$build_dir/upstream.rs"
python3 - "$build_dir/upstream.rs" "$build_dir/kernel.rs" <<'PY'
from pathlib import Path
import sys
source = Path(sys.argv[1]).read_text()
start_marker = 'use crate::binary_ecc::{F2mElement, IrreduciblePoly};\n'
stop_marker = '    /// Lift to the crate\'s general element type.'
start = source.index(start_marker) + len(start_marker)
end = source.index(stop_marker, start)
kernel = source[start:end].replace('const MAX_DEG: usize = 6;\n', '')
Path(sys.argv[2]).write_text(kernel + '}\n')
PY
S3_FIELD_KERNEL="$build_dir/kernel.rs" "${RUSTC:-rustc}" --edition 2021 -O     "$study_dir/native_coset.rs" -o "$build_dir/native_coset"
"$build_dir/native_coset" "$@"
