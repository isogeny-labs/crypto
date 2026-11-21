#!/usr/bin/env python3
"""Replay one registered local catalog-generation control or candidate arm."""
from __future__ import annotations
import argparse
import contextlib
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import time
import types


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def semantic_digest(document: dict) -> str:
    # Only the generator's documented volatile citation counters are omitted.
    normalized = json.loads(json.dumps(document))
    for row in normalized.get("candidates", []):
        for key in ("mentions", "files", "where"):
            row.pop(key, None)
    return digest(json.dumps(normalized, sort_keys=True, separators=(",", ":"),
                             ensure_ascii=False, allow_nan=False).encode())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--protocol", type=Path, required=True)
    parser.add_argument("--variant", choices=("baseline", "candidate"), required=True)
    parser.add_argument("--workload-commit", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root = Path.cwd().resolve()
    def git(*argv: str) -> bytes:
        return subprocess.check_output(["git", *argv], cwd=root)
    protocol = json.loads(args.protocol.read_bytes())
    head = git("rev-parse", "HEAD").decode().strip()
    if head != args.workload_commit:
        raise ValueError("workload HEAD changed after registration")
    subprocess.run(["git", "diff", "--quiet", "HEAD", "--"], cwd=root, check=True)
    script = "scripts/build_lab_browser.py"
    if args.variant == "baseline":
        source = git("cat-file", "--filters", f"{protocol['baseline_commit']}:{script}")
    else:
        source = (root / script).read_bytes()
    if digest(source) != protocol["source_sha256"][args.variant]:
        raise ValueError("generator source differs from the registered arm")
    for name, sha in protocol["input_sha256"].items():
        if digest((root / name).read_bytes()) != sha:
            raise ValueError(f"registered input changed: {name}")
    module = types.ModuleType("registered_browser_replay")
    module.__file__ = str(root / script)
    exec(compile(source, str(root / script), "exec"), module.__dict__)
    original_build = module.build
    captured = []
    def capture_build():
        result = original_build()
        captured.append(result)
        return result
    module.build = capture_build
    old_argv = sys.argv
    sys.argv = [str(root / script), "--check"]
    message = io.StringIO()
    try:
        start = time.perf_counter()
        with contextlib.redirect_stdout(message):
            result = module.main()
        elapsed = time.perf_counter() - start
    finally:
        sys.argv = old_argv
    if result or len(captured) != 1:
        raise ValueError("complete document correctness check failed")
    semantic = semantic_digest(captured[0])
    if semantic != protocol["semantic_sha256"]:
        raise ValueError("independent scientific-document digest changed")
    record = {
        "schema": "lab-browser-software-replay/v1", "variant": args.variant,
        "workload_commit": head, "generator_sha256": digest(source),
        "protocol_sha256": digest(args.protocol.read_bytes()),
        "semantic_sha256": semantic, "counts": captured[0]["counts"],
        "wall_s": elapsed, "timing_scope": "generator main; excludes module import and independent digest",
        "isolation": "unisolated local software control", "correctness": True,
        "message": message.getvalue().strip(),
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(record, indent=2, allow_nan=False) + "\n")
    print(json.dumps({"variant": args.variant, "wall_s": elapsed, "correctness": True}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
