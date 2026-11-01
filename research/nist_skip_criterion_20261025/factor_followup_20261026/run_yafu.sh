#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(git -C "$here" rev-parse --show-toplevel)
: "${NIST_FACTOR_OUT:?set a fresh output under this follow-up runs directory}"
: "${NIST_YAFU_MODE:?set ecm or nfs}"
: "${YAFU_BIN:=/tmp/nist-yafu-source/yafu}"
: "${GGNFS_DIR:=/tmp/nist-yafu-source/factor/lasieve5_64/bin/avx512/}"
: "${PARI_PREFIX:=/workspace/tools/sage_skeleton_env}"
: "${NIST_YAFU_THREADS:=2}"
: "${NIST_YAFU_CURVES:=1024}"
: "${NIST_YAFU_B1:=11000000}"
: "${NIST_YAFU_B2:=1100000000}"
if [ "$NIST_YAFU_MODE" = ecm ]; then
    : "${NIST_YAFU_SEED:=20261026}"
    : "${NIST_YAFU_WATCHDOG:=1800}"
else
    : "${NIST_YAFU_SEED:=20261027}"
    : "${NIST_YAFU_WATCHDOG:=3600}"
fi
for parameter in "$NIST_YAFU_THREADS" "$NIST_YAFU_CURVES" "$NIST_YAFU_B1" \
                 "$NIST_YAFU_B2" "$NIST_YAFU_SEED" "$NIST_YAFU_WATCHDOG"; do
    case "$parameter" in ''|*[!0-9]*) echo 'invalid numeric parameter' >&2; exit 1;; esac
done
checkpoint() {
    if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then
        "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@"
    else
        "$repo/scripts/checkpoint.sh" "$@"
    fi
}
case "$NIST_FACTOR_OUT" in "$here"/runs/*) ;; *) exit 1;; esac
[ ! -e "$NIST_FACTOR_OUT" ] || { echo 'output exists' >&2; exit 1; }
case "$NIST_YAFU_MODE" in ecm|nfs) ;; *) exit 1;; esac
checkpoint gate
mkdir -p "$NIST_FACTOR_OUT"
mkdir "$NIST_FACTOR_OUT/source-snapshot"
cp "$0" "$here/FACTORING_PROTOCOL.md" "$NIST_FACTOR_OUT/source-snapshot/"
cp "$here/runs/refine-001/p521-1136-plus.residuals.txt" "$NIST_FACTOR_OUT/input.txt"
value=$(cat "$NIST_FACTOR_OUT/input.txt")
case "$value" in ''|*[!0-9]*) exit 1;; esac
sha256sum "$YAFU_BIN" "$NIST_FACTOR_OUT/input.txt" > "$NIST_FACTOR_OUT/native-input.sha256"
printf '%s\n' "$NIST_YAFU_MODE" > "$NIST_FACTOR_OUT/method.txt"
printf 'threads=%s\ncurves=%s\nB1=%s\nB2=%s\nseed=%s\nwatchdog_seconds=%s\n' \
    "$NIST_YAFU_THREADS" "$NIST_YAFU_CURVES" "$NIST_YAFU_B1" "$NIST_YAFU_B2" \
    "$NIST_YAFU_SEED" "$NIST_YAFU_WATCHDOG" > "$NIST_FACTOR_OUT/parameters.txt"
checkpoint push
cd "$NIST_FACTOR_OUT"
set +e
if [ "$NIST_YAFU_MODE" = ecm ]; then
    LD_LIBRARY_PATH="$PARI_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
    timeout --kill-after=60 "$NIST_YAFU_WATCHDOG" "$YAFU_BIN" "ecm(@,$NIST_YAFU_CURVES)" \
      -threads "$NIST_YAFU_THREADS" -B1ecm "$NIST_YAFU_B1" -B2ecm "$NIST_YAFU_B2" \
      -seed "$NIST_YAFU_SEED" -saveB1 \
      -logfile factor.log -session session.log < input.txt > run.stdout 2> run.stderr
else
    LD_LIBRARY_PATH="$PARI_PREFIX/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
    timeout --kill-after=60 "$NIST_YAFU_WATCHDOG" "$YAFU_BIN" 'nfs(@)' \
      -threads "$NIST_YAFU_THREADS" -seed "$NIST_YAFU_SEED" -ggnfs_dir "$GGNFS_DIR" \
      -logfile factor.log -session session.log < input.txt > run.stdout 2> run.stderr
fi
status=$?
set -e
printf '%s\n' "$status" > run.status
if grep -Eq 'ecm:|nfs:|AVX-ECM|GMP-ECM|\*\*\*factors found' run.stdout factor.log 2>/dev/null; then
    printf '{"target_evaluated":true,"completion_requires_independent_proof":true}\n' > admission.json
else
    printf '{"target_evaluated":false,"complete":false}\n' > admission.json
    checkpoint push
    echo 'backend did not emit a target-execution marker' >&2
    exit 65
fi
checkpoint push
printf 'Backend attempt retained; status %s is not a prime or completion proof.\n' "$status"
