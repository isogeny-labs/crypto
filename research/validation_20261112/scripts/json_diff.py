#!/usr/bin/env python3
"""Leaf-by-leaf comparison of two JSON documents (validator tool).
Prints every differing leaf path; paths whose last key matches a timing pattern are
listed apart.  Exit 0 iff every non-timing leaf is identical (exact equality, ints and
floats compared as parsed).  Usage: json_diff.py A.json B.json [--timing REGEX]"""
import json, re, sys
a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
pat = re.compile(sys.argv[4] if len(sys.argv) > 4 and sys.argv[3] == "--timing" else r"(wall|_ms$|_s$|^secs$|_ns_|bookkeeping|unisolated)", re.I)
diffs, tdiffs, n = [], [], [0]
def walk(x, y, path, timing):
    if isinstance(x, dict) and isinstance(y, dict):
        for k in sorted(set(x) | set(y)):
            t = timing or bool(pat.search(str(k)))
            if k not in x or k not in y: (tdiffs if t else diffs).append((path + [k], "missing key")); continue
            walk(x[k], y[k], path + [k], t)
    elif isinstance(x, list) and isinstance(y, list):
        if len(x) != len(y): (tdiffs if timing else diffs).append((path, f"len {len(x)} vs {len(y)}")); return
        for i, (u, v) in enumerate(zip(x, y)): walk(u, v, path + [i], timing)
    else:
        n[0] += 1
        if x != y or type(x) != type(y): (tdiffs if timing else diffs).append((path, f"{x!r} vs {y!r}"))
walk(a, b, [], False)
print(f"leaves compared {n[0]}; non-timing differences {len(diffs)}; timing-field differences {len(tdiffs)}")
for p, m in diffs[:30]: print("  DIFF", "/".join(map(str, p)), m[:200])
keys = sorted({str(p[-1]) if p else "" for p, _ in tdiffs})
print("  timing keys that differ:", keys[:20])
sys.exit(0 if not diffs else 1)
