"""Engine availability, host record, and the frozen m83 verification/preflight scripts.

The frozen scripts write next to themselves, so they are run on a scratch copy of the
m83 directory with the raw receipts archive extracted there; nothing under
../m83_solver_matrix_20260927 is modified.  Output: results/preflight.json.
"""
import base64
import gzip
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time

import tcommon as tc

HERE = Path(__file__).resolve().parent
M83 = HERE.parent / "m83_solver_matrix_20260927"


def extract_archive(dest):
    data = base64.b64decode((M83 / "results" / "raw_receipts.tar.gz.base64").read_bytes())
    with tarfile.open(fileobj=io.BytesIO(gzip.decompress(data)), mode="r") as tar:
        tar.extractall(dest, filter="data")


def main():
    out = {"versions": tc.versions(), "loadavg": os.getloadavg(), "time_unix": time.time()}
    out["binaries"] = {name: shutil.which(name) for name in
                       ("msolve", "WDSat", "sage", "cryptominisat5", "typst", "pandoc", "pdflatex")}
    smoke = {}
    try:
        import pycryptosat
        s = pycryptosat.Solver(threads=1)
        s.add_xor_clause([1, 2], True)
        s.add_clause([1])
        sat, model = s.solve()
        smoke["pycryptosat_native_xor"] = bool(sat) and bool(model[1]) and not model[2]
    except Exception as error:
        smoke["pycryptosat_native_xor"] = f"{type(error).__name__}: {error}"
    try:
        import z3
        x, y = z3.Bools("x y")
        sol = z3.Solver()
        sol.add(z3.Xor(x, y), x)
        smoke["z3_and_xor"] = sol.check() == z3.sat
    except Exception as error:
        smoke["z3_and_xor"] = f"{type(error).__name__}: {error}"
    try:
        sys.path.append(str(M83 / "source" / "f5b"))
        from boolean_f5b import BooleanF5B
        f = BooleanF5B(2, timeout=5)
        basis = f.basis([f.from_terms({0b01, 0b10, 0}), f.from_terms({0b01, 0})])
        smoke["f5b_prototype_import_and_tiny_basis"] = len(basis) > 0
    except Exception as error:
        smoke["f5b_prototype_import_and_tiny_basis"] = f"{type(error).__name__}: {error}"
    out["engine_smoke"] = smoke
    scratch = Path(tempfile.mkdtemp(prefix="m83_scratch_"))
    try:
        copy = scratch / "m83"
        shutil.copytree(M83, copy, ignore=shutil.ignore_patterns("__pycache__"))
        extract_archive(copy / "results")
        env = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
        runs = {}
        for script in ("verify_archive.py", "preflight_engines.py"):
            p = subprocess.run([sys.executable, script], cwd=copy, capture_output=True,
                               text=True, timeout=120, env=env)
            runs[script] = {"returncode": p.returncode, "stdout": p.stdout.strip()[-500:],
                            "stderr": p.stderr.strip()[-500:]}
        rows = json.loads((copy / "results" / "engine_preflight.json").read_text())
        runs["preflight_engines.py"]["engine_rows"] = [r for r in rows if "case" not in r]
        runs["preflight_engines.py"]["unsupported_memory_cases"] = sum(
            r.get("status") == "unsupported_memory" for r in rows)
        out["frozen_m83_scripts"] = runs
    finally:
        shutil.rmtree(scratch, ignore_errors=True)
    out["ok"] = (all(v is True for v in smoke.values()) and
                 all(r["returncode"] == 0 for r in out["frozen_m83_scripts"].values()))
    (HERE / "results" / "preflight.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"ok": out["ok"], "smoke": smoke,
                      "engine_rows": out["frozen_m83_scripts"]["preflight_engines.py"]["engine_rows"],
                      "verify_archive": out["frozen_m83_scripts"]["verify_archive.py"]["stdout"]}, indent=1))


if __name__ == "__main__":
    main()
