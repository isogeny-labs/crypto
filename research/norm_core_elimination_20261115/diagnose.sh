#!/usr/bin/env bash
# Two prospectively fixed native diagnostics; no cryptographic computation here.
set -euo pipefail
norm_diag_backend=$(realpath "${1:?usage: diagnose.sh BACKEND PRESERVED_INPUT NEW_OUTDIR}")
norm_diag_input=$(realpath "${2:?preserved input required}")
norm_diag_root=$(realpath -m "${3:?new output directory required}")
mkdir "$norm_diag_root"
sha256sum "$norm_diag_backend" "$norm_diag_input" > "$norm_diag_root/identities.sha256"
for norm_diag_linear in 2 1; do
    norm_diag_dir="$norm_diag_root/linear-$norm_diag_linear"
    mkdir "$norm_diag_dir"
    cp "$norm_diag_input" "$norm_diag_dir/input.ms"
    printf '%s\n' '-t 1 -g 2 -v 2' "-l $norm_diag_linear" > "$norm_diag_dir/flags.txt"
    if (
        ulimit -v 2097152 || exit 125
        ulimit -t 20 || exit 125
        exec timeout --signal=KILL 20 "$norm_diag_backend" -t 1 -g 2 -v 2 \
            -l "$norm_diag_linear" -f "$norm_diag_dir/input.ms" -o "$norm_diag_dir/basis.gb"
    ) > "$norm_diag_dir/backend.stdout" 2> "$norm_diag_dir/backend.stderr"; then
        norm_diag_exit=0
    else
        norm_diag_exit=$?
    fi
    printf '%s\n' "$norm_diag_exit" > "$norm_diag_dir/exit.txt"
done
