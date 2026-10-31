#!/usr/bin/env python3
"""Prepare direct Linux x64 workflow jobs for opt-in ARC routing.

The repository variables are empty by default, so edited jobs run on
`ubuntu-latest` until a tested workflow is explicitly added to
`CRYPTO_EKS_WORKFLOWS` (Spot `crypto-eks-linux`) or
`CRYPTO_EKS_HIGHMEM_WORKFLOWS` (Spot `crypto-eks-highmem`).
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
WORKFLOWS = ROOT / ".github" / "workflows"
DIRECT = re.compile(r"^(\s*runs-on:) (ubuntu-24\.04|ubuntu-latest)(\s*)$", re.M)
# ic-dashboard.yml drives a real Chrome; the ARC runner image has none.
# ecc2k130-pdp-difficulty-protocol.yml is hashed by SOURCE_LOCK.json and SOURCE_CHECK.json
# under research/notes/ecc2k130/pdp_difficulty_features_20261001; rewriting it breaks verify.py.
SEALED = {
    "weak-curve-lean.yml",
    "weak-curve-conjectures.yml",
    "ic-dashboard.yml",
    "ecc2k130-pdp-difficulty-protocol.yml",
    # docs/curves/cover-transfer-assessments.json pins both by SHA-256.
    "release.yml",
    "curve-covers.yml",
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
