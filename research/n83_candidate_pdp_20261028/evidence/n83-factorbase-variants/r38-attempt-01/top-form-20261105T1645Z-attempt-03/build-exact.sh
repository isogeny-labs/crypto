#!/usr/bin/env bash
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
EXPECTED_PDP=b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca
EXPECTED_TOP=4749b49ba7e98cd916c9d36d4b14c0396041fd6ff31e3708e17e7a91749fe53a
start=$(date -u +%FT%TZ)
finish(){ rc=$?; trap - EXIT; python3 - "$HERE" "$start" "$rc" <<'PY'
import hashlib,json,platform,subprocess,sys,datetime
from pathlib import Path
r=Path(sys.argv[1]);out={'schema':'n83.exact-source-build-receipt/v1','started_utc':sys.argv[2],'finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit_code':int(sys.argv[3]),'rustc':subprocess.run(['rustc','-Vv'],capture_output=True,text=True).stdout,'cargo':subprocess.run(['cargo','-V'],capture_output=True,text=True).stdout.strip(),'platform':platform.platform(),'files':{}}
for n in ['Cargo.toml','Cargo.lock','source-pdp.rs','source-top.rs']:
 p=r/n;out['files'][n]={'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
for n in ['pdp','top']:
 p=r/'build-target/release'/n
 if p.exists():out['files']['built-'+n]={'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
(r/'build-receipt.json').write_text(json.dumps(out,indent=2)+'\n')
PY
exit "$rc"; }
trap finish EXIT
[[ ! -e "$HERE/build-target" && ! -e "$HERE/build-project" ]] || { echo 'attempt03 build paths already exist' >&2; exit 20; }
(cd "$HERE" && sha256sum -c build-inputs.sha256)
mkdir -p "$HERE/build-project/src/bin"
cp "$HERE/Cargo.toml" "$HERE/Cargo.lock" "$HERE/build-project/"
cp "$HERE/source-pdp.rs" "$HERE/build-project/src/bin/pdp.rs"
cp "$HERE/source-top.rs" "$HERE/build-project/src/bin/top.rs"
rustc -Vv > "$HERE/rustc-version.txt"; cargo -V > "$HERE/cargo-version.txt"
export CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=1
(ulimit -v 8388608; timeout --signal=TERM --kill-after=30 600 cargo build --locked --release --manifest-path "$HERE/build-project/Cargo.toml" --target-dir "$HERE/build-target" --bin pdp --bin top) > "$HERE/build.log" 2>&1
printf '%s  %s\n' "$EXPECTED_PDP" "$HERE/build-target/release/pdp" | sha256sum -c -
printf '%s  %s\n' "$EXPECTED_TOP" "$HERE/build-target/release/top" | sha256sum -c -
