#!/usr/bin/env python3
"""Preserve shared metadata during this merge; performs no cryptographic math.

Pass the maintained PCC API snapshot used before current main removed local
tooling. Its digest is recorded. Claim/note bodies remain byte-equivalent as
canonical JSON; main's ledger byte prefix is never rewritten.
"""
import argparse
import collections
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess


def staged(stage, path):
    return subprocess.check_output(["git", "show", f":{stage}:{path}"])


def key(entry):
    return json.dumps([entry["type"], entry["body"]], sort_keys=True,
                      separators=(",", ":"), ensure_ascii=False)


def blocks(text):
    parts = re.split(r"(?m)(?=^## )", text)
    return parts[0], [part.rstrip("\n") for part in parts[1:] if part.strip()]


def stamp(block):
    match = re.search(r"20\d\d-\d\d-\d\d[T ]\d\d:\d\d(?::\d\d)?",
                      block.splitlines()[0])
    return match.group(0).replace(" ", "T") if match else "0000"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--pcc-api", type=Path, required=True)
    args = parser.parse_args()
    root = Path.cwd()
    evidence = root / "research/nist_cover_three_tests_20261116/evidence"
    out = evidence / "quotient-main-history-retention.json"
    raw_path = evidence / "quotient-ledger-branch-suffix.jsonl"
    assert not out.exists() and not raw_path.exists(), "fresh receipts required"
    ours = staged(2, "pcc/ledger.jsonl")
    canonical = staged(3, "pcc/ledger.jsonl")
    aa, bb = ours.splitlines(keepends=True), canonical.splitlines(keepends=True)
    common = 0
    while common < min(len(aa), len(bb)) and aa[common] == bb[common]:
        common += 1
    suffix = b"".join(aa[common:])
    entries = [json.loads(row) for row in aa[common:]]
    present = {key(json.loads(row)) for row in bb}
    missing = [entry for entry in entries if key(entry) not in present]
    assert all(entry["type"] in ("claim", "note") and not entry.get("sig")
               for entry in missing), "never rewrite an absent signed verdict"
    spec = importlib.util.spec_from_file_location("maintained_pcc", args.pcc_api)
    pcc = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(pcc)
    errors = []
    pcc.check_chain(pcc.read_ledger_text(canonical.decode()), errors)
    assert not errors, errors
    journal = "docs/session-logs/2026-11-16.md"
    head1, old = blocks(staged(2, journal).decode())
    head2, upstream = blocks(staged(3, journal).decode())
    assert head1.strip() == head2.strip(), "review differing journal preambles"
    prefix = 0
    while prefix < min(len(old), len(upstream)) and old[prefix] == upstream[prefix]:
        prefix += 1
    want = collections.Counter(old) | collections.Counter(upstream)
    seen = collections.Counter(old[:prefix])
    tail = []
    for block in upstream[prefix:] + old[prefix:]:
        if seen[block] < want[block]:
            tail.append(block)
            seen[block] += 1
    merged = old[:prefix] + sorted(tail, key=stamp)
    assert collections.Counter(merged) == want
    raw_path.write_bytes(suffix)
    ledger = root / "pcc/ledger.jsonl"
    ledger.write_bytes(canonical)
    appended = []
    for entry in missing:
        if key(entry) in present:
            continue
        updated = pcc.append_entry(ledger, entry["type"], entry["body"])
        assert updated["body"] == entry["body"]
        appended.append(updated)
        present.add(key(entry))
    assert ledger.read_bytes().startswith(canonical)
    errors = []
    pcc.check_chain(pcc.read_ledger(ledger), errors)
    assert not errors, errors
    (root / journal).write_text(head2 + "\n\n".join(merged) + "\n")
    receipt = {
        "main_commit": subprocess.check_output(["git", "rev-parse", "MERGE_HEAD"], text=True).strip(),
        "branch_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
        "pcc_api_sha256": hashlib.sha256(args.pcc_api.read_bytes()).hexdigest(),
        "ledger_common_rows": common,
        "canonical_main_rows": len(bb),
        "raw_branch_suffix_rows": len(entries),
        "raw_branch_suffix_sha256": hashlib.sha256(suffix).hexdigest(),
        "reappended_unchanged_bodies": len(appended),
        "canonical_prefix_retained": True,
        "ledger_chain_errors": errors,
        "journal_branch_blocks": len(old),
        "journal_main_blocks": len(upstream),
        "journal_result_blocks": len(merged),
        "maximum_complete_block_multiset_retained": True,
        "main_journal_preamble_retained": True,
        "branch_preamble_only_extra_leading_whitespace": head1 != head2,
    }
    out.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt, indent=2))


if __name__ == "__main__":
    main()
