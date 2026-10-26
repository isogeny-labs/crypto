import json, statistics, os
ROOT = "/home/user/crypto/research/binary_mitm_oracle_l9_l10_20261015"
for rung in ["l9n27", "l10n31"]:
    d = json.load(open(os.path.join(ROOT, rung, "results.json")))
    dec = d["decomposition"]
    pt = dec["per_target"]
    mit = [x["mitm_ms"] for x in pt]
    prs = [x["pairs_ms"] for x in pt]
    print(rung, "targets", dec["targets"], "n_per_target", len(pt))
    print(rung, "median mitm_ms", round(statistics.median(mit), 6), "median pairs_ms", round(statistics.median(prs), 6))
    print(rung, "median ratio pairs/mitm", round(statistics.median(prs) / statistics.median(mit), 4))
    print(rung, "stored median_ms_per_target", dec["median_ms_per_target"])
    print(rung, "stored speedup_median_ratio", dec.get("speedup_median_ratio"))
    print(rung, "disagreements field", dec["disagreements"], "agree True", sum(1 for x in pt if x["agree"]), "agree False", sum(1 for x in pt if not x["agree"]))
    print(rung, "all_witnesses_group_verified field", dec["all_witnesses_group_verified"])
    print(rung, "witness_group_verified True count", sum(1 for x in pt if x.get("witness_group_verified")), "of", len(pt))
    print(rung, "max mitm_ops", max(x["mitm_ops"] for x in pt), "max pairs_pairs", max(x["pairs_pairs"] for x in pt))
    print(rung, "verdict_mix", dec.get("verdict_mix"), "per_target_ops", str(dec.get("per_target_ops"))[:200])
    fb = d["factor_base"]
    print(rung, "factor_base points", fb["points"], "dimension", fb["dimension"], "table_entries", fb["table_entries"], "retained_bytes", fb["retained_bytes"], "MiB", round(fb["retained_bytes"] / 2**20, 3))
    print(rung, "kinds", sorted(set(x["kind"] for x in pt)), "mitm_found True", sum(1 for x in pt if x["mitm_found"]), "pairs_found True", sum(1 for x in pt if x["pairs_found"]))
