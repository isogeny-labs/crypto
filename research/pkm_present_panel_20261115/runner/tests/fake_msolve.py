#!/usr/bin/env python3
"""Stub of `msolve -t 1 -v 2 -c 0 -g 2 -f X.ms -o Y`: prints a minimal -v 2 trace parseable by parse_msolve_log.py.
If the .ms file contains `sleep=N` it sleeps N s after printing the first row (a timeout test)."""
import re, sys, time
a = sys.argv[1:]
txt = open(a[a.index("-f") + 1]).read()
m = re.search(r"sleep=(\d+)", txt)
w = re.search(r"work=(\d+)", txt)
rows = int(w.group(1)) if w else 10
print("Legend for f4 information")
print("deg     sel   pairs        mat          density   new   zero   time(rd) in sec (real|cpu)")
print(f"  3     4     4   {rows} x 20   50.00%    2 new    0 zero   0.00 | 0.00", flush=True)
if m:
    time.sleep(int(m.group(1)))
print(f"  4     4     4   {rows} x 20   50.00%    2 new    0 zero   0.00 | 0.00")
print("reduce final basis   5 x 10   30.00%   3 new   0 zero   0.00 | 0.00")
open(a[a.index("-o") + 1], "w").write("[1]:\n")
