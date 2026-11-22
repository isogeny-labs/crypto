#!/usr/bin/env bash
# PS-6: write the ecbench specs (specs/counted_n<N>.json, specs/timed_n<N>.json), one per rung.
# The specs are committed before any session runs; this script is how they were written.
#
# Rungs are the boundary ledger's char2 ladder: random_binary_instance(n, 0x1CB00BDA7A, 8) for
# n = 15, 18, 21, 24, 27 (BoundaryConfig::default), factor base the low-order subspace of
# dimension ceil(n/3), three summands.  The seed 0x1CB00BDA7A is 123212651130.
#
#   counted: counted unit only (no wall-time claim), 8 targets x 2 rounds (x 1 round at n = 27, where
#            one baseline run takes about a minute and a half), L0, --cpus none.
#   timed:   wall time too: 8 targets x 5 rounds (+1 warm-up) at n <= 24, an A/A control arm
#            (the baseline repeated), isolation level L2 required for a wall figure.  At n = 27
#            the baseline costs about a minute and a half per run, so the timed session has
#            TIMED27_TARGETS targets (default 4) and TIMED27_ROUNDS rounds (default 5).
set -euo pipefail
cd "$(dirname "$0")"
SEED=123212651130
TARGET_SEED=20261018
T27=${TIMED27_TARGETS:-4}
R27=${TIMED27_ROUNDS:-5}

arm() {   # name role oracle
  printf '    {"name":"%s","role":"%s","method":{"id":"ic.pipeline","params":{"factor_base":"binary-subspace:dimension=%s","oracle":"%s","targets":"random","max_trials":"200000"}}}' "$1" "$2" "$DIM" "$3"
}

spec() {   # kind n targets rounds warmup level timeout
  local kind=$1 n=$2 targets=$3 rounds=$4 warmup=$5 level=$6 timeout=$7
  DIM=$(( (n + 2) / 3 ))
  local f=specs/${kind}_n${n}.json
  {
    echo '{'
    echo '  "schema": "ecbench.spec/v1",'
    echo "  \"label\": \"PS-6 ${kind}: S4 pairs-and-solve baseline, PS-2 group-law kernel and the MITM incumbent, random binary n=${n}, l=${DIM}\","
    echo '  "description": "Frozen by research/pdp_oracle_speedup_20261011/protocols/PS-6.md. Ledger char2 rung; one-target workloads; same curve, factor base, targets and seeds for every arm. The group-law kernel row is a lower bound (its pairs have no pinned conversion); see ps6_charge/README.md.",'
    echo '  "workloads": {'
    echo "    \"curves\": [{\"kind\": \"binary_random\", \"n\": ${n}, \"seed\": ${SEED}, \"max_cofactor\": 8}],"
    echo "    \"targets_per_curve\": ${targets},"
    echo "    \"target_seed\": ${TARGET_SEED}"
    echo '  },'
    echo '  "arms": ['
    printf '    {"name":"rho-negation","role":"reference","method":{"id":"rho.negation","params":{}}},\n'
    arm s4-baseline baseline "semaev-s4-baseline:m=3"; echo ','
    if [ "$kind" = timed ]; then
      arm s4-baseline-control control "semaev-s4-baseline:m=3"; echo ','
    fi
    arm s4-grouplaw candidate "semaev-s4-grouplaw:m=3"; echo ','
    if [ "$kind" = counted ]; then
      arm s4-grouplaw-audit candidate "semaev-s4-grouplaw:m=3,audit=1"; echo ','
    fi
    arm mitm candidate "mitm:m=3"; echo ','
    arm mitm-negfold candidate "mitm:m=3,negation_folded=1"; echo
    echo '  ],'
    echo '  "measurement": {'
    echo "    \"rounds\": ${rounds}, \"warmup\": ${warmup}, \"order\": \"alternate\", \"seed\": 20261018,"
    echo "    \"isolation_required\": \"${level}\", \"timeout_seconds\": ${timeout}"
    echo '  }'
    echo '}'
  } > "$f"
  jq -e . "$f" > /dev/null
}

for n in 15 18 21 24; do
  spec counted "$n" 8 2 0 L0 14400
done
spec counted 27 8 1 0 L0 14400
for n in 15 18 21 24; do
  spec timed "$n" 8 5 1 L2 3600
done
spec timed 27 "$T27" "$R27" 1 L2 7200
ls -l specs
