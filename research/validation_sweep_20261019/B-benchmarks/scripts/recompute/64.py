# SWB-RECOMPUTE-20 row 64: recompute headline numbers from committed per-target / per-pair records.
# Point geomeans and counts only. Bootstrap CIs use rand::StdRng (ChaCha) in the Rust fit tool and are not reproduced here.
import json, math, re
BASE = "/home/user/crypto/research/ic_improve_20261015/05-linear-harvest/"
R = BASE + "runs/"
REFD = "/home/user/crypto/research/ic_improve_20261015/03-fallrow-gate/runs/"
COUNTERS = ["splits", "reductions", "elim", "word_ops", "calls", "spec_ops", "roots", "root_hash"]

def rd(p):
    with open(p) as f:
        return [json.loads(l) for l in f if l.strip()]

def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))

out = {}
test = []
for f in ["test_a.jsonl", "test_b.jsonl", "test_c.jsonl"]:
    test += rd(R + f)
out["test_rows"] = len(test)
out["test_rows_gate_ok_not_true"] = sum(1 for r in test if r.get("gate_ok") is not True)
out["test_root_hash_rows_all_arms_equal_to_i"] = sum(
    1 for r in test if all(r[a + "_root_hash"] == r["i_root_hash"] and r[a + "_roots"] == r["i_roots"]
                           for a in ["g", "r", "u", "h", "i2"]))

RATIOS = {"I/G": ("i_cpu", "g_cpu"), "I/R": ("i_cpu", "r_cpu"), "I/U": ("i_cpu", "u_cpu"),
          "I/H": ("i_cpu", "h_cpu"), "AA": ("i_cpu", "i2_cpu"),
          "W_I/G": ("i_word_ops", "g_word_ops"), "W_I/R": ("i_word_ops", "r_word_ops")}
stored = {}
for r in rd(R + "fit_test.jsonl"):
    if "ratio" in r and "cell" in r and "gm" in r:
        stored[(r["ratio"], r["cell"])] = (r["gm"], r.get("verdict"))

cells = {}
for r in test:
    cells.setdefault(r["cell"], []).append(r)
per_cell = {}
mismatch_vs_stored = []
for cell in sorted(cells):
    rows = [r for r in cells[cell] if r.get("gate_ok") is True]
    d = {"n": len(rows), "m_ell_n": cell}
    for name, (a, b) in RATIOS.items():
        xs = [r[a] / r[b] for r in rows if r.get(a) and r.get(b)]
        if not xs:
            continue
        g = gm(xs)
        d[name] = round(g, 4)
        if (name, cell) in stored:
            sg, sv = stored[(name, cell)]
            if abs(round(g, 4) - sg) > 1e-4:
                mismatch_vs_stored.append((name, cell, round(g, 4), sg))
            d[name + "_stored_verdict_fit_test"] = sv
    # identical counters, arm i vs arm g, per target
    d["g_counters_identical_to_i"] = sum(
        1 for r in cells[cell] if all(r["i_" + c] == r["g_" + c] for c in COUNTERS))
    d["n_targets_in_cell"] = len(cells[cell])
    per_cell[cell] = d
out["per_cell"] = per_cell
out["ratio_vs_stored_fit_test_mismatches"] = mismatch_vs_stored

ih = [per_cell[c]["I/H"] for c in per_cell if "I/H" in per_cell[c]]
out["I/H_cells"] = len(ih)
out["I/H_min"] = round(min(ih), 4)
out["I/H_max"] = round(max(ih), 4)

# identity: opt-out arm e vs incumbent i, and default arm d vs frozen spec g (identity files)
idn = rd(R + "identity.jsonl")
out["identity_rows"] = len(idn)
out["identity_i_eq_e_counters"] = sum(1 for r in idn if all(r["i_" + c] == r["e_" + c] for c in COUNTERS))
idd = rd(R + "identity_default.jsonl")
out["identity_default_rows"] = len(idd)
out["identity_default_g_eq_d_counters"] = sum(1 for r in idd if all(r["g_" + c] == r["d_" + c] for c in COUNTERS))

# item-03 committed incumbent reference (arm i) matched on (cell without pipe- prefix, target)
ref = {}
for f in ["test_pipe3.jsonl", "test_pipe2_barrel_small.jsonl"]:
    for r in rd(REFD + f):
        k = (r["cell"][5:] if r["cell"].startswith("pipe-") else r["cell"], r["target"])
        ref[k] = r
matched = identical = 0
fields_used = set()
for r in idn:
    k = (r["cell"][5:] if r["cell"].startswith("pipe-") else r["cell"], r["target"])
    if k not in ref:
        continue
    rr = ref[k]
    fs = [c for c in COUNTERS if ("i_" + c) in rr]
    fields_used.update(fs)
    matched += 1
    if all(r["i_" + c] == rr["i_" + c] for c in fs):
        identical += 1
out["identity_vs_item03_ref_i_matched"] = matched
out["identity_vs_item03_ref_i_identical"] = identical
out["identity_vs_item03_ref_fields"] = sorted(fields_used)

# end-to-end: rows come in consecutive pairs per replicate (arm 0 = incumbent, arm unset = default)
e2e = rd(R + "e2e.jsonl")
out["e2e_rows"] = len(e2e)
out["e2e_log_ok_true"] = sum(1 for r in e2e if r["log_ok"] is True)
pairs = {}
for k in range(0, len(e2e), 2):
    a, b = e2e[k], e2e[k + 1]
    assert (a["n"], a["ell"], a["m"]) == (b["n"], b["ell"], b["m"]) and {a["arm"], b["arm"]} == {"0", "unset"}
    i0 = a if a["arm"] == "0" else b
    iu = b if a["arm"] == "0" else a
    pairs.setdefault((iu["n"], iu["ell"], iu["m"]), []).append(i0["cpu_s"] / iu["cpu_s"])
e2e_cells = {}
for key, xs in pairs.items():
    rows = [r for r in e2e if (r["n"], r["ell"], r["m"]) == key]
    e2e_cells[str(key)] = {
        "pair_ratios_I_over_G": [round(x, 4) for x in xs],
        "geomean_I_over_G": round(gm(xs), 4),
        "reductions_by_arm": {arm: sorted({r["reductions"] for r in rows if r["arm"] == arm}) for arm in ["0", "unset"]},
        "harvests_by_arm": {arm: sorted({r["harvests"] for r in rows if r["arm"] == arm}) for arm in ["0", "unset"]},
        "dispositions_hash_set": sorted({r["dispositions_hash"] for r in rows}),
        "relations_trials": sorted({(r["relations"], r["trials"]) for r in rows}),
    }
    red = e2e_cells[str(key)]["reductions_by_arm"]
    e2e_cells[str(key)]["reduction_change_pct"] = round(
        100.0 * (red["unset"][0] - red["0"][0]) / red["0"][0], 2)
out["e2e_cells"] = e2e_cells

# unit tests (logs)
for f in ["unit_tests_default.log", "unit_tests_optout.log"]:
    txt = open(R + f).read()
    m = re.search(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", txt)
    out[f] = {"passed": int(m.group(1)), "failed": int(m.group(2)), "ignored": int(m.group(3))}

print(json.dumps(out, indent=1, sort_keys=True))
