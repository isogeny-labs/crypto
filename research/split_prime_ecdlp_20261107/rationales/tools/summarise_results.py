#!/usr/bin/env python3
"""Lay out the landed P-192 class-group results as Markdown rows (recorded
values only; AGENTS.md: Python lays out what was recorded). Reads
data/cl/rust_<label>.json, structure_<label>.json, pari_check_<label>.json."""
import glob, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
CL = os.path.join(HERE, "..", "data", "cl")
NAMES = {"p192-139": "P-192", "p192-5228": "prime192v3", "p192-6013": "prime192v2", "p224-7026": "P-224",
         "p239-2185": "prime239v1", "p239-2320": "prime239v2", "p239-3048": "prime239v3", "p256-1842": "P-256"}
ROLE = {}
INPUTS = os.path.join(HERE, "..", "..", "..", "nist_skip_criterion_20261025", "inputs.tsv")
for line in open(INPUTS):
    if line.startswith("#") or not line.strip():
        continue
    f = line.rstrip("\n").split("\t")
    ROLE[f[0]] = f[2] + (f" ({NAMES[f[0]]})" if f[0] in NAMES else "")
rows = []
for f in sorted(glob.glob(os.path.join(CL, "rust_p*.json"))):
    r = json.load(open(f)); lab = r["label"]
    s = json.load(open(os.path.join(CL, f"structure_{lab}.json"))) if os.path.exists(os.path.join(CL, f"structure_{lab}.json")) else None
    c = json.load(open(os.path.join(CL, f"pari_check_{lab}.json"))) if os.path.exists(os.path.join(CL, f"pari_check_{lab}.json")) else None
    fac = " · ".join(f"{p}^{e}" if e > 1 else f"{p}" for p, e in zip(r["h_factorisation"]["primes"], r["h_factorisation"]["exponents"]))
    syl = "; ".join(f"{x['q']}: {x['invariant_exponents']}" for x in s["sylow"]) if s else "–"
    idx = {b["bound"]: b["index_split_odd"] for b in s["by_bound"]} if s else {}
    rows.append({
        "label": lab, "role": ROLE.get(lab, "?"), "h": r["h"], "h_bits": r["h_bits"], "ratio": r["ratio_to_analytic"],
        "fac": fac, "two_rank": r["two_rank_genus"], "sylow": syl,
        "idx50": idx.get(50, "–"), "idx163": idx.get(163, "–"), "idx691": idx.get(691, "–"), "idx2000": idx.get(2000, "–"),
        "mazur": s["index_mazur"] if s else "–", "gen": s["smallest_split_prime_bound_generating"] if s else "–",
        "rel": r["relations"], "prod": r["products_sieved"], "dense": r["dense_columns"], "wall": r["wall_s"]["total"],
        "pari_bad": c["f_pow_N_not_identity"] if c else "–", "pari_n": c["split_primes_le_300"] if c else "–",
    })
mode = sys.argv[1] if len(sys.argv) > 1 else "md"
if mode == "md":
    print("| curve | role | h(O_K) | log₂ h / analytic ratio | factorisation | 2-rank (genus) | Sylow invariants (exponents) | index of ⟨split odd ℓ ≤ 50 / 163 / 691 / 2000⟩ | index of ⟨Mazur split⟩ | smallest generating bound | PARI: f^h ≠ 1 among split ℓ ≤ 300 |")
    print("|---|---|---|---|---|---:|---|---|---:|---:|---|")
    for r in rows:
        print(f"| {r['label']} | {r['role']} | {r['h']} | {r['h_bits']} bits / {r['ratio']:.5f} | {r['fac']} | {r['two_rank']} | {r['sylow']} | {r['idx50']} / {r['idx163']} / {r['idx691']} / {r['idx2000']} | {r['mazur']} | {r['gen']} | {r['pari_bad']} of {r['pari_n']} |")
    print()
    print("| curve | relations | products sieved | dense columns | wall (s, 4 threads) |")
    print("|---|---:|---:|---:|---:|")
    for r in rows:
        print(f"| {r['label']} | {r['rel']} | {r['prod']} | {r['dense']} | {r['wall']:.0f} |")
elif mode == "typ":
    # two tables: identity (wide numbers) and structure
    for r in rows:
        print(f"  [{r['label']}], [{r['role']}], [{r['h']}], [{r['h_bits']} / {r['ratio']:.5f}], [{r['fac']}],")
    print("---")
    for r in rows:
        print(f"  [{r['label']}], [{r['two_rank']}], [{r['sylow']}], [{r['idx50']} / {r['idx163']} / {r['idx691']} / {r['idx2000']}], [{r['mazur']}], [{r['gen']}], [{r['pari_bad']} of {r['pari_n']}],")
