#!/usr/bin/env bash
# Thin orchestration only. Every mathematical computation/check is native Rust.
set -eu
study=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
worker=/workspace/relation-trace-exact-417
data=/workspace/relation-degree-scan-417
manifest="$study/FIXED_INPUTS_412.json"
calls="$study/raw/calls-417.tsv"
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
mkdir /workspace/relation-degree-existing-417
printf 'existing directory must remain unchanged\n' > /workspace/relation-degree-existing-417/sentinel
sha256sum /workspace/relation-degree-existing-417/sentinel > "$study/raw/existing-before-417.sha256"
run existing-417 scan "$manifest" /workspace/relation-degree-existing-417
sha256sum /workspace/relation-degree-existing-417/sentinel > "$study/raw/existing-after-417.sha256"
# An expected preflight refusal never prevents the fresh-directory scan.
run scan-417 scan "$manifest" "$data"
if test "$call_exit" -eq 0; then
    run replay-417 replay "$manifest" "$data"
    run index-before-417 index "$data" "$study/ARTIFACT_INDEX_BEFORE_417.json"
    for mutation in summary proof degree roster inputhash; do
        canary=/workspace/relation-degree-canary-417-$mutation
        run canary-$mutation-417 tamper-$mutation "$data" "$canary"
        if test "$call_exit" -eq 0; then
            run reject-$mutation-417 replay "$manifest" "$canary"
            cp "$canary.index.json" "$study/CANARY_INDEX_${mutation}_417.json"
            if test "$mutation" = summary; then
                cp "$canary/summary.json" "$study/contexts/canary-summary-417.json"
            elif test "$mutation" != roster; then
                cp "$canary/p2147483647-n3-d3-single-0.json" "$study/contexts/canary-$mutation-417.json"
            fi
        fi
    done
    run index-after-417 index "$data" "$study/ARTIFACT_INDEX_AFTER_417.json"
fi
for prime in 2147483647 9007199254740881; do
    run labels-$prime-417 labels "$study/label-inputs/p$prime.json" "$study/LABEL_RESULT_${prime}_417.json"
    if test "$call_exit" -eq 0; then
        run labels-replay-$prime-417 labels-replay "$study/label-inputs/p$prime.json" "$study/LABEL_RESULT_${prime}_417.json"
    fi
done
cat "$calls"
