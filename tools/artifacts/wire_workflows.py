#!/usr/bin/env python3
"""Add an artifact-store pull step to every workflow that names moved data.

After `artifacts.py push` moves files out of Git, a workflow that reads them
(a replay of `research/x/`, a check over `experiments/y/results/`) would find
pointers only. This finds, per workflow file, the repository paths it names
that hold moved files, and inserts after each `actions/checkout` step:

    - name: Pull store data this workflow reads (tools/artifacts)
      env: {AWS_ACCESS_KEY_ID: ..., AWS_SECRET_ACCESS_KEY: ..., AWS_REGION: ...}
      run: |
        if ! command -v aws ...; then (install the AWS CLI into $RUNNER_TEMP); fi
        python3 tools/artifacts/artifacts.py pull research/x experiments/y/results

A sparse checkout also gets `tools/artifacts` and `.artifacts` (the store
configuration) added to its patterns. The edit is textual (comments and
layout survive) and idempotent; checkouts into a `path:` other than the
workspace are reported, not edited.

    python3 tools/artifacts/wire_workflows.py [--check] [WORKFLOW ...]

`--check` edits nothing and exits 1 if any workflow lacks the step it needs,
which is how CI keeps new workflows honest.
"""

from __future__ import annotations

import argparse
import fnmatch
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import artifacts  # noqa: E402

STEP_NAME = "Pull store data this workflow reads (tools/artifacts)"
PATH_RE = re.compile(r"(?<![A-Za-z0-9_./*-])((?:[A-Za-z0-9_+*-][A-Za-z0-9_.+*-]*/)+[A-Za-z0-9_.+*-]*)")


def moved_paths(root: Path) -> set[str]:
    out = set()
    for d, name, _ in artifacts.selected(root, []):
        drel = artifacts.rel(root, d)
        out.add(f"{drel}/{name}" if drel else name)
    return out


def moved_dirs(moved: set[str]) -> set[str]:
    """Directories specific enough to identify a moved-data subtree."""
    return {
        parent
        for p in moved
        for n in range(1, p.count("/") + 1)
        if (parent := p.rsplit("/", n)[0]).count("/") >= 1
    }


def needed(text: str, moved: set[str], dirs: set[str]) -> list[str]:
    """Paths the workflow names that are, or contain, moved files; minimal cover."""
    found = set()
    # A sibling script may read data in the same artifact directory. Do not
    # infer that it reads every artifact in a broad ancestor such as
    # research/notes: many independent studies share that namespace.
    artifact_dirs = {p.rsplit("/", 1)[0] for p in moved if "/" in p}
    # YAML comments name paths without reading them.
    text = "\n".join(line for line in text.split("\n") if not line.lstrip().startswith("#"))
    for m in PATH_RE.finditer(text):
        raw = m.group(1)
        head = raw.split("*", 1)[0]
        p = head.rstrip("/.")
        if "*" not in raw:
            if p not in moved and p not in dirs:
                # A workflow can name a script or input in the directory
                # holding moved data. Pull that directory, while leaving
                # scripts in shared ancestor namespaces alone.
                while p and p not in artifact_dirs:
                    p = p.rsplit("/", 1)[0] if "/" in p else ""
            if p:
                found.add(p)
            continue
        # A glob names the files it matches: walk up to the directory holding
        # the moved files only when some moved file matches the whole pattern
        # (`*` spans `/`, as fnmatch does). A plain path that merely sits
        # beside moved data (research/other_study/scripts next to a moved
        # research/x/) names none.
        pat = raw + "*" if raw.endswith("/") else raw
        if any(fnmatch.fnmatchcase(x, pat) or fnmatch.fnmatchcase(x, pat + "/*") for x in moved):
            while p and p not in dirs:
                p = p.rsplit("/", 1)[0] if "/" in p else ""
            if p:
                found.add(p)
    return sorted(p for p in found if not any(p.startswith(q + "/") for q in found))


def step_lines(indent: str, paths: list[str]) -> list[str]:
    i2 = indent + "  "
    return [
        f"{indent}- name: {STEP_NAME}",
        f"{i2}shell: bash",
        f"{i2}working-directory: ${{{{ github.workspace }}}}",
        f"{i2}env:",
        f"{i2}  AWS_ACCESS_KEY_ID: ${{{{ secrets.ARTIFACTS_AWS_ACCESS_KEY_ID }}}}",
        f"{i2}  AWS_SECRET_ACCESS_KEY: ${{{{ secrets.ARTIFACTS_AWS_SECRET_ACCESS_KEY }}}}",
        f"{i2}  AWS_REGION: ${{{{ vars.ARTIFACTS_AWS_REGION || 'us-west-2' }}}}",
        f"{i2}run: |",
        # A pinned, isolated CLI works on Linux, macOS and Windows without sudo.
        f"{i2}  if ! command -v aws >/dev/null; then",
        f'{i2}    python3 -m venv "$RUNNER_TEMP/artifacts-awscli"',
        f'{i2}    artifacts_cli_python="$RUNNER_TEMP/artifacts-awscli/bin/python"',
        f'{i2}    artifacts_cli_aws="$RUNNER_TEMP/artifacts-awscli/bin/aws"',
        f'{i2}    if [ -d "$RUNNER_TEMP/artifacts-awscli/Scripts" ]; then',
        f'{i2}      artifacts_cli_python="$RUNNER_TEMP/artifacts-awscli/Scripts/python.exe"',
        f'{i2}      artifacts_cli_aws="$RUNNER_TEMP/artifacts-awscli/Scripts/aws.exe"',
        f'{i2}    fi',
        f'{i2}    "$artifacts_cli_python" -m pip install --disable-pip-version-check awscli==1.46.2',
        f'{i2}    export ARTIFACTS_AWS="$artifacts_cli_aws"',
        f"{i2}  fi",
        f"{i2}  python3 tools/artifacts/artifacts.py pull --jobs 16 {' '.join(paths)}",
    ]


def wire(text: str, paths: list[str]) -> tuple[str, list[str]]:
    """Insert the pull step after each workspace checkout; returns (text, notes)."""
    lines = text.split("\n")
    notes: list[str] = []
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r"^(\s*)- [A-Za-z_-]+:", line)
        if not m:
            out.append(line)
            i += 1
            continue
        indent = m.group(1)
        j = i + 1
        while j < len(lines) and (not lines[j].strip() or len(lines[j]) - len(lines[j].lstrip()) > len(indent)):
            j += 1
        block = lines[i:j]
        while block and not block[-1].strip():  # keep trailing blank lines after the inserted step
            block.pop()
            j -= 1
        is_checkout = any(re.match(rf"^{indent}(?:- |  )uses: actions/checkout@", b) for b in block)
        if not is_checkout:
            out.append(line)
            i += 1
            continue
        if any(re.match(rf"^{indent}    path: ", b) for b in block):
            notes.append("checkout into a `path:` left unwired")
            out.extend(block)
            i = j
            continue
        block = add_sparse_pattern(block, indent)
        out.extend(block)
        k = next((k for k in range(j, len(lines)) if lines[k].strip()), len(lines))
        if k < len(lines) and STEP_NAME in lines[k]:
            # A step inserted earlier: replace it, so a changed template or
            # path list regenerates (and --check flags the stale copy).
            out.extend(lines[j:k])
            j = k + 1
            while j < len(lines) and (not lines[j].strip() or len(lines[j]) - len(lines[j].lstrip()) > len(indent)):
                j += 1
            while j > k + 1 and not lines[j - 1].strip():
                j -= 1
        step = step_lines(indent, paths)
        cond = next((re.sub(rf"^{indent}(?:- |  )", "", b) for b in block
                     if re.match(rf"^{indent}(?:- |  )if: ", b)), None)
        if cond:  # skipped with its checkout, so it never runs against an empty workspace
            step.insert(1, f"{indent}  {cond}")
        out.extend(step)
        i = j
    return "\n".join(out), notes


def add_sparse_pattern(block: list[str], indent: str) -> list[str]:
    """Make a sparse checkout also fetch what the pull step runs on: the tool
    (tools/artifacts) and the store configuration (.artifacts/config.json)."""
    for k, b in enumerate(block):
        inline = re.match(rf"^({indent}    sparse-checkout: )(?!\|)(\S.*)$", b)
        if inline and "${{" in inline.group(2):
            return block
        if not inline and not re.match(rf"^{indent}    sparse-checkout: \|", b):
            continue
        cone = not any("sparse-checkout-cone-mode: false" in x for x in block)
        if inline:  # single-line form (`sparse-checkout: some/path`): rewrite as a block scalar
            pad = indent + "      "
            head, items, n = [inline.group(1) + "|"], [pad + inline.group(2).strip()], k + 1
        else:
            head, items, n = [b], [], k + 1
            while n < len(block) and len(block[n]) - len(block[n].lstrip()) > len(indent) + 4:
                items.append(block[n])
                n += 1
            pad = (items[0][: len(items[0]) - len(items[0].lstrip())]) if items else indent + "      "
        have = [x.strip() for x in items]
        if "/*" in have:
            return block
        add = []
        if not any("tools/artifacts" in x or x in ("tools", "/tools/") for x in have):
            add.append("tools/artifacts" if cone else "/tools/artifacts/")
        if not any(x.strip("/") in (".artifacts", ".artifacts/config.json") for x in have):
            add.append(".artifacts" if cone else "/.artifacts/")
        if inline or add:
            return block[:k] + head + items + [pad + x for x in add] + block[n:]
        return block
    """Make a sparse checkout also fetch tools/artifacts and .artifacts (the
    store config the pull step reads; without it the remote is empty)."""
    cone = not any("sparse-checkout-cone-mode: false" in x for x in block)
    want = [("tools/artifacts", "/tools/artifacts/", ("tools", "/tools/")),
            (".artifacts", "/.artifacts/", ())]
    for k, b in enumerate(block):
        scalar = re.match(rf"^({indent}    sparse-checkout: )(?!\|)(\S.*)$", b)
        if scalar and "${{" in scalar.group(2):
            # A conditional empty value means a full checkout. Appending literal
            # patterns would silently narrow it to the artifact bootstrap only.
            return block
        if scalar:  # one literal pattern on the key line: rewrite as a block list
            pad = indent + "      "
            block = block[:k] + [f"{scalar.group(1)}|", pad + scalar.group(2)] + block[k + 1:]
            b = block[k]
        if re.match(rf"^{indent}    sparse-checkout: \|", b):
            items = []
            n = k + 1
            while n < len(block) and len(block[n]) - len(block[n].lstrip()) > len(indent) + 4:
                items.append(block[n])
                n += 1
            if any(x.strip() == "/*" for x in items):
                return block
            pad = (items[0][: len(items[0]) - len(items[0].lstrip())]) if items else indent + "      "
            extra = [pad + (c if cone else nc) for c, nc, covers in want
                     if not any(c in x or x.strip() in covers for x in items)]
            return block[:n] + extra + block[n:]
    return block


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("workflows", nargs="*")
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    root = artifacts.repo_root()
    moved = moved_paths(root)
    # Keep matching paths specific to a data subtree. Treating a top-level
    # directory such as `research/` as a hit would make every workflow that
    # mentions any research path pull the entire store.
    dirs = moved_dirs(moved)
    files = [Path(w) for w in args.workflows] or sorted((root / ".github/workflows").glob("*.y*ml"))
    missing = 0
    for f in files:
        text = f.read_text()
        paths = needed(text, moved, dirs)
        if not paths:
            continue
        new, notes = wire(text, paths)
        for n in notes:
            print(f"{f.name}: {n}", file=sys.stderr)
        if new != text:
            missing += 1
            if args.check:
                print(f"{f.name}: needs `{STEP_NAME}` for {' '.join(paths)}")
            else:
                f.write_text(new)
                print(f"{f.name}: pull {' '.join(paths)}")
    if args.check and missing:
        print(f"\n{missing} workflow(s) read store data without pulling it; run "
              f"python3 tools/artifacts/wire_workflows.py", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
