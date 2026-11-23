"""Replay 50-case correctness gates and paired *specialized root oracle* timings.

    python -m summation_split.sector_experiment --out summation_split

No Sage, F4, first-fall degree, unspecialized-ideal, or index-calculus
claims. Two large-field samples per cell are timed AFTER 50 exact gates.
"""
from __future__ import annotations
import argparse
import csv
import itertools
import json
import os
import random
import resource
import statistics
import sys
import time

from .sector import frobenius_sector_roots, zero_only_roots, two_torsion
from .test_sector import CELLS, exact_roots

SEED=12042026


def cases_for(p,a,b,m,count,seed):
    rr=random.Random(seed+p*10007+m*733)
    return [tuple(rr.randrange(p) for _ in range(m)) for _ in range(count)]


def write_csv(path,records):
    if not records:
        raise ValueError("no records")
    with open(path,"w",newline="") as fh:
        writer=csv.DictWriter(fh,fieldnames=list(records[0]))
        writer.writeheader()
        writer.writerows(records)


def run(out,samples=50,repeats=3,seed=SEED,skip_exhaustive=False):
    if samples<50 or repeats<1:
        raise ValueError("require at least 50 cases and >=1 repetition")
    os.makedirs(out,exist_ok=True)
    cold_setup={}
    for p,a,b,_ in CELLS:
        key=(p,a,b)
        if key not in cold_setup:
            start=time.perf_counter_ns()
            torsion=two_torsion(p,a,b)
            cold_setup[key]={"setup_ns":time.perf_counter_ns()-start,
                             "num_rational_2tors_including_O":len(torsion)}
    gate=[]
    instances={}
    for p,a,b,m in CELLS:
        cases=cases_for(p,a,b,m,samples,seed)
        total=0
        for i,xs in enumerate(cases):
            expected=exact_roots(p,a,b,xs)
            actual=frobenius_sector_roots(p,a,b,xs)
            if actual!=expected:
                raise AssertionError(f"gate mismatch {(p,a,b,m,i,xs)} {expected=} {actual=}")
            total+=len(expected)
        instances[(p,a,b,m)]=cases
        gate.append(dict(p=p,a=a,b=b,m=m,gate_instances=samples,
                         projected_X_roots_total=total,exact=True))
    write_csv(os.path.join(out,"sector_gate.csv"),gate)
    exhaustive_counts={}
    if not skip_exhaustive:
        for p,a,b,m in ((5,0,1,3),(7,1,1,3),
                        (11,1,1,3),(11,1,1,4)):
            c=0
            for xs in itertools.product(range(p),repeat=m):
                if exact_roots(p,a,b,xs)!=frobenius_sector_roots(p,a,b,xs):
                    raise AssertionError(f"exhaustive mismatch {(p,a,b,m,xs)}")
                c+=1
            exhaustive_counts[str((p,a,b,m))]=c
    if zero_only_roots(5,0,1,(0,1,2))!=set() or \
            frobenius_sector_roots(5,0,1,(0,1,2))!={3}:
        raise AssertionError("missing essential 2-torsion regression")
    bench=[]
    for (p,a,b,m),cases in instances.items():
        # Tiny exploratory paired subset; gate tested *all* 50 first.
        timed_cases=cases[:2 if p>=1000 else 10]
        for i,xs in enumerate(timed_cases):
            expected=exact_roots(p,a,b,xs)
            sector_stats=frobenius_sector_roots(p,a,b,xs,return_stats=True)[1]
            for name,fn in (
                ("semaev_resultant_and_root_extraction",exact_roots),
                ("frobenius_sector",frobenius_sector_roots),
            ):
                times=[]
                for j in range(min(repeats,2) if p>=1000 else repeats):
                    start=time.perf_counter_ns()
                    ans=fn(p,a,b,xs)
                    duration=time.perf_counter_ns()-start
                    if ans!=expected:
                        raise AssertionError(f"timed mismatch {name} {xs}")
                    times.append(duration)
                bench.append(dict(
                    p=p,a=a,b=b,m=m,sample_id=i,
                    xs=":".join(map(str,xs)),strategy=name,
                    gate_instances=samples,root_count=len(expected),
                    wall_ns_median=int(statistics.median(times)),
                    wall_ns_min=min(times),wall_ns_max=max(times),
                    rss_highwater_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                    plus_states=sector_stats["plus_states"],
                    minus_states=sector_stats["minus_states"],
                    backend="Python exact specialized roots; NOT GB",
                    first_fall_degree="",d_F4="",status="producer_only",
                ))
    write_csv(os.path.join(out,"sector_bench.csv"),bench)
    medians={}
    for p,a,b,m in CELLS:
        times={}
        for method in ("semaev_resultant_and_root_extraction","frobenius_sector"):
            vals=[r["wall_ns_median"] for r in bench
                  if (r["p"],r["a"],r["b"],r["m"])==(p,a,b,m)
                  and r["strategy"]==method]
            times[method]=statistics.median(vals)
        times["sector_to_resultant_ratio"]=(
            times["frobenius_sector"]/times["semaev_resultant_and_root_extraction"])
        medians[str((p,a,b,m))]=times
    result={
        "date":"2026-11-23",
        "validation":"producer only; not independent/PCC",
        "scope":"S_(m+1) specialized root extraction; no factor-base map",
        "correctness_cells":len(CELLS),
        "random_gate_instances":len(CELLS)*samples,
        "small_field_exhaustive":exhaustive_counts,
        "smallest_zero_only_counterexample":{"p":5,"a":0,"b":1,
                                             "xs":[0,1,2],"X":3},
        "cold_curve_setup":{str(k):v for k,v in cold_setup.items()},
        "medians_ns":medians,
        "caveat":"No matched unspecialized Gröbner/F4, solving-degree, or first-fall measurements; torsion setup warmed in timed rows.",
        "python":sys.version,
    }
    with open(os.path.join(out,"sector_summary.json"),"w") as fh:
        json.dump(result,fh,indent=2)
    return result


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("--out",default="summation_split")
    parser.add_argument("--samples",type=int,default=50)
    parser.add_argument("--repeats",type=int,default=3)
    parser.add_argument("--seed",type=int,default=SEED)
    parser.add_argument("--skip-exhaustive",action="store_true")
    args=parser.parse_args()
    print(json.dumps(run(args.out,args.samples,args.repeats,args.seed,
                         args.skip_exhaustive),indent=2))


if __name__=="__main__":
    main()
