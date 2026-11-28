#!/usr/bin/env python3
"""Extend R106's two base local series from 32 to 128 bits."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HASHES = (
    "196593c612e43fe185041373bc481abf4c3a0556a3504b8b0327f26b70ec7ffd",
    "501a0e250ea38997f9520fb864dbbf4c40bf73fd75ac0eabcff61fcccab8efbe",
    "c112aeedde039a3b36376a0e720e00f62da64c5abe336d77296b97baa4cc12b8",
)


def main():
    r105, solver, r106, output = map(Path, sys.argv[1:5])
    for path, expected in zip((r105, solver, r106), HASHES):
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"source hash mismatch: {path}")
    spec = importlib.util.spec_from_file_location("series106", solver)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.PRECISION = 128
    module.MASK = (1 << 128) - 1
    old = json.loads(r106.read_text())
    supports = json.loads(r105.read_text())
    out = {"source_sha256": HASHES, "precision": 128}
    for key, z_power, zv, tv in (("y0", 2, 2, 3), ("y1", 3, 3, 4)):
        new = module.solve([tuple(p) for p in supports[f"f_{key}_support"]], z_power, zv, tv)
        if [i for i in new["z_exponents"] if i < 32] != old[key]["z_exponents"]:
            raise ValueError("R106 z low-bit mismatch")
        if [i for i in new["t_exponents"] if i < 32] != old[key]["t_exponents"]:
            raise ValueError("R106 t low-bit mismatch")
        out[key] = new
    output.mkdir(parents=True, exist_ok=True)
    (output / "result.json").write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print("EXTEND108_ALL_CHECKS_PASS")


if __name__ == "__main__": main()
