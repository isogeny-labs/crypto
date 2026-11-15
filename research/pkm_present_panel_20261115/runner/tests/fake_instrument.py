#!/usr/bin/env python3
"""Stub of pkm_present_panel for runner self-tests: `solve --system F --max-degree N --deadline S --out OUT --extract`.
The behaviour is read from the "fake" object of the system JSON: sleep (s), ops, solve, extract, decomps, drop (field
names to omit from the output, to test the loud failure), cpu_s."""
import json, sys, time
a = sys.argv[1:]
g = lambda f: a[a.index(f) + 1]
if a[0] == "export":
    import os
    cd, td, m = g("--cell"), g("--targets"), int(g("--m"))
    out = g("--out")
    if os.environ.get("FAKE_EXPORT_FAIL"):
        sys.stderr.write("boom\n")
        sys.exit(2)
    breal = json.load(open(cd + "/cell.json"))["B_real"]
    tj = json.load(open(td + "/targets.json"))
    arms = ["chain-tower", "tree-tower", "chain-direct", "mono-tower", "mono-direct", "sym-tower", "sym-direct"]
    arms = [x for x in arms if not (x == "tree-tower" and m != 4) and not (x.startswith("mono") and m != 3)]
    app = breal >= m + 1
    for tg in tj["targets"]:
        ent = []
        for arm in arms:
            if arm.startswith("sym") and not app:
                ent.append({"presentation": arm, "skipped": True})
                continue
            cell_muls = 1000 if arm.startswith("mono") or arm.startswith("sym") else 0
            if os.environ.get("FAKE_EXPORT_SKIP") == arm:
                continue
            json.dump({"construction_field_multiplications": 5, "construction_cell_field_multiplications": cell_muls,
                       "nterms_total": 9, "max_total_degree": 4, "solution_semantics": "ordered_repeats", "nvars": 7,
                       "nequations": 8, "fake": {}}, open(f"{out}/{tg['name']}.{arm}.system.json", "w"))
            open(f"{out}/{tg['name']}.{arm}.ms", "w").write("# stub ms\nwork=10\n")
            ent.append({"presentation": arm, "construction_field_multiplications": 5,
                        "construction_cell_field_multiplications": cell_muls, "construction_cell_cpu_s": 0.0})
        json.dump({"sym_applicable": app, "real_count": tg["real_count"], "real_count_distinct": tg["real_count_distinct"],
                   "entries": ent}, open(f"{out}/{tg['name']}.manifest.json", "w"))
    sys.exit(0)
assert a[0] == "solve", a
sysj = json.load(open(g("--system")))
fk = sysj.get("fake", {})
out = g("--out")
if fk.get("sleep"):
    json.dump({"ops": fk.get("ops", 1000), "degree": 4, "step": 7, "elapsed_s": 0.5}, open(out + ".progress.json", "w"))
    time.sleep(fk["sleep"])
if fk.get("exit"):
    sys.stderr.write(fk.get("stderr", ""))
    sys.exit(fk["exit"])
doc = {"solve_field_multiplications": fk.get("solve", 100), "extract_field_multiplications": fk.get("extract", 10),
       "solving_degree": 3, "degree_reached": 4, "basis_size": 5, "threads_used": 1, "cpu_s": 0.01, "field_ops": 100,
       "censored": fk.get("icens", "no"), "censored_reason": "deadline" if fk.get("icens") == "timeout" else None,
       "verified_real_decompositions": fk.get("decomps", []), "unlifted_solutions": 0,
       "fp_rational_solutions": len(fk.get("decomps", [])),
       "extraction": {"status": fk.get("ext_status", "ok")}}
if "null_verified" in fk:  # extraction not done: nulls, as the instrument writes them
    doc.update(verified_real_decompositions=None, unlifted_solutions=None, fp_rational_solutions=None,
               extract_field_multiplications=0)
    doc["extraction"]["status"] = fk["null_verified"]
for d in fk.get("drop", []):
    doc.pop(d, None)
json.dump(doc, open(out, "w"))
