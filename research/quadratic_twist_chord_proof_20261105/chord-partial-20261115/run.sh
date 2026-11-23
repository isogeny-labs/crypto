#!/usr/bin/env bash
# Published-source GitHub checkpoint transport expressly selected by the user.
set -euo pipefail
if [ "${1:-}" = --help ]; then echo 'CHORD_PRODUCER=... CHORD_AUDITOR=... CHORD_OUTPUT=/fresh/outside CHORD_GITHUB_REF_RECEIPT=/fresh/receipt timeout 3600 bash run.sh'; exit 0; fi
[ "$#" = 0 ] || exit 2
root=$(git rev-parse --show-toplevel); cd "$root"
base=research/quadratic_twist_chord_proof_20261105/chord-partial-20261115
fail() { echo "chord partial: $*" >&2; exit 2; }
[ "$(git branch --show-current)" = codex/twist-chord-proof ] || fail branch
[ "$(git remote get-url origin)" = https://github.com/isogeny-labs/crypto.git ] || fail repo
[ -z "$(git status --porcelain)" ] || fail 'publish clean committed source first'
[ -z "${CHECKPOINT_SKIP:-}" ] || fail 'no skip flag'
: "${CHORD_PRODUCER:?}" "${CHORD_AUDITOR:?}" "${CHORD_OUTPUT:?}" "${CHORD_GITHUB_REF_RECEIPT:?}"
head=$(git rev-parse HEAD)
python3 - "$root" "$base" "$head" "$CHORD_GITHUB_REF_RECEIPT" "$CHORD_PRODUCER" "$CHORD_AUDITOR" <<'PY'
import json,pathlib,hashlib,sys,datetime
root,base,head,receipt,producer,auditor=sys.argv[1:];root=pathlib.Path(root)
v=json.loads(pathlib.Path(receipt).read_text())
assert v['repository']=='isogeny-labs/crypto' and v['branch']=='codex/twist-chord-proof'
assert v['head']==head and v['state']=='open' and v['auto_merge'] is None and v['source']=='connected-github-read'
t=datetime.datetime.fromisoformat(v['observed_at'].replace('Z','+00:00'));assert 0<=(datetime.datetime.now(datetime.timezone.utc)-t).total_seconds()<=60
p=json.loads((root/base/'RUN_PINS.json').read_text());h=lambda x:hashlib.sha256(pathlib.Path(x).read_bytes()).hexdigest()
assert p['checkpoint_transport']=='github-explicit-user-selection-20261115'
for path,want in p['files'].items():assert h(root/path)==want,path
for name,path in [('producer',producer),('auditor',auditor)]:assert h(path)==p['binaries'][name],name
PY
out=$(realpath -m "$CHORD_OUTPUT")
case "$out" in "$root"|"$root"/*) fail 'output must be outside checkout';; esac
[ ! -e "$out" ] || fail 'fresh output required'
mkdir -m 700 "$out"
cp "$base/protocol.json" "$base/RUN_PINS.json" "$base/PROTOCOL.md" "$out/"
cp "$CHORD_GITHUB_REF_RECEIPT" "$out/github-ref-receipt.json"
printf '%s\n' "$head" > "$out/head.txt"
step=none
finish() { local code=$?; printf '%s\n' "$code" > "$out/launcher.exit"; printf '%s\n' "$step" > "$out/last-step.txt"; }
trap finish EXIT
run() { step=$1; shift; printf '%s\n' "$step" >> "$out/steps.txt"; printf '%q ' "$@" > "$out/$step.command"; printf '\n' >> "$out/$step.command"; local code=0; timeout --signal=TERM --kill-after=10 600 "$@" > "$out/$step.log" 2>&1 || code=$?; printf '%s\n' "$code" > "$out/$step.exit"; return "$code"; }
for repeat in A B; do
 run "produce-$repeat" "$CHORD_PRODUCER" produce "$out/$repeat.json"
 sha256sum "$out/$repeat.json" > "$out/$repeat.sha256"
 run "audit-$repeat" "$CHORD_AUDITOR" audit "$out/$repeat.json" "$out/$repeat.receipt.json"
done
run audit-positive "$CHORD_AUDITOR" audit "$out/A.json" "$out/positive.receipt.json"
cmp "$out/A.json" "$out/B.json"
cmp "$out/A.receipt.json" "$out/B.receipt.json"
cmp "$out/A.receipt.json" "$out/positive.receipt.json"
for mode in source map missing-case cap columns rank missing-failed-query witness root log scalar row diagonal; do
 run "mutate-$mode" "$CHORD_PRODUCER" mutate "$out/A.json" "$out/$mode.json" "$mode"
 ! cmp -s "$out/A.json" "$out/$mode.json" || fail 'control unchanged'
 sha256sum "$out/$mode.json" > "$out/$mode.sha256"
 code=0; run "reject-$mode" "$CHORD_AUDITOR" audit "$out/$mode.json" "$out/$mode.receipt.json" || code=$?
 [ "$code" = 101 ] || fail "control $mode returned $code"
 [ ! -e "$out/$mode.receipt.json" ] || fail 'rejected receipt exists'
done
step=declared-scope-complete
echo 'Full declared partial-chord source/twist and cold-recovery run finished; unsigned observations only.'
