#!/usr/bin/env python3
"""Export every (cell, curve, instance) of a plan once, with every presentation, into systems/<cell>/<curve>/<instance>/.

Usage:  export_all.py --plan plan.json [--instrument BIN] [--only-cell C] [--force]

For each (cell, curve) block of the plan: one `pkm_present_panel export --cell <cell_dir> --targets <targets_dir>
--target all --m M --presentation all --format both --out <staging>` (one export process per block, so the per-cell
construction work is metered once), then the files are renamed to systems/<cell>/<curve>/<instance>/<arm>.{ms,system.json}
(+ manifest.json).  Every `.system.json` / `.ms` the plan references is then checked to exist (loud failure), and the
manifest's sym_applicable / real_count are checked against the plan.  systems/construction.json records, per cell,
curve and arm, `construction_cell_field_multiplications` (target-independent; reported per cell, never added to w^+)
and the per-target construction counts.  Paths are relative to the directory holding plan.json.  [pcc:unverified]
"""
import argparse, hashlib, json, os, shutil, subprocess, sys
from datetime import datetime, timezone
from pathlib import Path


class ExportError(Exception):
    pass


def utcnow():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def sha256_file(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def blocks_of(plan, only_cell=None):
    """-> list of (cell summary, curve summary, plan rows of the block)."""
    out = []
    for cs in plan["cells"]:
        if only_cell and cs["cell"] != only_cell:
            continue
        for cv in cs["curves"]:
            rows = [r for r in plan["rows"] if r["cell"] == cs["cell"] and r["curve"] == cv["curve"]]
            out.append((cs, cv, rows))
    return out


def expected_files(rows):
    return sorted({r["system"] for r in rows if r.get("system")})


def export_block(cs, cv, rows, root, inst, force):
    cell, curve, m = cs["cell"], cs["m"] and cv["curve"], cs["m"]
    cell, curve = cs["cell"], cv["curve"]
    base = root / "systems" / cell / curve
    exp = expected_files(rows)
    if not force and exp and all((root / f).is_file() for f in exp):
        return None, "present"
    stage = base / "_export"
    if stage.exists():
        shutil.rmtree(stage)
    stage.mkdir(parents=True)
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    cmd = [str(inst), "export", "--cell", str((root / cv["cell_dir"]).resolve()), "--targets",
           str((root / cv["targets_dir"]).resolve()), "--target", "all", "--m", str(m), "--presentation", "all",
           "--format", "both", "--out", str(stage)]
    r = subprocess.run(cmd, capture_output=True, text=True, env=env)
    (base / "export.stdout").write_text(r.stdout)
    (base / "export.stderr").write_text(r.stderr)
    if r.returncode != 0:
        raise ExportError(f"export failed for {cell}/{curve} (rc {r.returncode}): {r.stderr.strip()[-400:]}")
    names = sorted({x["instance"] for x in rows})
    for name in names:
        d = base / name
        d.mkdir(parents=True, exist_ok=True)
        man = stage / f"{name}.manifest.json"
        if not man.is_file():
            raise ExportError(f"{cell}/{curve}: no manifest for instance {name} in {stage}")
        shutil.move(str(man), str(d / "manifest.json"))
        for f in sorted(stage.glob(f"{name}.*")):
            rest = f.name[len(name) + 1:]
            for ext in (".system.json", ".ms"):
                if rest.endswith(ext):
                    shutil.move(str(f), str(d / (rest[: -len(ext)] + ext)))
                    break
    left = [p.name for p in stage.iterdir()]
    if left:
        raise ExportError(f"{cell}/{curve}: unexpected files in the export (instances not in the plan?): {left[:5]}")
    stage.rmdir()
    return " ".join(cmd), "exported"


def verify_and_collect(cs, cv, rows, root):
    cell, curve, m = cs["cell"], cv["curve"], cs["m"]
    missing = [f for f in expected_files(rows) if not (root / f).is_file()]
    if missing:
        raise ExportError(f"{cell}/{curve}: {len(missing)} system file(s) the plan references are missing, e.g. {missing[:4]}")
    for r in rows:
        if r.get("status") == "not_applicable" and r.get("system"):
            raise ExportError(f"{cell}/{curve}: not_applicable row with a system path")
    res = {}
    for name in sorted({x["instance"] for x in rows}):
        man = json.loads((root / "systems" / cell / curve / name / "manifest.json").read_text())
        ref = next(x for x in rows if x["instance"] == name)
        if bool(man.get("sym_applicable")) != bool(ref["sym_applicable"]):
            raise ExportError(f"{cell}/{curve}/{name}: manifest sym_applicable {man.get('sym_applicable')} != plan {ref['sym_applicable']}")
        if man.get("real_count") != ref.get("real_count") or man.get("real_count_distinct") != ref.get("real_count_distinct"):
            raise ExportError(f"{cell}/{curve}/{name}: manifest real_count differs from the plan's")
        for e in man["entries"]:
            if e.get("skipped"):
                continue
            a = res.setdefault(e["presentation"], {"per_target_field_multiplications": {}, "cell_values": []})
            a["per_target_field_multiplications"][name] = e.get("construction_field_multiplications")
            a["cell_values"].append(e.get("construction_cell_field_multiplications"))
            a.setdefault("cell_cpu_s", []).append(e.get("construction_cell_cpu_s"))
    out = {}
    for arm, a in res.items():
        vals = [v for v in a["cell_values"] if v is not None]
        if len(vals) != len(a["cell_values"]):
            raise ExportError(f"{cell}/{curve}/{arm}: construction_cell_field_multiplications missing in a manifest entry")
        out[arm] = {"construction_cell_field_multiplications": max(vals), "cell_values_distinct": sorted(set(vals)),
                    "construction_cell_cpu_s_max": max([c for c in a["cell_cpu_s"] if c is not None] or [None]),
                    "per_target_field_multiplications": a["per_target_field_multiplications"], "n_instances": len(vals)}
    return out


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    repo = Path(__file__).resolve().parents[3]
    ap.add_argument("--plan", type=Path, required=True)
    ap.add_argument("--instrument", type=Path, default=repo / "target/release/examples/pkm_present_panel")
    ap.add_argument("--only-cell")
    ap.add_argument("--force", action="store_true", help="re-export even if every referenced file exists")
    a = ap.parse_args(argv)
    plan_path = a.plan.resolve()
    root = plan_path.parent
    sf = root / "plan.sha256"
    if sf.is_file() and sf.read_text().split()[0] != sha256_file(plan_path):
        print("export_all: plan.json does not match plan.sha256", file=sys.stderr)
        return 3
    plan = json.loads(plan_path.read_text())
    if not a.instrument.is_file():
        print(f"export_all: instrument not found: {a.instrument}", file=sys.stderr)
        return 3
    cpath = root / "systems" / "construction.json"
    doc = json.loads(cpath.read_text()) if cpath.is_file() else {"cells": {}}
    doc.update(schema="pkm.present-panel.construction/v1", plan_sha256=sha256_file(plan_path),
               instrument_sha256=sha256_file(a.instrument), updated_utc=utcnow(),
               note="construction_cell_field_multiplications is target-independent and metered once per export process "
                    "(one process per cell, curve); it is reported per cell and never added to w^+")
    try:
        for cs, cv, rows in blocks_of(plan, a.only_cell):
            cmd, state = export_block(cs, cv, rows, root, a.instrument, a.force)
            doc["cells"].setdefault(cs["cell"], {})[cv["curve"]] = verify_and_collect(cs, cv, rows, root)
            if cmd:
                doc["cells"][cs["cell"]][cv["curve"]]["_export_command"] = cmd
            print(f"export_all: {cs['cell']}/{cv['curve']}: {state}", file=sys.stderr)
            cpath.parent.mkdir(parents=True, exist_ok=True)
            cpath.write_text(json.dumps(doc, indent=1))
    except ExportError as e:
        print(f"export_all: {e}", file=sys.stderr)
        return 3
    return 0


if __name__ == "__main__":
    sys.exit(main())
