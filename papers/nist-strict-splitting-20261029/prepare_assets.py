"""Copy frozen observations and typeset them; no curve or statistical computation."""
import csv
import hashlib
import json
from pathlib import Path
import shutil
import fitz

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
STUDY = ROOT / "research/nist_skip_criterion_20261025/strict_elkies_20261029"
SNAPSHOT = "c51b0c32620ab43c73b1c0a34c236412f466f89a"
for name in ["data", "figures", "tables", "reproduction"]:
    (BASE / name).mkdir(exist_ok=True)

imports = {
    "data/inputs.tsv": STUDY / "inputs.tsv",
    "data/at691.tsv": STUDY / "results/at691.tsv",
    "data/summary.json": STUDY / "results/summary.json",
    "data/verification.json": STUDY / "results/verification.json",
    "data/original-four-models.icv1.json":
        ROOT / "research/curve_candidates_dissect_20261021/evidence/curves.icv1.json",
    "reproduction/producer.cpp": STUDY / "producer.cpp",
    "reproduction/verify.cpp": STUDY / "verify.cpp",
    "reproduction/BUILD.json": STUDY / "BUILD.json",
    "reproduction/PROTOCOL.md": STUDY / "PROTOCOL.md",
    "figures/strict-counts.svg": STUDY / "strict-counts.svg",
    "figures/evidence-boundary.svg": STUDY / "evidence-boundary.svg",
}
records = []
for destination, source in imports.items():
    target = BASE / destination
    shutil.copyfile(source, target)
    assert target.read_bytes() == source.read_bytes()
    records.append({
        "paper_path": destination,
        "repository_path": source.relative_to(ROOT).as_posix(),
        "source_commit": SNAPSHOT,
        "sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
    })

for stem in ["strict-counts", "evidence-boundary"]:
    with fitz.open(BASE / "figures" / (stem + ".svg")) as document:
        pdf = document.convert_to_pdf()
    (BASE / "figures" / (stem + ".pdf")).write_bytes(pdf)

names = {
    "p192-139": "NIST P-192",
    "p192-5228": "prime192v3",
    "p192-6013": "prime192v2",
    "p224-7026": "NIST P-224",
    "p239-2185": "prime239v1",
    "p239-2320": "prime239v2",
    "p239-3048": "prime239v3",
    "p256-1842": "NIST P-256",
    "p384-2688": "NIST P-384",
    "p521-1136": "NIST P-521",
}
with (BASE / "data/at691.tsv").open() as stream:
    rows = list(csv.DictReader(stream, delimiter="\t"))
assert len(rows) == 21
all_rows, sensitivity = [], []
for row in rows:
    name = r"\texttt{" + row["label"] + "}"
    if row["label"] in names:
        name += " / " + names[row["label"]]
    role = "P" if row["role"] == "published" else "U"
    values = [row["strict691"], row["ramified691"], row["inclusive691"]]
    all_rows.append(" & ".join([name, role, *values]) + r" \\")
    if row["bits"] == "192":
        label = row["label"].split("-")[1]
        outcome = "NIST P-192" if label == "139" else row["role"]
        sensitivity.append(" & ".join([label, outcome, *values]) + r" \\")
all_rows.append(r"\bottomrule")
sensitivity.append(r"\bottomrule")
(BASE / "tables/all-candidates.tex").write_text("\n".join(all_rows) + "\n")
(BASE / "tables/p192-sensitivity.tex").write_text("\n".join(sensitivity) + "\n")
(BASE / "data/source-bindings.json").write_text(json.dumps({
    "kind": "nist-strict-splitting-paper-imports/v1",
    "source_snapshot": SNAPSHOT,
    "files": records,
    "document_only_transformations": [
        "SVG-to-PDF conversion using PyMuPDF " + fitz.VersionBind,
        "Literal table layout of already recorded row values",
    ],
    "original_four_icv1_note":
        "Supplied coefficient representatives; the strict cohort can use the "
        "other isomorphic P-224 representative. No input value is rewritten.",
    "new_scientific_computation": False,
}, indent=2) + "\n")
print("Copied 21 native observations and bound 11 source files; no scientific rerun.")
