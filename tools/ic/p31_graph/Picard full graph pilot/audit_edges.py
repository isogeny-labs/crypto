"""Independent integer replay of bounded p31 PP edge records."""
from pathlib import Path
import hashlib
import json
import math
import struct
import time

HERE = Path(__file__).resolve().parent
EDGE = HERE/'edges_first20m_s4of5.bin'
INPUT = Path('/Users/adamburan/hcdlp-p31-checkpoints/triangle_bootstrap_n60000/input.txt')
FB = Path('/Users/adamburan/hcdlp-p31-checkpoints/triangle_bootstrap_n60000/collection/fb.bin')
start = time.perf_counter()
p,tw,a,b,c,ell,mu,zeta,seed,B,*_ = map(int, INPUT.open().readline().split())
assert p == 2147483647 and B == 60000
fb = list(struct.iter_unpack('<II', FB.read_bytes()))
assert len(fb) == B
fb_x = {x for x,y in fb}
assert len(fb_x) == B
cutoff = 4*p//5
record_count = EDGE.stat().st_size//16
assert EDGE.stat().st_size == record_count*16 == 6397029*16

MASK = (1<<64)-1
def mix(x):
    x &= MASK
    x = ((x^(x>>30))*0xbf58476d1ce4e5b9)&MASK
    x = ((x^(x>>27))*0x94d049bb133111eb)&MASK
    return (x^(x>>31))&MASK

pair_space = B*(B-1)//2
half = (pair_space.bit_length()+1)//2
bitmask = (1<<half)-1
keys = [mix(seed+111+i*0x9e3779b97f4a7c15) for i in range(6)]
def permute(v):
    left,right=v>>half,v&bitmask
    for key in keys:
        left,right=right,(left^(mix(right+key)&bitmask))
    return (left<<half)|right

def pair_for(ordinal):
    rank=permute(ordinal//3)
    while rank>=pair_space:
        rank=permute(rank)
    i=(1+math.isqrt(1+8*rank))//2
    while i*(i-1)//2>rank:i-=1
    while (i+1)*i//2<=rank:i+=1
    j=rank-i*(i-1)//2
    assert 0<=j<i<B
    phase=(mix(rank+seed)%3+ordinal%3)%3
    return i,j,phase

def f(x):return tw*(x**4+a*x*x+b*x+c)%p
def replay(ordinal):
    i,j,phase=pair_for(ordinal)
    x1,y1=fb[i];x2,y0=fb[j];y2=y0*pow(zeta,phase,p)%p
    m=(y1-y2)*pow((x1-x2)%p,-1,p)%p
    bb=(y1-m*x1)%p
    c3=-m**3%p;c2=(tw*a-3*m*m*bb)%p;c1=(tw*b-3*m*bb*bb)%p;c0=(tw*c-bb**3)%p
    q32=(c3+x1*tw)%p;q31=(c2+x1*q32)%p;q30=(c1+x1*q31)%p
    assert (c0+x1*q30)%p==0
    q21=(q32+x2*tw)%p;q20=(q31+x2*q21)%p
    assert (q30+x2*q20)%p==0
    disc=(q21*q21-4*tw*q20)%p
    root=pow(disc,(p+1)//4,p)
    assert root*root%p==disc
    inv2tw=pow(2*tw,-1,p)
    r=(-q21+root)*inv2tw%p;s=(-q21-root)*inv2tw%p
    assert r!=s and r not in fb_x and s not in fb_x
    assert all(y**3%p==f(x) for x,y in [(x1,y1),(x2,y2),(r,(m*r+bb)%p),(s,(m*s+bb)%p)])
    return r,s

positions=sorted({0,record_count-1,*[(record_count-1)*i//1023 for i in range(1024)]})
with EDGE.open('rb') as stream:
    for position in positions:
        stream.seek(position*16)
        ordinal,x0,x1=struct.unpack('<QII',stream.read(16))
        assert 0<=ordinal<20000000 and x0<cutoff and x1<cutoff
        assert replay(ordinal)==(x0,x1)
digest=hashlib.sha256()
last=-1
seen=0
with EDGE.open('rb') as stream:
    while block:=stream.read(1<<20):
        digest.update(block)
        assert len(block)%16==0
        for ordinal,x0,x1 in struct.iter_unpack('<QII',block):
            assert ordinal>last and x0<cutoff and x1<cutoff
            last=ordinal;seen+=1
assert seen==record_count
out=dict(status='independent_integer_replay_passed',p=p,pairs_horizon=20000000,
         records=seen,record_bytes=16,source_input_sha256=hashlib.sha256(INPUT.read_bytes()).hexdigest(),
         factor_base_sha256=hashlib.sha256(FB.read_bytes()).hexdigest(),
         edge_sha256=digest.hexdigest(),sampled_records_recomputed=len(positions),
         all_ordinals_monotone_and_cutoff_checked=True,
         sampled_line_polynomial_and_curve_points_checked=True,
         audit_seconds=time.perf_counter()-start,
         full_graph_rank_not_tested=True,original_source_scalar_not_tested=True)
path=HERE/'audit_first20m.json';tmp=path.with_suffix('.json.tmp')
tmp.write_text(json.dumps(out,indent=2)+'\n');tmp.replace(path)
print({k:out[k] for k in ('status','records','sampled_records_recomputed','audit_seconds')})
