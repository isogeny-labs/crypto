#!/usr/bin/env python3
"""Check CI metadata: routine PR jobs must have a proven budget <=25 minutes.

Manual-only, disabled and explicit label experiments keep their full budgets;
the jobs in NAMED_BUDGET_EXCEPTIONS keep theirs.
Unknown conditions are treated as applicable; unknown budgets fail closed.
This reads YAML/expressions only and never executes a workflow command.
"""
import ast
from pathlib import Path
import re
import sys

import yaml

UNKNOWN = object()
EVENTS = ("pull_request", "merge_group")
ROUTINE_PR_TYPES = {"opened", "synchronize", "reopened", "ready_for_review"}

# These existing PR jobs are historical proof artifacts: their exact workflow
# bytes are included in checked Lean evidence seals. Keep each exception tied
# to the exact seal and workflow content, rather than allowing a workflow name
# or a proof-job label to exempt future work.
FROZEN_BUDGET_EXCEPTIONS = {
    ".github/workflows/weak-curve-lean.yml": {
        "job": "proofs",
        "timeout": 60,
        "seal": "research/weak_curve_properties_lean_20261011/EVIDENCE_SHA256.tsv",
        "seal_sha256": "bd7de95c3c6fc9060a15cd03a99fe6bea2272ccf815cee59a0d1d02e5606e625",
        "pin_path": "../../.github/workflows/weak-curve-lean.yml",
    },
    ".github/workflows/weak-curve-conjectures.yml": {
        "job": "evidence",
        "timeout": 60,
        "seal": "research/weak_curve_conjectures_20261012/EVIDENCE_SHA256SUMS",
        "seal_sha256": "bd1fbaf84fcb00ca2cf7d2eb7c6691463c6e1d7e5bd00c6744fa1870b1ff711c",
        "pin_path": "../../.github/workflows/weak-curve-conjectures.yml",
    },
}
REPO_ROOT = Path(__file__).resolve().parents[2]

# Named jobs whose routine PR budget is longer than 25 minutes, each with the
# reason. Exact on workflow path, job name and timeout; nothing else matches.
NAMED_BUDGET_EXCEPTIONS = {
    (".github/workflows/certicom-devices.yml", "metal-qualification"): {
        "timeout": 45,
        # The macOS library phase took 27m42s in run 40571119394 and
        # 30m17s in run 40511856969; both completed successfully.
        "reason": "Native Metal compilation and solver qualification on macOS",
    },
    (".github/workflows/release.yml", "isogeny"): {
        "timeout": 45,
        # Run 39652680641, job 119434898023 exhausted 25 minutes during
        # the second Windows build after native compilation/tests passed.
        "reason": "Windows native and software-hash builds plus installed CLI checks",
    },
    (".github/workflows/curve-covers.yml", "certificates"): {
        "timeout": 45,
        # Job 118198334025 exhausted 25 minutes while replaying the expanded
        # 2,572-model catalog after its library and binary-transfer tests passed.
        "reason": "Complete curve-cover certificate and expanded catalog replay",
    },
    (".github/workflows/codeql.yml", "analyze"): {
        "timeout": 60,
        # CodeQL's Rust queries cover every Rust source in the repository and
        # were cancelled at the 25-minute cap on several PRs (2026-10-17).
        "reason": "CodeQL Rust queries over the whole repository",
    },
    (".github/workflows/codeql.yml", "warm-rust-cache"): {
        "timeout": 45,
        # Type inference over the merged tree passed 25 minutes and was
        # cancelled mid-evaluation (run 39226121956, 2026-10-19); an
        # interrupted evaluation leaves no reusable cache.
        "reason": "CodeQL Rust type-inference cache warming over the whole repository",
    },
    (".github/workflows/koblitz-sota-reproduction.yml", "reproduce-toy-controls"): {
        "timeout": 45,
        # The workflow's pull_request budget expression evaluates to 45.
        "reason": "Koblitz SOTA-gate reproduction toy controls; runs 22-25 min on healthy runners and hit the 25-minute cap on slow shared runners (2026-10-17)",
    },
    (".github/workflows/polynomial-reuse.yml", "correctness"): {
        "timeout": 30,
        "reason": "Polynomial-reuse correctness job; 15-minute runs that hit the 25-minute cap on slow shared runners (2026-10-17)",
    },
    (".github/workflows/ecbench.yml", "harness"): {
        "timeout": 45,
        # The workflow's pull_request budget expression evaluates to 45.
        "reason": "ecbench harness; cancelled at the 25-minute cap on #2135, #2149 and #2153 (2026-10-18)",
    },
    (".github/workflows/release.yml", "isogeny"): {
        "timeout": 45,
        # The workflow's pull_request budget expression evaluates to 45.
        "reason": "Release isogeny matrix (x86_64-pc-windows-msvc, windows-latest); cancelled at the 25-minute cap on 22 PRs while main's push run took 24.9 minutes (2026-10-22)",
    },
    (".github/workflows/ic-producer.yml", "archived-producer"): {
        "timeout": 45,
        # The workflow's pull_request budget expression evaluates to 45.
        "reason": "Archived IC producer replay; the scaled variant was cancelled at the 25-minute cap on #2149 and #2153 (2026-10-18)",
    },
}


def named_budget_exception(label, job_name, budget):
    """Allow the listed jobs exactly their listed timeout on routine PR events."""
    exception = NAMED_BUDGET_EXCEPTIONS.get((Path(label).as_posix(), job_name))
    return exception is not None and budget == exception["timeout"]


def expression(value, event, action=None, matrix=None):
    if not isinstance(value, str):
        return value
    source = value.strip()
    if source.startswith("${{") and source.endswith("}}"):
        source = source[3:-2].strip()
    # GitHub strings containing operators are outside this small evaluator.
    # Keep them unknown rather than guessing their meaning.
    for quoted in re.findall(r"'[^']*'|\"[^\"]*\"", source):
        if any(operator in quoted for operator in ("&&", "||", "!")):
            return UNKNOWN
    source = source.replace("&&", " and ").replace("||", " or ")
    source = re.sub(r"!(?!=)", " not ", source).strip()

    def visit(node):
        if isinstance(node, ast.Constant):
            return node.value
        if isinstance(node, ast.Name):
            return {"true": True, "false": False, "null": None}.get(node.id, UNKNOWN)
        if isinstance(node, ast.Attribute):
            parts = []
            while isinstance(node, ast.Attribute):
                parts.insert(0, node.attr)
                node = node.value
            if isinstance(node, ast.Name):
                parts.insert(0, node.id)
            identity = ".".join(parts)
            if identity == "github.event_name":
                return event
            if identity == "github.event.action" and action is not None:
                return action
            if len(parts) == 2 and parts[0] == "matrix" and isinstance(matrix, dict):
                item = matrix.get(parts[1], UNKNOWN)
                return item if isinstance(item, (str, int, float, bool)) else UNKNOWN
            return UNKNOWN
        if isinstance(node, ast.Call):
            return True if isinstance(node.func, ast.Name) and node.func.id == "always" and not node.args and not node.keywords else UNKNOWN
        if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.Not):
            operand = visit(node.operand)
            return UNKNOWN if operand is UNKNOWN else not operand
        if isinstance(node, ast.Compare) and len(node.ops) == 1:
            left, right = visit(node.left), visit(node.comparators[0])
            if left is UNKNOWN or right is UNKNOWN:
                return UNKNOWN
            if isinstance(left, str) and isinstance(right, str):
                left, right = left.casefold(), right.casefold()
            if isinstance(node.ops[0], ast.Eq):
                return left == right
            if isinstance(node.ops[0], ast.NotEq):
                return left != right
            return UNKNOWN
        if isinstance(node, ast.BoolOp):
            values = [visit(item) for item in node.values]
            unknown = False
            for item in values:
                if item is UNKNOWN:
                    unknown = True
                elif isinstance(node.op, ast.And) and not item:
                    return item
                elif isinstance(node.op, ast.Or) and item:
                    return UNKNOWN if unknown else item
            return UNKNOWN if unknown else values[-1]
        return UNKNOWN

    try:
        return visit(ast.parse("(" + source + ")", mode="eval").body)
    except (SyntaxError, ValueError):
        return UNKNOWN


def problems(document, label="workflow"):
    triggers = document.get("on", document.get(True, {}))
    if isinstance(triggers, str):
        triggers = {triggers: None}
    elif isinstance(triggers, list):
        triggers = dict.fromkeys(triggers)
    jobs = document.get("jobs", {})
    found = []
    for event in EVENTS:
        if event not in triggers:
            continue
        options = triggers[event] or {}
        types = options.get("types", []) if isinstance(options, dict) else []
        if event == "pull_request" and types and not ROUTINE_PR_TYPES.intersection(types):
            continue
        actions = sorted(ROUTINE_PR_TYPES.intersection(types)) if types else ["opened", "synchronize", "reopened"]
        if event == "merge_group":
            actions = ["checks_requested"]
        for action in actions:
            found.extend(job_problems(jobs, event, action, label))
    return found


def job_problems(jobs, event, action, label):
    applicable = {}

    def may_run(name):
        if name in applicable:
            return applicable[name]
        # Conservatively allow a malformed dependency cycle/missing job.
        applicable[name] = True
        job = jobs.get(name, {})
        condition = job.get("if", True)
        result = expression(condition, event, action)
        if result is not UNKNOWN and not result:
            applicable[name] = False
        else:
            needs = job.get("needs", [])
            needs = [needs] if isinstance(needs, str) else needs
            # Status functions can override default skipped-needs handling.
            overrides = isinstance(condition, str) and re.search(r"\b(always|failure|cancelled|success)\s*\(", condition)
            if not overrides and any(not may_run(parent) for parent in needs):
                applicable[name] = False
        return applicable[name]

    found = []
    for name, job in jobs.items():
        if not may_run(name):
            continue
        # Include-only static matrices enumerate every possible row. Other
        # matrix shapes remain unknown rather than guessing expansion rules.
        matrix = job.get("strategy", {}).get("matrix")
        rows = [None]
        if (isinstance(matrix, dict) and set(matrix) == {"include"}
                and isinstance(matrix["include"], list) and matrix["include"]
                and all(isinstance(row, dict) for row in matrix["include"])):
            rows = matrix["include"]
        for row in rows:
            budget = expression(job.get("timeout-minutes", UNKNOWN), event, action, row)
            if isinstance(budget, bool) or not isinstance(budget, (int, float)) or not 0 < budget <= 25:
                if named_budget_exception(label, name, budget):
                    continue
                found.append(f"{label}: {name} can run on {event}/{action} without a proven 1–25 minute timeout")
                break
    return found

def main():
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(".github/workflows")
    files = sorted(root.glob("*.yml")) + sorted(root.glob("*.yaml"))
    if not files:
        raise SystemExit(f"no workflows found in {root}")
    failures = []
    for path in files:
        failures.extend(problems(yaml.safe_load(path.read_text()), str(path)))
    for failure in failures:
        print(failure, file=sys.stderr)
    print(f"Checked routine PR/merge-group budgets in {len(files)} workflows: {len(failures)} violations")
    raise SystemExit(bool(failures))


if __name__ == "__main__":
    main()
