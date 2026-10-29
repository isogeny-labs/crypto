#!/bin/bash
# Run scripts/validation_gate.py (git show HEAD at bf899979a6) on a scratch mirror of the
# evidence directory holding VALIDATION.draft.json as VALIDATION.json, the two pinned
# validator files and the raw_data files copied from the evidence directory; then a
# negative control (one raw file changed by one byte must fail). Writes work/gatecheck.txt.
set -u
V=/dev/shm/dgb-validation
G=/dev/shm/dgb-val/gatecheck
EVD=/dev/shm/wt-blk/research/ic_dg_blocked_20261029
rm -rf "$G"; mkdir -p "$G/repo/scripts"
git -C /dev/shm/wt-blk show bf899979a6:scripts/validation_gate.py > "$G/repo/scripts/validation_gate.py"
E="$G/repo/research/ic_dg_blocked_20261029"
mkdir -p "$E/validation"
cp "$V/VALIDATION.draft.json" "$E/VALIDATION.json"
cp "$V/replay-certificate.json" "$V/validator-report.json" "$E/validation/"
python3 -I - "$V/validator-report.json" "$EVD" "$E" <<'EOF'
import json, os, shutil, sys
rep, src, dst = sys.argv[1:4]
for it in json.load(open(rep))["raw_data"]:
    os.makedirs(os.path.dirname(f"{dst}/{it['path']}"), exist_ok=True)
    shutil.copyfile(f"{src}/{it['path']}", f"{dst}/{it['path']}")
EOF
{
  echo "checker: git show bf899979a6:scripts/validation_gate.py sha256 $(sha256sum "$G/repo/scripts/validation_gate.py" | cut -c1-64)"
  echo "positive: $(cd "$G/repo" && python3 -I scripts/validation_gate.py evidence research/ic_dg_blocked_20261029; echo "rc=$?")"
  printf ' ' >> "$E/P28/wall/eks-results/job-exit.json"
  echo "negative control (P28/wall/eks-results/job-exit.json + 1 byte): $(cd "$G/repo" && python3 -I scripts/validation_gate.py evidence research/ic_dg_blocked_20261029 | tr '\n' ' '; echo "rc=${PIPESTATUS[0]}")"
} > "$V/work/gatecheck.txt"
rm -rf "$G"
cat "$V/work/gatecheck.txt"
