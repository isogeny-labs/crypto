"""Maintained append_entry API only; claim creation/checking was performed in Rust.

This administrative transport does not call verify_claim, sign a verdict, or
execute a scientific verifier. It refuses any unexpected canonical-ledger tail.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

repo = Path(__file__).resolve().parents[5]
here = Path(__file__).resolve().parent
prepared = here.parent / "prepared-02"
base = "816f9c3a942efed569049618c056d12a0c56d693"
claim_id = "PCC-20261123-023d90"
expected_digest = "6c2736591b1abbfff75044ee7beb9acbdf23af98f1a59786852f57c9ee4785f7"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def write_new(name, value):
    with (here / name).open("x", encoding="utf-8") as output:
        json.dump(value, output, sort_keys=True, indent=2, ensure_ascii=False)
        output.write("\n")


assert subprocess.check_output(["git", "rev-parse", "origin/main"], cwd=repo).decode().strip() == base
base_bytes = subprocess.check_output(["git", "show", f"{base}:pcc/ledger.jsonl"], cwd=repo)
ledger = repo / "pcc/ledger.jsonl"
before = ledger.read_bytes()
assert before == base_bytes, "refusing a changed prefix or an unexpected local tail"
assert before.endswith(b"\n")

body = json.loads((prepared / "registration-body.json").read_text())
native_check = json.loads((here / "native-check.stdout.json").read_text())
assert set(body) == {"claim_id", "claim_sha256", "path"}
assert body["claim_id"] == claim_id and body["claim_sha256"] == expected_digest
assert body["path"] == f"pcc/claims/{claim_id}.json"
assert native_check["result"] == "metadata_consistent"
assert native_check["claim_id"] == claim_id
assert native_check["claim_sha256"] == expected_digest
assert (here / "native-check.exit-status.txt").read_text().strip() == "0"
assert (repo / body["path"]).read_bytes() == (prepared / f"{claim_id}.json").read_bytes()

source = repo / "tools/pcc/pcc.py"
assert digest(source.read_bytes()) == "00036f8b524bf070a5cf36e02b2599148d390bdf30562d81a1a073df7777da2a"
spec = importlib.util.spec_from_file_location("rr5_pcc_append_contract", source)
pcc = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = pcc
spec.loader.exec_module(pcc)
entries = pcc.read_ledger(ledger)
errors = []
pcc.check_chain(entries, errors)
assert not errors, errors
assert not any(e.get("type") == "claim" and e.get("body", {}).get("claim_id") == claim_id for e in entries)
prefix = {
    "schema": "rr5.pcc-prefix-observation/v1",
    "canonical_base_commit": base,
    "canonical_ledger_path": "pcc/ledger.jsonl",
    "prefix_sha256": digest(before),
    "prefix_bytes": len(before),
    "prefix_entries": len(entries),
    "prefix_tail_hash": entries[-1]["hash"],
    "local_ledger_exactly_equals_base": True,
    "reconstruction": f"git show {base}:pcc/ledger.jsonl",
    "scope": "administrative byte equality and maintained ledger-chain checks; no scientific verification"
}
write_new("prefix.json", prefix)

entry = pcc.append_entry(ledger, "claim", body)
after = ledger.read_bytes()
assert after.startswith(before), "canonical prefix changed during append"
after_entries = pcc.read_ledger(ledger)
errors = []
pcc.check_chain(after_entries, errors)
assert not errors, errors
assert len(after_entries) == len(entries) + 1
assert after_entries[-1] == entry and "sig" not in entry
write_new("append-entry.json", entry)
receipt = {
    "schema": "rr5.pcc-administrative-registration/v1",
    "status": "unsigned statement registration only",
    "claim_id": claim_id,
    "claim_canonical_sha256": expected_digest,
    "claim_file_raw_sha256": native_check["claim_file"]["sha256"],
    "canonical_base_commit": base,
    "prefix_sha256": digest(before),
    "prefix_bytes": len(before),
    "prefix_entries": len(entries),
    "prefix_preserved_byte_for_byte": True,
    "claim_bytes_preserved": True,
    "append_api": "tools/pcc/pcc.py:append_entry",
    "append_api_source_sha256": digest(source.read_bytes()),
    "new_entry_seq": entry["seq"],
    "new_entry_hash": entry["hash"],
    "new_entry_prev": entry["prev"],
    "ledger_entries_after": len(after_entries),
    "ledger_bytes_after": len(after),
    "ledger_sha256_after": digest(after),
    "no_signed_verdict_appended": True,
    "scientific_verifier_executed": False,
    "claim_generation_and_artifact_checks": "Rust native tool; see retained native-check.stdout.json"
}
write_new("append-receipt.json", receipt)
print(json.dumps(receipt, sort_keys=True, indent=2))
