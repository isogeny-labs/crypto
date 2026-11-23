#!/usr/bin/env bash
set -euo pipefail
if [ "${1:-}" = --help ]; then echo 'PROOF_ENV_ROOT=/restored PROOF_GATE_DIR=/official PROOF_REF=/fresh/ref.json PROOF_OUTPUT=/fresh/outside timeout 3600 bash run.sh'; exit 0; fi
[ "$#" = 0 ] || exit 2
root=$(git rev-parse --show-toplevel);cd "$root"
base=research/quadratic_twist_field_cost_20261116/torus-proof
fail(){ echo "torus proof: $*" >&2;exit 2; }
[ "$(git branch --show-current)" = codex/twist-field-cost ] || fail branch
[ "$(git remote get-url origin)" = https://github.com/isogeny-labs/crypto.git ] || fail repository
[ -z "$(git status --porcelain)" ] || fail 'publish clean source first'
[ -z "${CHECKPOINT_SKIP:-}" ] || fail 'no skip flag'
: "${PROOF_ENV_ROOT:?}" "${PROOF_GATE_DIR:?}" "${PROOF_REF:?}" "${PROOF_OUTPUT:?}"
head=$(git rev-parse HEAD)
timeout 600 python3 - "$root" "$base" "$head" "$PROOF_ENV_ROOT" "$PROOF_GATE_DIR" "$PROOF_REF" <<'PY'
from pathlib import Path
import json,hashlib,sys,datetime,subprocess
root,base,head,env,gate,ref=sys.argv[1:];root=Path(root);env=Path(env);h=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();r=json.loads(Path(ref).read_text());assert r['repository']=='isogeny-labs/crypto' and r['branch']=='codex/twist-field-cost' and r['head']==head and r['state']=='open' and r['auto_merge'] is None and r['source']=='connected-github-read';t=datetime.datetime.fromisoformat(r['observed_at'].replace('Z','+00:00'));assert 0<=(datetime.datetime.now(datetime.timezone.utc)-t).total_seconds()<=60
pins=json.loads((root/base/'PINS.json').read_text())
protocol=json.loads((root/base/'protocol.json').read_text());assert protocol['freeze']['status']=='frozen' and protocol['freeze']['filed_before_affected_data'] is True
subprocess.run([sys.executable,'/workspace/canonical-protocol-20261115/tools/experiment_protocol.py','validate',str(root/base/'protocol.json')],check=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
for p,z in pins['repository_files'].items():assert h(root/p)==z,p
for p,z in pins['gate_files'].items():assert h(Path(gate)/p)==z,p
summary=json.loads((root/base/'ENVIRONMENT.json').read_text());assert h(env/'lean-4.34.1-linux/bin/lean')==summary['lean_binary_sha256'];assert h(env/'environment-inventory.json')==summary['inventory_sha256'];inventory=json.loads((env/'environment-inventory.json').read_text())
for p,v in inventory['compiled_libraries'].items():assert h(env/'mathlib'/p)==v['sha256'],p
assert subprocess.check_output(['git','-C',str(env/'mathlib'),'rev-parse','HEAD'],text=True).strip()==summary['mathlib_commit']
for package in summary['packages']:assert subprocess.check_output(['git','-C',str(env/'mathlib/.lake/packages'/package['name']),'rev-parse','HEAD'],text=True).strip()==package['commit']
PY
out=$(realpath -m "$PROOF_OUTPUT");case "$out" in "$root"|"$root"/*)fail 'output outside checkout required';;esac
[ ! -e "$out" ] || fail 'fresh output required';mkdir -m 700 "$out"
cp "$base/PINS.json" "$base/protocol.json" "$base/PROTOCOL.md" "$base/ENVIRONMENT.json" "$base/CONTROL_SOURCES.json" "$out/"
cp research/quadratic_twist_field_cost_20261116/torus-proof/Torus.lean "$out/original-source.lean.txt"
cp "$PROOF_REF" "$out/github-ref.json";printf '%s\n' "$head" > "$out/head.txt"
export PATH="$PROOF_ENV_ROOT/lean-4.34.1-linux/bin:$PATH" LEAN_NUM_THREADS=2
export LEAN_GATE_TRUSTED="$PROOF_GATE_DIR/trusted-native.txt"
paths=("$PROOF_ENV_ROOT/mathlib/.lake/build/lib/lean");for d in "$PROOF_ENV_ROOT/mathlib/.lake/packages/"*;do [ ! -d "$d/.lake/build/lib/lean" ] || paths+=("$d/.lake/build/lib/lean");done
export LEAN_PATH=$(IFS=:;printf '%s' "${paths[*]}")
lean --version > "$out/toolchain.txt"
step=none
finish(){ local code=$?;printf '%s\n' "$code" > "$out/launcher.exit";printf '%s\n' "$step" > "$out/last-step.txt"; };trap finish EXIT
run(){ local remaining budget;remaining=$((1794802500 - $(date -u +%s)));[ "$remaining" -gt 0 ] || fail "proof window ended";budget=$remaining;[ "$budget" -le 600 ] || budget=600;step=$1;shift;printf '%s\n' "$step" >> "$out/steps.txt";printf '%q ' "$@" > "$out/$step.command";printf '\n' >> "$out/$step.command";local code=0;timeout --signal=TERM --kill-after=10 "$budget" "$@" > "$out/$step.log" 2>&1 || code=$?;printf '%s\n' "$code" > "$out/$step.exit";return "$code"; }
for stage in gate-source-A gate-source-B gate-positive;do
 run "$stage" bash "$PROOF_GATE_DIR/proof-gate.sh" --out "$out/$stage" research/quadratic_twist_field_cost_20261116/torus-proof/Torus.lean
done
slug=research_quadratic_twist_field_cost_20261116_torus-proof_Torus_lean
for stage in gate-source-A gate-source-B gate-positive;do
 grep -q '0 failing, 0 trusted' "$out/$stage/$slug/audit.log" || fail 'unaccepted axiom inventory'
 for name in product_real product_imag product_denominator decompressed_norm denominator_of_compressed decompress_compress compress_decompress membership_linearization;do grep -q "ok    TwistTorus.$name " "$out/$stage/$slug/audit.log" || fail "missing public theorem $name";done
done
cmp "$out/gate-source-A/$slug/build.log" "$out/gate-source-B/$slug/build.log"
cmp "$out/gate-source-A/$slug/audit.log" "$out/gate-source-B/$slug/audit.log"
cmp "$out/gate-source-A/$slug/audit.log" "$out/gate-positive/$slug/audit.log"
controlroot="$base/.proof-controls/$(basename "$out")";[ ! -e "$controlroot" ] || fail 'fresh false-source directory';mkdir -p "$controlroot"
for mode in product-real norm decompress-compress compress-decompress membership;do
 input="$base/controls/$mode.lean.txt";target="$controlroot/False_${mode//-/_}.lean";cp "$input" "$target";cmp "$input" "$target";cp "$target" "$out/$(basename "$target").txt";sha256sum "$target" >> "$out/false-sources.sha256"
 code=0;run "reject-$mode" bash "$PROOF_GATE_DIR/proof-gate.sh" --out "$out/reject-$mode" "$target" || code=$?
 [ "$code" = 1 ] || fail "false-source $mode returned $code"
 if grep -Rq '^PASS' "$out/reject-$mode" --include=status;then fail 'false PASS receipt';fi
 grep -REiq 'unsolved goals|type mismatch|tactic.*failed|ring.*failed' "$out/reject-$mode"/*/build.log || fail 'negative did not fail by mathematical checking'
done
step=full-proof-plan-finished
echo 'All declared proof stages finished; review actual axiom/source output, no signed PCC verdict implied.'
