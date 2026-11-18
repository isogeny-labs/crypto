#!/usr/bin/env bash
# Local launcher for research/ic_gpu_rootfind_devmap_20261115 (run under `benchrec run`): bundles the
# release ecbench and gpu/rootfind of HEAD with specs/ and job.sh, creates one A100 pod, runs job.sh,
# fetches /work/out_rdm.tgz into raw/ and deletes the pod on every exit path.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
D=research/ic_gpu_rootfind_devmap_20261115; R=research/ic_gpu_measure_20261112/rp_gpu.sh; P=icgpu-rdm
B=$(mktemp -d); trap 'bash $R delete $P >/dev/null 2>&1 || true; rm -rf $B' EXIT
mkdir -p $B/rdm; git rev-parse --short=10 HEAD > $B/rdm/REV; cp target/release/ecbench $B/rdm/; sha256sum target/release/ecbench | tee $D/ecbench.sha256 > $B/rdm/ecbench.sha256
git archive HEAD gpu/rootfind | tar -x -C $B; mv $B/gpu/rootfind $B/rdm/rootfind; rm -rf $B/gpu $B/rdm/rootfind/rs
cp -r $D/specs $D/job.sh $B/rdm/; (cd $B && tar czf rdm.tgz rdm)
RP_CUDA="" timeout 300 bash $R create $P "NVIDIA A100 80GB PCIe" | tail -2
timeout 900 bash $R wait $P | tail -1
# The proxy has answered 404 to uploads just after the pod first reports ready (twice, 2026-11-15
# 18:55-19:05Z): wait, then retry the upload up to three times.
sleep 30
for i in 1 2 3; do timeout 600 bash $R put $P $B/rdm.tgz rdm.tgz | tail -1 && break; echo "put retry $i" >&2; sleep 30; done
ex() { id=$(bash $R exec $P "$1" | python3 -c "import json,sys;print(json.load(sys.stdin)['id'])"); sleep ${2:-3}; bash $R log $P $id; }
ex 'cd /work && rm -rf rdm && tar xzf rdm.tgz && nvidia-smi -L && nohup bash rdm/job.sh > /work/job.out 2>&1 &' 6
until ex 'cat /work/out_rdm/progress.log 2>/dev/null' | tee $B/prog | grep -q "\] done\|ABORT"; do sleep 60; done
cat $B/prog
mkdir -p $D/raw
timeout 900 bash $R get $P out_rdm.tgz $D/raw/out_rdm.tgz
timeout 60 bash $R get $P out_rdm.tgz.sha256 $D/raw/out_rdm.tgz.sha256
[ "$(cut -c1-64 $D/raw/out_rdm.tgz.sha256)" = "$(sha256sum $D/raw/out_rdm.tgz | cut -c1-64)" ] && echo "sha256 ok"
grep -q "ABORT" $B/prog && exit 1 || exit 0
