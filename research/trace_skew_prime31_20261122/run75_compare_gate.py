#!/usr/bin/env python3
"""Run75 original-Rust comparison-log gate. Analyses x-only extraction, not ECDLP.

Usage: python3 research/trace_skew_prime31_20261122/run75_compare_gate.py CI.log
Fails closed if any size or exact reference comparison is missing.
"""
import json
import re
import sys

ROW = re.compile(r"^monic-compare p=(\d+) B=(\d+) z=(\d+) pairs=(\d+) new_tree_median_ms=([\d.]+) old_newton_median_ms=([\d.]+) speedup_newton_over_tree=([\d.]+) exact=pass paired_reps=5 postsolve_reference=pass$")
TOTAL = re.compile(r"^monic-compare-total p=(\d+) B=(\d+) cells=(\d+) old_newton_sum_median_ms=([\d.]+) new_tree_sum_median_ms=([\d.]+) speedup=([\d.]+) positives=(\d+) pairs=(\d+) exact=pass$")
SIZES = (64, 128, 256, 677)
P = 2147483647

def analyze(log):
    cells = {b: {} for b in SIZES}
    totals = {}
    for line in log.splitlines():
        # Native GitHub job logs may prefix messages with timestamps.
        if "monic-compare" not in line:
            continue
        line = line[line.index("monic-compare"):]
        if line.startswith("monic-compare-total "):
            match = TOTAL.fullmatch(line)
            if not match:
                raise ValueError(f"invalid total: {line}")
            p,b,n,old,new,ratio,pos,pairs = match.groups()
            p,b,n,pos,pairs = map(int,(p,b,n,pos,pairs))
            old,new,ratio = map(float,(old,new,ratio))
            if p!=P or b not in cells or b in totals or (n,pos,pairs)!=(8,3,3):
                raise ValueError("wrong total population")
            if new<=0 or abs(old/new-ratio) > max(.003,ratio*.003):
                raise ValueError("wrong total ratio")
            totals[b]=(old,new)
        elif line.startswith("monic-compare "):
            match = ROW.fullmatch(line)
            if not match:
                raise ValueError(f"invalid cell: {line}")
            p,b,z,hits,tree,old,ratio = match.groups()
            p,b,z,hits = map(int,(p,b,z,hits))
            tree,old,ratio = map(float,(tree,old,ratio))
            if p!=P or b not in cells or z in cells[b] or not (0<=z<P):
                raise ValueError("duplicate or out-of-domain cell")
            if tree<=0 or old<=0 or abs(old/tree-ratio)>max(.003,ratio*.003):
                raise ValueError("wrong cell ratio")
            cells[b][z]=(hits,tree,old)
    out=[]
    for b in SIZES:
        sample=list(cells[b].values())
        if len(sample)!=8 or sum(x[0]>0 for x in sample)!=3 or sum(x[0] for x in sample)!=3:
            raise ValueError(f"incomplete B={b} target corpus")
        if b not in totals:
            raise ValueError(f"missing B={b} total")
        old,new=totals[b]
        if abs(sum(x[2] for x in sample)-old)>.015 or abs(sum(x[1] for x in sample)-new)>.015:
            raise ValueError(f"incorrect B={b} total")
        out.append(dict(B=b,positive=3,negative=5,newton_ms=old,tree_ms=new,
            speedup=old/new,
            positive_speedup=sum(x[2] for x in sample if x[0])/sum(x[1] for x in sample if x[0]),
            negative_speedup=sum(x[2] for x in sample if not x[0])/sum(x[1] for x in sample if not x[0])))
    return out

if __name__=="__main__":
    if len(sys.argv)!=2:
        raise SystemExit("usage: run75_compare_gate.py PATH_TO_NATIVE_CI_LOG")
    with open(sys.argv[1],encoding="utf-8") as stream:
        results=analyze(stream.read())
    print(json.dumps({"scope":"Prime31 x-only S3 extraction stage, not complete ECDLP",
                      "rows":results},indent=2))
