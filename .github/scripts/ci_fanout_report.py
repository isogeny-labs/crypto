#!/usr/bin/env python3
"""Measure CI fan-out: how many PR-triggered workflows each merged PR starts.

The report replays merged PRs against the ``pull_request`` (and, by default,
``pull_request_target``) trigger filters of every workflow in a git repository
and prints, over the replayed PRs:

  * median / mean / p90 / max number of workflows triggered per PR,
  * the workflows triggered most often,
  * the shared ``paths:`` entries that cause the most fan-out.

It also doubles as a budget check (``--max-fanout N`` / ``--max-median N``,
non-zero exit when exceeded) and can score one prospective change set with
``--diff BASE HEAD`` instead of replaying history.

Everything is read through git plumbing (``rev-list``/``log``, ``diff-tree``,
``ls-tree``, ``cat-file``).  No worktree is materialised, nothing is fetched,
nothing is written.  Only the Python standard library is required, plus PyYAML
to read workflow files (checked when the first workflow is parsed, with a clear
error if it is missing).

What is modelled (GitHub "filter pattern cheat sheet" semantics):

  * ``*``   zero or more characters, not ``/``
  * ``**``  zero or more of any character, including ``/``
  * ``?``   zero or one of the PRECEDING character (a quantifier, not a
            single-character wildcard; this is what GitHub documents)
  * ``+``   one or more of the preceding character
  * ``[a-z0-9]`` character sets, ``\\x`` escapes a special character
  * a leading ``!`` negates a pattern.  Patterns are evaluated in order and the
    last matching pattern wins, so a later ``!x`` excludes and a still-later
    positive pattern re-includes.  A workflow runs when at least one changed
    file ends up included.
  * ``paths-ignore``: the workflow runs when at least one changed file is NOT
    ignored (same ordered rule, ``!`` re-includes).
  * ``branches`` / ``branches-ignore`` are checked against ``--base-branch``.
  * ``types:`` that cannot fire on open/update (for example ``[labeled]``) take
    the workflow out of the per-PR count; they are listed separately.

What is NOT modelled (all of these make the numbers an approximation):

  * Job-level ``if:`` conditions and runtime scope jobs: a workflow whose jobs
    all skip still counts as triggered, because it still creates a run and
    often still bills runner time.  This is a trigger-level fan-out measure.
  * GitHub's 300-file limit on path-filter diffs.  PRs above it are counted in
    the output so they can be treated with care.
  * Draft state, fork restrictions, ``workflow_run`` chains, required checks.
  * Squash/rebase merges are only visible with ``--mode first-parent``; the
    default (first-parent *merge* commits) matches "Create a merge commit".
  * The diff for a replayed PR is ``MERGE^1..MERGE``.  Renames are reported as
    delete + add (``--no-renames``), which can only over-count, never under.
"""
from __future__ import annotations

import argparse
import dataclasses
import json
import math
import os
import re
import statistics
import subprocess
import sys
from collections import Counter, defaultdict
from typing import Dict, Iterable, List, NamedTuple, Optional, Sequence, Set, Tuple

WORKFLOW_DIR = ".github/workflows"
WORKFLOW_FILE_RE = re.compile(r"^\.github/workflows/[^/]+\.ya?ml$")
DEFAULT_EVENTS = ("pull_request", "pull_request_target")
# Activity types that fire when a PR is opened or updated.  A workflow whose
# ``types`` list excludes all of them (for example ``[labeled]``) is not part
# of per-PR fan-out.
ROUTINE_TYPES = frozenset({"opened", "synchronize", "reopened"})
GH_PATH_DIFF_LIMIT = 300
EXIT_OK = 0
EXIT_BUDGET = 1
EXIT_ERROR = 2


class ReportError(Exception):
    """A user-facing failure (bad ref, git error, missing PyYAML)."""


# --------------------------------------------------------------------------
# GitHub filter-pattern globs
# --------------------------------------------------------------------------

def glob_to_regex(pattern: str) -> "re.Pattern[str]":
    """Translate one GitHub filter pattern (without a leading ``!``) to a regex.

    Use with ``fullmatch``.  Raises ``re.error`` for patterns that cannot be
    translated into a valid regex (callers treat that as "never matches" and
    warn).
    """
    out: List[str] = []
    atom: Optional[int] = None  # index in ``out`` of the last quantifiable atom
    i, n = 0, len(pattern)
    while i < n:
        c = pattern[i]
        if c == "\\" and i + 1 < n:
            out.append(re.escape(pattern[i + 1]))
            atom = len(out) - 1
            i += 2
        elif c == "*":
            j = i
            while j < n and pattern[j] == "*":
                j += 1
            out.append(".*" if j - i >= 2 else "[^/]*")
            atom = None
            i = j
        elif c in "?+":
            if atom is not None:
                out[atom] += c
                atom = None  # a quantifier cannot itself be quantified
            else:
                out.append(re.escape(c))
                atom = None
            i += 1
        elif c == "[":
            j = pattern.find("]", i + 2)  # at least one member before ']'
            if j == -1:
                out.append(re.escape(c))
                atom = len(out) - 1
                i += 1
            else:
                body = pattern[i + 1:j]
                neg = body[:1] in ("!", "^")
                if neg:
                    body = body[1:]
                body = body.replace("\\", "\\\\")
                out.append("[" + ("^" if neg else "") + body + "]")
                atom = len(out) - 1
                i = j + 1
        else:
            out.append(re.escape(c))
            atom = len(out) - 1
            i += 1
    return re.compile("".join(out), re.DOTALL)


class Pat(NamedTuple):
    raw: str          # as written in the workflow (including any leading "!")
    neg: bool
    rx: Optional["re.Pattern[str]"]  # None when the pattern could not be compiled


def compile_patterns(raws: Iterable[object], warnings: List[str], where: str) -> List[Pat]:
    pats: List[Pat] = []
    for item in raws:
        raw = str(item)
        neg = raw.startswith("!")
        body = raw[1:] if neg else raw
        try:
            rx: Optional["re.Pattern[str]"] = glob_to_regex(body)
        except re.error as exc:
            warnings.append(f"{where}: pattern {raw!r} does not compile ({exc}); treated as never matching")
            rx = None
        pats.append(Pat(raw, neg, rx))
    return pats


def ordered_match(value: str, pats: Sequence[Pat]) -> Optional[Pat]:
    """GitHub ordered include/exclude evaluation.

    Returns the positive pattern that last set the file's state to "included",
    or None when the value ends up excluded / unmatched.
    """
    state: Optional[Pat] = None
    for p in pats:
        if p.rx is not None and p.rx.fullmatch(value):
            state = None if p.neg else p
    return state


# --------------------------------------------------------------------------
# Workflow model
# --------------------------------------------------------------------------

@dataclasses.dataclass
class Trigger:
    event: str
    paths: Optional[List[Pat]] = None
    paths_ignore: Optional[List[Pat]] = None
    branches: Optional[List[Pat]] = None
    branches_ignore: Optional[List[Pat]] = None

    @property
    def kind(self) -> str:
        if self.paths is not None:
            return "paths"
        if self.paths_ignore is not None:
            return "paths-ignore"
        if self.branches is not None or self.branches_ignore is not None:
            return "branches"
        return "always"


@dataclasses.dataclass
class Workflow:
    path: str
    triggers: List[Trigger] = dataclasses.field(default_factory=list)
    excluded: Optional[str] = None   # why a PR-event workflow is not per-PR
    error: Optional[str] = None      # YAML / structure problem
    warnings: List[str] = dataclasses.field(default_factory=list)

    @property
    def name(self) -> str:
        return self.path.rsplit("/", 1)[-1]

    @property
    def per_pr(self) -> bool:
        return bool(self.triggers)

    @property
    def kind(self) -> str:
        kinds = {t.kind for t in self.triggers}
        if "always" in kinds:
            return "always"
        return "+".join(sorted(kinds)) or "none"


def _load_yaml():
    try:
        import yaml  # type: ignore
    except ImportError:
        raise ReportError(
            "PyYAML is required to read workflow files but is not installed.\n"
            "  Install it with:  python3 -m pip install pyyaml\n"
            "  (the glob and replay logic itself uses only the standard library)"
        )
    return yaml


def _as_list(value: object) -> List[object]:
    if value is None:
        return []
    if isinstance(value, (list, tuple)):
        return list(value)
    return [value]


def parse_workflow(path: str, text: str, events: Sequence[str] = DEFAULT_EVENTS) -> Workflow:
    wf = Workflow(path=path)
    yaml = _load_yaml()
    loader = getattr(yaml, "CSafeLoader", yaml.SafeLoader)
    try:
        doc = yaml.load(text, Loader=loader)
    except Exception as exc:  # yaml.YAMLError and friends
        wf.error = f"YAML parse error: {str(exc).splitlines()[0] if str(exc) else type(exc).__name__}"
        return wf
    if not isinstance(doc, dict):
        wf.error = "top level is not a mapping"
        return wf
    # YAML 1.1 parses a bare ``on`` key as boolean True.
    on = doc["on"] if "on" in doc else doc.get(True)
    if isinstance(on, str):
        on_map: Dict[str, object] = {on: None}
    elif isinstance(on, list):
        on_map = {str(x): None for x in on}
    elif isinstance(on, dict):
        on_map = {str(k): v for k, v in on.items()}
    else:
        on_map = {}

    for event in events:
        if event not in on_map:
            continue
        cfg = on_map[event]
        where = f"{wf.name}:{event}"
        if cfg is None:
            cfg = {}
        if not isinstance(cfg, dict):
            wf.warnings.append(f"{where}: config is not a mapping; treated as unfiltered")
            cfg = {}
        types = {str(t) for t in _as_list(cfg.get("types"))}
        if types and not (types & ROUTINE_TYPES):
            wf.excluded = f"{event} types {sorted(types)} never fire on open/update"
            continue
        trig = Trigger(event=event)
        if "paths" in cfg:
            trig.paths = compile_patterns(_as_list(cfg["paths"]), wf.warnings, where + ".paths")
            if not any(not p.neg for p in trig.paths):
                wf.warnings.append(f"{where}: paths has no positive pattern; GitHub rejects this, nothing matches")
        if "paths-ignore" in cfg:
            if trig.paths is not None:
                wf.warnings.append(f"{where}: both paths and paths-ignore set (invalid on GitHub); using paths")
            else:
                trig.paths_ignore = compile_patterns(_as_list(cfg["paths-ignore"]), wf.warnings, where + ".paths-ignore")
        if "branches" in cfg:
            trig.branches = compile_patterns(_as_list(cfg["branches"]), wf.warnings, where + ".branches")
        if "branches-ignore" in cfg:
            trig.branches_ignore = compile_patterns(_as_list(cfg["branches-ignore"]), wf.warnings, where + ".branches-ignore")
        wf.triggers.append(trig)
    if wf.triggers:
        wf.excluded = None
    return wf


def evaluate_trigger(trig: Trigger, files: Sequence[str], base_branch: str) -> Tuple[bool, Set[str]]:
    """Would this trigger start a run for these changed files? Plus the
    positive ``paths`` entries that included files (the causes)."""
    if trig.branches is not None and ordered_match(base_branch, trig.branches) is None:
        return False, set()
    if trig.branches_ignore is not None and ordered_match(base_branch, trig.branches_ignore) is not None:
        return False, set()
    if trig.paths is not None:
        causes: Set[str] = set()
        for f in files:
            hit = ordered_match(f, trig.paths)
            if hit is not None:
                causes.add(hit.raw)
        return bool(causes), causes
    if trig.paths_ignore is not None:
        return any(ordered_match(f, trig.paths_ignore) is None for f in files), set()
    return True, set()


def evaluate_workflow(wf: Workflow, files: Sequence[str], base_branch: str) -> Tuple[bool, Set[str]]:
    triggered = False
    causes: Set[str] = set()
    for trig in wf.triggers:
        ok, c = evaluate_trigger(trig, files, base_branch)
        if ok:
            triggered = True
            causes |= c
    return triggered, causes


# --------------------------------------------------------------------------
# git plumbing (read only)
# --------------------------------------------------------------------------

class Git:
    def __init__(self, repo: str):
        self.repo = repo
        self.env = dict(os.environ, GIT_OPTIONAL_LOCKS="0", LC_ALL="C")

    def run(self, *args: str, input_bytes: Optional[bytes] = None, check: bool = True) -> bytes:
        cmd = ["git", "-C", self.repo, *args]
        try:
            proc = subprocess.run(cmd, input=input_bytes, stdout=subprocess.PIPE,
                                  stderr=subprocess.PIPE, env=self.env)
        except FileNotFoundError:
            raise ReportError("git executable not found")
        if check and proc.returncode != 0:
            raise ReportError(f"git {' '.join(args[:3])} failed: {proc.stderr.decode('utf-8', 'replace').strip()}")
        return proc.stdout

    def rev_parse(self, rev: str) -> str:
        return self.run("rev-parse", "--verify", "--quiet", rev + "^{commit}").decode().strip()

    def workflow_blobs(self, rev: str) -> Dict[str, str]:
        """{path: blob sha} for top-level workflow files at ``rev``."""
        out = self.run("ls-tree", "-z", rev, WORKFLOW_DIR + "/", check=False)
        blobs: Dict[str, str] = {}
        for rec in out.split(b"\0"):
            if not rec:
                continue
            meta, _, path = rec.partition(b"\t")
            parts = meta.split()
            if len(parts) == 3 and parts[1] == b"blob":
                p = path.decode("utf-8", "surrogateescape")
                if WORKFLOW_FILE_RE.match(p):
                    blobs[p] = parts[2].decode()
        return blobs

    def read_blobs(self, shas: Sequence[str]) -> Dict[str, bytes]:
        if not shas:
            return {}
        data = self.run("cat-file", "--batch", input_bytes=("\n".join(shas) + "\n").encode())
        result: Dict[str, bytes] = {}
        pos = 0
        for sha in shas:
            nl = data.index(b"\n", pos)
            header = data[pos:nl].split()
            if len(header) != 3:
                raise ReportError(f"cat-file: cannot read blob {sha}")
            size = int(header[2])
            result[sha] = data[nl + 1:nl + 1 + size]
            pos = nl + 1 + size + 1
        return result

    def changed_files(self, base: str, head: str, three_dot: bool = False) -> List[str]:
        if three_dot:
            out = self.run("diff", "--name-only", "--no-renames", "-z", f"{base}...{head}")
        else:
            out = self.run("diff-tree", "-r", "--name-only", "--no-renames", "--no-commit-id", "-z", base, head)
        return [p.decode("utf-8", "surrogateescape") for p in out.split(b"\0") if p]


@dataclasses.dataclass
class PR:
    sha: str
    parent: str
    subject: str
    date: str
    files: List[str] = dataclasses.field(default_factory=list)


def collect_prs(git: Git, rng: str, max_prs: int, mode: str) -> List[PR]:
    args = ["log", "--first-parent", "-z", "--format=%H%x1f%P%x1f%cs%x1f%s"]
    if mode == "merges":
        args.append("--merges")
    if max_prs > 0:
        args += ["-n", str(max_prs)]
    args.append(rng)
    out = git.run(*args)
    prs: List[PR] = []
    for rec in out.split(b"\0"):
        if not rec.strip():
            continue
        sha, parents, date, subject = rec.decode("utf-8", "replace").lstrip("\n").split("\x1f", 3)
        plist = parents.split()
        if not plist:
            continue  # root commit: nothing to diff against
        prs.append(PR(sha=sha, parent=plist[0], subject=subject, date=date))
    return prs


class WorkflowCache:
    """Parse each distinct workflow blob once."""

    def __init__(self, git: Git, events: Sequence[str]):
        self.git = git
        self.events = tuple(events)
        self._parsed: Dict[Tuple[str, str], Workflow] = {}

    def at(self, rev: str) -> List[Workflow]:
        blobs = self.git.workflow_blobs(rev)
        # A blob can appear under several paths; parse keyed by (path, sha).
        missing = sorted({sha for path, sha in blobs.items() if (path, sha) not in self._parsed})
        contents = self.git.read_blobs(missing) if missing else {}
        wfs = []
        for path in sorted(blobs):
            key = (path, blobs[path])
            if key not in self._parsed:
                text = contents[blobs[path]].decode("utf-8", "replace")
                self._parsed[key] = parse_workflow(path, text, self.events)
            wfs.append(self._parsed[key])
        return wfs


# --------------------------------------------------------------------------
# Analysis
# --------------------------------------------------------------------------

@dataclasses.dataclass
class PRResult:
    pr: PR
    triggered: List[str]
    causes: Dict[str, Set[str]]   # workflow name -> positive path entries that matched


def replay(prs: Sequence[PR], workflows_for, base_branch: str) -> List[PRResult]:
    results = []
    for pr in prs:
        wfs = workflows_for(pr)
        triggered: List[str] = []
        causes: Dict[str, Set[str]] = {}
        for wf in wfs:
            if not wf.per_pr:
                continue
            ok, c = evaluate_workflow(wf, pr.files, base_branch)
            if ok:
                triggered.append(wf.name)
                if c:
                    causes[wf.name] = c
        results.append(PRResult(pr=pr, triggered=sorted(triggered), causes=causes))
    return results


def percentile(sorted_vals: Sequence[int], q: float) -> int:
    if not sorted_vals:
        return 0
    rank = max(1, math.ceil(q * len(sorted_vals)))
    return sorted_vals[rank - 1]


def num(x: float) -> object:
    return int(x) if float(x).is_integer() else round(float(x), 2)


HIST_EDGES = [(0, 0), (1, 5), (6, 10), (11, 20), (21, 40), (41, 80), (81, None)]


def histogram(counts: Sequence[int]) -> Dict[str, int]:
    hist: Dict[str, int] = {}
    for lo, hi in HIST_EDGES:
        label = f"{lo}" if hi == lo else (f"{lo}+" if hi is None else f"{lo}-{hi}")
        hist[label] = sum(1 for c in counts if c >= lo and (hi is None or c <= hi))
    return hist


def build_report(results: Sequence[PRResult], universe: Sequence[Workflow], top: int,
                 min_share: int) -> dict:
    n = len(results)
    counts = [len(r.triggered) for r in results]
    sorted_counts = sorted(counts)
    per_pr = [wf for wf in universe if wf.per_pr]
    always = [wf for wf in per_pr if wf.kind == "always"]
    kinds = {wf.name: wf.kind for wf in per_pr}

    trig_count: Counter = Counter()
    for r in results:
        trig_count.update(r.triggered)

    # Path-entry attribution.  Credit goes to the positive entry that last
    # included each changed file.  "decisive" means that entry was the only one
    # keeping the workflow triggered for that PR.
    listed_in: Dict[str, Set[str]] = defaultdict(set)
    for wf in per_pr:
        for t in wf.triggers:
            for p in t.paths or []:
                if not p.neg:
                    listed_in[p.raw].add(wf.name)
    entry_events: Counter = Counter()
    entry_decisive: Counter = Counter()
    entry_prs: Dict[str, Set[str]] = defaultdict(set)
    for r in results:
        for wf_name, entries in r.causes.items():
            for e in entries:
                entry_events[e] += 1
                entry_prs[e].add(r.pr.sha)
            if len(entries) == 1:
                entry_decisive[next(iter(entries))] += 1

    top_entries = []
    for e, ev in entry_events.most_common():
        if len(listed_in[e]) < min_share:
            continue
        top_entries.append({
            "entry": e,
            "listed_in_workflows": len(listed_in[e]),
            "prs_hit": len(entry_prs[e]),
            "workflow_triggers": ev,
            "decisive_triggers": entry_decisive[e],
        })
    top_entries.sort(key=lambda d: (-d["workflow_triggers"], -d["listed_in_workflows"], d["entry"]))

    worst = sorted(results, key=lambda r: (-len(r.triggered), r.pr.date, r.pr.sha))[:top]
    return {
        "prs_replayed": n,
        "files_per_pr": {
            "median": num(statistics.median(len(r.pr.files) for r in results)) if n else 0,
            "max": max((len(r.pr.files) for r in results), default=0),
            "prs_over_github_300_file_diff_limit": sum(1 for r in results if len(r.pr.files) > GH_PATH_DIFF_LIMIT),
        },
        "workflow_universe": {
            "workflow_files": len(universe),
            "pr_triggered": len(per_pr),
            "always_on_unfiltered": sorted(wf.name for wf in always),
            "filtered": len(per_pr) - len(always),
            "excluded_by_activity_type": sorted(wf.name for wf in universe if wf.excluded and not wf.per_pr),
            "parse_errors": sorted(f"{wf.name}: {wf.error}" for wf in universe if wf.error),
        },
        "fanout": {
            "median": num(statistics.median(counts)) if n else 0,
            "mean": num(round(statistics.fmean(counts), 2)) if n else 0,
            "p90": percentile(sorted_counts, 0.9),
            "min": sorted_counts[0] if n else 0,
            "max": sorted_counts[-1] if n else 0,
            "always_on_floor": len(always),
            "triggered_by_every_pr": sum(1 for c in trig_count.values() if n and c == n),
            "histogram": histogram(counts),
        },
        "top_workflows": [
            {"workflow": w, "kind": kinds.get(w, "?"), "triggers": c,
             "pct_of_prs": round(100.0 * c / n, 1) if n else 0.0}
            for w, c in sorted(trig_count.items(), key=lambda kv: (-kv[1], kv[0]))[:top]
        ],
        "top_shared_entries": top_entries[:top],
        "worst_prs": [
            {"sha": r.pr.sha[:10], "date": r.pr.date, "subject": r.pr.subject,
             "files": len(r.pr.files), "fanout": len(r.triggered)}
            for r in worst
        ],
        "warnings": sorted({w for wf in universe for w in wf.warnings}),
    }


def check_budget(report: dict, max_fanout: Optional[int], max_median: Optional[float]) -> List[str]:
    problems = []
    fan = report["fanout"]
    if max_fanout is not None and fan["max"] > max_fanout:
        problems.append(f"max workflows triggered per PR is {fan['max']}, budget is {max_fanout}")
    if max_median is not None and fan["median"] > max_median:
        problems.append(f"median workflows triggered per PR is {fan['median']}, budget is {max_median}")
    return problems


# --------------------------------------------------------------------------
# Rendering
# --------------------------------------------------------------------------

def render_text(report: dict, meta: dict) -> str:
    lines: List[str] = []
    add = lines.append
    fan, uni, files = report["fanout"], report["workflow_universe"], report["files_per_pr"]
    add("CI fan-out report")
    add(f"  repo:       {meta['repo']}")
    add(f"  scope:      {meta['scope']}")
    add(f"  workflows:  {meta['workflows']}")
    add(f"  base branch for branch filters: {meta['base_branch']}")
    add("")
    add(f"PRs replayed: {report['prs_replayed']}   changed files per PR: median {files['median']}, max {files['max']}"
        + (f"   ({files['prs_over_github_300_file_diff_limit']} over GitHub's {GH_PATH_DIFF_LIMIT}-file path-filter diff limit; "
           "their counts are approximate)" if files["prs_over_github_300_file_diff_limit"] else ""))
    add(f"Workflow files: {uni['workflow_files']}; PR-triggered: {uni['pr_triggered']} "
        f"({len(uni['always_on_unfiltered'])} unfiltered, {uni['filtered']} filtered by paths or branches); "
        f"{len(uni['excluded_by_activity_type'])} excluded by activity type; {len(uni['parse_errors'])} unparseable")
    add("")
    add("Workflows triggered per PR")
    add(f"  median {fan['median']}   mean {fan['mean']}   p90 {fan['p90']}   min {fan['min']}   max {fan['max']}"
        f"   (workflows hit by every PR: {fan['triggered_by_every_pr']}, of which no path/branch filter: {fan['always_on_floor']})")
    add("  distribution: " + "  ".join(f"{k}:{v}" for k, v in fan["histogram"].items()))
    add("")
    add(f"Top workflows by trigger frequency (of {report['prs_replayed']} PRs)")
    if report["top_workflows"]:
        w = max(len(t["workflow"]) for t in report["top_workflows"])
        for t in report["top_workflows"]:
            add(f"  {t['workflow']:<{w}}  {t['triggers']:>5}  {t['pct_of_prs']:>5.1f}%  {t['kind']}")
    else:
        add("  (none)")
    add("")
    add("Shared path entries by fan-out (listed in >=%d workflows)" % meta["min_share"])
    add("  triggers = (PR, workflow) pairs that entry helped start; decisive = it was the only entry that did")
    if report["top_shared_entries"]:
        w = max(len(t["entry"]) for t in report["top_shared_entries"])
        add(f"  {'entry':<{w}}  {'workflows':>9}  {'PRs':>5}  {'triggers':>8}  {'decisive':>8}")
        for t in report["top_shared_entries"]:
            add(f"  {t['entry']:<{w}}  {t['listed_in_workflows']:>9}  {t['prs_hit']:>5}  "
                f"{t['workflow_triggers']:>8}  {t['decisive_triggers']:>8}")
    else:
        add("  (none)")
    add("")
    add("Highest fan-out PRs")
    for p in report["worst_prs"]:
        add(f"  {p['fanout']:>4} workflows  {p['files']:>5} files  {p['sha']}  {p['date']}  {p['subject'][:70]}")
    if uni["always_on_unfiltered"]:
        add("")
        add("Unfiltered (always-on) PR workflows: " + ", ".join(uni["always_on_unfiltered"]))
    if uni["parse_errors"]:
        add("")
        add("Unparseable workflows (not counted):")
        lines.extend("  " + e for e in uni["parse_errors"])
    if report["warnings"]:
        add("")
        add("Warnings:")
        lines.extend("  " + w for w in report["warnings"])
    if "budget" in report:
        add("")
        b = report["budget"]
        add("Budget: " + ("EXCEEDED" if b["violations"] else "ok") +
            f" (max-fanout={b['max_fanout']}, max-median={b['max_median']})")
        lines.extend("  " + v for v in b["violations"])
    return "\n".join(lines) + "\n"


# --------------------------------------------------------------------------
# CLI
# --------------------------------------------------------------------------

def parse_args(argv: Optional[Sequence[str]]) -> argparse.Namespace:
    ap = argparse.ArgumentParser(
        description="Replay merged PRs against PR-trigger path filters and report CI fan-out.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Exit status: 0 ok, 1 fan-out budget exceeded, 2 usage or environment error.")
    ap.add_argument("--repo", default=".", help="git repository (default: current directory)")
    ap.add_argument("--range", dest="rng", default="origin/main",
                    help="ref or A..B range to replay, first-parent (default: origin/main)")
    ap.add_argument("--max-prs", type=int, default=100,
                    help="replay at most this many most-recent PRs; 0 = no limit (default: 100)")
    ap.add_argument("--mode", choices=("merges", "first-parent"), default="merges",
                    help="'merges': first-parent merge commits (default); "
                         "'first-parent': every first-parent commit, for squash/rebase merge repos")
    ap.add_argument("--base-branch", default="main",
                    help="base branch used for branches/branches-ignore filters (default: main)")
    ap.add_argument("--workflows-ref", default="tip",
                    help="where to read workflows from: 'tip' = newest commit of the range (default), "
                         "'merge' = each PR's own merge commit, or any git rev (e.g. a branch, for before/after comparisons)")
    ap.add_argument("--events", default=",".join(DEFAULT_EVENTS),
                    help="comma list of PR events to count (default: %(default)s)")
    ap.add_argument("--diff", nargs=2, metavar=("BASE", "HEAD"),
                    help="score ONE change set (git diff BASE...HEAD) instead of replaying history; "
                         "workflows are read from HEAD unless --workflows-ref is given")
    ap.add_argument("--top", type=int, default=15, help="rows in each top-N table (default: 15)")
    ap.add_argument("--min-share", type=int, default=2,
                    help="only list path entries that appear in at least this many workflows (default: 2)")
    ap.add_argument("--json", action="store_true", help="emit JSON instead of text")
    ap.add_argument("--max-fanout", type=int, metavar="N",
                    help="budget: exit 1 if any replayed PR triggers more than N workflows")
    ap.add_argument("--max-median", type=float, metavar="N",
                    help="budget: exit 1 if the median fan-out exceeds N")
    return ap.parse_args(argv)


def resolve_tip(git: Git, rng: str) -> str:
    tip = rng.split("..")[-1] if ".." in rng else rng
    tip = tip.lstrip(".") or "HEAD"
    sha = git.rev_parse(tip)
    if not sha:
        raise ReportError(f"cannot resolve {tip!r} in {git.repo}")
    return sha


def run(args: argparse.Namespace) -> Tuple[dict, dict]:
    git = Git(args.repo)
    events = tuple(e.strip() for e in args.events.split(",") if e.strip())
    if not events:
        raise ReportError("--events is empty")
    if not git.rev_parse("HEAD") and not git.rev_parse(args.rng.split("..")[-1] or "HEAD"):
        raise ReportError(f"{args.repo} is not a git repository with commits")
    cache = WorkflowCache(git, events)

    if args.diff:
        base, head = args.diff
        for r in (base, head):
            if not git.rev_parse(r):
                raise ReportError(f"cannot resolve {r!r}")
        head_sha = git.rev_parse(head)
        pr = PR(sha=head_sha, parent=git.rev_parse(base), subject=f"{base}...{head}", date="",
                files=git.changed_files(base, head, three_dot=True))
        prs = [pr]
        wf_rev = head_sha if args.workflows_ref in ("tip", "merge") else _resolve(git, args.workflows_ref)
        scope = f"one change set: {base}...{head} ({len(pr.files)} files)"
        workflows_for = lambda _pr, rev=wf_rev: cache.at(rev)  # noqa: E731
        wf_desc = f"read at {wf_rev[:10]}"
        newest_rev = wf_rev
    else:
        if not git.rev_parse(args.rng.split("..")[-1] or "HEAD"):
            raise ReportError(f"cannot resolve range {args.rng!r}")
        prs = collect_prs(git, args.rng, args.max_prs, args.mode)
        for pr in prs:
            pr.files = git.changed_files(pr.parent, pr.sha)
        tip = resolve_tip(git, args.rng)
        if args.workflows_ref == "merge":
            workflows_for = lambda pr: cache.at(pr.sha)  # noqa: E731
            wf_desc = "each PR's own merge commit"
            newest_rev = prs[0].sha if prs else tip
        else:
            rev = tip if args.workflows_ref == "tip" else _resolve(git, args.workflows_ref)
            workflows_for = lambda _pr, rev=rev: cache.at(rev)  # noqa: E731
            wf_desc = f"read at {rev[:10]}" + (" (tip of range)" if args.workflows_ref == "tip" else "")
            newest_rev = rev
        scope = (f"{len(prs)} most recent {'merge commits' if args.mode == 'merges' else 'first-parent commits'} "
                 f"of {args.rng}" + (f" (limit {args.max_prs})" if args.max_prs else ""))

    results = replay(prs, workflows_for, args.base_branch)
    universe = cache.at(newest_rev)
    report = build_report(results, universe, args.top, args.min_share)
    if args.max_fanout is not None or args.max_median is not None:
        report["budget"] = {"max_fanout": args.max_fanout, "max_median": args.max_median,
                            "violations": check_budget(report, args.max_fanout, args.max_median)}
    meta = {"repo": os.path.abspath(args.repo), "scope": scope, "workflows": wf_desc,
            "base_branch": args.base_branch, "min_share": args.min_share}
    report["meta"] = {k: v for k, v in meta.items() if k != "min_share"}
    return report, meta


def _resolve(git: Git, rev: str) -> str:
    sha = git.rev_parse(rev)
    if not sha:
        raise ReportError(f"cannot resolve {rev!r}")
    return sha


def main(argv: Optional[Sequence[str]] = None) -> int:
    args = parse_args(argv)
    try:
        report, meta = run(args)
    except ReportError as exc:
        print(f"ci_fanout_report: error: {exc}", file=sys.stderr)
        return EXIT_ERROR
    if args.json:
        json.dump(report, sys.stdout, indent=2, sort_keys=True)
        sys.stdout.write("\n")
    else:
        sys.stdout.write(render_text(report, meta))
    violations = report.get("budget", {}).get("violations", [])
    if violations:
        for v in violations:
            print(f"ci_fanout_report: BUDGET EXCEEDED: {v}", file=sys.stderr)
        return EXIT_BUDGET
    return EXIT_OK


if __name__ == "__main__":
    sys.exit(main())
