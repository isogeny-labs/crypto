#!/usr/bin/env python3
"""Add an artifact-store pull step to every workflow that names moved data.

After `artifacts.py push` moves files out of Git, a workflow that reads them
(a replay of `research/x/`, a check over `experiments/y/results/`) would find
pointers only. This finds, per workflow job, the repository paths its
executable steps name that hold moved files, and inserts after that job's
`actions/checkout` step:

    - name: Pull store data this workflow reads (tools/artifacts)
      env: {AWS_ACCESS_KEY_ID: ..., AWS_SECRET_ACCESS_KEY: ..., AWS_REGION: ...}
      run: |
        if ! command -v aws ...; then (install the AWS CLI into $RUNNER_TEMP); fi
        python3 tools/artifacts/artifacts.py pull research/x experiments/y/results

A sparse checkout also gets `tools/artifacts` added to its patterns. The edit
is textual (comments and layout survive) and idempotent; checkouts into a
`path:` other than the workspace are reported, not edited.

    python3 tools/artifacts/wire_workflows.py [--check] [WORKFLOW ...]

`--check` edits nothing and exits 1 if any workflow lacks the step it needs
or carries a stale one, which is how CI keeps new workflows honest.

The pull list is derived from each job's text outside checkout and generated
steps. Event path filters and another job's inputs are not reads. The
generated step's own `pull` line is output, never a reference, so a directory
the job stops naming is dropped on regeneration. Inputs a workflow's scripts
discover at run time are declared with `# tools/artifacts pull: PATH ...`
comment lines inside the consuming job; hand edits to the generated line do
not survive.
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
# A workflow whose scripts find their inputs at run time (a gate that follows
# VALIDATION.json to the session records it names) declares them, one or more
# paths per comment line, since its text never names them:
#     # tools/artifacts pull: research/x/sessions research/y/results
DECLARE_RE = re.compile(r"^\s*#\s*tools/artifacts pull:(.*)$")
PATH_RE = re.compile(r"(?<![A-Za-z0-9_./*-])((?:[A-Za-z0-9_+*-][A-Za-z0-9_.+*-]*/)+[A-Za-z0-9_.+*-]*)")


def moved_paths(root: Path) -> set[str]:
    out = set()
    for d, name, _ in artifacts.selected(root, []):
        drel = artifacts.rel(root, d)
        out.add(f"{drel}/{name}" if drel else name)
    # Manifests retain historical objects after a path returns to Git (for
    # example the append-only PCC ledger). Never hydrate an old object over
    # the authoritative tracked file, regardless of local hydration state.
    tracked = set(artifacts.git(root, "ls-files", "-z").decode().split("\0"))
    return out - tracked


def moved_dirs(moved: set[str]) -> set[str]:
    """Directories specific enough to identify a moved-data subtree."""
    return {
        parent
        for p in moved
        for n in range(1, p.count("/") + 1)
        if (parent := p.rsplit("/", n)[0]).count("/") >= 1
    }


def strip_generated(text: str) -> str:
    """The workflow without the pull steps this tool generated.

    The generated `artifacts.py pull ...` line names every directory it pulls,
    so scanning it would make each pulled directory a reference to itself and
    keep it after the workflow stopped naming it.
    """
    lines = text.split("\n")
    out: list[str] = []
    i = 0
    while i < len(lines):
        m = re.match(r"^(\s*)- name: (.*)$", lines[i])
        if not (m and m.group(2).strip() == STEP_NAME):
            out.append(lines[i])
            i += 1
            continue
        indent = len(m.group(1))
        i += 1
        while i < len(lines) and (not lines[i].strip() or len(lines[i]) - len(lines[i].lstrip()) > indent):
            i += 1
    return "\n".join(out)


def strip_checkouts(text: str) -> str:
    """Remove checkout steps before discovering data reads.

    Sparse-checkout entries describe which tracked paths are materialized;
    they do not mean a later command opens every file below those paths.
    Counting them as reads made one packaging job hydrate unrelated objects in
    every job of its workflow.
    """
    lines = text.split("\n")
    out: list[str] = []
    i = 0
    while i < len(lines):
        m = re.match(r"^(\s*)- (?:uses: actions/checkout@|name:)(.*)$", lines[i])
        if not m:
            out.append(lines[i])
            i += 1
            continue
        indent = len(m.group(1))
        j = i + 1
        while j < len(lines) and (
            not lines[j].strip() or len(lines[j]) - len(lines[j].lstrip()) > indent
        ):
            j += 1
        block = lines[i:j]
        if any(
            re.match(rf"^{m.group(1)}(?:- |  )uses: actions/checkout@", line)
            for line in block
        ):
            i = j
            continue
        out.extend(block)
        i = j
    return "\n".join(out)


def _may_discover_sibling_data(path: str) -> bool:
    """Whether a named tracked file may open moved data beside itself."""
    name = path.rsplit("/", 1)[-1]
    suffix = Path(name).suffix.lower()
    return suffix in {
        ".js",
        ".mjs",
        ".pl",
        ".py",
        ".rb",
        ".rs",
        ".sage",
        ".sh",
        ".ts",
    } or name in {"GNUmakefile", "Justfile", "Makefile", "makefile"}


def needed(text: str, moved: set[str], dirs: set[str]) -> list[str]:
    """Paths the workflow names that are, or contain, moved files; minimal cover."""
    found = set()
    text = strip_checkouts(strip_generated(text))
    for line in text.split("\n"):
        if m := DECLARE_RE.match(line):
            # Declared, so kept while it still names moved data, and dropped
            # (like any stale entry) once it does not.
            found.update(p for p in (x.rstrip("/") for x in m.group(1).split())
                         if p in moved or any(x.startswith(p + "/") for x in moved))
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
                if not _may_discover_sibling_data(p):
                    continue
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
    """Insert the pull step after each workspace checkout; returns (text, notes).

    With no paths, an earlier generated step is removed and nothing is added.
    """
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
        if paths:
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
        if not paths:
            i = j
            continue
        step = step_lines(indent, paths)
        cond = next((re.sub(rf"^{indent}(?:- |  )", "", b) for b in block
                     if re.match(rf"^{indent}(?:- |  )if: ", b)), None)
        if cond:  # skipped with its checkout, so it never runs against an empty workspace
            step.insert(1, f"{indent}  {cond}")
        out.extend(step)
        i = j
    return "\n".join(out), notes


JOB_HEADER_RE = re.compile(r"(?m)^  ([A-Za-z0-9_-]+):(?:\s*#.*)?$")


def job_spans(text: str) -> list[tuple[str, int, int]]:
    """Return `(job_id, start, end)` spans under the top-level `jobs:` map."""
    jobs = re.search(r"(?m)^jobs:(?:\s*#.*)?$", text)
    if not jobs:
        return []
    end = len(text)
    top = re.search(r"(?m)^[A-Za-z0-9_-]+:(?:\s*#.*)?$", text[jobs.end():])
    if top:
        end = jobs.end() + top.start()
    headers = list(JOB_HEADER_RE.finditer(text, jobs.end(), end))
    return [
        (
            match.group(1),
            match.start(),
            headers[index + 1].start() if index + 1 < len(headers) else end,
        )
        for index, match in enumerate(headers)
    ]


def requirements(
    text: str,
    moved: set[str],
    dirs: set[str],
    evidence: dict[str, set[str]],
) -> list[str]:
    """Store paths read by one job, including derived validation evidence."""
    paths = needed(text, moved, dirs)
    if reads_validation_gate(text):
        files = {path for group in evidence.values() for path in group}
        both = set(paths) | set(cover(files, moved))
        paths = sorted(
            path
            for path in both
            if not any(path.startswith(parent + "/") for parent in both)
        )
    return paths


def wire_workflow(
    text: str,
    moved: set[str],
    dirs: set[str],
    evidence: dict[str, set[str]],
) -> tuple[str, list[str], dict[str, list[str]]]:
    """Wire each job from only that job's executable inputs.

    Top-level declarations remain a compatibility surface. New
    `tools/artifacts pull:` declarations belong inside the consuming job so a
    multi-job workflow does not hydrate them everywhere.
    """
    spans = job_spans(text)
    if not spans:
        paths = requirements(text, moved, dirs, evidence)
        wired, notes = wire(text, paths)
        return wired, notes, {"<workflow>": paths} if paths else {}

    global_declarations = "\n".join(
        line for line in text[: spans[0][1]].split("\n") if DECLARE_RE.match(line)
    )
    out: list[str] = [text[: spans[0][1]]]
    notes: list[str] = []
    by_job: dict[str, list[str]] = {}
    cursor = spans[0][1]
    for job_id, start, end in spans:
        out.append(text[cursor:start])
        block = text[start:end]
        scan = f"{global_declarations}\n{block}" if global_declarations else block
        paths = requirements(scan, moved, dirs, evidence)
        wired, job_notes = wire(block, paths)
        out.append(wired)
        notes.extend(f"{job_id}: {note}" for note in job_notes)
        if paths:
            by_job[job_id] = paths
        cursor = end
    out.append(text[cursor:])
    return "".join(out), notes, by_job


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
        new, notes, by_job = wire_workflow(text, moved, dirs, evidence)
        if new == text:
            continue
        for n in notes:
            print(f"{f.name}: {n}", file=sys.stderr)
        missing += 1
        detail = "; ".join(f"{job}: {' '.join(paths)}" for job, paths in by_job.items())
        if args.check:
            if detail:
                print(f"{f.name}: needs per-job `{STEP_NAME}` for {detail}")
            else:
                print(f"{f.name}: stale `{STEP_NAME}`: no job names moved data")
            spans = job_spans(text) or [("<workflow>", 0, len(text))]
            for job_id, start, end in spans:
                block = text[start:end]
                if not reads_validation_gate(block):
                    continue
                have = pulled_by(block)
                for validation, files_read in sorted(evidence.items()):
                    gap = sorted(
                        path
                        for path in files_read
                        if not any(path == prefix or path.startswith(prefix + "/") for prefix in have)
                    )
                    if gap:
                        print(
                            f"  {f.name}:{job_id}: the validation gate reads store-backed evidence of "
                            f"{validation} that this job does not pull: {', '.join(gap[:3])}"
                            f"{' ...' if len(gap) > 3 else ''}; run python3 tools/artifacts/wire_workflows.py "
                            f"(the list is derived, do not hand-edit it)"
                        )
        else:
            f.write_text(new)
            print(f"{f.name}: per-job pull {detail}" if detail else f"{f.name}: pull step removed")
    if args.check and missing:
        print(f"\n{missing} workflow(s) lack the pull step they need or carry a stale one; run "
              f"python3 tools/artifacts/wire_workflows.py", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
