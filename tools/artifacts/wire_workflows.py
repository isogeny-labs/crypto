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

A sparse checkout also gets `tools/artifacts` added to its patterns. The edit
is textual (comments and layout survive) and idempotent; checkouts into a
`path:` other than the workspace are reported, not edited.

    python3 tools/artifacts/wire_workflows.py [--check] [WORKFLOW ...]

`--check` edits nothing and exits 1 if any workflow lacks the step it needs,
which is how CI keeps new workflows honest.
"""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
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


GATE_RUN_RE = re.compile(r"\bpython3?\s+(?:\./)?scripts/(?:test_)?validation_gate\.py\b")


def reads_validation_gate(text: str) -> bool:
    """True when a workflow step runs the validation gate or its tests.

    A mention in a `paths:` filter or a comment does not count: those do not
    open the evidence directories. The IC leaderboard builder reads the gate
    for the leaderboard rows only and keeps its own pull list.
    """
    return any(GATE_RUN_RE.search(line) for line in text.split("\n") if not line.lstrip().startswith("#"))


def _evidence_file(base: Path, name: object, root: Path) -> str | None:
    """Repository-relative path of `name` under an evidence directory, or None."""
    if not isinstance(name, str) or not name:
        return None
    p = Path(os.path.normpath(base / name))
    try:
        return p.relative_to(root).as_posix()
    except ValueError:
        return None


def admitted_evidence_dirs(root: Path) -> tuple[set[str] | None, str]:
    """Evidence directories the gate opens: those of the admitted benchmarks rows
    and of the leaderboard rows (the gate's own readers), or (None, reason) when
    those tables cannot be read here, in which case callers take every directory.
    """
    try:
        import importlib.util
        spec = importlib.util.spec_from_file_location("_vg_for_wire", root / "scripts" / "validation_gate.py")
        vg = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = vg
        spec.loader.exec_module(vg)
        out = {f"benchmarks/{d}" for d in vg.benchmark_groups((root / vg.BENCHMARKS_README).read_text())}
        out |= {vg.board_evidence_dir(r) for r in json.loads((root / vg.LEADERBOARD_JSON).read_text())["board"]}
        return {d.rstrip("/") for d in out}, ""
    except Exception as e:  # noqa: BLE001 - any unreadable table means "take all"
        return None, f"{type(e).__name__}: {e}"


def evidence_store_files(root: Path, moved: set[str]) -> tuple[dict[str, set[str]], list[str]]:
    """Store-backed files that the validation gate reads, per VALIDATION.json.

    The gate (scripts/validation_gate.py, `check_evidence_dir`) opens, for each
    evidence directory, the pinned replay certificate and validator report and
    every `raw_data` file the report lists. Any of those that sits in the
    artifact store must be hydrated before the gate runs. Returns
    ({VALIDATION.json path: {store-backed files it reads}}, notes).
    """
    need: dict[str, set[str]] = {}
    notes: list[str] = []
    admitted, why = admitted_evidence_dirs(root)
    if admitted is None:
        notes.append(f"admitted rows unreadable ({why}); every VALIDATION.json is treated as read by the gate")
    for vj in sorted(root.rglob("VALIDATION.json")):
        parts = vj.relative_to(root).parts
        if "fixtures" in parts or ".git" in parts:
            continue
        if admitted is not None and vj.parent.relative_to(root).as_posix() not in admitted:
            continue
        rel_vj = vj.relative_to(root).as_posix()
        try:
            spec = json.loads(vj.read_text())
        except (OSError, ValueError):
            continue
        if not isinstance(spec, dict):
            continue
        base = vj.parent
        files: set[str] = set()
        report_rel = None
        for key in ("replay_certificate", "validator_report"):
            item = spec.get(key)
            rel = _evidence_file(base, item.get("path") if isinstance(item, dict) else None, root)
            if rel is None:
                continue
            if rel in moved:
                files.add(rel)
            if key == "validator_report":
                report_rel = rel
        raw: list = []
        if report_rel is not None:
            try:
                rep = json.loads((root / report_rel).read_text())
                raw = (rep.get("raw_data") or []) if isinstance(rep, dict) else []
            except (OSError, ValueError):
                if report_rel in moved:
                    # Its raw_data are unknowable until it is hydrated: pull the whole
                    # study (first two path components) rather than guess narrower.
                    files.update(m for m in moved if m.startswith("/".join(report_rel.split("/")[:2]) + "/"))
                    notes.append(f"{rel_vj}: validator report {report_rel} is store-backed and not hydrated, "
                                 "so its raw_data cannot be read; its whole study is pulled instead")
        for it in raw:
            rel = _evidence_file(base, it.get("path") if isinstance(it, dict) else it, root)
            if rel is not None and rel in moved:
                files.add(rel)
        if files:
            need[rel_vj] = files
    return need, notes


def cover(files: set[str], moved: set[str]) -> list[str]:
    """Pull prefixes that hydrate exactly `files` among the moved files.

    A file's own directory is used when every moved file below it is wanted
    (one short prefix per result directory); otherwise the file itself, so a
    directory that also holds unread store data is never pulled whole.
    """
    out: set[str] = set()
    for f in files:
        parent = f.rsplit("/", 1)[0] if "/" in f else ""
        ok = parent.count("/") >= 1 and all(m in files for m in moved if m.startswith(parent + "/"))
        out.add(parent if ok else f)
    return sorted(p for p in out if not any(p.startswith(q + "/") for q in out))


def pulled_by(text: str) -> list[str]:
    """Prefixes named by the pull command of an inserted step."""
    m = re.search(r"artifacts\.py pull (?:--jobs \d+ )?(.*)$", text, re.M)
    return m.group(1).split() if m else []


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
    evidence, evidence_notes = evidence_store_files(root, moved)
    for n in evidence_notes:
        print(n, file=sys.stderr)
    for f in files:
        text = f.read_text()
        paths = needed(text, moved, dirs)
        if reads_validation_gate(text):
            # Derived, not hand-listed: whatever store-backed evidence the gate
            # opens (VALIDATION.json -> report -> raw_data) is pulled.
            both = set(paths) | set(cover({x for v in evidence.values() for x in v}, moved))
            paths = sorted(p for p in both if not any(p.startswith(q + "/") for q in both))
        if not paths:
            continue
        new, notes = wire(text, paths)
        for n in notes:
            print(f"{f.name}: {n}", file=sys.stderr)
        if new != text:
            missing += 1
            if args.check:
                print(f"{f.name}: needs `{STEP_NAME}` for {' '.join(paths)}")
                if reads_validation_gate(text):
                    have = pulled_by(text)
                    for v, fs in sorted(evidence.items()):
                        gap = sorted(x for x in fs if not any(x == q or x.startswith(q + "/") for q in have))
                        if gap:
                            print(f"  {f.name}: the validation gate reads store-backed evidence of {v} "
                                  f"that this workflow does not pull: {', '.join(gap[:3])}"
                                  f"{' ...' if len(gap) > 3 else ''}; run python3 tools/artifacts/wire_workflows.py "
                                  f"(the list is derived, do not hand-edit it)")
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
