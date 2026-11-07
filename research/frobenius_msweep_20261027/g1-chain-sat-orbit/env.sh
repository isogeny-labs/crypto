# Source at the top of every RUN_CARD block:  . /home/user/crypto/research/frobenius_msweep_20261027/g1-chain-sat-orbit/env.sh
# Defines ROOT, G1, BIN, RUN, R (run directory) in a fresh shell. RUN comes from the environment, else from
# reports/RUN (written once by RUN_CARD section 3, step "mint RUN"); it is an error if neither exists, except for
# blocks that set MINT_RUN=1 first.
ROOT=/home/user/crypto
G1=research/frobenius_msweep_20261027/g1-chain-sat-orbit
BIN=$ROOT/target/release/examples/frobenius_msweep_g1_chain_sat_orbit
cd "$ROOT" || return 1
if [ -z "${RUN:-}" ] && [ -z "${MINT_RUN:-}" ]; then
  if [ -s "$G1/reports/RUN" ]; then RUN=$(cat "$G1/reports/RUN"); else echo "RUN unset and $G1/reports/RUN missing: run the 'mint RUN' block first" >&2; return 1; fi
fi
R=$G1/reports/run-${RUN:-UNSET}
