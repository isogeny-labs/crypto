"""Index native receipts and hashes; no mathematical recomputation."""
from pathlib import Path
import collections
import hashlib
import json
import re

here = Path(__file__).resolve().parent
rows = []
theorems = set()
for folder in sorted((here / "runs").iterdir()):
    if not folder.is_dir():
        continue
    row = {"attempt": folder.name}
    revision = folder / "source-revision.txt"
    row["source_revision"] = revision.read_text().strip() if revision.exists() else None
    if (folder / "compile.log").exists():
        row["kind"] = "Lean"
        for phase in ("compile", "audit"):
            receipt = folder / f"{phase}.exit"
            row[f"{phase}_exit"] = int(receipt.read_text()) if receipt.exists() else None
        log = (folder / "audit.log").read_text() if (folder / "audit.log").exists() else ""
        ok = row["compile_exit"] == row["audit_exit"] == 0 and "0 failing, 0 trusted" in log
        terminal = row["compile_exit"] not in (None, 0) or row["audit_exit"] is not None
        row["status"] = "accepted" if ok else "rejected" if terminal else "running"
        row["audited_theorems"] = re.findall(r"^ok\s+(\S+)\s+\[", log, re.M) if ok else []
        theorems.update(row["audited_theorems"])
    elif (folder / "input.sing").exists():
        row["kind"] = "Singular"
        receipt = folder / "exit.txt"
        row["native_exit"] = int(receipt.read_text()) if receipt.exists() else None
        log = (folder / "native.log").read_text() if (folder / "native.log").exists() else ""
        errors = re.findall(r"^ *\?.*$", log, re.M)
        markers = re.findall(r'print\("([A-Z][A-Z0-9_]*PASS)"\)', (folder / "input.sing").read_text())
        assert markers, folder.name
        marker = markers[-1]
        row.update(marker=marker, marker_present=marker in log.splitlines(), native_errors=errors,
                   timeout=row["native_exit"] == 124)
        ok = row["native_exit"] == 0 and not errors and row["marker_present"]
        row["status"] = "accepted" if ok else "rejected" if receipt.exists() else "running"
        row["result_lines"] = [line for line in log.splitlines()
                               if re.match(r"^[A-Z][A-Z0-9_]*(RESULT|COEFFICIENT_FIELD) ", line)]
        row["exports"] = {p.name: {"sha256": hashlib.sha256(p.read_bytes()).hexdigest(),
                                  "bytes": p.stat().st_size}
                          for p in sorted(folder.glob("*.sing")) if p.name != "input.sing"}
    else:
        row.update(kind="admission", status="running")
    rows.append(row)
accepted = {row["attempt"] for row in rows if row["status"] == "accepted"}
result = {
    "schema": "koblitz.results/v1", "round": 50, "sealed": False, "runs": rows,
    "status_counts": dict(collections.Counter(row["status"] for row in rows)),
    "audited_unique_theorems": sorted(theorems),
    "construction_counts": {
        "proper_monic_charts": 2 if "chart50-01" in accepted else 0,
        "integral_v_charts": 2 if "relative50-01" in accepted else 0,
        "complete_forward_source_products": 4 if "forward50-02" in accepted else 0,
        "boundary_cyclic_F2_dimension": 128 if "local50-01" in accepted else None,
        "boundary_exact_saturation_exponent": 8 if "local50-01" in accepted else None,
        "finite_coordinate_defect_F2_dimension": 898 if "defect50-01" in accepted else None,
        "infinity_coordinate_defect_F2_dimension": 94 if "defect50-01" in accepted else None,
        "global_coordinate_defect_F2_dimension": 992 if "defect50-01" in accepted else None,
        "verified_affine_normalizations": sum(name.startswith("verify-relative-") for name in accepted),
        "audited_unique_theorems": len(theorems),
    },
    "native_arrays_formalized": False,
    "independent_scientific_replay": False,
    "independent_semantic_review": False,
}
(here / "results.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({"attempts": len(rows), **result["status_counts"],
                  "audited_unique_theorems": len(theorems)}))
