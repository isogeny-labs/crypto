"""Metadata-only refresh of unpublished registrations; preserves all old bytes.

Requires main to extend the last published ledger, archives the unpublished
envelopes and journal tail, merges main, then registers unchanged claim JSONs.
No curve arithmetic, verifier trust change or signature generation occurs.
"""
import datetime
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path
import subprocess

root = Path.cwd()
phase = Path("research/nist_cover_three_tests_20261116/recorded_pool_20261123")
archive = phase / "evidence" / (sys.argv[1] if len(sys.argv) >= 2 else "registration-refresh-v1")
assert archive.parent == phase / "evidence", "archive must be a direct evidence child"
ledger = Path("pcc/ledger.jsonl")
journal = Path("docs/session-logs/2026-11-23.md")
resume = len(sys.argv) == 3 and sys.argv[2] == "--resume"
if not resume:
    assert not archive.exists(), "fresh metadata archive required"
    published = subprocess.check_output(["git", "show", "HEAD:pcc/ledger.jsonl"])
    incoming = subprocess.check_output(["git", "show", "origin/main:pcc/ledger.jsonl"])
    working = ledger.read_bytes()
    assert working.startswith(published) and incoming.startswith(published)
    tail = working[len(published):]
    entries = [json.loads(line) for line in tail.splitlines()]
    assert entries and all(e["type"] in ("claim", "note") and "sig" not in e for e in entries)
    old_journal = subprocess.check_output(["git", "show", "HEAD:docs/session-logs/2026-11-23.md"])
    assert journal.read_bytes().startswith(old_journal)
    journal_tail = journal.read_bytes()[len(old_journal):]
    archive.mkdir()
    (archive / "unpublished-ledger-tail.jsonl").write_bytes(tail)
    (archive / "unpublished-journal-tail.txt").write_bytes(journal_tail)
    before = {p: hashlib.sha256(p.read_bytes()).hexdigest() for p in
              (Path("pcc/claims") / (e["body"]["claim_id"] + ".json") for e in entries if e["type"] == "claim")}
    subprocess.run(["git", "restore", "--source=HEAD", "--worktree", str(ledger), str(journal)], check=True)
    subprocess.run(["git", "merge", "--no-stat", "--no-edit", "origin/main"], check=True)
else:
    assert archive.exists() and not (archive / "registration-map.json").exists()
    incoming = subprocess.check_output(["git", "show", "origin/main:pcc/ledger.jsonl"])
    entries = [json.loads(line) for line in (archive / "unpublished-ledger-tail.jsonl").read_bytes().splitlines()]
    journal_tail = (archive / "unpublished-journal-tail.txt").read_bytes()
    before = {Path(e["body"]["path"]): hashlib.sha256(Path(e["body"]["path"]).read_bytes()).hexdigest()
              for e in entries if e["type"] == "claim"}
assert ledger.read_bytes() == incoming, "main ledger must be untouched before append"
spec = importlib.util.spec_from_file_location("pcc_api", root / "tools/pcc/pcc.py")
pcc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pcc)
mapping = []
for old in entries:
    if old["type"] == "note":
        new = pcc.append_entry(ledger, "note", old["body"])
        mapping.append({"entry_type": "note", "old_envelope_hash": old["hash"], "new_envelope_hash": new["hash"]})
        continue
    path = Path("pcc/claims") / (old["body"]["claim_id"] + ".json")
    assert hashlib.sha256(path.read_bytes()).hexdigest() == before[path]
    new = pcc.append_entry(ledger, old["type"], old["body"])
    mapping.append({"claim_id": old["body"]["claim_id"], "claim_file_sha256": before[path],
                    "old_envelope_hash": old["hash"], "new_envelope_hash": new["hash"]})
mapping_path = archive / "registration-map.json"
mapping_path.write_text(json.dumps(mapping, indent=2) + "\n")
def subject(path):
    return {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
pcc.append_entry(ledger, "note", {
    "text": "Main advanced while local producer registrations/notes were unpublished. "
            "The incoming main ledger extends every previously published branch byte. "
            "Preserve all unpublished envelopes in the pinned archive and register "
            "the identical immutable claim payloads after the complete main prefix. "
            "No signed verification or changed claim bytes.",
    "original_tail": subject(archive / "unpublished-ledger-tail.jsonl"),
    "registration_mapping": subject(mapping_path),
    "canonical_base_commit": subprocess.check_output(["git", "rev-parse", "origin/main"], text=True).strip()
})
# Preserve main blocks and each unpublished block with its original timestamp.
from collections import Counter
pattern = re.compile(r"(?=^## \d{4}-\d\d-\d\d)", re.MULTILINE)
def blocks(value):
    pieces = pattern.split(value)
    return pieces[0], pieces[1:]
def norm(block):
    return "\n".join(line.rstrip() for line in block.strip().splitlines())
def stamp(block):
    head = block.splitlines()[0]
    hit = re.search(r"(\d{4}-\d\d-\d\d)[T ](\d\d:\d\d)", head)
    return hit.group(1) + "T" + hit.group(2) if hit else ""
main_text = journal.read_text()
prefix, main_blocks = blocks(main_text)
_, own_blocks = blocks(journal_tail.decode())
assert own_blocks and not journal_tail.decode().split("## ",1)[0].strip()
merged = list(main_blocks)
for block in own_blocks:
    position = next((i for i, other in enumerate(merged) if stamp(other) > stamp(block)), len(merged))
    merged.insert(position, block)
assert Counter(map(norm, merged)) == Counter(map(norm, main_blocks)) + Counter(map(norm, own_blocks))
assert [norm(block) for block in merged if norm(block) not in set(map(norm, own_blocks))] == [norm(block) for block in main_blocks if norm(block) not in set(map(norm, own_blocks))]
journal.write_text(prefix + "".join(merged))
print(json.dumps({"registrations_preserved": len(mapping), "archive": str(archive),
                  "published_prefix_retained": True, "claim_bytes_unchanged": True,
                  "journal_block_multiset_preserved": True, "journal_timestamps_preserved": True}))
