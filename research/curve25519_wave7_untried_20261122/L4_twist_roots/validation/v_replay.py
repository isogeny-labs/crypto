#!/usr/bin/env python3
"""Replay of the sampled cells from the PROTOCOL seed rule (numpy default_rng seeded with the first 8
bytes, big-endian, of sha256("L4|k|tag|stratum|chunk"), chunks of 2e6, x1 then x2 drawn uniformly
from the ascending class list), classified with the validator's own root code.  Also records, for
the 'given' variant, the x1 == x2 count separately from the a == 0 (x1 x2 == 1) count.
Usage: v_replay.py OUT.json k variant n"""
import hashlib, json, sys, time
import numpy as np
sys.path.insert(0, __import__("os").path.dirname(__file__))
from vcore import CURVES, Tables, classify

out_fn, k, variant, n = sys.argv[1], int(sys.argv[2]), sys.argv[3], int(sys.argv[4])
p, A = CURVES[k]
t0 = time.process_time()
T = Tables(p, A)
H = np.zeros(9 * 2 * 27, dtype=np.int64)
diag_eq = {}           # per stratum: pairs with x1 == x2, and with a == 0
CH = 2_000_000


def seed(*parts):
    return int.from_bytes(hashlib.sha256("|".join(str(q) for q in parts).encode()).digest()[:8], "big")


lists = {"E": T.Ex, "T": T.Tx}
for name, ca, cb in (("EE", "E", "E"), ("ET", "E", "T"), ("TT", "T", "T")):
    done, ci, neq = 0, 0, 0
    ndeg = 0
    while done < n:
        m = min(CH, n - done)
        rng = np.random.default_rng(seed("L4", k, "main", name, ci))
        x1 = lists[ca][rng.integers(0, len(lists[ca]), m)]
        x2 = lists[cb][rng.integers(0, len(lists[cb]), m)]
        pc, deg, code = classify(T, x1, x2, variant)
        assert int(code.min()) >= 0
        H += np.bincount((pc * 2 + deg) * 27 + code, minlength=9 * 2 * 27)
        neq += int((x1 == x2).sum())
        ndeg += int(deg.sum())
        done += m
        ci += 1
    diag_eq[name] = {"x1_eq_x2": neq, "leading_coeff_zero": ndeg}
allx = np.arange(p, dtype=np.int64)
for z in T.Zx.tolist():
    pc, deg, code = classify(T, z, allx, variant)
    H += np.bincount((pc * 2 + deg) * 27 + code, minlength=9 * 2 * 27)
NAMES = {0: "ZZ", 1: "ZE", 2: "ZT", 4: "EE", 5: "ET", 8: "TT"}
table = []
codes = np.arange(27)
nZ, nT, nE = codes % 3, (codes // 3) % 3, codes // 9
nr = nZ + nT + nE
for pc, nm in NAMES.items():
    for dg in (0, 1):
        sl = H[(pc * 2 + dg) * 27:(pc * 2 + dg + 1) * 27]
        if sl.sum() == 0:
            continue
        table.append(dict(pair_class=nm, leading_coeff_zero=bool(dg), pairs=int(sl.sum()),
                          roots0=int(sl[nr == 0].sum()), roots1=int(sl[nr == 1].sum()), roots2=int(sl[nr == 2].sum()),
                          roots_on_E=int((sl * nE).sum()), roots_on_twist=int((sl * nT).sum()),
                          roots_on_Z=int((sl * nZ).sum()), pairs_with_root_on_Z=int(sl[nZ > 0].sum())))
res = dict(k=k, variant=variant, n_per_stratum=n, npairs=int(H.sum()), table=table, diag_vs_a0=diag_eq,
           cpu_s=round(time.process_time() - t0, 1))
json.dump(res, open(out_fn, "w"), indent=1)
for t in table:
    print(t)
print(diag_eq, "npairs", res["npairs"], "cpu", res["cpu_s"])
