"""Metadata-only canonical Git/PCC/journal merge; no scientific computation.

Run once during the recorded merge conflict, with exact local, canonical-main
and last-published local commit IDs. Preserve old registration envelopes and
use the maintained PCC append API for unchanged registration bodies.
"""
from pathlib import Path
from collections import Counter
from datetime import datetime, timezone
import hashlib
import json
import re
import runpy
import subprocess
import sys

repo = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
ours, theirs, published = sys.argv[1:]
base = subprocess.check_output(["git", "merge-base", ours, theirs], text=True).strip()
def read(ref, path):
    return subprocess.check_output(["git", "show", ref + ":" + path])
assert subprocess.check_output(["git", "ls-files", "-u", "pcc/ledger.jsonl"]), "require an unresolved ledger merge; do not repeat"
root = repo / "research/nist_cover_three_tests_20261116/local_collection_20261124/evidence/merge-v1"
root.mkdir()
ledger_path = "pcc/ledger.jsonl"
common = read(base, ledger_path)
old, canonical, public = read(ours, ledger_path), read(theirs, ledger_path), read(published, ledger_path)
assert old.startswith(common) and canonical.startswith(common) and public.startswith(common)
(root / "local-registration-tail.jsonl").write_bytes(old[len(common):])
(root / "published-registration-tail.jsonl").write_bytes(public[len(common):])
old_entries = [json.loads(line) for line in old[len(common):].splitlines()]
assert len(old_entries) == 5 and all(entry["type"] == "claim" for entry in old_entries)
api = runpy.run_path(str(repo / "tools/pcc/pcc.py"))
(repo / ledger_path).write_bytes(canonical)
api["append_entry"](repo / ledger_path, "note", {
    "text": "Canonical main merge; unchanged local claim registrations follow. Original local and published envelopes are archived, and Git retains both parents.",
    "local_commit": ours, "canonical_commit": theirs, "last_published_local": published,
    "archive": str(root.relative_to(repo) / "local-registration-tail.jsonl"),
    "archive_sha256": hashlib.sha256(old[len(common):]).hexdigest(),
})
mapping = []
for old_entry in old_entries:
    body = old_entry["body"]
    claim_bytes = (repo / body["path"]).read_bytes()
    claim = json.loads(claim_bytes)
    assert api["claim_digest"](claim) == body["claim_sha256"]
    new_entry = api["append_entry"](repo / ledger_path, "claim", dict(body))
    assert (repo / body["path"]).read_bytes() == claim_bytes
    mapping.append({"claim_id": body["claim_id"], "unchanged_body": body,
                    "old_seq": old_entry["seq"], "old_hash": old_entry["hash"],
                    "new_seq": new_entry["seq"], "new_hash": new_entry["hash"]})
assert (repo / ledger_path).read_bytes().startswith(canonical)

journal_path = "docs/session-logs/2026-11-24.md"
common_j = read(base, journal_path).decode()
local_j, main_j = read(ours, journal_path).decode(), read(theirs, journal_path).decode()
assert local_j.startswith(common_j) and main_j.startswith(common_j)
def blocks(text):
    starts = [m.start() for m in re.finditer(r"(?m)^## \d{4}-\d{2}-\d{2}T", text)]
    assert not text[:starts[0] if starts else len(text)].strip()
    return [text[start:starts[i+1] if i+1 < len(starts) else len(text)].strip("\n")
            for i, start in enumerate(starts)]
local_blocks = blocks(local_j[len(common_j):])
main_blocks = blocks(main_j[len(common_j):])
lc, mc = Counter(local_blocks), Counter(main_blocks)
merged = []
for block in mc.keys() | lc.keys():
    merged.extend([block] * max(lc[block], mc[block]))
merged.sort(key=lambda block: block.splitlines()[0])
combined = common_j + "\n" + "\n\n".join(merged) + "\n"
assert Counter(blocks(combined[len(common_j):])) == (lc | mc)
(repo / journal_path).write_text(combined)

active_path = "docs/ACTIVE.md"
local_active, main_active = read(ours, active_path).decode(), read(theirs, active_path).decode()
line = next(line for line in local_active.splitlines() if line.startswith("- [x] Execute the user-authorized local P-192"))
assert line not in main_active
main_active = main_active.replace("## Active items\n", "## Active items\n" + line + "\n", 1)
old_line = next(line for line in main_active.splitlines() if line.startswith("- [ ] Complete P-192 genus3 signed independent replay"))
new_line = "- [ ] Complete P-192 genus3 independent signed verification and original map-synthesis/full-cost accounting (Codex; actual user-authorized local split-tangent collection completed in PR #5123 with closed gain0; the local backend remains authorized; existing CI signing key and original synthesis accounting remain open). [pcc:unverified]"
main_active = main_active.replace(old_line, new_line, 1)
(repo / active_path).write_text(main_active)

receipt = {"scope": "metadata reconciliation only; no mathematical rerun or signed verdict",
           "checked_at": datetime.now(timezone.utc).isoformat(), "base": base,
           "local_checkpoint": ours, "canonical_main": theirs, "published_local": published,
           "canonical_ledger_prefix_retained": True, "unchanged_claim_registrations": mapping,
           "journal_blocks_preserved_with_multiplicity": True,
           "keyring_unchanged": read(ours, "pcc/keyring.json") == read(theirs, "pcc/keyring.json")}
(root / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print("Canonical prefix, unchanged local claim bodies and full journal block multiplicities preserved")
