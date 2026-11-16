#!/usr/bin/env python3
"""Compare two run logs as multisets of lines after removing '(N ms)' timings and
output-path lines (thread order differs between runs). Usage: compare_runlog.py A B"""
import re, sys
from collections import Counter
def norm(path):
    out = []
    for l in open(path):
        l = re.sub(r"\(\d+ ms\)", "(ms)", l.rstrip("\n"))
        if l.startswith("wrote "): continue
        out.append(l)
    return Counter(out)
a, b = norm(sys.argv[1]), norm(sys.argv[2])
print(f"lines {sum(a.values())} vs {sum(b.values())}; only in first {sum((a-b).values())}; only in second {sum((b-a).values())}")
for l in list((a - b).elements())[:5]: print("  -", l)
for l in list((b - a).elements())[:5]: print("  +", l)
sys.exit(0 if a == b else 1)
