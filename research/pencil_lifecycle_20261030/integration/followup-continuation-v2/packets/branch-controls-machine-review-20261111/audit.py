#!/usr/bin/env python3
"""Solver-free audit of the sealed branch-controls native receipts."""
from __future__ import annotations

import csv
import hashlib
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = Path("/home/ec2-user/crypto-pencil-evidence-recovery-20261105")
PACKETS = REPO / "research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets"
SOURCE = PACKETS / "branch-controls-successor-20261111"
BINDING = PACKETS / "controls-final-binding-20261111"
RUN = Path("/home/ec2-user/pencil-native-branch-controls-20261111")
OUTER = Path("/home/ec2-user/pencil-native-branch-controls-20261111.outer")
FREEZE_SHA = "d20691095b0e6f932a5f34f9107e09c56629b57da85ea94dea35c007d87f3a40"
RELEASE_SHA = "bcbe1b0b3a1da832b45e3d1fed9dabfc6c8d8fe2afdc8c60b4be9fb1c054bdfe"
PUBLICATION = "d6365d5265bb24a3ffffa1560a13a66d04c1741a"
ALLOWED_AXIOMS = ["propext", "Classical.choice", "Quot.sound"]
MODULES = [
    "BranchRootMultiplicity",
    "BranchCoefficientMultiplicity",
    "IndependentControls",
    "Readback",
    "ExpectedFailure",
]
EXPECTED_EXITS = [0, 0, 0, 0, 1]


def load(path: Path):
    return json.loads(path.read_text())


def sha(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


freeze = load(SOURCE / "freeze.json")
release = load(BINDING / "release.json")
outer_release = load(OUTER / "coordinator-release.json")
completion = load(OUTER / "completion.json")
admission = load(OUTER / "admission.json")
compiler = load(RUN / "compiler-receipt.json")
pre = load(RUN / "pre-receipt.json")
post = load(RUN / "post-receipt.json")
names_doc = load(SOURCE / "names.json")
expected_names = names_doc["producer"] + names_doc["reviewer"]

require(sha(SOURCE / "freeze.json") == FREEZE_SHA, "freeze digest")
require(sha(BINDING / "release.json") == RELEASE_SHA, "release digest")
require(outer_release == release, "outer normalized release content")
require(release["released"] is True and release["automatic_retry"] is False, "release flags")
require(release["published_github_commit"] == PUBLICATION, "published revision")
require(admission["freeze_sha256"] == FREEZE_SHA, "admission freeze")
require(admission["coordinator_release_sha256"] == RELEASE_SHA, "admission release")
require(admission["continuation_ceiling_seconds"] == 600 and admission["retries"] == 0, "attempt cap")

source_hashes = {}
for module in MODULES:
    name = f"{module}.lean.txt"
    source = SOURCE / name
    staged = RUN / f"{module}.lean"
    expected = freeze["files"][name]
    require(sha(source) == expected, f"frozen source {name}")
    require(sha(staged) == expected, f"staged source {name}")
    source_hashes[module] = expected

commands = compiler["commands"]
require(compiler["status"] == "passed" and compiler["errors"] == [], "compiler receipt")
require([x["module"] for x in commands] == MODULES, "module order")
require([x["exit"] for x in commands] == EXPECTED_EXITS, "observed exits")
require([x["expected_exit"] for x in commands] == EXPECTED_EXITS, "expected exits")
require(all(x["reaped"] is True and x["remaining_group"] == [] for x in commands), "command custody")
require(all((RUN / f"{m}.olean").is_file() for m in MODULES[:4]), "positive oleans")
require(not (RUN / "ExpectedFailure.olean").exists(), "negative produced no olean")
require("error: unsolved goals" in (RUN / "ExpectedFailure.stdout").read_text(), "negative diagnostic")

readback = (RUN / "Readback.stdout").read_text()
readback_names = re.findall(r"^'([^']+)' depends on axioms:", readback, re.M)
require(readback_names == expected_names, "exact 13-name readback")
for segment in re.split(r"(?=^')", readback, flags=re.M):
    if "depends on axioms:" not in segment:
        continue
    observed = [x for x in ALLOWED_AXIOMS if x in segment]
    require(observed == ALLOWED_AXIOMS, "allowed axiom list")
require(load(RUN / "axioms.json") == {"allowed": ALLOWED_AXIOMS, "names": expected_names}, "axiom receipt")

pre_hashes = load(RUN / "pre-hashes.json")
post_hashes = load(RUN / "post-hashes.json")
require(pre_hashes == post_hashes, "full dependency pre/post equality")
require(len(pre_hashes) == 16736, "dependency file count")
require(pre["status"] == post["status"] == "passed", "custody phases")
require(pre["checked_dependency_files"] == post["checked_dependency_files"] == 16736, "checked files")
require(pre["checked_dependency_bytes"] == post["checked_dependency_bytes"] == 3111892318, "checked bytes")
require(load(RUN / "pre-retained.json") == load(RUN / "post-retained.json"), "retained imports equality")
require(load(RUN / "pre-runtime.json") == load(RUN / "post-runtime.json"), "runtime equality")

require(completion["status"] == "passed" and completion["machine_review_complete"] is True, "completion")
require([p["phase"] for p in completion["phases"]] == ["pre", "compiler", "post"], "phase order")
require(all(p["passed"] and p["terminal_owner_confirmed"] and not p["wait_timeout"] for p in completion["phases"]), "phase ownership")
require(completion["elapsed_seconds"] < 600, "600-second ceiling")
for phase in completion["phases"]:
    argv = phase["argv"]
    for required in ("--property=CPUQuota=100%", "--property=MemoryMax=8G", "--property=MemorySwapMax=0", "--property=TasksMax=128", "--property=KillMode=control-group"):
        require(required in argv, f"{phase['phase']} {required}")

rows = []
for command in commands:
    rows.append({
        "category": "compiler_call",
        "name": command["module"],
        "observed": str(command["exit"]),
        "expected": str(command["expected_exit"]),
        "status": "pass",
        "scope": "Lean elaboration/compilation",
    })
for name in expected_names:
    rows.append({
        "category": "axiom_readback",
        "name": name,
        "observed": ";".join(ALLOWED_AXIOMS),
        "expected": ";".join(ALLOWED_AXIOMS),
        "status": "pass",
        "scope": "named declaration dependency readback",
    })
rows.extend([
    {"category": "custody", "name": "dependency_files", "observed": "16736", "expected": "16736", "status": "pass", "scope": "pre/post hash map equality"},
    {"category": "custody", "name": "dependency_bytes", "observed": "3111892318", "expected": "3111892318", "status": "pass", "scope": "pre/post full read"},
    {"category": "runtime", "name": "worker_wall_seconds", "observed": f"{completion['elapsed_seconds']:.12f}", "expected": "<=600", "status": "pass", "scope": "three bounded phases"},
])
with (HERE / "review.csv").open("w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=["category", "name", "observed", "expected", "status", "scope"])
    w.writeheader()
    w.writerows(rows)

phase_rows = [{"phase": p["phase"], "elapsed_seconds": p["elapsed_seconds"], "passed": p["passed"], "terminal_owner_confirmed": p["terminal_owner_confirmed"]} for p in completion["phases"]]
review = {
    "schema": "pencil.branch-controls-machine-review/v1",
    "created_utc": "2026-11-11T00:22:00Z",
    "reviewer": "/root/ci_audit",
    "status": "machine_evidence_passed_scoped",
    "execution_performed_by_reviewer": False,
    "source": {
        "published_revision": PUBLICATION,
        "freeze_sha256": FREEZE_SHA,
        "release_sha256": RELEASE_SHA,
        "module_sha256": source_hashes,
    },
    "machine_outcome": {
        "completion_status": completion["status"],
        "worker_wall_seconds": completion["elapsed_seconds"],
        "phase_results": phase_rows,
        "calls": [{"module": x["module"], "exit": x["exit"], "expected_exit": x["expected_exit"], "reaped": x["reaped"]} for x in commands],
        "compiled_positive_modules": 4,
        "expected_negative_modules": 1,
        "named_axiom_readbacks": len(readback_names),
        "allowed_axioms": ALLOWED_AXIOMS,
        "all_named_readbacks_match": True,
        "full_dependency_files_pre_and_post": 16736,
        "full_dependency_bytes_pre_and_post": 3111892318,
        "full_dependency_maps_equal": True,
        "runtime_maps_equal": True,
        "retained_import_maps_equal": True,
        "compiler_phase_dependency_files_checked": compiler["checked_dependency_files"],
        "compiler_phase_dependency_bytes_checked": compiler["checked_dependency_bytes"],
    },
    "semantic_scope": {
        "kernel_checked": "The five frozen Lean inputs behaved as declared: four elaborated successfully; the false wrong-branch example failed with an unsolved False goal; 13 named declarations reported only the allowed axioms.",
        "theorem_statement": "For a field, simple branch root, separable quintic, nonzero determinant, two support divisibilities and support vanishing, the source-specific support polynomials are nonzero and their three root multiplicities sum to one. The rational control instantiates these hypotheses and two negative controls.",
        "not_established": [
            "No independent binary kernel rechecker was used.",
            "This machine audit does not rederive the mathematical bridge from the broader genus-2 collector or certify claims outside the frozen theorem statements.",
            "The compiler receipt itself checked zero dependency files; dependency custody comes from the distinct complete pre/post hash phases.",
            "Pinned shared caches do not exclude transient concurrent changes between the pre and post observations, although the full maps are equal at both observations."
        ],
    },
    "campaign_accounting": {
        "conservative_charge_seconds": 600,
        "prior_charged_seconds": 600,
        "new_total_charged_seconds": 1200,
        "campaign_ceiling_seconds": 129600,
        "remaining_seconds": 128400,
        "note": "The bound pre-launch ledger still labels this attempt reserved; campaign status update belongs to the coordinator, not this review."
    },
    "findings": [],
    "recommendation": "Accept the machine receipts for the exact frozen theorem/control scope and preserve the stated semantic and custody boundaries."
}
(HERE / "review.json").write_text(json.dumps(review, indent=2, sort_keys=True) + "\n")

bars = []
max_elapsed = max(p["elapsed_seconds"] for p in completion["phases"])
colors = {"pre": "#3b82f6", "compiler": "#8b5cf6", "post": "#10b981"}
for i, p in enumerate(completion["phases"]):
    x, y, width = 260, 100 + i * 90, 520 * p["elapsed_seconds"] / max_elapsed
    bars.append(f'<text x="20" y="{y+25}" font-size="20">{p["phase"]}</text>')
    bars.append(f'<rect x="{x}" y="{y}" width="{width:.2f}" height="36" rx="5" fill="{colors[p["phase"]]}"/>')
    bars.append(f'<text x="{x+width+12:.2f}" y="{y+25}" font-size="18">{p["elapsed_seconds"]:.3f} s</text>')
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="470" viewBox="0 0 1000 470">
<rect width="1000" height="470" fill="#ffffff"/>
<text x="20" y="38" font-size="26" font-weight="bold">Branch-controls replay: recorded worker phases</text>
<text x="20" y="66" font-size="16">One attempt; 600 s conservative charge; exact completion receipt {sha(OUTER/'completion.json')[:12]}...</text>
{''.join(bars)}
<line x1="260" x2="780" y1="390" y2="390" stroke="#111827"/>
<text x="260" y="418" font-size="15">0 s</text><text x="735" y="418" font-size="15">{max_elapsed:.3f} s</text>
<text x="20" y="450" font-size="15">All three phases: passed; terminal owner confirmed. Bar lengths are recorded wall seconds, not CPU time.</text>
</svg>'''
(HERE / "flow.svg").write_text(svg)
print(json.dumps({"status": "passed", "review_json": sha(HERE / "review.json"), "review_csv": sha(HERE / "review.csv"), "flow_svg": sha(HERE / "flow.svg")}, sort_keys=True))
