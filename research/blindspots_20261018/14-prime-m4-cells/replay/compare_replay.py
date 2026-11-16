#!/usr/bin/env python3
"""Replay comparison for blind-spot item 14 (prime-field m=4 cells).

Implements replay/PLAN.md ("What must match exactly") with the standard library only.

Rule, applied to every pair of files:
  * parse both files as JSON lines, select the rows named in PLAN.md (whole file, or the
    sliced subset for the two partial replays), require the same row count as PLAN.md;
  * delete every key named `ms` (at any depth) from each row;
  * require exact equality, row by row in file order, of every remaining key: the keys
    PLAN.md lists for the row kind must be present in both rows, and every other key must
    also be equal (a stricter reading of the plan; nothing is excluded but `ms`);
  * no tolerance of any kind; ints, floats, strings, booleans and nulls compare with ==
    and must also have the same JSON type.
Selftest: replay stdout and recorded stdout have the same line count; after removing timing
text (`in <x>s`, `<n> ms`, `(<x>s)`) every line is equal; the last line reads `SELFTEST OK`.
Receipts: every command has exit_code 0, status `completed`, and receipt card_sha256 equals
the sha256 of the card file; every receipt-listed output hash equals the file on disk.
Instrument: sha256 equals the pinned value, on disk and at each receipt commit.

A `status` difference is reported as a mismatch; if either side's status is `budget-deadline`
the cause is recorded as the wall-clock deadline, and it is not excused.

Usage: compare_replay.py [--json OUT] [--certificate OUT]
Exit status 0 only if every check passed.
"""
import hashlib
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
CELLS = os.path.dirname(HERE)
REPO = os.path.abspath(os.path.join(CELLS, "..", "..", ".."))
REL = os.path.relpath(CELLS, REPO)  # research/blindspots_20261018/14-prime-m4-cells
RAW = os.path.join(CELLS, "raw")
RRAW = os.path.join(HERE, "raw")
INSTRUMENT = "research/blindspots_20261018/instruments/blindspots_14_prime_m4.rs"
INSTRUMENT_SHA = "f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376"
SCRIPT_REL = REL + "/replay/compare_replay.py"

# Keys PLAN.md lists, per row kind (and arm).
LISTED = {
    "cell": ["bits", "p", "a", "b", "n", "B", "bx", "signed", "seed"],
    "rho": ["bits", "run", "ok", "setup_gadd", "walks", "counts", "gae", "native", "n",
            "sqrt_pi_n_over_2"],
    ("setup", "a"): ["bits", "B", "bx", "entries", "counts"],
    ("tdec", "a"): ["bits", "B", "bx", "target", "tkind", "hit", "valid",
                    "crosscheck_decomposable", "planted", "counts", "native"],
    ("setup", "b"): ["counts", "note"],
    ("tdec", "b"): ["target", "tkind", "status", "dim", "brute_roots", "planted_vanishes",
                    "steps", "max_degree", "max_rows", "max_cols", "peak_nnz", "zero_rows",
                    "counts", "native"],
    "fullic": ["arm", "bits", "bx", "run", "ok", "attempts", "hits", "relations", "rank",
               "cols", "setup", "online", "verify", "total", "gae_total", "native_total",
               "n", "note"],
}


def sel_all(rows):
    return rows


def sel_first_two(rows):
    return rows[:2]


def sel_bx8_t0(rows):
    out = [r for r in rows if r.get("kind") in ("cell", "setup")]
    out += [r for r in rows if r.get("kind") == "tdec" and r.get("target") == 0]
    # file order: cell, setup, tdec target 0
    return out


# (card, command id, replay file, recorded file, selector, expected rows, plan description)
PAIRS = [
    ("r01", "rho-b16", "rho-b16.jsonl", "rho-b16.jsonl", sel_all, 16, "16 rho rows"),
    ("r01", "rho-b28", "rho-b28.jsonl", "rho-b28.jsonl", sel_all, 16, "16 rho rows"),
    ("r01", "tdec-a-b16", "tdec-a-b16.jsonl", "tdec-a-b16.jsonl", sel_all, 22, "cell, setup, 16 random + 4 planted tdec"),
    ("r01", "tdec-a-b28", "tdec-a-b28.jsonl", "tdec-a-b28.jsonl", sel_all, 22, "cell, setup, 16 random + 4 planted tdec"),
    ("r01", "tdec-a-b16-bx3", "tdec-a-b16-bx3.jsonl", "tdec-a-b16-bx3.jsonl", sel_all, 22, "cell, setup, 16 random + 4 planted tdec"),
    ("r01", "fullic-a-b16", "fullic-a-b16.jsonl", "fullic-a-b16.jsonl", sel_all, 16, "16 fullic rows"),
    ("r01", "fullic-a-b28", "fullic-a-b28.jsonl", "fullic-a-b28.jsonl", sel_all, 16, "16 fullic rows"),
    ("r01", "fullic-a-b16-bx3", "fullic-a-b16-bx3.jsonl", "fullic-a-b16-bx3.jsonl", sel_all, 16, "16 fullic rows"),
    ("r02", "tdec-b-b16-bx3", "tdec-b-b16-bx3.jsonl", "tdec-b-b16-bx3.jsonl", sel_all, 20, "cell, setup, 16 random + 2 planted tdec"),
    ("r02", "tdec-b-b16-bx4", "tdec-b-b16-bx4.jsonl", "tdec-b-b16-bx4.jsonl", sel_all, 20, "cell, setup, 16 random + 2 planted tdec"),
    ("r02", "tdec-b-b16-bx5", "tdec-b-b16-bx5.jsonl", "tdec-b-b16-bx5.jsonl", sel_all, 20, "cell, setup, 16 random + 2 planted tdec"),
    ("r02", "fullic-b-b16-bx3", "fullic-b-b16-bx3.jsonl", "fullic-b-b16-bx3.jsonl", sel_all, 16, "16 fullic rows"),
    ("r02", "fullic-b-b16-bx4-r00-02", "fullic-b-b16-bx4-r00-02.jsonl", "fullic-b-b16-bx4-r00-06.jsonl", sel_first_two, 2, "first two rows (run 0, 1) of the recorded r00-06 file"),
    ("r03", "tdec-b-b28-bx63", "tdec-b-b28-bx63.jsonl", "tdec-b-b28-bx63.jsonl", sel_all, 18, "cell, setup, 16 random tdec"),
    ("r03", "tdec-b-b16-bx8-t00-01", "tdec-b-b16-bx8-t00-01.jsonl", "tdec-b-b16-bx8-t00-05.jsonl", sel_bx8_t0, 3, "cell, setup, tdec target 0 of the recorded t00-05 file"),
]

CARDS = ["r01", "r02", "r03"]


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def rel(path):
    return os.path.relpath(path, REPO)


def read_rows(path):
    rows = []
    with open(path) as f:
        for ln, line in enumerate(f, 1):
            line = line.rstrip("\n")
            if not line.strip():
                continue
            rows.append(json.loads(line))
    return rows


def drop_ms(x):
    if isinstance(x, dict):
        return {k: drop_ms(v) for k, v in x.items() if k != "ms"}
    if isinstance(x, list):
        return [drop_ms(v) for v in x]
    return x


def flatten(x, prefix=""):
    """Leaf path -> value, with the JSON type kept for the comparison."""
    out = {}
    if isinstance(x, dict):
        for k, v in x.items():
            out.update(flatten(v, f"{prefix}.{k}" if prefix else k))
    elif isinstance(x, list):
        for i, v in enumerate(x):
            out.update(flatten(v, f"{prefix}[{i}]"))
    else:
        out[prefix] = x
    return out


def same(a, b):
    return type(a) is type(b) and a == b


def row_label(r):
    k = r.get("kind")
    if k in ("fullic", "rho"):
        return f"{k} run {r.get('run')}"
    if k == "tdec":
        return f"tdec target {r.get('target')}"
    return k


def compare_rows(replay, recorded):
    """Return (mismatch list, count of listed keys compared)."""
    mism = []
    listed_compared = 0
    for i, (a, b) in enumerate(zip(replay, recorded)):
        a2, b2 = drop_ms(a), drop_ms(b)
        kind = a2.get("kind")
        base = {"row_index": i, "row": row_label(a2)}
        if a2.get("kind") != b2.get("kind"):
            mism.append({**base, "key": "kind", "replay": a2.get("kind"), "recorded": b2.get("kind")})
            continue
        lk = LISTED.get(kind) if kind in ("cell", "rho", "fullic") else LISTED.get((kind, a2.get("arm")))
        if lk is None:
            mism.append({**base, "key": "(kind)", "replay": kind, "recorded": b2.get("kind"),
                         "cause": "row kind/arm not in PLAN.md key list"})
            continue
        for k in lk:
            if k not in a2 or k not in b2:
                mism.append({**base, "key": k, "replay": a2.get(k, "<absent>"),
                             "recorded": b2.get(k, "<absent>"), "cause": "listed key absent"})
            else:
                listed_compared += 1
        fa, fb = flatten(a2), flatten(b2)
        for k in sorted(set(fa) | set(fb)):
            va, vb = fa.get(k, "<absent>"), fb.get(k, "<absent>")
            if k not in fa or k not in fb or not same(va, vb):
                m = {**base, "key": k, "replay": va, "recorded": vb}
                if k == "status":
                    if "budget-deadline" in (va, vb):
                        m["cause"] = "status differs because of the wall-clock deadline (budget-deadline); reported as a mismatch, not excused"
                    else:
                        m["cause"] = "status differs"
                mism.append(m)
    return mism, listed_compared


def strip_timing(line):
    line = re.sub(r"\s*\(\d+(?:\.\d+)?s\)", "", line)
    line = re.sub(r"\bin \d+(?:\.\d+)?s\b", "in <t>", line)
    line = re.sub(r"\b\d+(?:\.\d+)? ms\b", "<t> ms", line)
    return line


def compare_selftest():
    rep_path = os.path.join(RRAW, "card-r01-selftest.stdout.log")
    rec_path = os.path.join(RAW, "card-01-selftest.stdout.log")
    rep = open(rep_path).read().split("\n")
    rec = open(rec_path).read().split("\n")
    # drop the single trailing empty element produced by the final newline
    if rep and rep[-1] == "":
        rep = rep[:-1]
    if rec and rec[-1] == "":
        rec = rec[:-1]
    mism = []
    if len(rep) != len(rec):
        mism.append({"key": "line_count", "replay": len(rep), "recorded": len(rec)})
    for i, (a, b) in enumerate(zip(rep, rec)):
        sa, sb = strip_timing(a), strip_timing(b)
        if sa != sb:
            mism.append({"key": f"line {i + 1}", "replay": a, "recorded": b,
                         "replay_stripped": sa, "recorded_stripped": sb})
    last_ok = bool(rep) and strip_timing(rep[-1]).strip() == "SELFTEST OK" and rep[-1].startswith("SELFTEST OK")
    if not last_ok:
        mism.append({"key": "last_line", "replay": rep[-1] if rep else None, "recorded": "SELFTEST OK ..."})
    return {
        "replay_file": rel(rep_path), "replay_sha256": sha256_file(rep_path),
        "reproduces_recorded_file": rel(rec_path), "recorded_sha256": sha256_file(rec_path),
        "comparison": "line-equal after stripping timing text; last line SELFTEST OK",
        "lines_recorded": len(rec), "lines_replayed": len(rep), "rows_compared": len(rep),
        "last_line_replay": rep[-1] if rep else None, "last_line_recorded": rec[-1] if rec else None,
        "mismatches": mism,
    }


def git_show_sha(commit, path):
    try:
        data = subprocess.run(["git", "-C", REPO, "show", f"{commit}:{path}"],
                              capture_output=True, check=True).stdout
    except Exception as e:  # noqa: BLE001
        return f"ERROR: {e}"
    return hashlib.sha256(data).hexdigest()


def check_receipt(card):
    card_path = os.path.join(HERE, f"card-{card}.yaml")
    receipt_path = os.path.join(RRAW, f"receipt-card-{card}.json")
    rec = json.load(open(receipt_path))
    problems = []
    card_sha = sha256_file(card_path)
    if rec.get("card_sha256") != card_sha:
        problems.append(f"receipt card_sha256 {rec.get('card_sha256')} != card file sha256 {card_sha}")
    if rec.get("status") != "completed":
        problems.append(f"receipt status is {rec.get('status')!r}, not 'completed'")
    cmds = rec.get("commands", [])
    if rec.get("commands_total") != len(cmds):
        problems.append(f"commands_total {rec.get('commands_total')} != {len(cmds)} commands listed")
    for c in cmds:
        if c.get("exit_code") != 0:
            problems.append(f"command {c.get('id')}: exit_code {c.get('exit_code')!r}")
        if c.get("timed_out"):
            problems.append(f"command {c.get('id')}: timed_out")
    out_problems = []
    out_checked = 0
    for c in cmds:
        for p, h in (c.get("outputs") or {}).items():
            out_checked += 1
            full = os.path.join(REPO, p)
            if not os.path.exists(full):
                out_problems.append(f"{p}: missing")
            elif sha256_file(full) != h:
                out_problems.append(f"{p}: file sha256 {sha256_file(full)} != receipt {h}")
    problems += out_problems
    instr = git_show_sha(rec.get("commit", ""), INSTRUMENT)
    if instr != INSTRUMENT_SHA:
        problems.append(f"instrument sha256 at commit {rec.get('commit')} is {instr}")
    return {
        "card_id": rec.get("card_id"),
        "card": rel(card_path),
        "card_sha256": card_sha,
        "receipt": rel(receipt_path),
        "receipt_sha256": sha256_file(receipt_path),
        "receipt_card_sha256": rec.get("card_sha256"),
        "commit": rec.get("commit"),
        "status": rec.get("status"),
        "commands": [{"id": c.get("id"), "exit_code": c.get("exit_code")} for c in cmds],
        "receipt_output_hashes_checked": out_checked,
        "instrument_sha256_at_commit": instr,
        "problems": problems,
    }


def main(argv):
    json_out = cert_out = None
    i = 1
    while i < len(argv):
        if argv[i] == "--json":
            json_out = argv[i + 1]
            i += 2
        elif argv[i] == "--certificate":
            cert_out = argv[i + 1]
            i += 2
        else:
            print(f"unknown argument {argv[i]}", file=sys.stderr)
            return 2

    failures = []

    instr_disk = sha256_file(os.path.join(REPO, INSTRUMENT))
    if instr_disk != INSTRUMENT_SHA:
        failures.append(f"instrument sha256 on disk {instr_disk} != {INSTRUMENT_SHA}")

    receipts = {c: check_receipt(c) for c in CARDS}
    for c, r in receipts.items():
        for p in r["problems"]:
            failures.append(f"receipt {c}: {p}")

    files = {c: [] for c in CARDS}
    total_rows = 0
    total_mism = 0
    total_listed = 0
    for card, cid, rep_name, rec_name, sel, expected, desc in PAIRS:
        rep_path = os.path.join(RRAW, rep_name)
        rec_path = os.path.join(RAW, rec_name)
        rep_all = read_rows(rep_path)
        rec_all = read_rows(rec_path)
        rep_rows = sel(rep_all)
        rec_rows = sel(rec_all)
        entry = {
            "command_id": cid,
            "replay_file": rel(rep_path), "replay_sha256": sha256_file(rep_path),
            "reproduces_recorded_file": rel(rec_path), "recorded_sha256": sha256_file(rec_path),
            "plan_rows": desc, "rows_expected_by_plan": expected,
            "rows_recorded_total": len(rec_all), "rows_replayed_total": len(rep_all),
            "rows_recorded_selected": len(rec_rows), "rows_replayed_selected": len(rep_rows),
        }
        mism = []
        if len(rep_rows) != expected or len(rec_rows) != expected:
            mism.append({"key": "row_count", "replay": len(rep_rows), "recorded": len(rec_rows),
                         "expected_by_plan": expected})
        if sel is sel_first_two:
            runs = [r.get("run") for r in rec_rows]
            if runs != [0, 1]:
                mism.append({"key": "selection", "recorded": runs, "expected": [0, 1]})
        if sel is sel_bx8_t0:
            order = [(r.get("kind"), r.get("target")) for r in rep_rows]
            if order != [("cell", None), ("setup", None), ("tdec", 0)]:
                mism.append({"key": "selection", "replay": order})
        if len(rep_rows) == len(rec_rows):
            m, listed = compare_rows(rep_rows, rec_rows)
            mism += m
            entry["listed_key_values_compared"] = listed
            total_listed += listed
        entry["rows_compared"] = min(len(rep_rows), len(rec_rows))
        entry["rows_equal"] = entry["rows_compared"] - len({m["row_index"] for m in mism if "row_index" in m})
        entry["mismatches"] = mism
        if sel is not sel_all:
            entry["comparison"] = "subset: " + desc
        total_rows += entry["rows_compared"]
        total_mism += len(mism)
        files[card].append(entry)
        for m in mism:
            failures.append(f"{rep_name} vs {rec_name}: {m}")

    selftest = compare_selftest()
    files["r01"].insert(0, {**selftest, "command_id": "selftest"})
    total_rows += selftest["rows_compared"]
    total_mism += len(selftest["mismatches"])
    for m in selftest["mismatches"]:
        failures.append(f"selftest: {m}")

    result = {
        "instrument": {"path": INSTRUMENT, "sha256_on_disk": instr_disk, "expected": INSTRUMENT_SHA},
        "receipts": receipts,
        "files": files,
        "total_rows_compared": total_rows,
        "total_listed_key_values_compared": total_listed,
        "total_mismatches": total_mism,
        "failures": failures,
        "verified": not failures,
    }

    if json_out:
        with open(json_out, "w") as f:
            json.dump(result, f, indent=1, sort_keys=False)
            f.write("\n")

    # human summary
    print(f"instrument sha256 on disk: {instr_disk} ({'OK' if instr_disk == INSTRUMENT_SHA else 'MISMATCH'})")
    for c in CARDS:
        r = receipts[c]
        print(f"receipt {c}: commit {r['commit']} status {r['status']} commands {len(r['commands'])} "
              f"all exit 0: {all(x['exit_code'] == 0 for x in r['commands'])} "
              f"card sha match: {r['card_sha256'] == r['receipt_card_sha256']} problems: {len(r['problems'])}")
    for c in CARDS:
        for e in files[c]:
            print(f"{c} {e['command_id']}: rows compared {e['rows_compared']} mismatches {len(e['mismatches'])}")
    print(f"TOTAL rows compared {total_rows}, listed key values compared {total_listed}, mismatches {total_mism}")
    for f_ in failures:
        print("FAIL:", f_)
    print("VERIFIED" if not failures else "FAILED")

    if cert_out:
        write_certificate(cert_out, result)
    return 0 if not failures else 1


def write_certificate(path, res):
    if os.path.exists(path):
        print(f"refusing to overwrite {path}", file=sys.stderr)
        return
    runs = []
    for c in CARDS:
        r = res["receipts"][c]
        runs.append({
            "card_id": r["card_id"],
            "card": r["card"],
            "card_sha256": r["card_sha256"],
            "receipt": r["receipt"],
            "receipt_sha256": r["receipt_sha256"],
            "commit": r["commit"],
            "status": r["status"],
            "commands": r["commands"],
            "files": res["files"][c],
            "rows_compared": sum(e["rows_compared"] for e in res["files"][c]),
            "mismatches": [m for e in res["files"][c] for m in e["mismatches"]],
        })
    verified = res["verified"]
    cert = {
        "schema": "replay-certificate/v1",
        "item": "BLINDSPOT-14",
        "evidence_directory": "benchmarks/20261018-blindspots-prime-m4-cells",
        "status": "verified" if verified else "failed",
        "independent": True,
        "independence_basis": (
            "Fresh build (rustc, per card, same build argv as the original cards) and fresh run "
            "on 2026-10-22 by executor sessions other than the one that produced the recorded "
            "2026-10-18 rows, as stated in the assessment dispatch; cards written by the "
            "experiment-director lane. Same instrument source (sha256 pinned below, also the "
            "sha256 of examples/blindspots_14_prime_m4.rs at the recorded commit 397d5ade...). "
            "Same host class is not claimed to be a different machine; neither the receipts nor "
            "this script can show the executor identity, which rests on the dispatch statement."
        ),
        "scalars_verified": verified,
        "scalars_scope": (
            f"All {res['total_rows_compared']} compared rows (selftest lines counted as rows): the sample in "
            "replay/PLAN.md: matched rho, arm (a) T_dec and full IC at 16 and 28 bits (design cells "
            "bx = 8 and bx = 63) and 16-bit bx = 3; arm (b) T_dec at 16-bit bx = 3, 4, 5; arm (b) "
            "full IC at 16-bit bx = 3 and bx = 4 runs 0-1; arm (b) 28-bit design-cell T_dec run to the "
            "nonzero cap; arm (b) 16-bit design-cell T_dec target 0 (cell, setup and target-0 rows); "
            "the deep selftest text. Every key but `ms` compared exactly, with no tolerance. Not "
            "replayed (see replay/PLAN.md 'What was dropped, and why'): 20 and 24 bits, 16-bit "
            "ladder bx = 6, 7, arm (b) T_dec bx = 8 targets 1-4 and planted targets, arm (b) full IC "
            "bx = 4 runs 2-15, arms (c) and (d) (`reuse-pkm`, `reach`). Timing fields (`ms`, selftest "
            "timing text, receipt seconds) are descriptive and excluded."
        ),
        "comparison_rule": (
            f"{REL}/replay/PLAN.md, 'What must match exactly': drop `ms` from each row; exact "
            "equality row by row in file order of every key (the PLAN.md-listed keys, and any "
            "other key); partial replays compare the corresponding subset; selftest line-equal "
            "after stripping timing text with last line SELFTEST OK; every receipt command exit 0; "
            "receipt card sha256 equals the card file."
        ),
        "compare_script": {"path": SCRIPT_REL, "sha256": sha256_file(os.path.abspath(__file__))},
        "instrument": {"path": INSTRUMENT, "sha256": INSTRUMENT_SHA},
        "total_rows_compared": res["total_rows_compared"],
        "total_listed_key_values_compared": res["total_listed_key_values_compared"],
        "total_mismatches": res["total_mismatches"],
        "mismatches": [m for r in runs for m in r["mismatches"]],
        "failures": res["failures"],
        "runs": runs,
    }
    with open(path, "w") as f:
        json.dump(cert, f, indent=1)
        f.write("\n")


if __name__ == "__main__":
    sys.exit(main(sys.argv))
