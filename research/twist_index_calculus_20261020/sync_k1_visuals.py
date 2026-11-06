#!/usr/bin/env python3
"""Copy all source-linked scaling, census and thinning visuals to the unpriced record."""
from __future__ import annotations

import hashlib
import json
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent
BENCH = ROOT.parent.parent / "benchmarks/20261022-quartic-twist-index-calculus"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


if __name__ == "__main__":
    reports = sorted(p for p in (ROOT / "reports").iterdir()
                     if p.is_dir() and ("quartic_twist_k1_scale_" in p.name
                                        or "quartic_twist_k1_census_" in p.name
                                        or "quartic_twist_k1_thin_" in p.name))
    assert len(reports) == 67, f"expected 13 scale, 12 census and 42 thinning reports, got {len(reports)}"
    files: list[tuple[Path, Path]] = []
    for report in reports:
        for original, extension in (("report.pdf", ".pdf"), ("graph.svg", ".svg"),
                                    ("graph.png", ".png")):
            files.append((report / original, BENCH / (report.name + extension)))
    for name in ("QUARTIC_TWIST24_REPORT.pdf", "K1_SCALING_REPORT.pdf",
                 "K1_ORBIT_THINNING.pdf", "K1_THIN_NATIVE_REPORT.pdf",
                 "K1_PEELED_ORBIT_REUSE.pdf",
                 "k1_scaling_overview.svg", "k1_scaling_overview.png",
                 "k1_orbit_thinning.svg", "k1_orbit_thinning.png",
                 "k1_thin_native_overview.svg", "k1_thin_native_overview.png",
                 "k1_peeled_orbit_reuse.svg", "k1_peeled_orbit_reuse.png",
                 "k1_peeled_subgroup_reuse.svg", "k1_peeled_subgroup_reuse.png"):
        files.append((ROOT / name, BENCH / name))
    manifest = []
    for source, target in files:
        shutil.copy2(source, target)
        assert digest(source) == digest(target)
        manifest.append({"source": str(source.relative_to(ROOT.parent.parent)),
                         "copy": str(target.relative_to(ROOT.parent.parent)),
                         "sha256": digest(source)})
    out = {"status": "pass", "reports": len(reports), "file_copies": len(manifest),
           "files": manifest}
    (BENCH / "K1_VISUALS.json").write_text(json.dumps(out, sort_keys=True, indent=2) + "\n")
    print(f"copied {len(reports)} reports, verified {len(manifest)} files")
