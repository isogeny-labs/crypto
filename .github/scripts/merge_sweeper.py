#!/usr/bin/env python3
"""Merge sweeper (.github/workflows/merge-sweeper.yml).

AGENTS.md ("Default workflow") lets the session that opened a PR merge it
once every applicable check on its head is green. Sessions end, though, and
their PRs stay open long after CI finished: on 2026-10-22 there were 187 open
PRs in this repository. The sweeper applies the same merge gate to every open
PR on a schedule, so a green PR lands without waiting for its author.

A PR is merged only when all of these hold on its current head:

  * not a draft, from this repository (no forks), based on the default branch
  * none of the hold labels (`do-not-merge`, `hold`, `needs-human`, `wip`,
    `autofix:running`, `merge-sweeper:off`)
  * GitHub reports it mergeable with `mergeable_state` `clean`
  * no reviewer's latest review is CHANGES_REQUESTED
  * at least one check run exists, every check run has completed, and each
    concluded success, neutral or skipped (cancelled, timed out, failed,
    action_required and stale are not a pass)
  * every legacy commit status is success

The merge uses a merge commit and the head SHA the checks ran on, so a push
that lands between the read and the merge makes GitHub refuse it.

Without --apply it only reports. Standard library only; talks to the GitHub
REST API with $GH_TOKEN or $GITHUB_TOKEN (a token is optional for a public
repository's read-only report).
"""
from __future__ import annotations

import argparse
import json
import os
import ssl
import sys
import time
import urllib.error
import urllib.request

API = os.environ.get("GITHUB_API_URL", "https://api.github.com")
PASS = {"success", "neutral", "skipped"}
HOLD_LABELS = {"do-not-merge", "hold", "needs-human", "wip",
               "autofix:running", "merge-sweeper:off"}
# The sweeper's own check run and advisory reviewers do not gate a merge.
IGNORE_CHECKS = {"sweep", "merge-sweeper"}


def api(method: str, path: str, body: dict | None = None) -> object:
    token = os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN")
    headers = {"Accept": "application/vnd.github+json",
               "X-GitHub-Api-Version": "2022-11-28"}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    data = None
    if body is not None:
        data = json.dumps(body).encode()
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(f"{API}/{path}", data=data, method=method,
                                 headers=headers)
    cafile = os.environ.get("SSL_CERT_FILE") or os.environ.get("REQUESTS_CA_BUNDLE")
    ctx = ssl.create_default_context(cafile=cafile) if cafile else None
    with urllib.request.urlopen(req, context=ctx, timeout=60) as res:
        raw = res.read()
    return json.loads(raw) if raw.strip() else None


def paged(path: str, key: str | None = None, max_pages: int = 20) -> list:
    items: list = []
    sep = "&" if "?" in path else "?"
    for page in range(1, max_pages + 1):
        data = api("GET", f"{path}{sep}per_page=100&page={page}")
        batch = data[key] if key else data
        items.extend(batch)
        if len(batch) < 100:
            break
    return items


def pr_detail(repo: str, number: int) -> dict:
    # `mergeable` is computed lazily; the first read can return null.
    for attempt in range(3):
        pr = api("GET", f"repos/{repo}/pulls/{number}")
        if pr.get("mergeable") is not None:
            return pr
        time.sleep(3 * (attempt + 1))
    return pr


def blocking_reviews(reviews: list[dict]) -> list[str]:
    """Reviewers whose latest decisive review requests changes."""
    latest: dict[str, str] = {}
    for r in reviews:
        state = r.get("state")
        if state in {"APPROVED", "CHANGES_REQUESTED", "DISMISSED"}:
            latest[(r.get("user") or {}).get("login", "?")] = state
    return sorted(u for u, s in latest.items() if s == "CHANGES_REQUESTED")


def check_verdict(runs: list[dict], statuses: list[dict]) -> tuple[str, list[str]]:
    """('pass' | 'pending' | 'fail' | 'none', names that keep it from passing).

    `runs` are the latest check runs of the head SHA, `statuses` the legacy
    commit statuses (newest first, as GitHub returns them).
    """
    runs = [r for r in runs if r.get("name") not in IGNORE_CHECKS]
    seen: set[str] = set()
    current = []
    for s in statuses:
        if s.get("context") not in seen:
            seen.add(s.get("context"))
            current.append(s)
    if not runs and not current:
        return "none", []
    pending = [r["name"] for r in runs if r.get("status") != "completed"]
    pending += [s["context"] for s in current if s.get("state") == "pending"]
    failed = [f"{r['name']} ({r.get('conclusion')})" for r in runs
              if r.get("status") == "completed" and r.get("conclusion") not in PASS]
    failed += [f"{s['context']} ({s.get('state')})" for s in current
               if s.get("state") in {"failure", "error"}]
    if failed:
        return "fail", sorted(failed)
    if pending:
        return "pending", sorted(pending)
    return "pass", []


def assess(repo: str, pr: dict, default_branch: str) -> tuple[str, str]:
    """(bucket, reason). Bucket 'ready' means the merge gate is satisfied."""
    labels = {l["name"] for l in pr.get("labels", [])}
    if pr.get("draft"):
        return "held", "draft"
    if held := sorted(labels & HOLD_LABELS):
        return "held", "label " + ", ".join(held)
    if (pr["head"].get("repo") or {}).get("full_name") != repo:
        return "held", "fork"
    if pr["base"]["ref"] != default_branch:
        return "held", f"based on {pr['base']['ref']}"

    sha = pr["head"]["sha"]
    runs = paged(f"repos/{repo}/commits/{sha}/check-runs?filter=latest", "check_runs")
    statuses = paged(f"repos/{repo}/commits/{sha}/statuses")
    verdict, names = check_verdict(runs, statuses)
    if verdict == "none":
        return "pending", "no checks reported on the head"
    if verdict != "pass":
        shown = ", ".join(names[:6]) + (" …" if len(names) > 6 else "")
        return verdict, shown

    detail = pr_detail(repo, pr["number"])
    if detail["head"]["sha"] != sha:
        return "pending", "head moved during the sweep"
    if detail.get("mergeable") is False or detail.get("mergeable_state") == "dirty":
        return "conflict", "merge conflict with the base branch"
    if detail.get("mergeable_state") != "clean":
        return "pending", f"mergeable_state {detail.get('mergeable_state')}"
    if blockers := blocking_reviews(paged(f"repos/{repo}/pulls/{pr['number']}/reviews")):
        return "held", "changes requested by " + ", ".join(blockers)
    return "ready", ""


def merge(repo: str, pr: dict) -> str:
    try:
        api("PUT", f"repos/{repo}/pulls/{pr['number']}/merge",
            {"merge_method": "merge", "sha": pr["head"]["sha"]})
    except urllib.error.HTTPError as exc:
        return f"refused ({exc.code}): {exc.read().decode(errors='replace')[:200]}"
    return "merged"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY"))
    ap.add_argument("--apply", action="store_true", help="merge ready PRs")
    ap.add_argument("--max-merges", type=int, default=10,
                    help="merge at most N PRs per sweep (default 10)")
    ap.add_argument("--pr", type=int, action="append", default=[],
                    help="only consider these PR numbers")
    args = ap.parse_args()
    if not args.repo:
        ap.error("--repo or GITHUB_REPOSITORY is required")

    default_branch = api("GET", f"repos/{args.repo}")["default_branch"]
    prs = paged(f"repos/{args.repo}/pulls?state=open&sort=created&direction=asc")
    if args.pr:
        prs = [p for p in prs if p["number"] in set(args.pr)]

    buckets: dict[str, list[str]] = {}
    merged = 0
    for pr in prs:
        try:
            bucket, reason = assess(args.repo, pr, default_branch)
        except urllib.error.HTTPError as exc:
            bucket, reason = "error", f"HTTP {exc.code} reading the PR"
        line = f"#{pr['number']} {pr['title'][:80]}" + (f" — {reason}" if reason else "")
        if bucket == "ready" and args.apply:
            if merged >= args.max_merges:
                bucket, line = "ready", line + " — over this sweep's merge cap"
            else:
                outcome = merge(args.repo, pr)
                merged += outcome == "merged"
                bucket = "merged" if outcome == "merged" else "error"
                line += f" — {outcome}"
        buckets.setdefault(bucket, []).append(line)
        print(f"[{bucket}] {line}", flush=True)

    order = ["merged", "ready", "pending", "fail", "conflict", "held", "error"]
    mode = "apply" if args.apply else "report only"
    out = [f"## Merge sweeper ({mode}): {len(prs)} open PRs", ""]
    out += [f"| {b} | {len(buckets.get(b, []))} |" for b in order]
    out.insert(2, "| bucket | PRs |")
    out.insert(3, "| --- | --- |")
    for b in order:
        if buckets.get(b):
            out += ["", f"<details><summary>{b} ({len(buckets[b])})</summary>", ""]
            out += [f"- {l}" for l in buckets[b]] + ["", "</details>"]
    summary = "\n".join(out) + "\n"
    if path := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(path, "a", encoding="utf-8") as fh:
            fh.write(summary)
    else:
        print("\n" + summary)
    return 1 if buckets.get("error") else 0


if __name__ == "__main__":
    sys.exit(main())
