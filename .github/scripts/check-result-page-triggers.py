#!/usr/bin/env python3
"""Check that only workflows which read the result pages trigger on them.

AGENTS.md section 7a has every result PR edit docs/index-calculus-scoreboard.html
(and often docs/ic-current-state.html). A workflow whose pull_request or push
`paths:` filter matches either page therefore runs on every result PR. That is
right for the few jobs that render, publish or scan those pages, and wasted
work for a frozen-result replay that never opens them (#1952, #2060).

Every workflow whose path filter matches a page must be listed in ALLOWED with
the reason it reads the page. An entry that no longer matches, or whose
workflow is gone, is an error too, so the list stays exact. Workflows with no
path filter run on every PR by design and are out of scope here.

This reads YAML only and never executes a workflow command.
"""
from pathlib import Path
import re
import sys

import yaml

PAGES = ("docs/index-calculus-scoreboard.html", "docs/ic-current-state.html")
EVENTS = ("pull_request", "pull_request_target", "push", "merge_group")
REPO_ROOT = Path(__file__).resolve().parents[2]

# Workflow path -> why it may trigger on the result pages.
ALLOWED = {
    ".github/workflows/ic-dashboard.yml": {
        "reason": "tools/render_ic_dashboard.mjs renders and checks the scoreboard overview",
    },
    ".github/workflows/ecc2k130-status.yml": {
        "reason": "scripts/site/build.py publishes both pages; scripts/site/test_build.py tests them",
    },
    ".github/workflows/curve-names.yml": {
        "reason": "scripts/check_curve_names.py scans every Markdown and HTML file, both pages included",
    },
    ".github/workflows/ecc2k130-n13-oaware-sat-benchmark.yml": {
        # Does not read the pages. Its FROZEN.json pins the workflow's exact
        # bytes and its ci_replay.py asserts that pin, so this line cannot be
        # edited out without re-sealing frozen evidence.
        "reason": "workflow bytes are pinned by its frozen evidence (FROZEN.json workflow_sha256)",
    },
}


def glob_regex(pattern):
    """GitHub path-filter glob: * and ? stay within a segment, ** spans them."""
    out, i = "", 0
    while i < len(pattern):
        if pattern.startswith("**/", i):
            out, i = out + "(?:.*/)?", i + 3
        elif pattern.startswith("**", i):
            out, i = out + ".*", i + 2
        elif pattern[i] == "*":
            out, i = out + "[^/]*", i + 1
        elif pattern[i] == "?":
            out, i = out + "[^/]", i + 1
        elif pattern[i] == "[" and "]" in pattern[i + 1:]:
            end = pattern.index("]", i + 1)
            out, i = out + "[" + pattern[i + 1:end] + "]", end + 1
        else:
            out, i = out + re.escape(pattern[i]), i + 1
    return re.compile(out + r"\Z")


def filter_matches(patterns, path):
    """Patterns apply in order; a leading ! excludes; the last match wins."""
    matched = False
    for pattern in patterns:
        if not isinstance(pattern, str):
            continue
        negated = pattern.startswith("!")
        if glob_regex(pattern[1:] if negated else pattern).match(path):
            matched = not negated
    return matched


def triggers(document):
    """The `on:` mapping; PyYAML reads the bare key `on` as True."""
    if not isinstance(document, dict):
        return {}
    on = document.get("on", document.get(True))
    return on if isinstance(on, dict) else {}


def matched_pages(document):
    found = set()
    for event, spec in triggers(document).items():
        if event not in EVENTS or not isinstance(spec, dict):
            continue
        paths = spec.get("paths")
        if not isinstance(paths, list):
            continue
        found.update(page for page in PAGES if filter_matches(paths, page))
    return sorted(found)


def problems(workflows, allowed=ALLOWED):
    """workflows maps a repository-relative path to its parsed YAML."""
    failures = []
    for path, document in sorted(workflows.items()):
        pages = matched_pages(document)
        entry = allowed.get(path)
        if pages and entry is None:
            failures.append(
                f"{path}: its path filter matches {', '.join(pages)}, which every result PR edits "
                f"(AGENTS.md section 7a). Drop that path unless the job reads the page, and then "
                f"add the workflow to ALLOWED in .github/scripts/check-result-page-triggers.py with the reason.")
        elif entry is not None and not pages:
            failures.append(f"{path}: listed in ALLOWED but no longer triggers on a result page; remove the entry.")
    for path in sorted(set(allowed) - set(workflows)):
        failures.append(f"{path}: listed in ALLOWED but the workflow does not exist; remove the entry.")
    return failures


def main():
    files = sorted((REPO_ROOT / ".github/workflows").glob("*.y*ml"))
    workflows = {}
    for file in files:
        rel = file.relative_to(REPO_ROOT).as_posix()
        workflows[rel] = yaml.safe_load(file.read_text())
    failures = problems(workflows)
    for failure in failures:
        print(failure, file=sys.stderr)
    print(f"Checked result-page triggers in {len(files)} workflows: {len(failures)} violations")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
