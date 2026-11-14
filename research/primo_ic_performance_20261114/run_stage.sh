#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 OUTPUT_DIRECTORY" >&2
  exit 2
fi

out=$1
ic_bin=${IC_BIN:-target/release/ic}
primo_bin=${SMT_PRIMO_BIN:-/workspace/.tools/primo/bin/primo}
mkdir -p "$out"

[[ -x "$ic_bin" ]] || { echo "ic binary is not executable: $ic_bin" >&2; exit 2; }
[[ -x "$primo_bin" ]] || { echo "Primo binary is not executable: $primo_bin" >&2; exit 2; }

git rev-parse HEAD > "$out/source-commit.txt"
git status --short > "$out/source-status.txt"
sha256sum "$ic_bin" "$primo_bin" > "$out/binaries.sha256"
uname -a > "$out/uname.txt"
lscpu > "$out/lscpu.txt"
"$primo_bin" --version > "$out/primo-version.txt"

common=(
  --families K,R
  --cells 11:6:2,13:7:2,15:8:2,13:5:3
  --seed 233163335
  --budget-seconds 30
  --solver exhaustive
  --solver sat-cdcl:sat_xor_encoding=native
  --solver "primo-smt-file:primo_binary=$primo_bin,primo_timeout_ms=30000"
  --solver "primo-smt:primo_binary=$primo_bin,primo_timeout_ms=30000"
)

timeout 120 "$ic_bin" --json descent "${common[@]}" --targets 2 --repeats 1 \
  > "$out/warmup.json" 2> "$out/warmup.stderr"

timeout 900 "$ic_bin" --json descent "${common[@]}" --targets 8 --repeats 7 \
  > "$out/measured.json" 2> "$out/measured.stderr"

jq -e '
  .status == "complete" and
  (.disagreements | length) == 0 and
  all(.cells[].engines[] | select(.engine == "primo-smt" or .engine == "primo-smt-file");
      .decided == 8 and .over_budget == 0 and .agrees_with_reference == true) and
  all(.cells[].per_target[].runs["primo-smt"][];
      .extra.invalid_model == 0 and
      (if .verdict == "solved" then .extra.model_checked == 1 else true end)) and
  all(.cells[].per_target[].runs["primo-smt-file"][];
      .extra.invalid_model == 0 and
      (if .verdict == "solved" then .extra.model_checked == 1 else true end)) and
  all(.cells[].per_target[];
      .runs["primo-smt-file"] as $legacy |
      .runs["primo-smt"] as $candidate |
      ($legacy | length) == 7 and ($candidate | length) == 7 and
      all(range(0; ($legacy | length));
          . as $i |
          $legacy[$i].verdict == $candidate[$i].verdict and
          $legacy[$i].solutions == $candidate[$i].solutions and
          $legacy[$i].ops == $candidate[$i].ops and
          $legacy[$i].extra.decisions == $candidate[$i].extra.decisions and
          $legacy[$i].extra.input_bytes == $candidate[$i].extra.input_bytes and
          $legacy[$i].extra.rows == $candidate[$i].extra.rows and
          $legacy[$i].extra.variables == $candidate[$i].extra.variables))
' "$out/measured.json" > "$out/correctness-gate.txt"
