#!/bin/sh
set -eu
: "${PROFILE_BASE:?absolute study path}"
: "${PROFILE_VALIDATOR:?frozen native validator binary}"
: "${PROFILE_OUT:?fresh validation directory}"
repo=$(git -C "$PROFILE_BASE" rev-parse --show-toplevel)
checkpoint() {
    if [ -n "${NIST_CHECKPOINT_WRAPPER:-}" ]; then "$NIST_CHECKPOINT_WRAPPER" "$repo/scripts/checkpoint.sh" "$@";
    else "$repo/scripts/checkpoint.sh" "$@"; fi
}
case "$PROFILE_OUT" in "$PROFILE_BASE"/runs/*) ;; *) exit 2 ;; esac
[ ! -e "$PROFILE_OUT" ]
checkpoint gate
(cd "$PROFILE_BASE" && sha256sum -c input.sha256 && sha256sum -c verification-input.sha256 && sha256sum -c validation-launcher.sha256)
mkdir -p "$PROFILE_OUT/bin" "$PROFILE_OUT/source-snapshot" "$PROFILE_OUT/controls"
cp "$PROFILE_VALIDATOR" "$PROFILE_OUT/bin/verify"
chmod 755 "$PROFILE_OUT/bin/verify"
(cd "$PROFILE_OUT/bin" && sha256sum -c "$PROFILE_BASE/validator.sha256")
cp "$PROFILE_BASE/verify.cpp" "$PROFILE_BASE/VALIDATION_PROTOCOL.md" "$PROFILE_BASE/verification-input.sha256" "$PROFILE_BASE/plan.json" "$0" "$PROFILE_OUT/source-snapshot/"
cp "$PROFILE_BASE/results/profile.tsv" "$PROFILE_OUT/producer-profile.tsv"
git -C "$repo" rev-parse HEAD > "$PROFILE_OUT/source-commit.txt"
run_validator() {
    profile_name=$1
    shift
    set +e
    (ulimit -c 0 && ulimit -v 786432 && timeout --kill-after=5s 60s "$PROFILE_OUT/bin/verify" "$PROFILE_BASE" "$PROFILE_BASE/../period_ecpp_20261026/runs/replay-004" "$@") > "$PROFILE_OUT/$profile_name.stdout" 2> "$PROFILE_OUT/$profile_name.stderr"
    profile_result=$?
    set -e
    printf '%s\n' "$profile_result" > "$PROFILE_OUT/$profile_name.status"
}
run_validator validation "$PROFILE_OUT/producer-profile.tsv" "$PROFILE_OUT/results"
[ "$profile_result" -eq 0 ] || { checkpoint push; exit "$profile_result"; }
PROFILE_STUDY="$PROFILE_BASE" PROFILE_SAVED_VALIDATION="$PROFILE_OUT" node --input-type=module <<'JS'
import fs from 'node:fs';
import assert from 'node:assert/strict';
const base=process.env.PROFILE_STUDY,out=process.env.PROFILE_SAVED_VALIDATION;
const native=JSON.parse(fs.readFileSync(base+'/results/summary.json'));
const verified=JSON.parse(fs.readFileSync(out+'/results/verification.json'));
assert.equal(verified.rows,21);assert.equal(verified.cells_checked,210);assert.equal(verified.envelopes.length,40);
for(const f of native.features)for(const e of f.envelopes){const row=verified.envelopes.find(x=>x.feature===f.name&&x.scope===e.scope&&x.orientation===e.orientation);assert.ok(row);const {feature,...counts}=row;assert.deepEqual(counts,e);}
const original=fs.readFileSync(out+'/producer-profile.tsv','utf8').trimEnd().split('\n');
for(const kind of ['wrong-count','missing-row','wrong-status']){const lines=original.slice();if(kind==='missing-row')lines.pop();else{const r=lines[1].split('\t');r[kind==='wrong-count'?4:3]=kind==='wrong-count'?'999999999':'FAKE_STATUS';lines[1]=r.join('\t');}fs.mkdirSync(out+'/controls/'+kind);fs.writeFileSync(out+'/controls/'+kind+'/profile.tsv',lines.join('\n')+'\n');}
fs.writeFileSync(out+'/results/metadata-comparison.json',JSON.stringify({kind:'native-output-byte-value-comparison/v1',native_envelopes_compared:40,all_identical:true,new_scientific_arithmetic_in_comparator:false})+'\n');
JS
for profile_control in wrong-count missing-row wrong-status; do
    run_validator "controls/$profile_control/validation" "$PROFILE_OUT/controls/$profile_control/profile.tsv" "$PROFILE_OUT/controls/$profile_control/admitted"
    [ "$profile_result" -eq 1 ] || { checkpoint push; exit 1; }
    case "$profile_control" in
        wrong-count) profile_reason='count mismatch' ;;
        missing-row) profile_reason='missing row' ;;
        wrong-status) profile_reason='identity/status binding' ;;
    esac
    case "$(cat "$PROFILE_OUT/controls/$profile_control/validation.stderr")" in "$profile_reason"*) ;; *) checkpoint push; exit 1 ;; esac
done
(cd "$PROFILE_BASE" && sha256sum -c input.sha256 && sha256sum -c verification-input.sha256 && sha256sum -c validation-launcher.sha256) > "$PROFILE_OUT/input-recheck.txt"
checkpoint push
