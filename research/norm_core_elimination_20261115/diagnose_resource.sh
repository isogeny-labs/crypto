#!/usr/bin/env bash
set -euo pipefail
norm_resource_backend=$(realpath "${1:?usage: diagnose_resource.sh BACKEND INPUT NEW_OUTDIR}")
norm_resource_input=$(realpath "${2:?preserved input required}")
norm_resource_root=$(realpath -m "${3:?new output directory required}")
mkdir "$norm_resource_root"
sha256sum "$norm_resource_backend" "$norm_resource_input" > "$norm_resource_root/identities.sha256"
for norm_resource_mode in memory-trace pair512; do
    norm_resource_dir="$norm_resource_root/$norm_resource_mode"
    mkdir "$norm_resource_dir"
    cp "$norm_resource_input" "$norm_resource_dir/input.ms"
    norm_resource_args=(-t 1 -g 2 -v 2 -l 2 -f "$norm_resource_dir/input.ms" -o "$norm_resource_dir/basis.gb")
    norm_resource_prefix=()
    if [ "$norm_resource_mode" = memory-trace ]; then
        norm_resource_prefix=(strace -qq -o "$norm_resource_dir/memory.strace" -e trace=brk,mmap,mremap,munmap -e signal=SIGSEGV)
    else
        norm_resource_args+=(-m 512)
    fi
    printf '%s\n' "${norm_resource_prefix[*]} $norm_resource_backend ${norm_resource_args[*]}" > "$norm_resource_dir/command.txt"
    if (
        ulimit -v 2097152 || exit 125
        ulimit -t 20 || exit 125
        exec timeout --signal=KILL 20 "${norm_resource_prefix[@]}" "$norm_resource_backend" "${norm_resource_args[@]}"
    ) > "$norm_resource_dir/backend.stdout" 2> "$norm_resource_dir/backend.stderr"; then
        norm_resource_exit=0
    else
        norm_resource_exit=$?
    fi
    printf '%s\n' "$norm_resource_exit" > "$norm_resource_dir/exit.txt"
done
