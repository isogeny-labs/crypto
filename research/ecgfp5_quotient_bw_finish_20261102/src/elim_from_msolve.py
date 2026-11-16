#!/usr/bin/env python3
"""Print msolve's eliminant (first polynomial of the rational parametrisation) as [c0,...,cD]."""
import re, sys
s = open(sys.argv[1]).read()
m = re.search(r"\[1,\s*\[\[(\d+),\s*\[([0-9,\s]+)\]\]", s)
d = int(m.group(1)); c = [int(x) for x in m.group(2).replace("\n", "").split(",")]
assert len(c) == d + 1, (len(c), d)
print("[" + ",".join(map(str, c)) + "]")
