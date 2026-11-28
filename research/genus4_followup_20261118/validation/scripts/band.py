#!/usr/bin/env python3
"""Section 5a band for genus4_followup_20261118 against every rho reference reading (validator).

Unit: group-addition equivalents (GAE): one Jacobian group operation = 1, every listed unpriced native
operation = w, w in {0, 0.1, 1}. Mean cost per key over the 108 keys (54 targets x 2 seeds) at p = 16411.

IC arms x and w: G + w (rel_mul + rel_add + la_mul + la_add)   [the frozen reading]
  - sym: plus the IC walk's own per-step partition hash and two coefficient additions mod r
    (genus_ladder.rs 1508-1512 at ae36cdc6e4: mix64(key) and two addm per step, not counted), as 3 * trials,
    the same closed form Q4 charges rho.
References:
  - neg_floor: ideal negation rho floor sqrt(pi r / 4) group operations, nothing else charged (a lower bound
    on every negation walk; the strongest reference). Used in VALIDATION.json.
  - neg_model_q3 / neg_model_q4: a negation walk at the ideal step count sqrt(pi r / 4) charged its minimal
    per-step native work: one canonicalisation (q3), plus one hash and two coefficient additions (q4). Model.
  - Q_Q3 / Q_Q4: measured plain rho Q (study rows), Q3 = G + w K (K = 0), Q4 = G + w (K + H + D).
  - B (k = 0..2 only, 18 keys): measured negation walk with Brent (lean-rho study rows), Q3 = G + w (C + K),
    Q4 = G + w (C + K + H + D), C = canonicalisations as emitted (floored at one per step).
Usage: band.py REPO OUT_JSON [COMMIT]"""
import gzip, json, math, statistics, subprocess, sys
REPO, OUT = sys.argv[1], sys.argv[2]
COMMIT = sys.argv[3] if len(sys.argv) > 3 else "6213ecbc0da0ed4917ab75805258201d6629b477"
D = "research/genus4_followup_20261118"
P = 16411
WS = (("0", 0.0), ("0.1", 0.1), ("1", 1.0))


def blob(path):
    raw = subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:{path}"], check=True, capture_output=True).stdout
    txt = gzip.decompress(raw).decode() if path.endswith(".gz") else raw.decode()
    return [json.loads(l) for l in txt.splitlines() if l.strip()]


tracked = subprocess.run(["git", "-C", REPO, "ls-tree", "-r", "--name-only", COMMIT], check=True,
                         capture_output=True).stdout.decode().splitlines()
lean = [r for f in sorted(t for t in tracked if t.startswith("research/genus4_lean_rho_20261116/p16411") and t.endswith(".jsonl"))
        for r in blob(f)]
bwx = blob(f"{D}/b_wx.jsonl.gz")
qrows = blob(f"{D}/b_q.jsonl.gz") + blob(f"{D}/b_q.jsonl.r1.gz") + lean
inst = {r["curve_seed"]: r for r in bwx if r["kind"] == "instance"}


def arm(rows, label):
    out = {}
    for r in rows:
        if r["kind"] == "run" and r["arm"] == label and r["p"] == P:
            out[(r["curve_seed"], r["target_seed"], r["seed"])] = r
    return out


X = arm(bwx, "C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge(4,1.5)")
W = arm(bwx, "C_double_lp+pre+coord+unitT")
Q = arm(qrows, "R_rho_plain_nowin")
B = arm(lean, "R_rho_neg_brent")
keys = sorted(X)
assert len(keys) == 108 and set(keys) == set(W) == set(Q)
kB = sorted(k for k in B if k in X)
mean = statistics.fmean
nat = lambda r: r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
ic = lambda r, w: r["group_ops"] + w * nat(r)
ic_sym = lambda r, w: r["group_ops"] + w * (nat(r) + 3 * r["trials"])
floor = lambda k, A: math.sqrt(math.pi * inst[k[0]]["r"] / (2 * A))
q3 = lambda r, w: r["group_ops"] + w * r["compares_uncharged"]
q4 = lambda r, w: r["group_ops"] + w * (r["compares_uncharged"] + r["hash_uncharged"] + r["coef_adds_uncharged"])
b3 = lambda r, w: r["group_ops"] + w * (r["native"] + r["compares_uncharged"])
b4 = lambda r, w: b3(r, w) + w * (r["hash_uncharged"] + r["coef_adds_uncharged"])
out = {"commit": COMMIT, "n_keys": len(keys), "n_keys_B": len(kB), "band": {}, "ratios": {}, "notes": __doc__}
for wn, w in WS:
    b = {"ic_x": mean(ic(X[k], w) for k in keys), "ic_w": mean(ic(W[k], w) for k in keys),
         "ic_x_sym": mean(ic_sym(X[k], w) for k in keys), "ic_w_sym": mean(ic_sym(W[k], w) for k in keys),
         "neg_floor": mean(floor(k, 2) for k in keys),
         "neg_model_q3": mean(floor(k, 2) * (1 + w) for k in keys),
         "neg_model_q4": mean(floor(k, 2) * (1 + 4 * w) for k in keys),
         "plain_theory": mean(floor(k, 1) for k in keys),
         "Q_Q3": mean(q3(Q[k], w) for k in keys), "Q_Q4": mean(q4(Q[k], w) for k in keys),
         "k0_2": {"ic_x": mean(ic(X[k], w) for k in kB), "Q_Q3": mean(q3(Q[k], w) for k in kB),
                  "Q_Q4": mean(q4(Q[k], w) for k in kB), "B_Q3": mean(b3(B[k], w) for k in kB),
                  "B_Q4": mean(b4(B[k], w) for k in kB), "neg_floor": mean(floor(k, 2) for k in kB)}}
    out["band"][wn] = b
    rr = {}
    for ia in ("ic_x", "ic_w", "ic_x_sym", "ic_w_sym"):
        for ref in ("neg_floor", "neg_model_q3", "neg_model_q4", "plain_theory", "Q_Q3", "Q_Q4"):
            rr[f"{ia}/{ref}"] = b[ia] / b[ref]
    kb = b["k0_2"]
    for ref in ("Q_Q3", "Q_Q4", "B_Q3", "B_Q4", "neg_floor"):
        rr[f"k0_2 ic_x/{ref}"] = kb["ic_x"] / kb[ref]
    rr["k0_2 B_Q3/Q_Q3"], rr["k0_2 B_Q4/Q_Q4"] = kb["B_Q3"] / kb["Q_Q3"], kb["B_Q4"] / kb["Q_Q4"]
    out["ratios"][wn] = rr
out["B_steps_over_neg_floor_k0_2"] = mean(B[k]["steps"] for k in kB) / mean(floor(k, 2) for k in kB)
out["B_compares_per_step_k0_2"] = sum(B[k]["compares_uncharged"] for k in kB) / sum(B[k]["steps"] for k in kB)
out["Q_steps_over_plain_theory"] = mean(Q[k]["steps"] for k in keys) / mean(floor(k, 1) for k in keys)
order = lambda ia, ref: [(out["band"][wn][ia] > out["band"][wn][ref]) - (out["band"][wn][ia] < out["band"][wn][ref]) for wn, _ in WS]
out["order"] = {f"{ia} vs {ref}": order(ia, ref) for ia in ("ic_x", "ic_w") for ref in
                ("neg_floor", "neg_model_q3", "neg_model_q4", "Q_Q3", "Q_Q4")}
json.dump(out, open(OUT, "w"), indent=1)
for wn, _ in WS:
    print(wn, {k: round(v, 4) for k, v in out["ratios"][wn].items()})
print("order (-1 IC below at w = 0, 0.1, 1):", out["order"])
print("B steps / sqrt(pi r/4):", out["B_steps_over_neg_floor_k0_2"], "B compares/step", out["B_compares_per_step_k0_2"],
      "Q steps / sqrt(pi r/2):", out["Q_steps_over_plain_theory"])
