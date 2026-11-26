#!/usr/bin/env bash
# Thin orchestration only. Every mathematical computation/check is native Rust.
set -eu
study=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
worker=/workspace/relation-trace-exact-419
data=/dev/shm/relation-degree-scan-419
manifest="$study/FIXED_INPUTS_412.json"
calls="$study/raw/calls-419.tsv"
set -C
: > "$calls"
set +C
run() {
    call_name=$1
    shift
    call_exit=0
    (ulimit -v 2097152; ulimit -f 32768
     OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RAYON_NUM_THREADS=1 \
       timeout 1200s taskset -c 0 "$worker" "$@") \
       > "$study/raw/$call_name.stdout" 2> "$study/raw/$call_name.stderr" || call_exit=$?
    printf '%s\t%s\n' "$call_name" "$call_exit" >> "$calls"
}
mkdir /workspace/relation-degree-existing-419
printf 'existing directory must remain unchanged\n' > /workspace/relation-degree-existing-419/sentinel
sha256sum /workspace/relation-degree-existing-419/sentinel > "$study/raw/existing-before-419.sha256"
run existing-419 scan "$manifest" /workspace/relation-degree-existing-419
sha256sum /workspace/relation-degree-existing-419/sentinel > "$study/raw/existing-after-419.sha256"
# An expected preflight refusal never prevents the fresh-directory scan.
run scan-419 scan "$manifest" "$data"
if test "$call_exit" -eq 0; then
    run replay-419 replay "$manifest" "$data"
    run index-before-419 index "$data" "$study/ARTIFACT_INDEX_BEFORE_419.json"
    for mutation in summary proof degree roster inputhash; do
        canary=/dev/shm/relation-degree-canary-419-$mutation
        run canary-$mutation-419 tamper-$mutation "$data" "$canary"
        if test "$call_exit" -eq 0; then
            run reject-$mutation-419 replay "$manifest" "$canary"
            cp "$canary.index.json" "$study/CANARY_INDEX_${mutation}_419.json"
            if test "$mutation" = summary; then
                cp "$canary/summary.json" "$study/contexts/canary-summary-419.json"
            elif test "$mutation" != roster; then
                cp "$canary/p2147483647-n3-d3-single-0.json" "$study/contexts/canary-$mutation-419.json"
            fi
        fi
    done
    run index-after-419 index "$data" "$study/ARTIFACT_INDEX_AFTER_419.json"
fi
for prime in 2147483647 9007199254740881; do
    run labels-$prime-419 labels "$study/label-inputs/p$prime.json" "$study/LABEL_RESULT_${prime}_419.json"
    if test "$call_exit" -eq 0; then
        run labels-replay-$prime-419 labels-replay "$study/label-inputs/p$prime.json" "$study/LABEL_RESULT_${prime}_419.json"
    fi
done
cat "$calls"
