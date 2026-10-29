#!/bin/bash
# run.sh -- single driver for E2-twin-target-amortization.  Executes the frozen commands of PREREGISTRATION.md section 8,
# positive controls first.  Re-runnable: completed raw files (containing a RESULT line) are kept.
D=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E2-twin-target-amortization
PY=/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/venv/bin/python
cd $D || exit 1
mkdir -p raw plots

# ---- freeze (once, BEFORE the first data run); afterwards verify
if [ ! -f FREEZE.sha256 ]; then sha256sum e2_*.gp plot.py PREREGISTRATION.md run.sh > FREEZE.sha256; fi
sha256sum -c FREEZE.sha256 > raw/freeze_check.out 2>&1 || { echo "FREEZE MISMATCH (see raw/freeze_check.out); changes must go to DEVIATIONS.md"; }

done_ok() { [ -s "$1" ] && grep -q '^RESULT,' "$1"; }

# ---- S0 calibration
[ -s raw/S0_micro.out ] || E2_BITS=26 gp -q -f e2_micro.gp < /dev/null > raw/S0_micro.out 2>&1
proj() { # bits mu arms -> seconds
  awk -F'[=,]' -v b="$1" -v m="$2" -v a="$3" '/^PROJ/ { if ($3==b && ($5+0)==(m+0)) { print (a=="all") ? $9 : $7 } }' raw/S0_micro.out
}
fits() { # bits mu arms
  p=$(proj "$1" "$2" "$3"); [ -n "$p" ] && awk -v p="$p" 'BEGIN{exit !(p<=720)}'
}

# ---- positive controls first: L2 (B=O, Frobenius +1/-1) and L1 at 22 bits, then the two PC1-critical grid cells at 22 bits
[ -s raw/L1_L2_b22.out ] || E2_BITS=22 gp -q -f e2_lemma.gp < /dev/null > raw/L1_L2_b22.out 2>&1
for mu in 3 0.75; do
  f=raw/run_b22_mu${mu}_c1_s1.out
  done_ok $f || E2_BITS=22 E2_MU=$mu E2_CURVE=1 E2_SEED=1 E2_ARMS=all timeout 900 gp -q -f e2_run.gp < /dev/null > $f 2>&1
done
[ -s raw/L1_L2_b26.out ] || E2_BITS=26 gp -q -f e2_lemma.gp < /dev/null > raw/L1_L2_b26.out 2>&1

# ---- main grid (frozen loops; only cells not yet done and projected <= 12 min are queued)
: > grid_cmds.txt
for b in 22 26; do for mu in 0.75 1.5 3; do for c in 1 2 3; do for s in 1 2 3; do
  f=raw/run_b${b}_mu${mu}_c${c}_s${s}.out
  done_ok $f && continue
  fits $b $mu all || { echo "DROPPED (S0 projection > 12 min): b=$b mu=$mu" >> raw/dropped_cells.txt; continue; }
  echo "E2_BITS=$b E2_MU=$mu E2_CURVE=$c E2_SEED=$s E2_ARMS=all timeout 900 gp -q -f e2_run.gp < /dev/null > $f 2>&1"
done; done; done; done >> grid_cmds.txt
for b in 30; do for mu in 0.75 1.5; do for c in 1 2; do for s in 1 2; do
  f=raw/run_b${b}_mu${mu}_c${c}_s${s}.out
  done_ok $f && continue
  fits $b $mu core || { echo "DROPPED (S0 projection > 12 min): b=$b mu=$mu" >> raw/dropped_cells.txt; continue; }
  echo "E2_BITS=$b E2_MU=$mu E2_CURVE=$c E2_SEED=$s E2_ARMS=core timeout 900 gp -q -f e2_run.gp < /dev/null > $f 2>&1"
done; done; done; done >> grid_cmds.txt
xargs -P 2 -I{} sh -c "{}" < grid_cmds.txt

# ---- stretch cell (only if the S0 projection allows)
f=raw/run_b32_mu1.5_c1_s1.out
if ! done_ok $f && fits 32 1.5 core; then
  E2_BITS=32 E2_MU=1.5 E2_CURVE=1 E2_SEED=1 E2_ARMS=core timeout 900 gp -q -f e2_run.gp < /dev/null > $f 2>&1
fi

# ---- matched rho references (bits 22, 26, 30)
for b in 22 26; do for c in 1 2 3; do
  [ -s raw/rho_b${b}_c$c.out ] || E2_BITS=$b E2_CURVE=$c timeout 900 gp -q -f e2_rho.gp < /dev/null > raw/rho_b${b}_c$c.out 2>&1
done; done
for c in 1 2; do
  [ -s raw/rho_b30_c$c.out ] || E2_BITS=30 E2_CURVE=$c timeout 900 gp -q -f e2_rho.gp < /dev/null > raw/rho_b30_c$c.out 2>&1
done
f=raw/rho_b32_c1.out
[ -s raw/run_b32_mu1.5_c1_s1.out ] && { [ -s $f ] || E2_BITS=32 E2_CURVE=1 timeout 900 gp -q -f e2_rho.gp < /dev/null > $f 2>&1; }

# ---- analysis and plots
gp -q -f e2_analyze.gp < /dev/null > raw/analysis.out 2>&1
$PY plot.py > raw/plot.out 2>&1
echo "run.sh finished"
