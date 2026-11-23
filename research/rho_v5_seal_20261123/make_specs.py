import json,sys
def curves(bits,n):
    rows=json.loads(open(f'{sys.argv[1]}/c{bits}.txt').read())[:n]
    return [{"kind":"prime_explicit","p":p,"a":a,"b":b,"group_order":N,"r":N,"gx":x,"gy":y} for p,a,b,N,x,y in rows]
def mk(bits,ncurves,tpc,rounds,out):
    d={"schema":"ecbench.spec/v1","label":f"rho-v5-seal: rho.negation_v5 vs rho.negation fp{bits}","description":"Seal audit: v5 (pow2, weight-2 cap 64) vs rho.negation on PARI-generated prime-order curves (cofactor 1); curve/target/measurement seeds disjoint from earlier panels. ABBA; [d]G=Q.",
    "workloads":{"curves":curves(bits,ncurves),"targets_per_curve":tpc,"target_seed":20261123+bits},
    "arms":[{"name":"rho-neg","role":"reference","method":{"id":"rho.negation","params":{"cap_multiple":"64"}}},
    {"name":"cand","role":"candidate","method":{"id":"rho.negation_v5","params":{"cap_multiple":"64"}}},
    {"name":"rho-neg-aa","role":"control","method":{"id":"rho.negation","params":{"cap_multiple":"64"}}}],
    "measurement":{"rounds":rounds,"warmup":0,"order":"alternate_measured","seed":20261123,"isolation_required":"L0","timeout_seconds":7200}}
    json.dump(d,open(out,'w'),indent=2)
bits,n,t,r,out=int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]),int(sys.argv[5]),sys.argv[6]
mk(bits,n,t,r,out)
