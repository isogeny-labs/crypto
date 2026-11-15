"""N* = common analysed sequence length at dimension l = min over seeds of INC targets done."""
import json, sys, glob
l = sys.argv[1]; d = sys.argv[2]
ns = []
for f in sorted(glob.glob('%s/raw_l%s_s*_A.jsonl' % (d, l))):
    ns.append(sum(1 for L in open(f) if '"arm": "INC"' in L))
print(min(ns))
