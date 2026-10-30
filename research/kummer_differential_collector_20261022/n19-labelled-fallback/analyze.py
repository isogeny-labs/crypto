#!/usr/bin/env python3
"""Independent N19 workload replay and full-cold operation-cost analysis.

Run from the repository root after paired_labelled has produced all seven
collector and rho slices. This program deliberately reads raw records rather
than importing the candidate's arithmetic or the earlier Rust analyzer.
"""

import csv
import json
import subprocess
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
STUDY = ROOT / "research/kummer_differential_collector_20261022"
HERE = STUDY / "n19-labelled-fallback"
RAW = HERE / "results/paired"
SETUP = STUDY / "results"
BITS = (16, 20, 24, 28, 32, 36, 40)
CELLS = tuple((s, i, l) for s in (0, 0.5, 1) for i in (1, 10, 100, 1000)
              for l in (0, 0.1, 1))


def rows(path):
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def keyed(items, key, count, label):
    out = {}
    for item in items:
        ident = item[key]
        assert ident not in out, f"duplicate {label}: {ident}"
        out[ident] = item
    assert len(out) == count, f"{label}: {len(out)} != {count}"
    return out


def archived_rho(bits):
    archive = ROOT / "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst"
    member = f"sessions_projective_chart_v2/order-genus-{bits}/records.jsonl"
    data = subprocess.check_output(["tar", "--zstd", "-xOf", str(archive), member])
    out = {}
    for row in map(json.loads, data.splitlines()):
        if row["arm"] != "rho_neg":
            continue
        ident = row["workload"]["workload_id"]
        assert ident not in out, f"duplicate archived rho {bits}: {ident}"
        out[ident] = row
    assert len(out) == 180, f"archived rho {bits}: {len(out)} != 180"
    return out


def fp_add(x, y, p):
    return ((x[0] + y[0]) % p, (x[1] + y[1]) % p)


def fp_sub(x, y, p):
    return ((x[0] - y[0]) % p, (x[1] - y[1]) % p)


def fp_mul(x, y, p):
    return ((x[0] * y[0] - x[1] * y[1]) % p,
            (x[0] * y[1] + x[1] * y[0]) % p)


def fp_inv(x, p):
    inverse = pow((x[0] * x[0] + x[1] * x[1]) % p, -1, p)
    return (x[0] * inverse % p, -x[1] * inverse % p)


def unpack(x, p):
    return (x % p, x // p)


def ec_add(x, y, p, a):
    if x is None:
        return y
    if y is None:
        return x
    xx, yy = x
    ux, uy = y
    if xx == ux and fp_add(yy, uy, p) == (0, 0):
        return None
    if x == y:
        slope = fp_mul(fp_add(fp_mul((3, 0), fp_mul(xx, xx, p), p), a, p),
                       fp_inv(fp_mul((2, 0), yy, p), p), p)
    else:
        slope = fp_mul(fp_sub(uy, yy, p), fp_inv(fp_sub(ux, xx, p), p), p)
    rx = fp_sub(fp_sub(fp_mul(slope, slope, p), xx, p), ux, p)
    ry = fp_sub(fp_mul(slope, fp_sub(xx, rx, p), p), yy, p)
    return (rx, ry)


def source_replay(spec, target, scalar):
    p = spec["p"]
    assert spec["n"] == 2 and spec["modulus"] == 1 and p % 4 == 3
    a = unpack(spec["a"], p)
    base = (unpack(spec["gx"], p), unpack(spec["gy"], p))
    want = tuple(unpack(int(value, 16), p) for value in target)
    acc = None
    while scalar:
        if scalar & 1:
            acc = ec_add(acc, base, p, a)
        scalar >>= 1
        if scalar:
            base = ec_add(base, base, p, a)
    return acc == want


def add_counts(*sources):
    out = Counter()
    for source in sources:
        out["m"] += source.get("m", 0)
        out["s"] += source.get("s", 0)
        out["i"] += source.get("i", 0)
        out["logical"] += source.get("a", 0) + source.get("field_adds", 0)
    return out


def setup_cost(model, target):
    return add_counts(*(model[k] for k in (
        "root_ops", "root_extension_counts", "root_base_counts", "theta_ops",
        "torsion_ops", "torsion_extension_counts", "torsion_base_counts")),
        *(target[k] for k in ("transfer_ops", "extension_counts", "base_counts")))


def candidate_cost(row):
    cost = add_counts(*(row[k] for k in ("walk_ops", "recover_ops", "field_counts")))
    cost["logical"] += (row["graph_operations"] + 3 * row["attempts"]
                        + row["recover_success"] + row["walk_restarts"])
    if row["schema"] == "kummer-labelled-fallback/v1":
        # Validity tests and projective comparisons are separately recorded
        # native logical operations. Their field arithmetic is already in
        # field_counts and walk_ops, respectively.
        cost["logical"] += row["fallback_validity_checks"] + row["transition_checks"]
    return cost


def rho_cost(row):
    counter = row["counters"]
    return Counter(m=counter["field_multiplications"],
                   s=counter["field_squarings"], i=counter["field_inversions"],
                   logical=sum(value for key, value in counter.items()
                               if key.endswith("_uncharged"))), (
        counter["source_add_multiplications"], counter["source_add_squarings"],
        counter["source_add_inversions"])


def price(cost, cell, unit):
    sq, inv, native = cell
    denom = unit[0] + sq * unit[1] + inv * unit[2]
    assert denom > 0
    return (cost["m"] + sq * cost["s"] + inv * cost["i"]) / denom + native * cost["logical"]


def verify_identity(candidate, reference, source, setup, archived, bits):
    w = archived["workload"]
    spec = w["curve_spec"]
    ident = w["workload_id"]
    for row in (candidate, reference):
        assert row["workload_id"] == ident
        assert row["bits"] == bits and row["p"] == spec["p"] and row["r"] == spec["r"]
        assert row["target_index"] == w["target_index"]
        assert row["planted"] == w["planted"]
    assert source["workload_id"] == ident and source["bits"] == bits
    assert source["target_index"] == w["target_index"]
    assert source["p"] == spec["p"] and source["r"] == spec["r"]
    assert source["planted"] == w["planted"] and source["source_scalar_verified"]
    assert setup["workload_id"] == ident and setup["bits"] == bits
    assert setup["target_index"] == w["target_index"] and setup["p"] == spec["p"]
    assert setup["source_scalar_verified"]
    for field, idx in (("anchor", 1), ("target", 2)):
        assert setup[field] == source["cases"][idx]["coordinate_witness"]["jacobian_expected"]
    assert reference["schema"] == "kummer-paired-rho/v1"
    assert reference["same_process_as_candidate"] is True
    assert reference["algorithm_seed"] == int(archived["algorithm_seed"])
    assert reference["trial_cap"] == int(archived["method"]["params"]["trial_cap"])
    assert reference["archived_arithmetic_match"] is True
    assert reference["status"] == "verified" and reference["source_verified"] is True
    assert reference["recovered"] == w["planted"]
    assert source_replay(spec, w["target"], reference["recovered"]), ident
    assert candidate["schema"] == "kummer-labelled-fallback/v1"
    assert candidate["transition_validation_enabled"] is False
    assert candidate["transition_checks"] == 0
    assert candidate["fallback_validity_checks"] == 2 * candidate["fallback_steps"]
    assert candidate["fallback_failures"] <= candidate["fallback_steps"]
    assert candidate["fallback_steps"] <= candidate["attempts"]
    if candidate["status"] == "verified":
        assert candidate["source_verified"] is True
        assert candidate["recovered"] == w["planted"]
        assert source_replay(spec, w["target"], candidate["recovered"]), ident
    else:
        assert candidate["source_verified"] is False
    return ident


def write_csv(path, header, body):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=header, lineterminator="\n")
        writer.writeheader()
        writer.writerows(body)


def main():
    assert len(CELLS) == 36
    model = keyed(rows(SETUP / "cold-model-setup.jsonl"), "p", 21, "models")
    setups = keyed(rows(SETUP / "cold-target-setup.jsonl"), "workload_id", 1260, "target setups")
    assert all(row["torsion_actions_match"] and len(row["roots"]) == 5 for row in model.values())
    assert Counter(row["p"] for row in setups.values()) == {p: 60 for p in model}
    records = []
    replay = []
    for bits in BITS:
        source = keyed(rows(ROOT / f"research/kummer_coordinates_20261022/results/target-gate-{bits}.jsonl"),
                       "workload_id", 180, f"source {bits}")
        candidate = keyed(rows(RAW / f"collector/collector-{bits}.jsonl"),
                          "workload_id", 180, f"candidate {bits}")
        baseline = keyed(rows(SETUP / f"uncached/collector-{bits}.jsonl"),
                         "workload_id", 180, f"frozen baseline {bits}")
        reference = keyed(rows(RAW / f"rho/rho-{bits}.jsonl"),
                          "workload_id", 180, f"paired rho {bits}")
        archived = archived_rho(bits)
        assert source.keys() == candidate.keys() == baseline.keys() == reference.keys() == archived.keys()
        for ident in sorted(source):
            c, r, a = candidate[ident], reference[ident], archived[ident]
            b = baseline[ident]
            t = setups[ident]
            verify_identity(c, r, source[ident], t, a, bits)
            assert b["schema"] == "kummer-differential-collector/v1"
            for key in ("bits", "p", "r", "target_index", "planted", "workload_id"):
                assert b[key] == c[key], (ident, key)
            assert b["status"] == "verified" and b["source_verified"] is True
            assert b["recovered"] == c["planted"]
            assert source_replay(a["workload"]["curve_spec"], a["workload"]["target"], b["recovered"])
            assert model[c["p"]]["p"] == c["p"]
            rho, unit = rho_cost(r)
            online = candidate_cost(c)
            setup = setup_cost(model[c["p"]], t)
            cold = online + setup
            baseline_online = candidate_cost(b)
            baseline_cold = baseline_online + setup
            status = c["status"]
            record = dict(schema="kummer-n19-cost-record/v1", bits=bits, p=c["p"],
                          target_index=c["target_index"], workload_id=ident,
                          candidate_status=status, candidate_verified=status == "verified",
                          rho_verified=True, candidate_online=dict(online),
                          candidate_cold=dict(cold), baseline_online=dict(baseline_online),
                          baseline_cold=dict(baseline_cold), baseline_verified=True,
                          rho=dict(rho), source_add_unit=unit,
                          fallback_steps=c["fallback_steps"], fallback_failures=c["fallback_failures"],
                          transition_validation_enabled=c["transition_validation_enabled"])
            records.append(record)
            replay.append(dict(workload_id=ident, bits=bits, candidate_status=status,
                               candidate_scalar_replayed=status == "verified",
                               rho_scalar_replayed=True, candidate_recovered=c["recovered"],
                               rho_recovered=r["recovered"]))
    assert len(records) == len(replay) == 1260
    target_rows = []
    grid_rows = []
    for cell in CELLS:
        sq, inv, logical = cell
        for record in records:
            candidate_online = price(record["candidate_online"], cell, record["source_add_unit"])
            candidate_cold = price(record["candidate_cold"], cell, record["source_add_unit"])
            baseline_cold = price(record["baseline_cold"], cell, record["source_add_unit"])
            rho = price(record["rho"], cell, record["source_add_unit"])
            verified = record["candidate_verified"]
            target_rows.append(dict(schema="kummer-n19-target-cost/v1", bits=record["bits"],
                p=record["p"], target_index=record["target_index"], workload_id=record["workload_id"],
                S_per_M=sq, I_per_M=inv, native_GAE_per_logical=logical,
                candidate_status=record["candidate_status"], candidate_verified=verified,
                baseline_verified=True, rho_verified=True, candidate_online_GAE=candidate_online,
                candidate_cold_GAE=candidate_cold, rho_GAE=rho,
                baseline_cold_GAE=baseline_cold, baseline_cold_over_rho=baseline_cold/rho,
                candidate_cold_over_baseline=candidate_cold/baseline_cold if verified else None,
                candidate_online_over_rho=candidate_online/rho if verified else None,
                candidate_cold_over_rho=candidate_cold/rho if verified else None,
                candidate_win=bool(verified and candidate_cold < rho),
                candidate_loss=bool(verified and candidate_cold >= rho)))
        for slice_name, subset in (("all", records), *( (f"bits{bits}", [r for r in records if r["bits"] == bits]) for bits in BITS)):
            matched = [row for row in target_rows if row["S_per_M"] == sq and row["I_per_M"] == inv
                       and row["native_GAE_per_logical"] == logical and
                       (slice_name == "all" or row["bits"] == int(slice_name[4:]))]
            assert len(matched) == len(subset)
            ok = [row for row in matched if row["candidate_verified"]]
            grid_rows.append(dict(schema="kummer-n19-price-grid/v1", slice=slice_name,
                targets=len(matched), verified=len(ok), failures=len(matched)-len(ok),
                S_per_M=sq, I_per_M=inv, native_GAE_per_logical=logical,
                candidate_cold_GAE=sum(x["candidate_cold_GAE"] for x in matched),
                baseline_cold_GAE=sum(x["baseline_cold_GAE"] for x in matched),
                rho_GAE=sum(x["rho_GAE"] for x in matched),
                candidate_cold_over_rho=(sum(x["candidate_cold_GAE"] for x in ok) /
                                         sum(x["rho_GAE"] for x in ok) if len(ok) == len(matched) else None),
                candidate_cold_over_baseline=(sum(x["candidate_cold_GAE"] for x in ok) /
                    sum(x["baseline_cold_GAE"] for x in ok) if len(ok) == len(matched) else None),
                baseline_cold_over_rho=(sum(x["baseline_cold_GAE"] for x in matched) /
                                         sum(x["rho_GAE"] for x in matched)),
                candidate_wins=sum(x["candidate_win"] for x in matched),
                candidate_losses=sum(x["candidate_loss"] for x in matched),
                null_ratios=len(matched)-len(ok)))
    assert len(target_rows) == 1260 * 36 and len(grid_rows) == 8 * 36
    out = HERE / "results/analysis"
    out.mkdir(parents=True, exist_ok=True)
    with (out / "cost-records.jsonl").open("w") as stream:
        for record in records:
            stream.write(json.dumps(record, sort_keys=True) + "\n")
    with (out / "replay.jsonl").open("w") as stream:
        for record in replay:
            stream.write(json.dumps(record, sort_keys=True) + "\n")
    write_csv(out / "target-price.csv", target_rows[0], target_rows)
    write_csv(out / "price-grid.csv", grid_rows[0], grid_rows)
    status = Counter(record["candidate_status"] for record in records)
    summary = dict(schema="kummer-n19-analysis/v1", targets=1260, price_cells=36,
                   slice_price_rows=8*36, status=dict(status),
                   candidate_verified=status.get("verified", 0), baseline_verified=1260,
                   rho_verified=1260,
                   fallback_steps=sum(x["fallback_steps"] for x in records),
                   fallback_failures=sum(x["fallback_failures"] for x in records),
                   transition_validation_enabled=False,
                   cold_model_charges=1260, cold_models=21,
                   ratios_null_if_any_candidate_failure=True,
                   cost_boundary="full cold per target: model and transfer setup, online collector, native validation, source verification")
    (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    print(json.dumps(summary, sort_keys=True))


if __name__ == "__main__":
    main()
