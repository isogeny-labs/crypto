#!/usr/bin/env bash
# Thin orchestration only. Mathematics runs in upstream DiSSECT or native PARI.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
commit=8edc3030fb91411a55aa6c07d050507d840c5e55

die() { echo "$*" >&2; exit 1; }
verify() {
  local out=$1 job
  (cd "$out" && sha256sum --check SHA256SUMS) >&2
  jq -e --arg commit "$commit" '.schema == "nist-skipped-job/v1" and .exit_code == 0 and .upstream_commit == $commit' "$out/receipt.json" >/dev/null
  cmp "$here/curves.json" "$out/curves.json" || die 'Artifact has different curve inputs'
  cmp "$here/jobs.json" "$out/jobs.json" || die 'Artifact has a different job grid'
  job=$(jq -r .job "$out/receipt.json")
  if [[ $job == algebra ]]; then
    jq -se --slurpfile input "$here/curves.json" '
      ([.[] | select(.kind == "curve")] | length) == 6 and
      ([.[] | select(.kind == "curve") | .fixture] | sort) == ($input[0].curves | map(.name) | sort) and
      ([.[] | select(.kind == "pair")] | length) == 7 and
      ([.[] | select(.kind == "complete")] == [{kind:"complete", curves:6, pairs:7}]) and
      all(.[] | select(.kind == "curve"); .order_verified and .twist_order_verified and .isogeny_verified)
    ' "$out/result.jsonl" >/dev/null
  else
    jq -e --arg job "$job" --slurpfile spec "$here/jobs.json" --slurpfile input "$here/curves.json" '
      .data as $data |
      ($spec[0][] | select(.job == $job)) as $expected |
      ($input[0].curves | map(.name) | sort) as $names |
      ($data | length) == (6 * ($expected.params | length)) and
      ($data | map(.curve) | unique | sort) == $names and
      all($names[]; . as $name |
        ([$data[] | select(.curve == $name) | .params] | sort_by(tojson)) ==
        ($expected.params | sort_by(tojson))) and
      all($data[]; has("result") and (.result | type == "object"))
    ' "$out/result.json" >/dev/null
  fi
  # This is a completeness/integrity check, not certification of every trait.
  echo "Verified artifact integrity and declared coverage: $job" >&2
}

case ${1:-} in
  verify) [[ $# == 2 ]] || die 'usage: run.sh verify OUTPUT'; verify "$2"; exit ;;
  run) [[ $# == 3 ]] || die 'usage: run.sh run JOB NEW_OUTPUT_DIRECTORY' ;;
  *) die 'usage: run.sh {run JOB NEW_OUTPUT_DIRECTORY | verify OUTPUT}' ;;
esac
job=$2
out=$3
jq -e --arg job "$job" 'any(.[]; .job == $job)' "$here/jobs.json" >/dev/null || die "Unknown job: $job"
mkdir -p -- "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || die "Output is not empty; preserve it and choose a new directory: $out"
out=$(cd -- "$out" && pwd)
stage=setup
record_artifacts() {
  local code=$1
  date -u +%FT%TZ > "$out/finished-utc.txt"
  jq -n --arg job "$job" --arg commit "$commit" --arg stage "$stage" --argjson status "$code" \
    '{schema:"nist-skipped-job/v1", job:$job, upstream_commit:$commit, exit_code:$status,
      execution_stage:$stage, mathematical_audit:"pending; see PROTOCOL.md", historical_skip_reason:"unknown"}' > "$out/receipt.json"
  (cd "$out" && find . -maxdepth 1 -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs sha256sum) > "$out/SHA256SUMS"
}
record_failure() {
  local code=$?
  trap - EXIT
  record_artifacts "$code"
}
trap record_failure EXIT
git -C "$here" rev-parse HEAD > "$out/code-commit.txt"
cp "$here/curves.json" "$here/provenance.json" "$here/jobs.json" "$out/"
(cd "$here" && sha256sum curves.json provenance.json jobs.json run.sh algebra.gp) > "$out/inputs.sha256"
printf '%s\n' "$commit" > "$out/upstream-commit.txt"
date -u +%FT%TZ > "$out/started-utc.txt"

if [[ $job == algebra ]]; then
  # Only hex strings and small indices pass through jq. Large decimal output
  # from DiSSECT must never be rounded through a JSON floating-point parser.
  jq -r '[.curves[] | [(.name | tojson), .field.p, .params.a.raw, .params.b.raw, .order] |
    "[" + join(",") + "]"] | "INPUTS=[" + join(",") + "];"' "$here/curves.json" > "$out/inputs.gp"
  "${GP_BIN:-gp}" --version > "$out/runtime.txt" 2>&1
  argv=("${GP_BIN:-gp}" -q -f -s 128000000 --default parisizemax=2000000000 --default factor_proven=1 "$here/algebra.gp")
  printf '%q ' "${argv[@]}" > "$out/command.txt"
  printf '\n' >> "$out/command.txt"
  stage=computation
  set +e
  CURVE_INPUTS="$out/inputs.gp" timeout --signal=TERM --kill-after=10s 3600 "${argv[@]}" > "$out/result.jsonl" 2> "$out/stderr.txt"
  status=$?
  set -e
  # GP may print a parser/runtime error yet exit zero. Keep it and fail closed.
  if rg -q '\*\*\*' "$out/stderr.txt"; then status=1; fi
else
  : "${DISSECT_ROOT:?Set DISSECT_ROOT to the pinned upstream checkout; see README.md}"
  [[ $(git -C "$DISSECT_ROOT" rev-parse HEAD) == "$commit" ]] || die 'Wrong DiSSECT revision'
  git -C "$DISSECT_ROOT" diff --exit-code HEAD -- >/dev/null || die 'Modified upstream DiSSECT source'
  "${SAGE_BIN:-sage}" --version > "$out/runtime.txt"
  rg -q 'SageMath version 9\.8,' "$out/runtime.txt" || die 'This protocol pins SageMath 9.8'
  export PYTHONPATH="$DISSECT_ROOT${DISSECT_DEPS:+:$DISSECT_DEPS}"
  "${SAGE_BIN:-sage}" -pip show pymongo dnspython > "$out/dependencies.txt"
  actual_deps=$(awk '/^Name: / {name=$2} /^Version: / {print name "==" $2}' "$out/dependencies.txt" | LC_ALL=C sort)
  [[ $actual_deps == $'dnspython==2.6.1\npymongo==4.6.3' ]] || die 'Wrong DiSSECT import dependency versions'
  argv=("${SAGE_BIN:-sage}" -python -m dissect.utils.compute_json -t "$job" -i "$out/curves.json" -o "$out/result.json")
  printf '%q ' "${argv[@]}" > "$out/command.txt"
  printf '\n' >> "$out/command.txt"
  stage=computation
  set +e
  timeout --signal=TERM --kill-after=10s 3600 "${argv[@]}" > "$out/stdout.txt" 2> "$out/stderr.txt"
  status=$?
  set -e
fi
record_artifacts "$status"
trap - EXIT
[[ $status == 0 ]] || die "Job failed (exit $status); raw artifacts preserved at $out"
verify "$out"
