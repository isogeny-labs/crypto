#!/usr/bin/env python3
"""Run the frozen 5+1 N83 split using the hash-pinned hosted stage runner."""

from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path

import audit
import protocol


HERE = Path(__file__).resolve().parent
RUNNER = HERE.parent / "hosted-sixsum-access-20261025" / "run.py"


def main() -> int:
    protocol.require(hashlib.sha256(RUNNER.read_bytes()).hexdigest()
                     == protocol.LEGACY_RUNNER_SHA256,
                     "pinned hosted stage runner changed")
    spec = importlib.util.spec_from_file_location("n83_pinned_hosted_stage_runner", RUNNER)
    protocol.require(spec is not None and spec.loader is not None,
                     "pinned hosted stage runner unavailable")
    runner = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runner)
    protocol.require(runner.six_protocol is protocol and runner.six_audit is audit,
                     "5+1 protocol or auditor import differs")
    runner.HERE = HERE
    runner.__doc__ = __doc__
    return runner.main()


if __name__ == "__main__":
    raise SystemExit(main())
