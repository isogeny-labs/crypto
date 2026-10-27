#!/usr/bin/env python3
"""Compare the replay outputs with the recorded rows; write the replay certificate.

    python3 -I check_replay.py --commit <snapshot> --out ../replay-certificate.json

Recorded rows are read with `git show <snapshot>:<path>`.  For every replayed
corpus system (the nonce-sampled ones and the rest of each replayed group):
delta, n, p, the arms present, every arm's solution count, agreement, the
planted-point finding and the fast give-up must equal the recorded row.  The
counted `muls` are compared exactly where the replay gives the system the same
index si as its card (the arm seed is 0x5EED ^ si); elsewhere the randomized
choices differ by construction and the comparison is reported, not required.
For jv_cover, every deterministic counter of the replayed (p=53, seed=1) row
must equal the recorded R2-D2 (JV_LIN=fast) row, and the sparse replay the
R2-D1 row; wall-clock fields are reported, not compared.
"""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path

REPO = Path("/home/user/crypto")
STUDY = "research/postgb_linear_stage_20261026"
HERE = REPO / STUDY / "validation/replay"
BENCH = ["R2-L1", "R2-L2", "R2-L3", "R2-L4", "R2-L5", "R2-L6"]
TIMING = {"f4_ms", "lin_ms", "wall_ms"}


def show(commit, rel):
    return json.loads(subprocess.run(["git", "show", f"{commit}:{rel}"], cwd=REPO, capture_output=True,
                                     check=True).stdout)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def arm(row, name):
    return next((a for a in row["arms"] if a["arm"] == name), None)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--commit", required=True)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    sample = json.loads((HERE / "sample.json").read_text())
    journal = json.loads((HERE / "journal.json").read_text())
    nonce = (HERE / "nonce.txt").read_text().strip()
    recorded = {}
    for card in BENCH:
        for si, row in enumerate(show(a.commit, f"{STUDY}/runs/{card}.json")["rows"]):
            recorded[(row["family"], row["label"])] = {"card": card, "si": si, **row}
    sampled = {(s["family"], s["label"]) for s in sample["sample"]}

    runs, all_ok = [], True
    sampled_seen = set()
    for entry in journal:
        name = entry["name"]
        out_json = REPO / [p for p in entry["outputs"] if p.endswith(".json")][0]
        run = {"name": name, "argv": entry["argv"], "cwd": entry["cwd"], "env": entry["env"],
               "binary_sha256": entry["binary_sha256"], "exit_code": entry["exit_code"],
               "seconds": entry["seconds"], "outputs": entry["outputs"]}
        ok = entry["exit_code"] == 0 and out_json.exists() and sha(out_json) == entry["outputs"][str(out_json.relative_to(REPO))]
        if name.startswith("bench_"):
            doc = json.loads(out_json.read_text())
            run["skipped"] = doc["skipped"]
            ok &= doc["skipped"] == []
            systems = []
            for si, row in enumerate(doc["rows"]):
                key = (row["family"], row["label"])
                rec = recorded.get(key)
                item = {"family": key[0], "label": key[1], "sampled": key in sampled,
                        "recorded_card": rec and rec["card"], "si_recorded": rec and rec["si"], "si_replay": si}
                if rec is None:
                    item["matched"] = False
                    item["mismatch"] = ["no recorded row"]
                    systems.append(item)
                    ok = False
                    continue
                if key in sampled:
                    sampled_seen.add(key)
                mism = []
                for f in ("n", "p", "delta", "agree", "planted_found"):
                    if row[f] != rec[f]:
                        mism.append(f"{f}: replay {row[f]} recorded {rec[f]}")
                if [x["arm"] for x in row["arms"]] != [x["arm"] for x in rec["arms"]]:
                    mism.append("arms present differ")
                for x in rec["arms"]:
                    y = arm(row, x["arm"])
                    if y is None:
                        continue
                    if y["sols"] != x["sols"]:
                        mism.append(f"{x['arm']}.sols: replay {y['sols']} recorded {x['sols']}")
                    if y["gave_up"] != x["gave_up"]:
                        mism.append(f"{x['arm']}.gave_up: replay {y['gave_up']} recorded {x['gave_up']}")
                    if y["fallback"] != x["fallback"]:
                        mism.append(f"{x['arm']}.fallback: replay {y['fallback']} recorded {x['fallback']}")
                muls_equal = {x["arm"]: arm(row, x["arm"])["muls"] == x["muls"] for x in rec["arms"]}
                item["muls_compared_exactly"] = rec["si"] == si
                item["muls_equal"] = muls_equal
                if rec["si"] == si and not all(muls_equal.values()):
                    mism.append(f"muls differ at the same si: {muls_equal}")
                item["sols"] = row["arms"][0]["sols"]
                item["agree"] = row["agree"]
                item["planted_found"] = row["planted_found"]
                item["delta"] = row["delta"]
                item["fast_gave_up"] = arm(row, "fast")["gave_up"]
                item["matched"] = not mism
                if mism:
                    item["mismatch"] = mism
                    ok = False
                systems.append(item)
            run["systems"] = systems
            run["matched"] = (f"{sum(s['matched'] for s in systems)}/{len(systems)} systems: n, p, delta, arms, "
                              f"per-arm sols, agree, planted_found, gave_up, fallback; muls exact on "
                              f"{sum(1 for s in systems if s.get('muls_compared_exactly'))} systems at the card's si")
        else:
            mode = entry["env"]["JV_LIN"]
            card = "R2-D2" if mode == "fast" else "R2-D1"
            rec = [r for r in show(a.commit, f"{STUDY}/runs/{card}.json") if r["p"] == 53 and r["seed"] == 1][0]
            row = [r for r in json.loads(out_json.read_text()) if r["p"] == 53 and r["seed"] == 1]
            ok &= len(row) == 1
            row = row[0]
            fields = sorted(set(rec) | set(row))
            diffs = {f: [row.get(f), rec.get(f)] for f in fields if f not in TIMING and row.get(f) != rec.get(f)}
            run["compared_with"] = f"{STUDY}/runs/{card}.json p=53 seed=1"
            run["required"] = {f: {"replay": row.get(f), "recorded": rec.get(f), "equal": row.get(f) == rec.get(f)}
                               for f in ("solved", "correct", "lin_muls", "relations")}
            run["deterministic_fields_compared"] = len([f for f in fields if f not in TIMING])
            run["deterministic_fields_differing"] = diffs
            run["timing_fields_not_compared"] = {f: {"replay": row.get(f), "recorded": rec.get(f)} for f in TIMING}
            run["rho_correct"] = [x["correct"] for x in row["rho"]]
            ok &= all(v["equal"] for v in run["required"].values()) and row["solved"] and row["correct"]
            ok &= not diffs
            run["matched"] = ("solved, correct, lin_muls, relations and all "
                              f"{run['deterministic_fields_compared']} non-timing fields equal the recorded row"
                              if not diffs else f"differing fields: {sorted(diffs)}")
        run["ok"] = bool(ok)
        all_ok &= bool(ok)
        runs.append(run)

    missing = sorted(sampled - sampled_seen)
    all_ok &= not missing
    cert = {
        "schema": "postgb-la-replay-certificate/v1",
        "status": "verified" if all_ok else "failed",
        "independent": True,
        "independence": "replayed by the validator (not the producer) with the pinned binaries; nonce drawn by "
                        "the validator from /dev/urandom after the snapshot commit; comparison code written by "
                        "the validator",
        "scalars_verified": False,
        "scalar_check_used": (
            "Not independently verified. The pinned jv_cover prints and writes neither the solved logarithm nor "
            "the instance (alpha, G, Q, l). The check used is the binary's own: after the linear algebra it "
            "tests dd*G == Q on E(F_{p^6}) (EllE::mul) and sets correct = (dd == planted d) "
            "(src/cryptanalysis/jv_cover.rs, run_cover_dlp, at the run commit). The replay reproduced "
            "solved=true and correct=true, and all four same-process rho runs correct=true, with every "
            "deterministic counter equal to the recorded row."),
        "nonce": nonce,
        "nonce_drawn_utc": (HERE / "nonce.drawn_at").read_text().strip(),
        "snapshot_commit": a.commit,
        "sampling": {
            "procedure": "validation/replay/sample.py: within each family, systems of R2-L1..R2-L6 ordered by "
                         "sha256(nonce:card:family:label); the first k taken",
            "k_per_family": sample["k_per_family"],
            "sampled_systems": [{"card": s["card"], "family": s["family"], "label": s["label"]}
                                for s in sample["sample"]],
            "sampled_not_replayed": missing,
            "reached_by": "one postgb_la_bench run per (family, p[, shape]) group with the card's --per 8 and "
                          "--reps 1; every system of the group is compared, the sampled ones are flagged",
        },
        "pins": {"postgb_la_bench": "c5612e2eb33393ca4d446b482685179a8418ff9d52682dd70c53d62f0e5ba1d3",
                 "jv_cover": "afa830fc99087f12978f1e072c16b3f6e164c99b2b15e17fd6a3a7cc92bb050f"},
        "runs": runs,
    }
    Path(a.out).write_text(json.dumps(cert, indent=1) + "\n")
    print(cert["status"], "missing", missing)
    for r in runs:
        print(r["name"], "ok" if r["ok"] else "FAIL", r["matched"])
        for s in r.get("systems", []):
            if not s["matched"]:
                print("   MISMATCH", s)


if __name__ == "__main__":
    main()
