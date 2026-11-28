#!/usr/bin/env python3
"""Exact frozen H4 n35 prime-subgroup and x-subspace screen."""
from sage.all import *
import hashlib,json,platform,random,resource,signal,time
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/"protocol.json").read_text())
assert hashlib.sha256((ROOT/SPEC["source"]).read_bytes()).hexdigest()==SPEC["source_sha256"]
def enc(e):return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
def save():
    (HERE/"result.json").write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
def timeout(*_):raise TimeoutError("frozen 180-second n35 screen cap")
signal.signal(signal.SIGALRM,timeout)
signal.alarm(SPEC["process_cap_seconds"])
out={"schema":"h4-n35-prime-subgroup-screen-result-v1","status":"running",
     "protocol_sha256":hashlib.sha256((HERE/"protocol.json").read_bytes()).hexdigest(),
     "runner_sha256":hashlib.sha256((HERE/"run.py").read_bytes()).hexdigest(),
     "host":{"node":platform.node(),"machine":platform.machine(),"system":platform.system()},
     "instances":[]}
save();start=time.perf_counter_ns()
try:
    n=SPEC["field_degree"];l=SPEC["subspace_dimension"]
    for idx in SPEC["instance_indices"]:
        t=time.perf_counter_ns()
        rng=random.Random(20261022*1000+n*10+idx)
        set_random_seed(20261022+n*10+idx)
        mod=PolynomialRing(GF(2),"x").irreducible_element(n)
        K=GF(2**n,"z",modulus=mod)
        poly=sum(int(c)<<i for i,c in enumerate(mod.list()))
        def elt(v):return K([(v>>i)&1 for i in range(n)])
        a2,a6=elt(rng.getrandbits(n)),elt(rng.getrandbits(n))
        while a6==0:a6=elt(rng.getrandbits(n))
        E=EllipticCurve(K,[1,a2,0,0,a6])
        while True:
            basis=[rng.getrandbits(n) for _ in range(l)]
            rows=list(basis);rank=0
            for bit in range(n):
                pivot=next((v for v in rows if (v>>bit)&1),None)
                if pivot is None:continue
                rows.remove(pivot);rank+=1
                rows=[v^pivot if (v>>bit)&1 else v for v in rows]
            if rank==l:break
        order=int(E.cardinality())
        fac=[[int(p),int(e)] for p,e in factor(order)]
        r=max(p for p,e in fac);h=order//r
        assert is_prime(r) and r*h==order
        out["instances"].append({"index":idx,"field_modulus":poly,"curve_a2":enc(a2),
                                 "curve_a6":enc(a6),"subspace_basis":basis,
                                 "group_order":order,"factorization":fac,
                                 "largest_prime_divisor":r,"cofactor":h,
                                 "eligible":r>=1073741824,"wall_ns":time.perf_counter_ns()-t})
        out["peak_rss_so_far_bytes"]=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        save()
        if out["peak_rss_so_far_bytes"]>SPEC["observed_peak_rss_bytes"]:
            raise MemoryError("frozen 8 GiB observed RSS threshold")
    out["selected_index"]=next((x["index"] for x in out["instances"] if x["eligible"]),None)
    out["status"]="complete"
except TimeoutError as exc:out.update(status="censored_timeout",error=str(exc))
except MemoryError as exc:out.update(status="censored_memory_limit",error=str(exc))
except Exception as exc:out.update(status="error",error=repr(exc));raise
finally:
    signal.alarm(0)
    out["total_wall_ns"]=time.perf_counter_ns()-start
    out["peak_rss_bytes"]=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save()
print(json.dumps({"status":out["status"],"selected_index":out.get("selected_index"),
                  "orders":[x["group_order"] for x in out["instances"]],
                  "largest_primes":[x["largest_prime_divisor"] for x in out["instances"]]},sort_keys=True))
