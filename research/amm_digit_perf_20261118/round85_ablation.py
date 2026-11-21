#!/usr/bin/env python3
"""Three-variant source-exact Rust ablation: Round84 baseline, only shape
shortcuts, all Round85 changes. Production root implementation remains verbatim
within each generated crate, apart from construction of the middle variant.
"""
import ast
import csv
from pathlib import Path
import os
import statistics
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[2]
source_path = "src/cryptanalysis/prime_root_trace.rs"
before = subprocess.check_output(
    ["git", "show", "af1310cf8b41b2839210172eed5403f956e0f1e6:" + source_path],
    text=True, cwd=repo
)
full = (repo / source_path).read_text()
driver_ast = ast.parse((repo / "research/amm_digit_perf_20261118/round85_full_ab.py").read_text())
values = {}
for stmt in driver_ast.body:
    if isinstance(stmt, ast.Assign) and len(stmt.targets) == 1 and isinstance(stmt.targets[0], ast.Name):
        if stmt.targets[0].id in ("main", "cargo"):
            values[stmt.targets[0].id] = ast.literal_eval(stmt.value)
main, cargo = values["main"], values["cargo"]

start = before.index("    fn div(&mut self, a:")
old_alloc = before.index("        let mut r = a.to_vec();", start)
new_start = full.index("        if a.len() < b.len()", full.index("    fn div(&mut self, a:"))
new_alloc = full.index("        let mut r = a.to_vec();", new_start)
shortcuts = before[:old_alloc] + full[new_start:new_alloc] + before[old_alloc:]
assert "for shift in (0..q.len()).rev()" not in shortcuts[start:shortcuts.index("    fn pow(",start)]
assert "if a.len() < b.len()" in shortcuts[start:shortcuts.index("    fn pow(",start)]
assert "for shift in (0..q.len()).rev()" in full[full.index("    fn div("):full.index("    fn pow(")]

try:
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    print(f"CPU pinned to logical {cpu}", flush=True)
except (AttributeError, OSError) as exc:
    print(f"affinity unavailable: {exc}", flush=True)

with tempfile.TemporaryDirectory(prefix="root85-ablate-") as d:
    d = Path(d)
    binaries = {}
    for label, src in [("baseline", before), ("shortcuts", shortcuts), ("full", full)]:
        root = d / label
        (root / "src/cryptanalysis").mkdir(parents=True)
        (root / "Cargo.toml").write_text(cargo)
        (root / "src/main.rs").write_text(main)
        (root / "src/cryptanalysis/prime_root_trace.rs").write_text(src)
        (root / "src/cryptanalysis/prime_root_bounded.rs").write_text("// benchmark stub\n")
        subprocess.run(["cargo", "build", "--release", "--quiet"], cwd=root, check=True)
        binaries[label] = root / "target/release/rootfind-isolated-ab"
    rows = []
    for trial in range(9):
        labels = ["baseline", "shortcuts", "full"]
        labels = labels[trial % 3:] + labels[:trial % 3]
        measured = {}
        for label in labels:
            data = subprocess.check_output([str(binaries[label])], text=True)
            measured[label] = {
                (row["prime"],row["case"]):row for row in csv.DictReader(data.splitlines())
            }
        keys = measured["baseline"].keys()
        assert keys == measured["shortcuts"].keys() == measured["full"].keys()
        for key in keys:
            values_for_key = [measured[label][key] for label in ("baseline", "shortcuts", "full")]
            assert len({(row["checksum"],row["completed"]) for row in values_for_key}) == 1,key
            rows.append({
                "trial":trial,"prime":key[0],"case":key[1],
                **{label+"_ns":measured[label][key]["ns_per_solve_verify"]
                   for label in ("baseline","shortcuts","full")}
            })
    destination=repo/"research/amm_digit_perf_20261118/round85_ablation_results.csv"
    with destination.open("w",newline="") as out:
        w=csv.DictWriter(out,fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    print("prime,case,baseline_ns,shortcuts_ns,full_ns,shortcuts_speedup,full_speedup")
    for key in sorted({(r["prime"],r["case"]) for r in rows}):
        subset = [r for r in rows if (r["prime"],r["case"]) == key]
        med = {name:statistics.median(float(r[name+"_ns"]) for r in subset)
               for name in ("baseline","shortcuts","full")}
        print(f"{key[0]},{key[1]},{med['baseline']:.3f},{med['shortcuts']:.3f},"
              f"{med['full']:.3f},{med['baseline']/med['shortcuts']:.4f},"
              f"{med['baseline']/med['full']:.4f}")
    print(f"ablation: {len(rows)} three-way matched trials; output checksums and completion identical")
