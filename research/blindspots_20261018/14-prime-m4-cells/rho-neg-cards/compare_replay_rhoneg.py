#!/usr/bin/env python3
"""Replay comparison for blind-spot item 14, rho-neg cards (card-r01 replays card-n01).

Implements rho-neg-cards/PLAN.md, section "Replay r01 (validation gate)", with the
standard library only. Nothing is excluded from a comparison but `ms` and timing text.

Checks (every one must pass; exit status 0 only then):
  1. selftest.stdout.log (replay, raw/rho-neg-replay/) is line-equal to the n01 file
     (raw/rho-neg/) after stripping timing text: ` NN ms` inside parentheses, `in N.NNs`,
     and the final `(N.Ns)` on the `SELFTEST OK` line; every other character equal; same
     line count; last line starts with `SELFTEST OK`.
  2. each of cont-rho-b{16,20,24,28}.jsonl, cont-fullic-a-b{16,20,24,28}.jsonl and
     rho-neg-b{16,20,24,28}.jsonl: replay equals n01, 16 rows each, parsed as JSON,
     every key at any depth equal (same JSON type) except `ms`, row by row in file order.
  3. each of the same 12 `.stdout.log` files: same rule as 2 (the stdout is JSON lines).
  4. every `.stderr.log` (selftest, 12 runs), build.stdout.log, build.stderr.log: size 0
     in both replay and n01.
  5. extra: replay cont-rho-b* equal the recorded raw/rho-b*.jsonl and replay
     cont-fullic-a-b* equal the recorded raw/fullic-a-b*.jsonl, 16 rows, same rule
     (and the same for n01 cont-* files, reported for information, also required).
  6. receipts (receipt-r01.json for the replay, receipt-n01.json for the original):
     status `completed`, commands_total equals the commands listed, every command exit 0
     and not timed out, receipt card_sha256 equals the sha256 of the card file, every
     receipt-listed output sha256 equals the file on disk, the card file for the replay
     is rho-neg-cards/card-r01.yaml and for the original rho-neg-cards/card-n01.yaml.
  7. instrument blindspots_14_prime_m4_neg.rs sha256 on disk equals
     1a7d51c3adcd7108de335cd7ac3b2d83a11f5ef1246b8b959271ced1f5e8710e, and equals the
     sha256 of the file at each receipt commit (git show; a missing git is a failure).

Usage: compare_replay_rhoneg.py [--json OUT]
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
RAW = os.path.join(CELLS, "raw")
REP = os.path.join(RAW, "rho-neg-replay")
ORIG = os.path.join(RAW, "rho-neg")
INSTRUMENT = "research/blindspots_20261018/instruments/blindspots_14_prime_m4_neg.rs"
INSTRUMENT_SHA = "1a7d51c3adcd7108de335cd7ac3b2d83a11f5ef1246b8b959271ced1f5e8710e"
BITS = [16, 20, 24, 28]
EXPECTED_ROWS = 16

JSONL_NAMES = ([f"cont-rho-b{b}" for b in BITS]
               + [f"cont-fullic-a-b{b}" for b in BITS]
               + [f"rho-neg-b{b}" for b in BITS])
STDERR_NAMES = (["selftest.stderr.log", "build.stdout.log", "build.stderr.log"]
                + [f"{n}.stderr.log" for n in JSONL_NAMES])
# replay cont file -> recorded file in raw/
RECORDED = {}
for b in BITS:
    RECORDED[f"cont-rho-b{b}.jsonl"] = f"rho-b{b}.jsonl"
    RECORDED[f"cont-fullic-a-b{b}.jsonl"] = f"fullic-a-b{b}.jsonl"


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
        for line in f:
            line = line.rstrip("\n")
            if line.strip():
                rows.append(json.loads(line))
    return rows


def drop_ms(x):
    if isinstance(x, dict):
        return {k: drop_ms(v) for k, v in x.items() if k != "ms"}
    if isinstance(x, list):
        return [drop_ms(v) for v in x]
    return x


def flatten(x, prefix=""):
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


def compare_rows(a_rows, b_rows, a_name, b_name):
    """a = replay (or first), b = recorded (or second). Returns mismatch list."""
    mism = []
    if len(a_rows) != len(b_rows):
        mism.append({"key": "row_count", a_name: len(a_rows), b_name: len(b_rows)})
    for i, (a, b) in enumerate(zip(a_rows, b_rows)):
        fa, fb = flatten(drop_ms(a)), flatten(drop_ms(b))
        for k in sorted(set(fa) | set(fb)):
            va, vb = fa.get(k, "<absent>"), fb.get(k, "<absent>")
            if k not in fa or k not in fb or not same(va, vb):
                mism.append({"row_index": i, "key": k, a_name: va, b_name: vb})
    return mism


def strip_timing(line):
    # final "(N.Ns)" on the SELFTEST OK line
    if line.startswith("SELFTEST OK"):
        line = re.sub(r"\s*\(\d+(?:\.\d+)?s\)\s*$", "", line)
    # "NN ms" inside parentheses
    line = re.sub(r"(\([^()]*?)\b\d+(?:\.\d+)? ms\b", r"\1<t> ms", line)
    # "in N.NNs"
    line = re.sub(r"\bin \d+(?:\.\d+)?s\b", "in <t>s", line)
    return line


def compare_selftest():
    rep_path = os.path.join(REP, "selftest.stdout.log")
    org_path = os.path.join(ORIG, "selftest.stdout.log")
    rep = open(rep_path).read().split("\n")
    org = open(org_path).read().split("\n")
    if rep and rep[-1] == "":
        rep = rep[:-1]
    if org and org[-1] == "":
        org = org[:-1]
    mism = []
    if len(rep) != len(org):
        mism.append({"key": "line_count", "replay": len(rep), "n01": len(org)})
    for i, (a, b) in enumerate(zip(rep, org)):
        sa, sb = strip_timing(a), strip_timing(b)
        if sa != sb:
            mism.append({"key": f"line {i + 1}", "replay": a, "n01": b,
                         "replay_stripped": sa, "n01_stripped": sb})
    for name, lines in (("replay", rep), ("n01", org)):
        if not lines or not lines[-1].startswith("SELFTEST OK"):
            mism.append({"key": f"last_line_{name}", "value": lines[-1] if lines else None})
    return {
        "replay_file": rel(rep_path), "replay_sha256": sha256_file(rep_path),
        "n01_file": rel(org_path), "n01_sha256": sha256_file(org_path),
        "comparison": "line-equal after stripping timing text; last line SELFTEST OK",
        "lines_n01": len(org), "lines_replay": len(rep), "rows_compared": len(rep),
        "last_line_replay": rep[-1] if rep else None, "last_line_n01": org[-1] if org else None,
        "mismatches": mism,
    }


def git_show_sha(commit, path):
    try:
        data = subprocess.run(["git", "-C", REPO, "show", f"{commit}:{path}"],
                              capture_output=True, check=True).stdout
    except Exception as e:  # noqa: BLE001
        return f"ERROR: {e}"
    return hashlib.sha256(data).hexdigest()


def check_receipt(receipt_path, card_path):
    rec = json.load(open(receipt_path))
    problems = []
    card_sha = sha256_file(card_path)
    if rec.get("card_sha256") != card_sha:
        problems.append(f"receipt card_sha256 {rec.get('card_sha256')} != card file sha256 {card_sha}")
    if rec.get("card_path") != rel(card_path):
        problems.append(f"receipt card_path {rec.get('card_path')} != {rel(card_path)}")
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
    out_checked = 0
    for c in cmds:
        for p, h in (c.get("outputs") or {}).items():
            out_checked += 1
            full = os.path.join(REPO, p)
            if not os.path.exists(full):
                problems.append(f"output {p}: missing")
            elif sha256_file(full) != h:
                problems.append(f"output {p}: file sha256 {sha256_file(full)} != receipt {h}")
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
        "commands_total": rec.get("commands_total"),
        "commands": [{"id": c.get("id"), "exit_code": c.get("exit_code")} for c in cmds],
        "receipt_output_hashes_checked": out_checked,
        "instrument_sha256_at_commit": instr,
        "problems": problems,
    }


def main(argv):
    json_out = None
    i = 1
    while i < len(argv):
        if argv[i] == "--json":
            json_out = argv[i + 1]
            i += 2
        else:
            print(f"unknown argument {argv[i]}", file=sys.stderr)
            return 2

    failures = []
    entries = []
    total_rows = 0
    total_mism = 0

    instr_disk = sha256_file(os.path.join(REPO, INSTRUMENT))
    if instr_disk != INSTRUMENT_SHA:
        failures.append(f"instrument sha256 on disk {instr_disk} != {INSTRUMENT_SHA}")

    receipts = {
        "r01": check_receipt(os.path.join(REP, "receipt-r01.json"),
                             os.path.join(HERE, "card-r01.yaml")),
        "n01": check_receipt(os.path.join(ORIG, "receipt-n01.json"),
                             os.path.join(HERE, "card-n01.yaml")),
    }
    for k, r in receipts.items():
        for p in r["problems"]:
            failures.append(f"receipt {k}: {p}")

    # 1. selftest
    st = compare_selftest()
    st["command_id"] = "selftest"
    entries.append(st)
    total_rows += st["rows_compared"]
    total_mism += len(st["mismatches"])
    for m in st["mismatches"]:
        failures.append(f"selftest: {m}")

    # 2. jsonl, 3. stdout, and 5. continuity against the recorded raw files
    for name in JSONL_NAMES:
        for ext, label in ((".jsonl", "jsonl"), (".stdout.log", "stdout")):
            rp, op = os.path.join(REP, name + ext), os.path.join(ORIG, name + ext)
            ra, ro = read_rows(rp), read_rows(op)
            mism = compare_rows(ra, ro, "replay", "n01")
            if len(ra) != EXPECTED_ROWS or len(ro) != EXPECTED_ROWS:
                mism.append({"key": "row_count_expected", "replay": len(ra), "n01": len(ro),
                             "expected": EXPECTED_ROWS})
            e = {"command_id": name, "file_kind": label,
                 "replay_file": rel(rp), "replay_sha256": sha256_file(rp),
                 "n01_file": rel(op), "n01_sha256": sha256_file(op),
                 "rows_replay": len(ra), "rows_n01": len(ro),
                 "rows_compared": min(len(ra), len(ro)), "mismatches": mism}
            entries.append(e)
            total_rows += e["rows_compared"]
            total_mism += len(mism)
            for m in mism:
                failures.append(f"{name}{ext}: {m}")
    for fname, recname in RECORDED.items():
        recp = os.path.join(RAW, recname)
        rec_rows = read_rows(recp)
        for tag, d in (("replay", REP), ("n01", ORIG)):
            p = os.path.join(d, fname)
            rows = read_rows(p)
            mism = compare_rows(rows, rec_rows, tag, "recorded")
            if len(rows) != EXPECTED_ROWS or len(rec_rows) != EXPECTED_ROWS:
                mism.append({"key": "row_count_expected", tag: len(rows),
                             "recorded": len(rec_rows), "expected": EXPECTED_ROWS})
            e = {"command_id": fname[:-6], "file_kind": f"continuity-{tag}-vs-recorded",
                 "replay_file" if tag == "replay" else "n01_file": rel(p),
                 "recorded_file": rel(recp), "recorded_sha256": sha256_file(recp),
                 "rows_compared": min(len(rows), len(rec_rows)), "mismatches": mism}
            entries.append(e)
            total_rows += e["rows_compared"]
            total_mism += len(mism)
            for m in mism:
                failures.append(f"{tag} {fname} vs recorded {recname}: {m}")

    # 4. empty logs
    empties = []
    for name in STDERR_NAMES:
        for tag, d in (("replay", REP), ("n01", ORIG)):
            p = os.path.join(d, name)
            size = os.path.getsize(p) if os.path.exists(p) else None
            empties.append({"file": rel(p), "size": size})
            if size != 0:
                failures.append(f"{rel(p)}: size {size}, expected 0")

    result = {
        "instrument": {"path": INSTRUMENT, "sha256_on_disk": instr_disk, "expected": INSTRUMENT_SHA},
        "receipts": receipts,
        "files": entries,
        "empty_logs_checked": empties,
        "total_rows_compared": total_rows,
        "total_mismatches": total_mism,
        "failures": failures,
        "verified": not failures,
    }
    if json_out:
        with open(json_out, "w") as f:
            json.dump(result, f, indent=1)
            f.write("\n")

    print(f"instrument sha256 on disk: {instr_disk} ({'OK' if instr_disk == INSTRUMENT_SHA else 'MISMATCH'})")
    for k, r in receipts.items():
        print(f"receipt {k}: commit {r['commit']} status {r['status']} commands {len(r['commands'])} "
              f"all exit 0: {all(x['exit_code'] == 0 for x in r['commands'])} "
              f"card sha match: {r['card_sha256'] == r['receipt_card_sha256']} "
              f"output hashes checked: {r['receipt_output_hashes_checked']} problems: {len(r['problems'])}")
    for e in entries:
        print(f"{e['command_id']} [{e.get('file_kind', 'selftest')}]: rows compared {e['rows_compared']} "
              f"mismatches {len(e['mismatches'])}")
    print(f"empty logs checked: {len(empties)}, non-empty: {sum(1 for x in empties if x['size'] != 0)}")
    print(f"TOTAL rows compared {total_rows}, mismatches {total_mism}")
    for f_ in failures:
        print("FAIL:", f_)
    print("VERIFIED" if not failures else "FAILED")
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
