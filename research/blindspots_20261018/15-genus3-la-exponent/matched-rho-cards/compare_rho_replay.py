#!/usr/bin/env python3
"""Independent comparison of replay card r01 (matched rho walks, item 15).

Stdlib only.  Run from anywhere:

    python3 research/blindspots_20261018/15-genus3-la-exponent/matched-rho-cards/compare_rho_replay.py
    python3 .../compare_rho_replay.py --json     # also print one JSON summary line

Rule: matched-rho-cards/PLAN.md, section "Replay card r01", "What each replay record
must equal".

 1. Receipt (raw/matched-replay/receipt-card-r01.json): status is `completed`, every
    command has exit_code 0 and did not time out, card_sha256 equals the sha256 of
    card-r01.yaml now, every output digest equals the sha256 of the file now, the
    instrument file now hashes to
    79a821504c5b7a4639470d8e04dfd35fc6ad1a91d34bacee466461f36811e991 and the card
    pins that same sha256 in `binds`, and the build command compiles that file.
 2. Per curve c in {0, 2}: raw/matched-replay/b20_c<c>_rho_replay.jsonl (one line)
    against raw/matched/b20_c<c>_matched.jsonl (one line).  The replayed targets are
    0..K-1 with K the `--rho-targets` value of the receipt's run command for that
    curve.  For each replayed target, the recorded `results[]` entry with arm "rho"
    and that target and the replay entry must be equal on every PER_RHO field (types
    and values; floats equal as parsed).  The per-curve instance fields PER_CURVE
    must be equal.
 3. Exempt (PLAN.md): wall_total of every rho record, build_wall, vmhwm_kb, arms_run,
    matched_rho, and the recorded IC results[] entries and rho walks beyond the prefix.

Exit status 0 only if everything passes; every mismatch is printed verbatim.
Nothing is written.
"""
import hashlib
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]  # matched-rho-cards -> item -> blindspots_20261018 -> research -> root
ITEM = "research/blindspots_20261018/15-genus3-la-exponent"
CARD = f"{ITEM}/matched-rho-cards/card-r01.yaml"
RECEIPT = f"{ITEM}/raw/matched-replay/receipt-card-r01.json"
INSTRUMENT = "research/blindspots_20261018/instruments/blindspots_15b_genus3_la_rho.rs"
INSTRUMENT_SHA = "79a821504c5b7a4639470d8e04dfd35fc6ad1a91d34bacee466461f36811e991"
CURVES = (0, 2)

PER_RHO = ["arm", "target", "k", "k_true", "correct", "steps", "counters", "setup_counters",
           "restarts", "escapes", "r", "cost", "cost_cl9", "cost_per_step", "rho_model_neg",
           "steps_over_sqrt_ell", "steps_over_model", "native", "gae"]
PER_CURVE = ["bits", "q", "curve", "seed", "mode", "build_tries", "N", "ell", "c", "h", "a", "b",
             "r0", "Kmod", "checks", "relprim", "walks", "instrument", "symmetry", "c_add",
             "native_names", "native_note"]
EXEMPT_RHO = {"wall_total"}
EXEMPT_CURVE = {"build_wall", "vmhwm_kb", "arms_run", "matched_rho", "results"}

mismatches = []
counts = {"receipt_checks": 0, "curve_fields": 0, "rho_records": 0, "rho_fields": 0, "leaves": 0}


def sha(path):
    return hashlib.sha256((ROOT / path).read_bytes()).hexdigest()


def bad(msg):
    mismatches.append(msg)
    print("MISMATCH", msg)


def check(cond, msg):
    counts["receipt_checks"] += 1
    if not cond:
        bad(msg)


def leaves(a, b, where):
    """Strict recursive equality; return number of leaves compared, report each difference."""
    if isinstance(a, dict) and isinstance(b, dict):
        n = 0
        for k in sorted(set(a) | set(b)):
            if k not in a:
                bad(f"{where}.{k}: missing in replay, recorded={json.dumps(b[k])}")
            elif k not in b:
                bad(f"{where}.{k}: missing in recorded, replay={json.dumps(a[k])}")
            else:
                n += leaves(a[k], b[k], f"{where}.{k}")
        return n
    if isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            bad(f"{where}: length replay={len(a)} recorded={len(b)}")
        n = 0
        for i, (x, y) in enumerate(zip(a, b)):
            n += leaves(x, y, f"{where}[{i}]")
        return n
    if type(a) is not type(b) or a != b:
        bad(f"{where}: replay={json.dumps(a)} recorded={json.dumps(b)}")
    return 1


def load_one(path):
    lines = [l for l in (ROOT / path).read_text().splitlines() if l.strip()]
    if len(lines) != 1:
        bad(f"{path}: expected 1 jsonl line, found {len(lines)}")
    return json.loads(lines[0])


def main():
    # 1. receipt
    receipt = json.loads((ROOT / RECEIPT).read_text())
    check(receipt.get("status") == "completed", f"{RECEIPT}: status={receipt.get('status')!r}, not 'completed'")
    check(receipt.get("card_id") == "CARD-BLINDSPOT-15-R01", f"{RECEIPT}: card_id={receipt.get('card_id')!r}")
    check(receipt.get("card_path") == CARD, f"{RECEIPT}: card_path={receipt.get('card_path')!r}, expected {CARD}")
    check(receipt.get("card_sha256") == sha(CARD),
          f"card sha256 receipt={receipt.get('card_sha256')} file={sha(CARD)}")
    check(receipt.get("commands_total") == len(receipt.get("commands", [])) == 6,
          f"commands_total={receipt.get('commands_total')} listed={len(receipt.get('commands', []))}, expected 6")
    for cmd in receipt["commands"]:
        check(cmd.get("exit_code") == 0 and cmd.get("succeeded") is True and cmd.get("timed_out") is False,
              f"command {cmd.get('id')}: exit_code={cmd.get('exit_code')} succeeded={cmd.get('succeeded')} "
              f"timed_out={cmd.get('timed_out')}")
        for path, digest in cmd.get("outputs", {}).items():
            check(sha(path) == digest, f"output {path}: receipt={digest} file={sha(path)}")
    card_text = (ROOT / CARD).read_text()
    check(f"{{path: {INSTRUMENT}, sha256: {INSTRUMENT_SHA}}}" in card_text,
          f"card does not pin {INSTRUMENT} at {INSTRUMENT_SHA}")
    check(sha(INSTRUMENT) == INSTRUMENT_SHA, f"instrument {INSTRUMENT}: file sha256={sha(INSTRUMENT)}, expected {INSTRUMENT_SHA}")
    build = [c for c in receipt["commands"] if c["id"] == "build"]
    check(len(build) == 1 and build[0]["argv"][-1] == INSTRUMENT, f"build argv does not compile {INSTRUMENT}")
    outs = {p for c in receipt["commands"] for p in c.get("outputs", {})}
    for c in CURVES:
        check(f"{ITEM}/raw/matched-replay/b20_c{c}_rho_replay.jsonl" in outs,
              f"receipt has no output digest for b20_c{c}_rho_replay.jsonl")

    # 2. records
    for c in CURVES:
        run = [x for x in receipt["commands"] if x["id"] == f"run-b20-c{c}-rho-replay"]
        if len(run) != 1:
            bad(f"receipt has {len(run)} commands with id run-b20-c{c}-rho-replay")
            continue
        argv = run[0]["argv"]
        K = int(argv[argv.index("--rho-targets") + 1])
        check(int(argv[argv.index("--curve") + 1]) == c, f"curve {c}: run argv --curve mismatch")
        check(argv[argv.index("--arms") + 1] == "rho", f"curve {c}: run argv --arms is not rho")
        new = load_one(f"{ITEM}/raw/matched-replay/b20_c{c}_rho_replay.jsonl")
        ref = load_one(f"{ITEM}/raw/matched/b20_c{c}_matched.jsonl")
        n_before = len(mismatches)
        for f in PER_CURVE:
            if f not in new or f not in ref:
                bad(f"curve {c} field {f}: present replay={f in new} recorded={f in ref}")
                continue
            counts["curve_fields"] += 1
            counts["leaves"] += leaves(new[f], ref[f], f"c{c}.{f}")
        if new["symmetry"].get("strongest_rho_symmetry") != "negation":
            bad(f"c{c}.symmetry.strongest_rho_symmetry replay={new['symmetry'].get('strongest_rho_symmetry')!r}, expected 'negation'")
        # fields in neither list
        listed = set(PER_CURVE) | EXEMPT_CURVE
        for f in sorted(set(new) - listed):
            print(f"NOTE curve {c}: replay field {f!r} is in neither PER_CURVE nor the exempt list (not compared)")
        newrho = [r for r in new["results"] if r.get("arm") == "rho"]
        if len(new["results"]) != K or len(newrho) != K:
            bad(f"curve {c}: replay has {len(new['results'])} results, {len(newrho)} rho; expected {K}")
        if sorted(r.get("target") for r in newrho) != list(range(K)):
            bad(f"curve {c}: replay rho targets {sorted(r.get('target') for r in newrho)}, expected {list(range(K))}")
        for t in range(K):
            a = [r for r in newrho if r.get("target") == t]
            b = [r for r in ref["results"] if r.get("arm") == "rho" and r.get("target") == t]
            if len(a) != 1 or len(b) != 1:
                bad(f"curve {c} target {t}: replay entries={len(a)} recorded entries={len(b)}")
                continue
            counts["rho_records"] += 1
            for f in PER_RHO:
                if f not in a[0] or f not in b[0]:
                    bad(f"c{c} target {t} field {f}: present replay={f in a[0]} recorded={f in b[0]}")
                    continue
                counts["rho_fields"] += 1
                counts["leaves"] += leaves(a[0][f], b[0][f], f"c{c}.rho[{t}].{f}")
            for f in sorted(set(a[0]) - set(PER_RHO) - EXEMPT_RHO):
                print(f"NOTE curve {c} target {t}: replay field {f!r} is in neither PER_RHO nor exempt (not compared)")
        print(f"curve {c}: replayed targets 0..{K - 1}, rho records compared={K}, mismatches={len(mismatches) - n_before}")

    summary = {"curves": list(CURVES), "rho_walks_replayed": counts["rho_records"], "receipt_checks": counts["receipt_checks"],
               "curve_fields_compared": counts["curve_fields"], "rho_fields_compared": counts["rho_fields"],
               "leaves_compared": counts["leaves"], "mismatch_count": len(mismatches), "mismatches": mismatches}
    print("TOTAL", json.dumps({k: v for k, v in summary.items() if k != "mismatches"}, sort_keys=True))
    if "--json" in sys.argv:
        print("JSON", json.dumps(summary, sort_keys=True))
    if mismatches:
        print("COMPARE FAILED")
        return 1
    print("COMPARE OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
