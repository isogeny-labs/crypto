#!/usr/bin/env python3
"""Compare the item 01 replay against the recorded originals (replay/PLAN.md rule).

Stdlib only. Read-only: writes nothing, prints one JSON report on stdout.
Exit status 0 when every check passes (no mismatch), 1 otherwise.

Rule (PLAN.md, "Comparison rule"):
  * join key per row: (family, bits, idx, target, arm)
  * per replay file, the replayed key set must equal the recorded key set of the
    named recorded file (same row count, no extra, no missing, no duplicate key)
  * every field of a row is compared for equality after JSON parse, except the
    descriptive wall-time fields detail.wall_seconds, detail.wall_seconds_filter,
    detail.wall_seconds_wiedemann (host speed; reported, never pass/fail)
  * this covers every exact field PLAN.md lists (instance identity, solve result,
    costs, natives, counted detail fields, rho counters, control fields) and any
    field the rows carry that PLAN.md does not list: nothing is skipped
  * floats are compared by equality; a differing float is reported with its
    relative difference, not rounded into a match
  * instances.txt must be byte-identical (sha256 equal) to the recorded file
  * the gls-b40-i2.jsonl on disk must hash to the digest in receipt-card-r05.json
"""
import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ITEM = os.path.dirname(HERE)
REC = os.path.join(ITEM, "raw")
REP = os.path.join(HERE, "raw")

EXPECT_INSTANCES_SHA = "21d288f71ffdd49e17dc30221205fad4a17202520c4cd73d4411d85483b0438a"
EXPECT_GLS_B40_I2_SHA_PREFIX = "da3b4362"
INSTRUMENT = os.path.join(os.path.dirname(ITEM), "instruments", "blindspots_01_fp2_gaudry.rs")
EXPECT_INSTRUMENT_SHA = "0629a73c7ff443e70c6ce16c0872b98a5aebc5a71d25f7bff60156c8693ff559"

# card -> list of (replay file, recorded file)
CARDS = {
    "r01": [("torsion-b24.jsonl", "torsion-b24.jsonl"),
            ("gls-b24.jsonl", "gls-b24.jsonl"),
            ("prime-b24.jsonl", "prime-b24.jsonl"),
            ("control-b24.jsonl", "control-b24.jsonl")],
    "r02": [("torsion-b40-i0.jsonl", "torsion-b40-i0.jsonl"),
            ("prime-b40-i0.jsonl", "prime-b40-i0.jsonl")],
    "r03": [("gls-b40-i0.jsonl", "gls-b40-i0.jsonl"),
            ("rate-b40-i0.jsonl", "rate-b40-i0.jsonl")],
    "r04": [("torsion-b40-i2.jsonl", "torsion-b40-i2.jsonl"),
            ("prime-b40-i2.jsonl", "prime-b40-i2.jsonl")],
    "r05": [("gls-b40-i2.jsonl", "gls-b40-i2.jsonl")],
}

# Descriptive only (host speed). Paths into the row.
DESCRIPTIVE = {("detail", "wall_seconds"),
               ("detail", "wall_seconds_filter"),
               ("detail", "wall_seconds_wiedemann")}


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def load_rows(path):
    """Return (rows, problems). rows: dict key -> parsed row."""
    rows, problems = {}, []
    with open(path, "r", encoding="utf-8") as f:
        for ln, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                row = json.loads(line)
            except ValueError as e:
                problems.append("line %d unparsable: %s" % (ln, e))
                continue
            try:
                key = (row["family"], row["bits"], row["idx"], row["target"], row["arm"])
            except KeyError as e:
                problems.append("line %d lacks join field %s" % (ln, e))
                continue
            if key in rows:
                problems.append("line %d duplicate key %r" % (ln, key))
                continue
            rows[key] = row
    return rows, problems


def rel(a, b):
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return None
    d = abs(a - b)
    m = max(abs(a), abs(b))
    return 0.0 if m == 0 else d / m


def walk(rec, rep, path, out, desc):
    """Compare two parsed JSON values; append mismatches to out, descriptive to desc."""
    if path in DESCRIPTIVE:
        desc.append({"field": ".".join(path), "recorded": rec, "replayed": rep})
        return
    if isinstance(rec, dict) and isinstance(rep, dict):
        for k in sorted(set(rec) | set(rep)):
            p = path + (k,)
            if k not in rec:
                if p in DESCRIPTIVE:
                    desc.append({"field": ".".join(p), "recorded": None, "replayed": rep[k]})
                else:
                    out.append({"field": ".".join(p), "recorded": "<absent>", "replayed": rep[k]})
            elif k not in rep:
                if p in DESCRIPTIVE:
                    desc.append({"field": ".".join(p), "recorded": rec[k], "replayed": None})
                else:
                    out.append({"field": ".".join(p), "recorded": rec[k], "replayed": "<absent>"})
            else:
                walk(rec[k], rep[k], p, out, desc)
        return
    if isinstance(rec, list) and isinstance(rep, list):
        if len(rec) != len(rep):
            out.append({"field": ".".join(path) + ".<len>", "recorded": len(rec), "replayed": len(rep)})
        for i in range(min(len(rec), len(rep))):
            walk(rec[i], rep[i], path + ("[%d]" % i,), out, desc)
        return
    # leaves; bool is distinct from int in JSON, so compare type too
    same = (rec == rep) and (isinstance(rec, bool) == isinstance(rep, bool))
    if not same:
        m = {"field": ".".join(path), "recorded": rec, "replayed": rep}
        if isinstance(rec, float) or isinstance(rep, float):
            r = rel(rec, rep)
            if r is not None:
                m["relative_difference"] = r
        out.append(m)


def compare_file(replay_name, recorded_name):
    rep_path = os.path.join(REP, replay_name)
    rec_path = os.path.join(REC, recorded_name)
    rep, p1 = load_rows(rep_path)
    rec, p2 = load_rows(rec_path)
    res = {
        "replay_file": os.path.relpath(rep_path, ITEM),
        "replay_sha256": sha256_file(rep_path),
        "recorded_file": os.path.relpath(rec_path, ITEM),
        "recorded_sha256": sha256_file(rec_path),
        "recorded_rows": len(rec),
        "replayed_rows": len(rep),
        "parse_problems": ["replay: " + x for x in p1] + ["recorded: " + x for x in p2],
        "missing_in_replay": sorted(map(list, set(rec) - set(rep))),
        "extra_in_replay": sorted(map(list, set(rep) - set(rec))),
        "rows_compared": 0,
        "rows_equal": 0,
        "mismatches": [],
        "descriptive_wall": [],
    }
    for key in sorted(set(rec) & set(rep)):
        out, desc = [], []
        walk(rec[key], rep[key], (), out, desc)
        res["rows_compared"] += 1
        if out:
            for m in out:
                m["row_key"] = list(key)
                res["mismatches"].append(m)
        else:
            res["rows_equal"] += 1
        for d in desc:
            d["row_key"] = list(key)
            res["descriptive_wall"].append(d)
    res["ok"] = (not res["parse_problems"] and not res["missing_in_replay"]
                 and not res["extra_in_replay"] and not res["mismatches"]
                 and len(rec) == len(rep) and res["rows_compared"] == len(rec) > 0)
    return res


def main():
    report = {"schema": "blindspot01.replay-comparison.v1", "checks": {}, "cards": {}}
    ok = True

    # instrument identity
    ins = sha256_file(INSTRUMENT)
    report["checks"]["instrument_sha256"] = {
        "path": os.path.relpath(INSTRUMENT, os.path.dirname(os.path.dirname(ITEM))),
        "sha256": ins, "expected": EXPECT_INSTRUMENT_SHA, "ok": ins == EXPECT_INSTRUMENT_SHA}
    ok &= ins == EXPECT_INSTRUMENT_SHA

    # instances.txt byte-identical
    a = sha256_file(os.path.join(REC, "instances.txt"))
    b = sha256_file(os.path.join(REP, "instances.txt"))
    with open(os.path.join(REC, "instances.txt"), "rb") as f1, open(os.path.join(REP, "instances.txt"), "rb") as f2:
        same_bytes = f1.read() == f2.read()
    c = {"recorded_sha256": a, "replayed_sha256": b, "expected_sha256": EXPECT_INSTANCES_SHA,
         "byte_identical": same_bytes,
         "ok": same_bytes and a == b == EXPECT_INSTANCES_SHA}
    report["checks"]["instances.txt"] = c
    ok &= c["ok"]

    # receipts: status, output digests equal to files on disk
    receipt_ok = True
    for card, files in CARDS.items():
        rp = os.path.join(HERE, "receipt-card-%s.json" % card)
        with open(rp, "r", encoding="utf-8") as f:
            rcpt = json.load(f)
        outs = {}
        for cmd in rcpt["commands"]:
            for p, h in cmd.get("outputs", {}).items():
                outs[os.path.basename(p)] = h
        entry = {
            "card_id": rcpt["card_id"],
            "receipt": os.path.relpath(rp, ITEM),
            "receipt_sha256": sha256_file(rp),
            "card_sha256_in_receipt": rcpt["card_sha256"],
            "card_file_sha256_now": sha256_file(os.path.join(HERE, "card-%s.yaml" % card)),
            "commit": rcpt["commit"],
            "status": rcpt["status"],
            "commands": [(c["id"], c["exit_code"]) for c in rcpt["commands"]],
            "files": [],
        }
        s_ok = (rcpt["status"] == "completed" and all(c["exit_code"] == 0 for c in rcpt["commands"])
                and entry["card_sha256_in_receipt"] == entry["card_file_sha256_now"])
        for rep_name, rec_name in files:
            res = compare_file(rep_name, rec_name)
            res["receipt_output_sha256"] = outs.get(rep_name)
            res["receipt_digest_matches_disk"] = outs.get(rep_name) == res["replay_sha256"]
            s_ok &= res["receipt_digest_matches_disk"]
            if rep_name == "gls-b40-i2.jsonl":
                res["expected_sha256_prefix"] = EXPECT_GLS_B40_I2_SHA_PREFIX
                res["sha256_prefix_ok"] = res["replay_sha256"].startswith(EXPECT_GLS_B40_I2_SHA_PREFIX)
                s_ok &= res["sha256_prefix_ok"]
            entry["files"].append(res)
            ok &= res["ok"]
        entry["receipt_checks_ok"] = s_ok
        entry["rows_compared"] = sum(x["rows_compared"] for x in entry["files"])
        entry["mismatch_count"] = sum(len(x["mismatches"]) for x in entry["files"])
        receipt_ok &= s_ok
        report["cards"][card] = entry
    report["checks"]["receipts_and_digests_ok"] = receipt_ok
    ok &= receipt_ok

    report["total_rows_compared"] = sum(e["rows_compared"] for e in report["cards"].values())
    report["total_mismatches"] = sum(e["mismatch_count"] for e in report["cards"].values())
    report["status"] = "verified" if ok else "failed"
    json.dump(report, sys.stdout, indent=1, sort_keys=False)
    sys.stdout.write("\n")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
