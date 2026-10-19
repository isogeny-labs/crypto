import json, glob, math, os, collections, statistics
RUN = "/home/user/crypto/research/ic_improve3_20261015/01-genus3-relations/results/run"
def load(bits):
    out = collections.defaultdict(dict)   # (curve) -> arm label -> result
    for p in sorted(glob.glob(os.path.join(RUN, "b%d_c*_*.jsonl" % bits))):
        for l in open(p):
            if not l.strip(): continue
            r = json.loads(l)
            if r.get("bits") != bits: continue
            key = (r["curve"], r["c"], os.path.basename(p).split("_")[-1])
            for x in r["results"]:
                out[r["curve"]][(x["arm"], os.path.basename(p))] = x
    return out
def arms_of(d):
    labs = set(a for c in d for (a, f) in d[c])
    return sorted(labs)
d = load(22)
print("bits 22 curves", sorted(d.keys()), "arm labels", arms_of(d))
