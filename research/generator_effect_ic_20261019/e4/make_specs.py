"""Write the E4 ecbench specs (research/generator_effect_ic_20261019, E0_E4.md §E4 PROTOCOL).

Eight 24-bit a = -3 prime-order curves (examples/a_minus_3_prime_order_curves.rs 24 8 20261019).
Per curve: gstd = the finder's smallest-abscissa point, grand1..4 = [c]gstd with c drawn
from random.Random(20261019 + i) in [2, r - 2]; one spec per curve (5 generator workloads),
and one A/A spec per curve (gstd again, target_seed 20261020).
"""
import json, math, random

CURVES = [
    (8582641, 6723661, 8580037, 1, 4004158),
    (13903607, 5849518, 13902853, 1, 2061011),
    (16341817, 4523893, 16344179, 0, 8138185),
    (10774969, 1003143, 10779323, 3, 353342),
    (15910397, 9754220, 15908297, 0, 6001216),
    (9559721, 6907412, 9565123, 2, 2072703),
    (12934321, 11171511, 12928247, 0, 930954),
    (10584193, 5641052, 10580503, 3, 3376584),
]
N = 2000

def add(P, Q, p):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2 and (y1 + y2) % p == 0: return None
    if P == Q: l = (3 * x1 * x1 - 3) * pow(2 * y1, -1, p) % p
    else: l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)

def mul(P, k, p):
    R = None
    while k:
        if k & 1: R = add(R, P, p)
        P = add(P, P, p); k >>= 1
    return R

gens = []
for i, (p, b, r, gx, gy) in enumerate(CURVES):
    assert (gy * gy - (gx ** 3 - 3 * gx + b)) % p == 0
    assert mul((gx, gy), r, p) is None
    rng = random.Random(20261019 + i)
    cs = [rng.randrange(2, r - 1) for _ in range(4)]
    cells = [("gstd", 1, (gx, gy))] + [(f"grand{j+1}", c, mul((gx, gy), c, p)) for j, c in enumerate(cs)]
    B = math.ceil(r ** (1 / 3))
    while (B - 1) ** 3 >= r: B -= 1
    for name, c, (x, y) in cells:
        gens.append({"curve": f"c{i+1}", "cell": name, "c": c, "gx": x, "gy": y, "p": p, "b": b, "r": r, "B": B})
    def curve(x, y):
        return {"kind": "prime_explicit", "p": p, "a": p - 3, "b": b, "group_order": r, "r": r, "gx": x, "gy": y}
    arms = [
        {"name": "ic-abscissa", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": f"prime-abscissa:size={B}", "oracle": "mitm:negation_folded=1,m=2"}}},
        {"name": "ic-knownlog-random", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": f"known-log:size={B},scalars=random", "oracle": "mitm:negation_folded=1,m=2"}}},
    ]
    meas = {"rounds": 1, "warmup": 0, "order": "alternate", "seed": 20261019, "isolation_required": "L0", "timeout_seconds": 600}
    main = {
        "schema": "ecbench.spec/v1",
        "label": f"generator-effect E4, curve c{i+1}: gstd and four random generators",
        "description": f"E4 of research/generator_effect_ic_20261019 (E0_E4.md, E4 PROTOCOL, frozen before this run). Curve c{i+1}, 24-bit a=-3 prime order; workloads = 5 generators (gstd, grand1-4) x {N} planted_shared targets; arms prime-abscissa and known-log scalars=random at B={B}, mitm negation_folded m=2.",
        "workloads": {"curves": [curve(x, y) for _, _, (x, y) in cells], "targets_per_curve": N, "target_seed": 20261019, "target_kind": "planted_shared"},
        "arms": arms, "measurement": meas,
    }
    aa = dict(main)
    aa["label"] = f"generator-effect E4, curve c{i+1}: A/A (gstd, second target seed)"
    aa["description"] = f"E4 A/A arm: gstd of c{i+1} again with target_seed 20261020, same arms, {N} targets."
    aa["workloads"] = {"curves": [curve(gx, gy)], "targets_per_curve": N, "target_seed": 20261020, "target_kind": "planted_shared"}
    json.dump(main, open(f"specs/e4-c{i+1}.json", "w"), indent=1)
    json.dump(aa, open(f"specs/e4-c{i+1}-aa.json", "w"), indent=1)
with open("generators.jsonl", "w") as f:
    for g in gens: f.write(json.dumps(g) + "\n")
print(len(gens), "generators;", [g["B"] for g in gens[::5]])
