#!/usr/bin/env python3
"""LEAD-C-jv-sieve-emit-price: C1 reproduction check and A_emit / proxy.

Run from the repo root:  python3 research/leads_20261111/runs/C_sieve_emit_price/analyze.py
Writes results/summary.json, results/emit_a_primary.json (seed -> A_emit, input of
check_emit) and results/emit_a_upper.json (sensitivity: F4 normalisations counted as adds).
"""
import json, os, sys

D = "research/leads_20261111/runs/C_sieve_emit_price"
R = f"{D}/results"
COMMITTED = "research/jv_sieve_negrho_20261102/results/inputs/ic_1511_sparse.json"
committed = {r["seed"]: r for r in json.load(open(COMMITTED))}


def diff_keys(new, old):
    """Keys of the committed record whose values differ (wall_ms excluded)."""
    return [k for k in old if k != "wall_ms" and new.get(k) != old[k]]


out = {"seeds": {}, "plain_control": {}}
c1_ok = True
for s in range(17, 33):
    jp, sp = f"{R}/emit_s{s}.json", f"{R}/emit_s{s}.side.json"
    if not (os.path.exists(jp) and os.path.exists(sp)):
        print(f"seed {s}: missing", file=sys.stderr)
        continue
    new = json.load(open(jp))[0]
    side = json.load(open(sp))
    old = committed[s]
    dk = diff_keys(new, old)
    extra = sorted(set(new) - set(old))
    c1_ok &= not dk
    ph = side["phases"]
    fa = lambda name: ph[name]["field_add"]
    # proxy (validator / fit tool): unemitted multiplications
    proxy = old["total_muls"] - old["sieve_muls"] - old["enum_muls"]
    f4_adds = side["f4_row_ops"] - side["f4_norm_ops"]  # multiply-subtracts only
    descent_other_adds = fa("descent") - side["f4_row_ops"]  # weil, lin, post, verify, stream (gaudry hooks)
    descent_adds = f4_adds + descent_other_adds
    la_adds = ph["linear_algebra"]["modr_add"] + ph["linear_algebra"]["field_add"]
    small_adds = fa("factor_base") + fa("model") + fa("verification") + fa("unattributed")
    # proxy-scope work inside the Relations phase (setup, table, extract, verify):
    # not separable from the sieve/enumeration adds in the bitops tally, carried at k = 1
    carried = old["setup_muls"] + old["table_muls"] + old["extract_muls"] + old["verify_muls"]
    a_emit = descent_adds + la_adds + small_adds + carried
    a_upper = a_emit + side["f4_norm_ops"]
    rec = {
        "c1_differing_committed_keys": dk,
        "keys_not_in_committed_record": extra,
        "new_fields": {k: new[k] for k in extra},
        "foreign_hook_calls": side["foreign_hook_calls"],
        "other_modulus": {k: v["other_modulus"] for k, v in ph.items() if v["other_modulus"]},
        "vm_hwm_kb": side["vm_hwm_kb"],
        "proxy": proxy,
        "proxy_parts": {"descent_muls": old["descent_muls"], "la_muls": old["la_muls"],
                        "setup_base_table_extract_verify": proxy - old["descent_muls"] - old["la_muls"]},
        "f4_row_ops": side["f4_row_ops"], "f4_norm_ops": side["f4_norm_ops"],
        "f4_row_ops_eq_descent_f4_muls": side["f4_row_ops"] == old["descent_f4_muls"],
        "f4_adds": f4_adds,
        "descent_other_adds": descent_other_adds,
        "descent_adds": descent_adds,
        "la_modr_add": ph["linear_algebra"]["modr_add"], "la_modr_mul": ph["linear_algebra"]["modr_mul"],
        "la_ops_legacy": old["la_ops"],
        "small_phase_adds": small_adds,
        "carried_at_k1": carried,
        "A_emit": a_emit, "A_emit_upper": a_upper,
        "ratio": a_emit / proxy, "ratio_upper": a_upper / proxy,
        "descent_ratio": descent_adds / old["descent_muls"],
        "la_ratio": la_adds / old["la_muls"],
        "bitops_descent_field_mul": ph["descent"]["field_mul"],
        "bitops_descent_field_inv": ph["descent"]["field_inv"],
        "legacy_descent_muls": old["descent_muls"],
    }
    out["seeds"][s] = rec
for s in (17, 24, 27, 32):
    jp = f"{R}/plain_s{s}.json"
    if os.path.exists(jp):
        new = json.load(open(jp))[0]
        out["plain_control"][s] = diff_keys(new, committed[s])


def agg(seeds, key_a="A_emit"):
    rs = [out["seeds"][s] for s in seeds if s in out["seeds"]]
    sa = sum(r[key_a] for r in rs)
    sp = sum(r["proxy"] for r in rs)
    per = [r[key_a] / r["proxy"] for r in rs]
    return {"n": len(rs), "mean_A": sa / max(len(rs), 1), "mean_proxy": sp / max(len(rs), 1),
            "ratio_of_means": sa / sp if sp else None, "per_seed_min": min(per) if per else None,
            "per_seed_max": max(per) if per else None}


out["c1_all_identical"] = c1_ok and all(not v for v in out["plain_control"].values())
out["agg"] = {
    "seeds_17_24": agg(range(17, 25)), "seeds_17_32": agg(range(17, 33)),
    "seeds_17_24_upper": agg(range(17, 25), "A_emit_upper"), "seeds_17_32_upper": agg(range(17, 33), "A_emit_upper"),
}
json.dump(out, open(f"{R}/summary.json", "w"), indent=1, sort_keys=True)
json.dump({str(s): r["A_emit"] for s, r in out["seeds"].items()}, open(f"{R}/emit_a_primary.json", "w"), indent=1)
json.dump({str(s): r["A_emit_upper"] for s, r in out["seeds"].items()}, open(f"{R}/emit_a_upper.json", "w"), indent=1)

print("C1 (every committed key except wall_ms identical, all emit + plain rows):", out["c1_all_identical"])
print("plain control differing keys:", out["plain_control"])
print("| seed | proxy | A_emit | ratio | F4 adds | F4 norm | descent other adds | LA adds | descent ratio | LA ratio | VmHWM MB |")
print("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for s, r in sorted(out["seeds"].items()):
    print(f"| {s} | {r['proxy']:.4e} | {r['A_emit']:.4e} | {r['ratio']:.4f} | {r['f4_adds']:.4e} | {r['f4_norm_ops']:.3e} | "
          f"{r['descent_other_adds']:.3e} | {r['la_modr_add']:.3e} | {r['descent_ratio']:.4f} | {r['la_ratio']:.4f} | {r['vm_hwm_kb']/1024:.1f} |")
for k, v in out["agg"].items():
    print(k, json.dumps(v))
print("foreign hook calls (max):", max(r["foreign_hook_calls"] for r in out["seeds"].values()))
print("other-modulus ops:", {s: r["other_modulus"] for s, r in out["seeds"].items() if r["other_modulus"]})
print("F4 row ops == legacy descent_f4_muls on every seed:", all(r["f4_row_ops_eq_descent_f4_muls"] for r in out["seeds"].values()))
