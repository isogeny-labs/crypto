#!/usr/bin/env python3
"""Prepare direct Linux x64 workflow jobs for opt-in ARC routing.

The repository variables are empty by default, so edited jobs continue to use
their original GitHub-hosted labels until a tested workflow is explicitly added.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
WORKFLOWS = ROOT / ".github" / "workflows"
DIRECT = re.compile(r"^(\s*runs-on:) (ubuntu-24\.04|ubuntu-latest)(\s*)$", re.M)
# Workflows whose exact bytes are pinned by sealed or frozen research evidence.
# Rewriting runs-on changes those bytes and breaks the pin (c1897c2c7 did so
# for the second group), so they keep their original GitHub-hosted labels.
SEALED = {
    "weak-curve-lean.yml",
    "weak-curve-conjectures.yml",
    "ecc2k130-dag-linux-v2-outcome-replay.yml",
    "ecc2k130-factor-base-orbit-holdouts.yml",
    "ecc2k130-leaf-m10-support.yml",
    "ecc2k130-m10-capacity-once.yml",
    "ecc2k130-m3-base-selector-source.yml",
    "ecc2k130-m3-four-policy.yml",
    "ecc2k130-n13-oaware-sat-benchmark.yml",
    "ecc2k130-n13-s5-fullpoint-control.yml",
    "ecc2k130-n13-s5-solver-panel.yml",
    "ecc2k130-n19-s6-fullpoint-exporter.yml",
    "ecc2k130-native-fullpoint-edge.yml",
    "ecc2k130-native-m3-chain.yml",
    "ecc2k130-oaware-sparse-dense-sat.yml",
    "ecc2k130-phase-charged-successor-design.yml",
    "ecc2k130-rotated-nearthreshold-slot.yml",
    "ecc2k130-rotated-phase-selector.yml",
    "ecc2k130-rotated-slot-placement.yml",
    "ecc2k130-rotated-solver-admission.yml",
    "ecc2k130-s3-sparse-cnf.yml",
    "ecc2k130-s3-sparse-n19-growth.yml",
    "ecc2k130-s3-sparse-n19-linear-replay.yml",
    "ecc2k130-symbolic-dag-dimacs-gate.yml",
    "ecc2k130-symbolic-dag-dimacs-linux-preparation.yml",
    "ecc2k130-symbolic-dag-dimacs-linux-v2-check.yml",
    "ecc2k130-symbolic-dag-dimacs-linux-v2-measure.yml",
    "ecc2k130-symbolic-dag-fullpoint.yml",
    "ecc2k130-symbolic-oaware-width-gate.yml",
    "endomorphism-search.yml",
    "n53-target-cyclic-rank.yml",
    "p192-geometry-collector-lean.yml",
}
TRUSTED = (
    "(github.event_name == 'push' || github.event_name == 'schedule' || "
    "github.event_name == 'release' || github.event_name == 'workflow_dispatch' || "
    "(github.event_name == 'pull_request' && "
    "github.event.pull_request.head.repo.full_name == github.repository))"
)


def routed_line(prefix: str, original: str, trailing: str, filename: str) -> str:
    marker = f"'|{filename}|'"
    highmem = f"contains(vars.CRYPTO_EKS_HIGHMEM_WORKFLOWS, {marker})"
    normal = f"contains(vars.CRYPTO_EKS_WORKFLOWS, {marker})"
    selection = (
        f"{TRUSTED} && {highmem} && 'crypto-eks-highmem' || "
        f"{TRUSTED} && {normal} && 'crypto-eks-linux' || '{original}'"
    )
    return f"{prefix} ${{{{ {selection} }}}}{trailing}"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="write the routing lines")
    args = parser.parse_args()
    files = jobs = 0
    for path in sorted(WORKFLOWS.glob("*.y*ml")):
        if path.name in SEALED:
            continue
        before = path.read_text()
        after, count = DIRECT.subn(
            lambda m: routed_line(m.group(1), m.group(2), m.group(3), path.name),
            before,
        )
        if count:
            files += 1
            jobs += count
            if args.apply:
                path.write_text(after)
    print(f"{'Updated' if args.apply else 'Would update'} {jobs} jobs in {files} workflows")


if __name__ == "__main__":
    main()
