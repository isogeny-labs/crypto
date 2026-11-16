set -eu
r=/home/ec2-user/research/cm-isogeny-attack-20261107/round67-p192
mkdir -p /tmp/p192-round67-fresh
cp -r "$r/native" /tmp/p192-round67-fresh/
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/p192-round67-fresh-target timeout 120 cargo build --release --locked --offline --manifest-path /tmp/p192-round67-fresh/native/Cargo.toml --bin signed-baby-response --bin cached-p192 --bin cancellation-identity > "$r/fresh-build.log" 2>&1
timeout 120 /tmp/p192-round67-fresh-target/release/cancellation-identity --check "$r/identity.json" > /tmp/p192-round67-fresh-identity.json
cmp "$r/identity.json" /tmp/p192-round67-fresh-identity.json
for mode in candidate expanded; do
 extra=''
 if [ "$mode" = expanded ]; then extra='--expanded-numerator'; fi
 timeout 120 /tmp/p192-round67-fresh-target/release/signed-baby-response $extra --check "$r/$mode-response.json" > "/tmp/p192-round67-fresh-$mode.json" 2> "/tmp/p192-round67-fresh-$mode.log"
 timeout 120 /tmp/p192-round67-fresh-target/release/cached-p192 "$r" $extra --check "$r/$mode-P192.json" > "/tmp/p192-round67-fresh-$mode-P192.json"
 cmp "$r/$mode-response.json" "/tmp/p192-round67-fresh-$mode.json"
 cmp "$r/$mode-response.log" "/tmp/p192-round67-fresh-$mode.log"
 cmp "$r/$mode-P192.json" "/tmp/p192-round67-fresh-$mode-P192.json"
done
python3 - <<'PY'
import json
from pathlib import Path
r=Path('/home/ec2-user/research/cm-isogeny-attack-20261107/round67-p192')
a=json.load(open(r/'candidate-response.json'));a['panels'][0]['rows'][0]['Bcal'][0]+=1;Path('/tmp/p192-round67-bad-small.json').write_text(json.dumps(a))
b=json.load(open(r/'candidate-P192.json'));b['rows'][0]['coefficients'][1][0]='0';Path('/tmp/p192-round67-bad-P192.json').write_text(json.dumps(b))
PY
set +e
timeout 120 /tmp/p192-round67-fresh-target/release/signed-baby-response --check /tmp/p192-round67-bad-small.json > /tmp/p192-round67-mutation-small.out 2> "$r/mutation-small.log"
a=$?
timeout 120 /tmp/p192-round67-fresh-target/release/cached-p192 "$r" --check /tmp/p192-round67-bad-P192.json > /tmp/p192-round67-mutation-P192.out 2> "$r/mutation-P192.log"
b=$?
set -e
printf '{"changed_small_exit":%s,"changed_P192_exit":%s}\n' "$a" "$b" > "$r/mutations.json"
[ "$a" -eq 101 ]
[ "$b" -eq 101 ]
sha256sum /tmp/p192-round67-fresh-target/release/signed-baby-response /tmp/p192-round67-fresh-target/release/cached-p192 > "$r/native-binaries.sha256"
echo 'seven fresh outputs/receipts match; two mutations reject101'
