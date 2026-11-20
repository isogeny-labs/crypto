#!/usr/bin/env python3
"""Validator sensitivities for study 2 (research/genus4_rho_refs_20261116), at
w = 0, 0.1, 1, with the protocol's bootstrap (targets, then seeds).  These are
the validator's readings, labelled as such; they are not the producer's frozen
readings.  Every record field is read with [].

  K-weight:    rho's window compares priced at kappa * w (kappa = 1 is Q3)
  P-windowless: the plain walk without its 16-entry short-cycle window
               (K_P = 0; the same walk if the window never fires on a plain
               walk, which the dumper's counters check), floored and unfloored
  break-even:  kappa at which IC / R = 1 at w = 1 (point estimate)."""
import json, random, sys

ROOT, OUT = sys.argv[1], sys.argv[2]
rows = [json.loads(l) for l in open(f"{ROOT}/research/genus4_rho_refs_20261116/run.jsonl")]
runs = [r for r in rows if r["kind"] == "run"]
A = {"w": "C_double_lp+pre+coord+unitT", "R": "R_rho_neg", "P": "R_rho_plain"}
X = {(r["arm"], r["curve_seed"], r["target_seed"], r["seed"]): r for r in runs}
keys = sorted({(r["curve_seed"], r["target_seed"], r["seed"]) for r in runs})
B, SEED = 4000, 778


def mean(v):
    return sum(v) / len(v)


def ic(r, w):
    return r["group_ops"] + w * (r["rel_mul"] + r["la_mul"] + r["rel_add"] + r["la_add"])


def rho(r, w, kappa=1.0, floor=True, window=True):
    C = r["native"]
    if r["arm"] == A["P"] and not floor:
        C = 0
    K = r["compares_uncharged"] if window else 0
    return r["group_ops"] + w * (C + kappa * K)


def boot(w, fR, fP, ks=keys):
    rnd = random.Random(SEED)
    tg = sorted({(c, t) for c, t, s in ks})
    by = {t: [k for k in ks if (k[0], k[1]) == t] for t in tg}
    def stat(kk):
        i = mean([ic(X[(A["w"],) + k], w) for k in kk])
        rr = mean([fR(X[(A["R"],) + k]) for k in kk])
        pp = mean([fP(X[(A["P"],) + k]) for k in kk])
        return i / min(rr, pp), "R" if rr <= pp else "P"
    est, which = stat(ks)
    bs = []
    for _ in range(B):
        kk = []
        for t in [rnd.choice(tg) for _ in tg]:
            kk += [rnd.choice(by[t]) for _ in by[t]]
        bs.append(stat(kk)[0])
    bs.sort()
    return {"ratio": est, "lo": bs[int(0.025 * B)], "hi": bs[int(0.975 * B) - 1], "cheapest": which}


out = {}
for name, fR, fP in [
    ("Q3 (as frozen)", lambda w: (lambda r: rho(r, w)), lambda w: (lambda r: rho(r, w))),
    ("Q3, K at kappa=1/4 (one op per 4-lane AVX2 compare)", lambda w: (lambda r: rho(r, w, 0.25)), lambda w: (lambda r: rho(r, w, 0.25))),
    ("Q3, plain walk without window (K_P=0), floored", lambda w: (lambda r: rho(r, w)), lambda w: (lambda r: rho(r, w, window=False))),
    ("Q3, plain walk without window (K_P=0), unfloored", lambda w: (lambda r: rho(r, w)), lambda w: (lambda r: rho(r, w, floor=False, window=False))),
]:
    out[name] = {str(w): boot(w, fR(w), fP(w)) for w in (0.0, 0.1, 1.0)}
# break-even kappa for negation rho at w = 1 (point estimate)
W = [X[(A["w"],) + k] for k in keys]; R = [X[(A["R"],) + k] for k in keys]
ic1 = mean([ic(r, 1.0) for r in W])
gc = mean([r["group_ops"] + r["native"] for r in R])
kk = mean([r["compares_uncharged"] for r in R])
out["break_even_kappa_R_w1"] = (ic1 - gc) / kk
out["share_of_R_Q3_cost_w1_from_K"] = kk / (gc + kk)
json.dump(out, open(OUT, "w"), indent=1)
for name, d in out.items():
    if isinstance(d, dict):
        print(f"{name:55s}", " | ".join(f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}] ({v['cheapest']})" for v in d.values()))
    else:
        print(f"{name:55s} {d:.4f}")
