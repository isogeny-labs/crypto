#!/usr/bin/env bash
# Runs on the GPU pod. Second A/B (after merging main): the binary protocol on top of main, ABBA per sweep:
#   main   = /work/ab/main   (ic from origin/main: fes_pack_direct, text --serve)
#   merged = /work/ab/merged (ic from this branch merged with main: fes_pack_direct + --serve-bin)
# Writes /work/out_ab2/<sweep>-<arm>-<k>.json.   job2.sh
set -uo pipefail
W=/work/ab; OUT=/work/out_ab2; mkdir -p $OUT
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
for a in main merged; do
  ( cd $W/$a/gpu/fes && make cuda host-worker ARCH=$ARCH > $OUT/build-$a.log 2>&1 && ./fes_cuda > $OUT/selftest-$a.log 2>&1 )
  log "built $a: $(tail -1 $OUT/selftest-$a.log)"
done
sha256sum $W/main/ic $W/merged/ic $W/main/gpu/fes/fes_cuda $W/merged/gpu/fes/fes_cuda > $OUT/sha256.txt
cat $W/main/REV $W/merged/REV > $OUT/revs.txt
# serve smoke: the new CUDA worker's binary replies must equal the new host worker's
python3 - $OUT/serve-smoke.bin <<'P'
import struct,sys,random
random.seed(5); out=open(sys.argv[1],'wb')
for k in range(30):
    n=random.randint(1,22); m=random.randint(1,8); t=n*(n+1)//2; mask=(1<<m)-1
    out.write(struct.pack('<4I',0x31534546,n,m,0)+struct.pack('<Q',random.getrandbits(64)&mask))
    out.write(b''.join(struct.pack('<Q',random.getrandbits(64)&mask) for _ in range(n+t)))
P
$W/merged/gpu/fes/fes_cuda --serve-bin < $OUT/serve-smoke.bin > $OUT/serve-smoke-cuda.out 2>&1
$W/merged/gpu/fes/fes_solve --serve-bin < $OUT/serve-smoke.bin > $OUT/serve-smoke-host.out 2>&1
python3 - $OUT/serve-smoke-cuda.out $OUT/serve-smoke-host.out > $OUT/serve-smoke.txt <<'P'
import struct,sys
def replies(p):
    b=open(p,'rb').read(); i=0; r=[]
    while i<len(b):
        mg,st,listed,total=struct.unpack_from('<IIQQ',b,i); i+=24
        xs=sorted(struct.unpack_from(f'<{listed}Q',b,i)); i+=8*listed; r.append((st,total,xs))
    return r
a,b=replies(sys.argv[1]),replies(sys.argv[2])
print(len(a),'replies cuda',len(b),'host','EQUAL' if a==b else 'DIFFER', sum(len(x[2]) for x in a),'solutions')
P
log "serve-bin smoke: $(cat $OUT/serve-smoke.txt)"
declare -A TMO=([koblitz-n23]=2400 [char2-n25]=1800 [char2-n29]=3600)
for s in koblitz-n23 char2-n25 char2-n29; do
  k=0
  for a in main merged merged main; do
    k=$((k+1))
    ICX_FES_WORKER=$W/$a/gpu/fes/fes_cuda timeout ${TMO[$s]} $W/$a/ic bench --json --sweep $W/sweeps/$s.json \
      --out $OUT/$s-$a-$k.json --solver-budget-seconds 120 --rho-runs 16 --max-trials 20000000 > $OUT/$s-$a-$k.log 2>&1
    log "$s $a run $k rc=$?"
  done
  cd /work && tar czf out_ab2.tgz out_ab2 && sha256sum out_ab2.tgz > out_ab2.tgz.sha256; cd - >/dev/null
done
log "done"
