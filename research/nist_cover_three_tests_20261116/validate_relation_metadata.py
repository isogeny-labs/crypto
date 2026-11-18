"""Reviewable metadata-only binding/history audit; no cryptographic arithmetic."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--receipt-name", default="maps-main-validation.json")
parser.add_argument("--run", action="append", dest="runs")
parser.add_argument("--base", default="MERGE_HEAD")
args = parser.parse_args()
assert Path(args.receipt_name).name == args.receipt_name

root = Path.cwd()
spec = importlib.util.spec_from_file_location("pcc_api", root / "tools/pcc/pcc.py")
pcc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pcc)
base = subprocess.check_output(["git", "rev-parse", args.base], text=True).strip()
base_text = subprocess.check_output(["git", "show", f"{base}:pcc/ledger.jsonl"], text=True)
base_entries = pcc.read_ledger_text(base_text)
head_entries = pcc.read_ledger(root / "pcc/ledger.jsonl")
structural = []
pcc.check_chain(base_entries, structural)
pcc.check_chain(head_entries, structural)
pcc.check_prefix(base_entries, head_entries, structural)
assert not structural, structural
base_names = subprocess.check_output(
    ["git", "ls-tree", "-r", "--name-only", base, "pcc/claims"], text=True
).splitlines()
changes = subprocess.check_output(
    ["git", "diff", "--name-status", base, "--", "pcc/claims"], text=True
).splitlines()
assert all(line.startswith("A\t") for line in changes), changes
dates = pcc._claim_entry_dates(base_entries)
base_errors = []
for name in base_names:
    claim = json.loads((root / name).read_text())
    problem = pcc.claim_type_problem(claim, dates.get(str(claim.get("id")), ""))
    if problem:
        base_errors.append(f"claim {claim.get('id')}: {problem}")
head_errors = []
pcc.check_claim_types(root, head_entries, head_errors)
assert sorted(base_errors) == sorted(head_errors), "new claim-type diagnostic"
runs = [
    "complete-maps-v1", "transpose-maps-v1", "closed-transpose-v1",
    "moving-transpose-v1", "pole-cases-v1", "complete-km-v3",
]
if args.runs:
    runs = args.runs
study = root / "research/nist_cover_three_tests_20261116"
receipts = []
for name in runs:
    run = study / "evidence" / name
    assert (run / "EXIT_CODE.txt").read_text().strip() == "0"
    checked = 0
    for line in (run / "pre-call-manifest.sha256").read_text().splitlines():
        digest, path = line.split(maxsplit=1)
        path = path.lstrip("*")
        subject = Path(path)
        if not subject.is_absolute():
            subject = study / subject
        assert hashlib.sha256(subject.read_bytes()).hexdigest() == digest, path
        checked += 1
    receipts.append({"run": name, "pre_call_bindings_match": checked})
out = {
    "base_commit": base,
    "pcc_api_sha256": hashlib.sha256((root / "tools/pcc/pcc.py").read_bytes()).hexdigest(),
    "base_ledger_rows": len(base_entries),
    "head_ledger_rows_at_check": len(head_entries),
    "structural_chain_and_prefix_errors": structural,
    "unchanged_historical_claim_files": len(base_names),
    "new_claim_files": len(changes),
    "base_claim_type_diagnostics": len(base_errors),
    "head_claim_type_diagnostics": len(head_errors),
    "diagnostics_identical": True,
    "diagnostics": sorted(head_errors),
    "native_pre_call_bindings": receipts,
    "standing": "metadata checks; no independent mathematical verification",
}
target = study / "evidence" / args.receipt_name
assert not target.exists(), "new metadata receipt required"
target.write_text(json.dumps(out, indent=2) + "\n")
print(json.dumps({key: val for key, val in out.items() if key != "diagnostics"}, indent=2))
