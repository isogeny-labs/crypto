"""Which switches change a result, and a record of them (repair item R9).

A run record that omits an environment variable or flag that changes what a
producer computes cannot be compared with another run.  Every harness that
spawns a measured producer calls :func:`record` on the environment it hands
the child and on its own parsed arguments, and writes the answer next to the
run's other artifacts.

The environment is recorded by name pattern, not by a list of known
variables: a new ``KIC_*`` or ``F4_*`` switch is recorded the day it is
added.  ``tools/test_result_switches.py`` scans the Rust sources for every
environment variable they read and fails if one matches neither
``RECORDED_PREFIXES``/``RECORDED_EXACT`` nor ``NOT_RESULT_AFFECTING``.
"""
from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path
from typing import Any, Mapping

SCHEMA = "result-switches/1"

# Producers read these families (see the scan in the test).
RECORDED_PREFIXES = (
    "KIC_", "F4_", "IC_", "ICX_", "GB_", "GAUDRY_", "KOBLITZ_", "JV_", "BSGS_",
    "BTW_", "DREG_", "SOLVER_", "SPOT_", "SOLINAS_", "TRAITS_", "WDSAT_",
    "ECBENCH_", "DQ_", "W1_", "N37_",
)
RECORDED_EXACT = (
    "RAYON_NUM_THREADS", "OMP_NUM_THREADS", "CUDA_VISIBLE_DEVICES",
    "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CC", "CFLAGS", "HASHCLASH_PREFIX",
)
# Read by the sources but cannot change a computed value.
NOT_RESULT_AFFECTING = frozenset({"HOSTNAME", "PATH", "TMPDIR", "CRYPTO_AUDIT_DIR", "TASKQ_TASK_ID"})


def is_recorded(name: str) -> bool:
    return name in RECORDED_EXACT or name.startswith(RECORDED_PREFIXES)


def result_env(env: Mapping[str, str]) -> dict[str, str]:
    return {k: str(env[k]) for k in sorted(env) if is_recorded(k)}


def _jsonable(value: Any) -> Any:
    if isinstance(value, (str, int, float, bool)) or value is None:
        return value
    if isinstance(value, (list, tuple)):
        return [_jsonable(v) for v in value]
    if isinstance(value, dict):
        return {str(k): _jsonable(v) for k, v in value.items()}
    return str(value)


def record(env: Mapping[str, str], args: Any = None, **extra: Any) -> dict[str, Any]:
    """The switches of one launch: the recorded environment, every parsed
    argument with its resolved value, and anything the caller adds (beat
    config, overrides handed to one arm)."""
    doc: dict[str, Any] = {"schema": SCHEMA, "env": result_env(env)}
    doc["env_sha256"] = hashlib.sha256(
        "\n".join(f"{k}={v}" for k, v in doc["env"].items()).encode()).hexdigest()
    if args is not None:
        ns = args if isinstance(args, dict) else vars(args)
        doc["args"] = {k: _jsonable(v) for k, v in sorted(ns.items()) if not callable(v)}
    for key, value in extra.items():
        doc[key] = _jsonable(value)
    return doc


def write(path: Path, env: Mapping[str, str], args: Any = None, **extra: Any) -> dict[str, Any]:
    """Write the record; a second write with different content (a resume under
    another environment) goes to ``<stem>.<n>.json`` instead of overwriting."""
    doc = record(env, args, **extra)
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    target, n = path, 0
    while target.exists():
        try:
            same = json.loads(target.read_text()) == doc
        except (OSError, ValueError):
            same = False
        if same:
            return doc
        n += 1
        target = path.with_name(f"{path.stem}.{n}{path.suffix}")
    target.write_text(json.dumps(doc, indent=1, sort_keys=True) + "\n")
    return doc


def rust_env_names(root: Path, subdirs: tuple[str, ...]) -> set[str]:
    """Every environment variable the Rust sources under ``subdirs`` read."""
    pat = re.compile(r'env::var(?:_os)?\(\s*"([A-Za-z0-9_]+)"')
    names: set[str] = set()
    for sub in subdirs:
        base = root / sub
        files = [base] if base.is_file() else base.rglob("*.rs")
        for f in files:
            names.update(pat.findall(f.read_text(errors="replace")))
    return names
