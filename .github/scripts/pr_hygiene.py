#!/usr/bin/env python3
"""PR capacity and branch hygiene: report, plan (dry-run by default), notice.

Standard library only.  Everything that touches GitHub goes through an injected
``api(method, path, body=None)`` callable and everything that touches git goes
through an injected ``git(args, cwd=None, env=None)`` callable, so the decision
logic is unit-tested offline (see test_pr_hygiene.py) with fixture JSON.

Subcommands
  report   Read-only census: open PRs per lane (against the lane caps), stale
           PRs, merged-but-undeleted branches, branches with no PR older than N
           days.  Markdown on stdout (suitable for $GITHUB_STEP_SUMMARY).
  plan     The exact actions the policy would take (delete branch, warn/close a
           stale PR).  DRY-RUN unless --apply is given.  --apply additionally
           requires at least one --allow GLOB (or PR_HYGIENE_ALLOW) and every
           branch deletion must match that glob AND the built-in lane
           allowlist AND not be protected; those checks run again immediately
           before each mutation, independently of the plan.
  notice   Advisory per-lane cap notice for one just-opened PR (label +
           comment).  Never fails: exit status is always 0.

Design rules (see .github/ci-pr-capacity.md)
  * Branch deletion is an ALLOWLIST (codex/ claude/ cursor/ feat/ fix/), never a
    denylist.  Protected families override the allowlist.
  * Fail closed: any incomplete listing or API error aborts before mutating.
  * Deletion uses ``git push --force-with-lease=refs/heads/<b>:<sha> --delete``
    so a branch that gained a commit after the decision is not deleted.
  * Branch names, PR titles and bodies are untrusted data.  Branch names must
    match NAME_RE or they are skipped; titles/bodies are never read or printed;
    names never go into URL paths or shell strings.
  * No pull_request_target / pull_request:closed automation lives here.
  * Never touches labels named ``run-*`` (one-shot experiment gates).
"""
from __future__ import annotations

import argparse
import base64
import calendar
import fnmatch
import json
import os
import re
import shlex
import subprocess
import sys
import tempfile
import time

# --------------------------------------------------------------------------
# Policy constants
# --------------------------------------------------------------------------

LANES = ("codex", "claude", "cursor", "research", "feat", "fix")
# The ONLY lanes whose branches may ever be planned for deletion.
DELETABLE_LANES = ("codex", "claude", "cursor", "feat", "fix")

DEFAULT_CAPS = {
    # Guesses, not measurements (no live data was available when written).
    # Override with the PR_LANE_CAPS repo variable (JSON); <=0 or null = no cap.
    "codex": 30,
    "claude": 30,
    "cursor": 15,
    "feat": 10,
    "fix": 10,
    "research": 15,
    "other": 10,
}

NAME_RE = re.compile(r"^[A-Za-z0-9._/-]{1,200}$")

PROTECTED_EXACT = frozenset({"main", "master", "perf-history", "head"})
PROTECTED_PREFIXES = (
    "research/",
    "lit2026",
    "gh-readonly-queue/",
    "copilot/",
    "dependabot/",
    "release/",
    "releases/",
    "refs/",
)

LABEL_CAP = "over-lane-cap"
LABEL_STALE = "pr-hygiene:stale"
LABEL_CAP_EXEMPT = "cap-exempt"
LABEL_KEEP_OPEN = "keep-open"
LABEL_NO_AUTO_DELETE = "no-auto-delete"
STALE_EXEMPT_LABELS = frozenset(
    {LABEL_KEEP_OPEN, "needs-human", "autofix:running", "large-file-ok"}
)
MARK_CAP = "<!-- pr-hygiene:lane-cap -->"
MARK_WARN = "<!-- pr-hygiene:stale-warn -->"
MARK_CLOSE = "<!-- pr-hygiene:stale-close -->"

PAGE = 100
MAX_PAGES = 400  # 40k items; exceeding it is an error, never a silent truncation
DAY = 86400.0


class IncompleteData(RuntimeError):
    """An API/git listing is missing or truncated: do not act on it."""


class ApiError(RuntimeError):
    pass


class GitError(RuntimeError):
    pass


# --------------------------------------------------------------------------
# Small helpers
# --------------------------------------------------------------------------

def parse_ts(value):
    if not value:
        return None
    try:
        return calendar.timegm(time.strptime(value, "%Y-%m-%dT%H:%M:%SZ"))
    except (TypeError, ValueError):
        return None


def safe_name(name):
    """A branch name that is safe to use as data (see NAME_RE) and not odd."""
    if not isinstance(name, str) or not NAME_RE.match(name):
        return False
    if (".." in name or "//" in name or name.endswith("/") or name.endswith(".")
            or name.endswith(".lock") or name.startswith("/") or name.startswith("-")
            or "/." in name or name.startswith(".") or "@{" in name):
        return False
    return True


def md_code(text, limit=120):
    """Render untrusted text as inert inline code in Markdown/log output."""
    s = "".join(c if (c.isprintable() and c not in "`|<>") else "?" for c in str(text))
    if len(s) > limit:
        s = s[:limit] + "..."
    return "`" + s + "`"


def lane_of(head_ref, head_repo, repo):
    """Lane of a PR by head ref.  Fork PRs are 'fork' and never acted on."""
    if not head_repo or head_repo != repo:
        return "fork"
    if not isinstance(head_ref, str) or "/" not in head_ref:
        return "other"
    first = head_ref.split("/", 1)[0]
    return first if first in LANES else "other"


def branch_lane(name):
    if isinstance(name, str) and "/" in name:
        first = name.split("/", 1)[0]
        if first in LANES:
            return first
    return "other"


def parse_caps(raw):
    """Merge a JSON object over DEFAULT_CAPS.  Raises ValueError if malformed."""
    caps = dict(DEFAULT_CAPS)
    if raw is None or not str(raw).strip():
        return caps
    data = json.loads(raw)
    if not isinstance(data, dict):
        raise ValueError("PR_LANE_CAPS must be a JSON object")
    for key, val in data.items():
        if val is None:
            caps[key] = 0
        elif isinstance(val, int) and not isinstance(val, bool):
            caps[key] = val
        else:
            raise ValueError("cap for %r must be an integer or null" % key)
    return caps


def cap_for(caps, lane):
    value = caps.get(lane, 0)
    return value if value and value > 0 else 0  # 0 = unlimited


# --------------------------------------------------------------------------
# GitHub / git I/O
# --------------------------------------------------------------------------

def gh_api(method, path, body=None):
    """Real API callable: shells out to ``gh api`` (arguments, never a shell)."""
    cmd = ["gh", "api", "-X", method, "-H", "Accept: application/vnd.github+json", path]
    if body is not None:
        cmd += ["--input", "-"]
    proc = subprocess.run(
        cmd, input=json.dumps(body) if body is not None else None,
        capture_output=True, text=True, timeout=180,
    )
    if proc.returncode != 0:
        raise ApiError("gh api %s %s failed: %s" % (method, path.split("?")[0], proc.stderr.strip()[:300]))
    text = proc.stdout.strip()
    return json.loads(text) if text else None


def run_git(args, cwd=None, env=None):
    proc = subprocess.run(
        ["git"] + list(args), cwd=cwd, env=env, capture_output=True, text=True, timeout=600,
    )
    if proc.returncode != 0:
        raise GitError("git %s failed: %s" % (args[0] if args else "", proc.stderr.strip()[:300]))
    return proc.stdout


def paginate(api, path):
    """All items of a list endpoint; raises IncompleteData rather than truncating."""
    items = []
    sep = "&" if "?" in path else "?"
    for page in range(1, MAX_PAGES + 1):
        try:
            chunk = api("GET", "%s%sper_page=%d&page=%d" % (path, sep, PAGE, page))
        except ApiError as exc:
            raise IncompleteData("listing %s page %d failed: %s" % (path.split("?")[0], page, exc))
        if not isinstance(chunk, list):
            raise IncompleteData("listing %s page %d is not a list" % (path.split("?")[0], page))
        items.extend(chunk)
        if len(chunk) < PAGE:
            return items
    raise IncompleteData("listing %s exceeds %d pages; refusing to truncate" % (path.split("?")[0], MAX_PAGES))


def normalize_pr(raw):
    head = raw.get("head") or {}
    base = raw.get("base") or {}
    hrepo = head.get("repo") or {}
    return {
        "number": raw.get("number"),
        "state": raw.get("state"),
        "draft": bool(raw.get("draft")),
        "merged": bool(raw.get("merged_at")),
        "head_ref": head.get("ref"),
        "head_sha": head.get("sha"),
        "head_repo": hrepo.get("full_name"),
        "base_ref": base.get("ref"),
        "labels": {l.get("name") for l in (raw.get("labels") or []) if isinstance(l, dict)},
        "updated": parse_ts(raw.get("updated_at")),
        "merged_at": parse_ts(raw.get("merged_at")),
        "closed_at": parse_ts(raw.get("closed_at")),
        "user": (raw.get("user") or {}).get("login"),
    }


def parse_branch_lines(text, merged_text):
    """Parse ``name<TAB>sha<TAB>committer-unix`` lines plus the merged-name list."""
    merged = {ln.strip() for ln in merged_text.splitlines() if ln.strip()}
    out = []
    for ln in text.splitlines():
        if not ln.strip():
            continue
        parts = ln.split("\t")
        if len(parts) != 3:
            raise IncompleteData("malformed branch listing line: %r" % ln[:80])
        name, sha, ts = parts
        if name == "HEAD":
            continue
        out.append({"name": name, "sha": sha, "ts": int(ts) if ts.isdigit() else None,
                    "merged": name in merged})
    return out


def collect_branches_git(git, git_dir, remote, default_branch):
    prefix = "refs/remotes/%s/" % remote
    fmt = "%(refname:lstrip=3)%09%(objectname)%09%(committerdate:unix)"
    allrefs = git(["-C", git_dir, "for-each-ref", "--format=" + fmt, prefix])
    merged = git(["-C", git_dir, "for-each-ref", "--format=%(refname:lstrip=3)",
                  "--merged=" + prefix + default_branch, prefix])
    branches = parse_branch_lines(allrefs, merged)
    if not branches:
        raise IncompleteData("no remote branches listed from git")
    return branches


def worktree_branches(git, git_dir):
    out = git(["-C", git_dir, "worktree", "list", "--porcelain"])
    names = set()
    for ln in out.splitlines():
        if ln.startswith("branch refs/heads/"):
            names.add(ln[len("branch refs/heads/"):].strip())
    return names


# --------------------------------------------------------------------------
# Protection model
# --------------------------------------------------------------------------

_TOKEN_RE = re.compile(r"[A-Za-z0-9._/*?-]+")


def scan_workflow_text(text, literal, globs):
    """Collect every branch-like token from workflow text.

    Conservative on purpose: any mention anywhere in a workflow file (push
    filters, ``head.ref ==`` conditions, scripts, comments) protects a branch
    of that exact name.  Wildcard tokens count only when they contain a '/'
    and start with a literal first segment (``codex/n83-*``), so a bare ``*``
    or ``fix**`` cannot protect everything.
    """
    for tok in _TOKEN_RE.findall(text):
        for variant in {tok, tok.rstrip(".")}:
            if not variant:
                continue
            if "*" in variant or "?" in variant:
                first = variant.split("/", 1)[0]
                if "/" in variant and first and not any(c in first for c in "*?") \
                        and first[0].isalnum():
                    globs.add(variant)
                continue
            literal.add(variant)
            if variant.startswith("refs/heads/"):
                literal.add(variant[len("refs/heads/"):])
            # origin/codex/x, remotes/origin/codex/x ...: every '/'-suffix
            idx = variant.find("/")
            while idx != -1:
                literal.add(variant[idx + 1:])
                idx = variant.find("/", idx + 1)


def scan_workflows_dir(path):
    literal, globs = set(), set()
    if not os.path.isdir(path):
        raise IncompleteData("workflows directory %s not found" % path)
    n = 0
    for fn in sorted(os.listdir(path)):
        if fn.endswith((".yml", ".yaml")):
            n += 1
            with open(os.path.join(path, fn), encoding="utf-8", errors="replace") as fh:
                scan_workflow_text(fh.read(), literal, globs)
    if n == 0:
        raise IncompleteData("no workflow files found in %s" % path)
    return literal, globs


def read_manifest(path):
    if not path or not os.path.exists(path):
        raise IncompleteData("protect manifest %s missing" % path)
    out = []
    with open(path, encoding="utf-8") as fh:
        for ln in fh:
            ln = ln.split("#", 1)[0].strip()
            if ln:
                out.append(ln)
    return out


def protected_reason(name, snap):
    """Why `name` must never be deleted (None if nothing protects it)."""
    low = name.casefold()
    if name == snap["default_branch"]:
        return "default branch"
    if low in PROTECTED_EXACT:
        return "protected name"
    for pre in PROTECTED_PREFIXES:
        if low.startswith(pre):
            return "protected family %s" % pre
    if name in snap["workflow_literal"]:
        return "referenced by a workflow"
    for pat in snap["workflow_globs"]:
        if fnmatch.fnmatchcase(name, pat):
            return "matches workflow pattern"
    for pat in snap["manifest"]:
        if name == pat or (any(c in pat for c in "*?[") and fnmatch.fnmatchcase(name, pat)):
            return "listed in branch-protect manifest"
    if name in snap["worktree_branches"]:
        return "checked out in a local worktree"
    return None


# --------------------------------------------------------------------------
# Analysis
# --------------------------------------------------------------------------

def default_cfg(**over):
    cfg = {
        "caps": dict(DEFAULT_CAPS),
        "stale_days": 14.0,
        "close_after_days": 7.0,
        "merged_days": 3.0,
        "closed_days": 30.0,
        "orphan_days": 14.0,
        "max_deletes": 50,
        "max_pr_actions": 10,
        "list_limit": 50,
    }
    cfg.update(over)
    return cfg


def lane_census(prs, repo, caps):
    """Open same-repo PRs per lane and which of them exceed the lane cap.

    Order is by PR number, so concurrent opens get distinct positions and the
    newest PRs are the ones reported over the cap.  ``cap-exempt`` PRs neither
    count nor are flagged.  Fork PRs are listed but never counted or flagged.
    """
    lanes = {}
    exempt = []
    forks = []
    for pr in sorted((p for p in prs if p["state"] == "open"), key=lambda p: p["number"]):
        lane = lane_of(pr["head_ref"], pr["head_repo"], repo)
        if lane == "fork":
            forks.append(pr["number"])
            continue
        if LABEL_CAP_EXEMPT in pr["labels"]:
            exempt.append(pr["number"])
            continue
        lanes.setdefault(lane, []).append(pr["number"])
    result = {}
    over = {}
    for lane, nums in lanes.items():
        cap = cap_for(caps, lane)
        result[lane] = {"open": len(nums), "cap": cap, "numbers": nums}
        if cap:
            for pos, num in enumerate(nums, start=1):
                if pos > cap:
                    over[num] = {"lane": lane, "position": pos, "cap": cap, "open": len(nums)}
    return {"lanes": result, "over": over, "exempt": exempt, "forks": forks}


def is_stale_exempt(pr, protected):
    if pr["labels"] & STALE_EXEMPT_LABELS:
        return "label"
    if any(l.startswith("run-") for l in pr["labels"]):
        return "run-* label"
    if protected:
        return "protected head"
    return None


def classify_branch(b, snap, cfg, idx):
    """Return (verdict, rule, reason) for one remote branch.

    verdict: delete | protected | in-use | skip | keep
    Only ``delete`` ever becomes an action.
    """
    name, now = b["name"], snap["now"]
    why = protected_reason(name, snap) if isinstance(name, str) else "invalid"
    if why:
        return "protected", "protected", why
    if not safe_name(name):
        return "skip", "invalid-name", "name fails validation; never touched"
    if name in idx["open_heads"] or name in idx["open_bases"]:
        return "in-use", "open-pr", "head or base of an open PR"
    if branch_lane(name) not in DELETABLE_LANES:
        return "skip", "not-allowlisted", "lane is not in the deletion allowlist"
    prs = idx["by_ref"].get(name, [])
    merged_prs = [p for p in prs if p["merged"]]
    closed_prs = [p for p in prs if p["state"] == "closed" and not p["merged"]]
    min_merged = cfg["merged_days"] * DAY
    min_closed = cfg["closed_days"] * DAY
    sha = b["sha"]
    # B: squash/rebase-merged PR whose final head is exactly the branch tip.
    for p in merged_prs:
        if p["head_sha"] == sha and p["merged_at"] and now - p["merged_at"] >= min_merged:
            return "delete", "B", "tip equals head of PR #%s merged >= %gd ago" % (p["number"], cfg["merged_days"])
    # A: tip already reachable from the default branch AND a merged PR exists
    # for this ref (a PR-less ancestor may be a fresh agent branch cut from main).
    if b["merged"] and merged_prs:
        latest = max((p["merged_at"] or 0) for p in merged_prs)
        if latest and now - latest >= min_merged:
            return "delete", "A", "tip is an ancestor of the default branch; PR merged >= %gd ago" % cfg["merged_days"]
    # C: closed-unmerged PR whose final head is the tip, closed long ago.
    for p in closed_prs:
        if p["head_sha"] == sha and p["closed_at"] and now - p["closed_at"] >= min_closed:
            return "delete", "C", "tip equals head of PR #%s closed unmerged >= %gd ago" % (p["number"], cfg["closed_days"])
    return "keep", "keep", "no deletion rule matched"


def analyze(snap, cfg):
    repo = snap["repo"]
    prs = snap["prs"]
    idx = build_index(prs, repo)
    census = lane_census(prs, repo, cfg["caps"])

    branch_rows = []
    deletes = []
    merged_undeleted = []
    orphans = []
    for b in snap["branches"]:
        verdict, rule, reason = classify_branch(b, snap, cfg, idx)
        has_pr = bool(idx["by_ref"].get(b["name"]))
        age = (snap["now"] - b["ts"]) / DAY if b["ts"] else None
        row = dict(b, verdict=verdict, rule=rule, reason=reason, has_pr=has_pr, age_days=age,
                   lane=branch_lane(b["name"]))
        branch_rows.append(row)
        if verdict == "delete":
            deletes.append(row)
        if verdict not in ("protected",) and b["merged"] and b["name"] != snap["default_branch"]:
            merged_undeleted.append(row)
        if (not has_pr and verdict not in ("protected",)
                and b["name"] != snap["default_branch"]
                and age is not None and age >= cfg["orphan_days"]):
            orphans.append(row)

    # Stale PRs.
    stale = []
    for pr in sorted((p for p in prs if p["state"] == "open"), key=lambda p: p["number"]):
        lane = lane_of(pr["head_ref"], pr["head_repo"], repo)
        if lane == "fork" or pr["updated"] is None:
            continue
        idle = (snap["now"] - pr["updated"]) / DAY
        prot = bool(pr["head_ref"] and (protected_reason(pr["head_ref"], snap)
                                        or pr["head_ref"] in idx["open_bases"]))
        # base-of-another-PR check
        exempt = is_stale_exempt(pr, prot)
        stage = None
        if LABEL_STALE in pr["labels"]:
            if idle >= cfg["close_after_days"]:
                stage = "close"
        elif idle >= cfg["stale_days"]:
            stage = "warn"
        stale.append({"number": pr["number"], "lane": lane, "idle_days": idle, "stage": stage,
                      "exempt": exempt, "head_ref": pr["head_ref"], "head_sha": pr["head_sha"]})

    return {"census": census, "branches": branch_rows, "deletes": deletes,
            "merged_undeleted": merged_undeleted, "orphans": orphans, "stale": stale,
            "index": idx}


def build_index(prs, repo):
    idx = {"open_heads": set(), "open_bases": set(), "by_ref": {}}
    for p in prs:
        if p["state"] == "open":
            if p["head_ref"]:
                idx["open_heads"].add(p["head_ref"])
            if p["base_ref"]:
                idx["open_bases"].add(p["base_ref"])
        if p["head_ref"] and p["head_repo"] == repo:
            idx["by_ref"].setdefault(p["head_ref"], []).append(p)
    return idx


def build_plan(snap, cfg):
    """The ordered list of actions.  Pure: touches nothing."""
    an = analyze(snap, cfg)
    actions = []
    dels = sorted(an["deletes"], key=lambda r: (r["ts"] or 0, r["name"]))
    for r in dels[: cfg["max_deletes"]]:
        actions.append({"kind": "delete_branch", "branch": r["name"], "sha": r["sha"],
                        "rule": r["rule"], "reason": r["reason"]})
    deferred_deletes = max(0, len(dels) - cfg["max_deletes"])
    pr_actions = []
    for s in an["stale"]:
        if s["stage"] and not s["exempt"]:
            pr_actions.append({"kind": "close_pr" if s["stage"] == "close" else "warn_pr",
                               "pr": s["number"], "branch": s["head_ref"], "sha": s["head_sha"],
                               "idle_days": round(s["idle_days"], 1)})
    # closes first (they have waited longest), then warnings
    pr_actions.sort(key=lambda a: (a["kind"] != "close_pr", a["pr"]))
    deferred_pr = max(0, len(pr_actions) - cfg["max_pr_actions"])
    actions.extend(pr_actions[: cfg["max_pr_actions"]])
    return {"actions": actions, "deferred_deletes": deferred_deletes,
            "deferred_pr_actions": deferred_pr, "analysis": an}


# --------------------------------------------------------------------------
# Rendering
# --------------------------------------------------------------------------

def render_report(snap, cfg):
    an = analyze(snap, cfg)
    lim = cfg["list_limit"]
    out = []
    w = out.append
    w("## PR and branch hygiene report")
    w("")
    w("Repository %s, default branch %s. Read-only: nothing below has been changed." %
      (md_code(snap["repo"]), md_code(snap["default_branch"])))
    w("")
    w("### Open PRs per lane")
    w("")
    w("| Lane | Open | Cap | Over cap |")
    w("|---|---:|---:|---:|")
    over_by_lane = {}
    for num, o in an["census"]["over"].items():
        over_by_lane.setdefault(o["lane"], []).append(num)
    for lane in sorted(an["census"]["lanes"]):
        c = an["census"]["lanes"][lane]
        w("| %s | %d | %s | %d |" % (lane, c["open"], c["cap"] or "none", len(over_by_lane.get(lane, []))))
    w("")
    w("Fork PRs (never counted or acted on): %d. `cap-exempt` PRs (not counted): %d." %
      (len(an["census"]["forks"]), len(an["census"]["exempt"])))
    if an["census"]["over"]:
        w("")
        w("Newest PRs over their lane cap: " + ", ".join(
            "#%d (%s, position %d of cap %d)" % (n, o["lane"], o["position"], o["cap"])
            for n, o in sorted(an["census"]["over"].items())[:lim]))
    w("")
    w("### Stale PRs")
    w("")
    pending = [s for s in an["stale"] if s["stage"] or s["exempt"]]
    if not pending:
        w("None.")
    else:
        w("| PR | Lane | Idle days | Next step | Exempt |")
        w("|---|---|---:|---|---|")
        for s in pending[:lim]:
            w("| #%d | %s | %.1f | %s | %s |" % (s["number"], s["lane"], s["idle_days"],
                                                s["stage"] or "-", s["exempt"] or "-"))
        if len(pending) > lim:
            w("")
            w("... %d more not shown." % (len(pending) - lim))
    w("")
    w("### Merged-but-undeleted branches")
    w("")
    mu = an["merged_undeleted"]
    by_lane = {}
    for r in mu:
        by_lane[r["lane"]] = by_lane.get(r["lane"], 0) + 1
    w("%d branches are already contained in %s and still exist (by lane: %s)." %
      (len(mu), md_code(snap["default_branch"]),
       ", ".join("%s %d" % kv for kv in sorted(by_lane.items())) or "none"))
    deletable = [r for r in mu if r["verdict"] == "delete"]
    w("Of these, %d satisfy a deletion rule right now; the rest are protected, in use, "
      "outside the lane allowlist, or too recent." % len(deletable))
    if deletable:
        w("")
        for r in deletable[:lim]:
            w("- %s (rule %s)" % (md_code(r["name"]), r["rule"]))
        if len(deletable) > lim:
            w("- ... %d more not shown." % (len(deletable) - lim))
    squash = [r for r in an["deletes"] if r["rule"] in ("B", "C")]
    if squash:
        w("")
        w("Branches deletable only through PR evidence (not git ancestry): %d." % len(squash))
    w("")
    w("### Branches with no PR older than %g days" % cfg["orphan_days"])
    w("")
    w("%d branches. Report only: branches with commits not in the default branch and no PR are "
      "NEVER deleted by this tool." % len(an["orphans"]))
    unmerged = [r for r in an["orphans"] if not r["merged"]]
    w("%d of them carry commits that are not in the default branch." % len(unmerged))
    for r in sorted(unmerged, key=lambda r: r["ts"] or 0)[:lim]:
        w("- %s (%s, %.0f days)" % (md_code(r["name"]), r["lane"], r["age_days"] or 0))
    if len(unmerged) > lim:
        w("- ... %d more not shown." % (len(unmerged) - lim))
    prot = {}
    for r in an["branches"]:
        if r["verdict"] == "protected":
            prot[r["reason"]] = prot.get(r["reason"], 0) + 1
    w("")
    w("### Protected branches")
    w("")
    w("%d of %d branches are protected: %s." % (sum(prot.values()), len(an["branches"]),
      ", ".join("%s %d" % (k, v) for k, v in sorted(prot.items())) or "none"))
    return "\n".join(out) + "\n"


def render_plan(plan, snap, cfg, apply_mode):
    out = []
    w = out.append
    w("## PR and branch hygiene plan (%s)" % ("APPLY" if apply_mode else "DRY-RUN"))
    w("")
    acts = plan["actions"]
    if not acts:
        w("No actions.")
    for a in acts:
        if a["kind"] == "delete_branch":
            w("DELETE branch %s at %s [rule %s: %s]" % (md_code(a["branch"]), a["sha"][:12], a["rule"], a["reason"]))
            w("    git push --force-with-lease=refs/heads/%s:%s <repo-url> --delete %s" %
              (shlex.quote(a["branch"]), a["sha"], shlex.quote(a["branch"])))
        elif a["kind"] == "warn_pr":
            w("WARN   PR #%d idle %.1fd: comment + label %s" % (a["pr"], a["idle_days"], LABEL_STALE))
        elif a["kind"] == "close_pr":
            w("CLOSE  PR #%d idle %.1fd after warning (branch %s kept at %s)" %
              (a["pr"], a["idle_days"], md_code(a["branch"]), (a["sha"] or "")[:12]))
    if plan["deferred_deletes"] or plan["deferred_pr_actions"]:
        w("")
        w("Deferred by per-run caps: %d branch deletions, %d PR actions." %
          (plan["deferred_deletes"], plan["deferred_pr_actions"]))
    if not apply_mode:
        w("")
        w("Dry run: nothing was changed. Applying needs --apply and --allow GLOB.")
    return "\n".join(out) + "\n"


# --------------------------------------------------------------------------
# Applying
# --------------------------------------------------------------------------

def deletion_allowed(action, snap, allow_globs):
    """Final, independent gate run immediately before each deletion."""
    name, sha = action["branch"], action["sha"]
    if not safe_name(name):
        return "unsafe name"
    if branch_lane(name) not in DELETABLE_LANES:
        return "lane not in allowlist"
    why = protected_reason(name, snap)
    if why:
        return why
    if not (isinstance(sha, str) and re.fullmatch(r"[0-9a-f]{40}", sha)):
        return "no full 40-hex expected sha"
    if not any(fnmatch.fnmatchcase(name, g) for g in allow_globs):
        return "no --allow pattern matches"
    return None


def git_auth_env(token, base=None):
    env = dict(base if base is not None else os.environ)
    env["GIT_TERMINAL_PROMPT"] = "0"
    if token:
        b64 = base64.b64encode(("x-access-token:%s" % token).encode()).decode()
        env["GIT_CONFIG_COUNT"] = "1"
        env["GIT_CONFIG_KEY_0"] = "http.https://github.com/.extraheader"
        env["GIT_CONFIG_VALUE_0"] = "AUTHORIZATION: basic " + b64
    return env


def delete_with_lease(git, remote_url, name, sha, env):
    with tempfile.TemporaryDirectory() as tmp:
        git(["init", "-q", tmp], env=env)
        git(["-C", tmp, "push", "--no-verify",
             "--force-with-lease=refs/heads/%s:%s" % (name, sha),
             remote_url, "--delete", name], env=env)


def pr_comment_body(kind, a):
    branch = a["branch"] if safe_name(a["branch"] or "") else "(branch name not printable)"
    sha = a["sha"] if isinstance(a["sha"], str) and re.fullmatch(r"[0-9a-f]{40}", a["sha"]) else "unknown"
    if kind == "warn":
        return ("%s\nThis pull request has had no activity for %.0f days. If it is still wanted, push, "
                "comment, or add the `%s` label. Otherwise it will be closed after a further quiet "
                "period; closing never deletes the branch.\n\nBranch: %s\nHead: `%s`\n"
                % (MARK_WARN, a["idle_days"], LABEL_KEEP_OPEN, md_code(branch), sha))
    return ("%s\nClosing after %.0f idle days (stale warning given earlier). The branch is kept: %s at "
            "`%s`. Reopen this PR to restore it.\n" % (MARK_CLOSE, a["idle_days"], md_code(branch), sha))


def apply_plan(plan, snap, args, api, git, token, out, environ=None):
    environ = os.environ if environ is None else environ
    allow = [g for g in (args["allow"] or []) if g]
    if not allow:
        raise SystemExit("refusing --apply: at least one --allow GLOB is required")
    if environ.get("PR_HYGIENE_PAUSED", "").strip().lower() in ("1", "true", "yes"):
        out.write("PR_HYGIENE_PAUSED is set: apply refused, nothing changed.\n")
        return 0
    if not snap["prs"] or not snap["branches"]:
        raise IncompleteData("refusing to apply with an empty PR or branch listing")
    env = git_auth_env(token, environ)
    remote = args["remote_url"] or ("https://github.com/%s.git" % snap["repo"])
    done = 0
    for a in plan["actions"]:
        if a["kind"] == "delete_branch":
            why = deletion_allowed(a, snap, allow)
            if why:
                out.write("SKIP delete %s: %s\n" % (md_code(a["branch"]), why))
                continue
            try:
                delete_with_lease(git, remote, a["branch"], a["sha"], env)
                out.write("DELETED %s at %s\n" % (md_code(a["branch"]), a["sha"][:12]))
                done += 1
            except GitError as exc:
                out.write("FAILED delete %s (left in place): %s\n" % (md_code(a["branch"]), exc))
        elif a["kind"] in ("warn_pr", "close_pr"):
            n = a["pr"]
            if not isinstance(n, int):
                continue
            base = "repos/%s/issues/%d" % (snap["repo"], n)
            try:
                if a["kind"] == "warn_pr":
                    api("POST", base + "/comments", {"body": pr_comment_body("warn", a)})
                    assert not LABEL_STALE.startswith("run-")
                    api("POST", base + "/labels", {"labels": [LABEL_STALE]})
                else:
                    api("POST", base + "/comments", {"body": pr_comment_body("close", a)})
                    api("PATCH", "repos/%s/pulls/%d" % (snap["repo"], n), {"state": "closed"})
                out.write("%s PR #%d\n" % ("WARNED" if a["kind"] == "warn_pr" else "CLOSED", n))
                done += 1
            except ApiError as exc:
                out.write("FAILED PR #%d: %s\n" % (n, exc))
    out.write("Applied %d actions.\n" % done)
    return 0


# --------------------------------------------------------------------------
# Lane cap notice
# --------------------------------------------------------------------------

def notice_text(info, repo):
    return ("%s\n**Advisory lane-capacity notice.** This PR is number %d of %d open `%s/` PRs, above the "
            "configured soft cap of %d for that lane. Every ready PR starts all PR-triggered workflows "
            "(median about 8.5, up to 128), so a full lane delays everyone's CI.\n\n"
            "Nothing is blocked and no check fails. Please finish, merge, or extend an existing PR "
            "in this lane instead of opening more, and close any PR this one supersedes. "
            "Bounded main-health fixes can carry the `%s` label. "
            "See `.github/ci-pr-capacity.md`.\n"
            % (MARK_CAP, info["position"], info["open"], info["lane"], info["cap"], LABEL_CAP_EXEMPT))


def run_notice(api, repo, pr_number, caps, dry_run, out, environ):
    if environ.get("PR_HYGIENE_PAUSED", "").strip().lower() in ("1", "true", "yes"):
        out.write("PR_HYGIENE_PAUSED is set: no notice.\n")
        return
    raw = api("GET", "repos/%s/pulls/%d" % (repo, pr_number))
    pr = normalize_pr(raw)
    if pr["state"] != "open":
        out.write("PR #%d is not open; nothing to do.\n" % pr_number)
        return
    if lane_of(pr["head_ref"], pr["head_repo"], repo) == "fork":
        out.write("PR #%d is from a fork; never acted on.\n" % pr_number)
        return
    if LABEL_CAP in pr["labels"]:
        out.write("PR #%d already carries %s.\n" % (pr_number, LABEL_CAP))
        return
    prs = [normalize_pr(p) for p in paginate(api, "repos/%s/pulls?state=open" % repo)]
    census = lane_census(prs, repo, caps)
    info = census["over"].get(pr_number)
    if not info:
        out.write("PR #%d is within its lane cap.\n" % pr_number)
        return
    out.write("PR #%d is over the %s cap (%d > %d).\n" % (pr_number, info["lane"], info["position"], info["cap"]))
    if dry_run:
        out.write("Dry run: would label %s and comment.\n" % LABEL_CAP)
        return
    comments = paginate(api, "repos/%s/issues/%d/comments" % (repo, pr_number))
    if not any(isinstance(c, dict) and MARK_CAP in (c.get("body") or "") for c in comments):
        api("POST", "repos/%s/issues/%d/comments" % (repo, pr_number), {"body": notice_text(info, repo)})
    assert not LABEL_CAP.startswith("run-")
    api("POST", "repos/%s/issues/%d/labels" % (repo, pr_number), {"labels": [LABEL_CAP]})


# --------------------------------------------------------------------------
# CLI
# --------------------------------------------------------------------------

def _num(text, name, lo, hi):
    try:
        val = float(text)
    except (TypeError, ValueError):
        raise SystemExit("%s must be a number" % name)
    if not lo <= val <= hi:
        raise SystemExit("%s must be between %g and %g" % (name, lo, hi))
    return val


def build_parser():
    p = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = p.add_subparsers(dest="cmd", required=True)
    for name in ("report", "plan", "notice"):
        s = sub.add_parser(name)
        s.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY"))
        s.add_argument("--caps", default=None, help="JSON object; default env PR_LANE_CAPS")
        if name == "notice":
            s.add_argument("--pr", required=True)
            s.add_argument("--dry-run", action="store_true")
            continue
        s.add_argument("--default-branch", default=None)
        s.add_argument("--git-dir", default=".")
        s.add_argument("--remote", default="origin")
        s.add_argument("--branches-file", default=None,
                       help="TSV name/sha/unix-ts plus --merged-file; default: read from git")
        s.add_argument("--merged-file", default=None)
        s.add_argument("--workflows-dir", default=".github/workflows")
        s.add_argument("--manifest", default=".github/branch-protect.txt")
        s.add_argument("--local-worktrees", action="store_true")
        s.add_argument("--stale-days", default=os.environ.get("PR_STALE_DAYS") or "14")
        s.add_argument("--close-after-days", default=os.environ.get("PR_CLOSE_AFTER_DAYS") or "7")
        s.add_argument("--merged-days", default=os.environ.get("PR_MERGED_DAYS") or "3")
        s.add_argument("--closed-days", default=os.environ.get("PR_CLOSED_DAYS") or "30")
        s.add_argument("--orphan-days", default=os.environ.get("PR_ORPHAN_DAYS") or "14")
        s.add_argument("--max-deletes", default=os.environ.get("PR_MAX_DELETES") or "50")
        s.add_argument("--max-pr-actions", default=os.environ.get("PR_MAX_PR_ACTIONS") or "10")
        s.add_argument("--list-limit", default="50")
        if name == "plan":
            s.add_argument("--apply", action="store_true")
            s.add_argument("--allow", action="append", default=None,
                           help="glob a branch must match to be deleted (repeatable)")
            s.add_argument("--remote-url", default=None)
    return p


def make_cfg(args, environ):
    try:
        caps = parse_caps(args.caps if args.caps is not None else environ.get("PR_LANE_CAPS"))
    except (ValueError, json.JSONDecodeError) as exc:
        raise SystemExit("invalid caps: %s" % exc)
    cfg = default_cfg(caps=caps)
    if hasattr(args, "stale_days"):
        cfg.update(
            stale_days=_num(args.stale_days, "stale-days", 1, 3650),
            close_after_days=_num(args.close_after_days, "close-after-days", 1, 3650),
            merged_days=_num(args.merged_days, "merged-days", 0, 3650),
            closed_days=_num(args.closed_days, "closed-days", 1, 3650),
            orphan_days=_num(args.orphan_days, "orphan-days", 1, 3650),
            max_deletes=int(_num(args.max_deletes, "max-deletes", 0, 500)),
            max_pr_actions=int(_num(args.max_pr_actions, "max-pr-actions", 0, 200)),
            list_limit=int(_num(args.list_limit, "list-limit", 1, 500)),
        )
    return cfg


def collect(args, api, git, now):
    repo = args.repo
    if not repo or not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repo):
        raise SystemExit("--repo (or GITHUB_REPOSITORY) must look like owner/name")
    default_branch = args.default_branch
    if not default_branch:
        info = api("GET", "repos/%s" % repo)
        default_branch = (info or {}).get("default_branch")
    if not default_branch:
        raise IncompleteData("cannot determine default branch")
    prs = [normalize_pr(p) for p in paginate(api, "repos/%s/pulls?state=all" % repo)]
    if args.branches_file:
        with open(args.branches_file, encoding="utf-8") as fh:
            btext = fh.read()
        mtext = ""
        if args.merged_file:
            with open(args.merged_file, encoding="utf-8") as fh:
                mtext = fh.read()
        branches = parse_branch_lines(btext, mtext)
        if not branches:
            raise IncompleteData("empty branch listing")
    else:
        branches = collect_branches_git(git, args.git_dir, args.remote, default_branch)
    literal, globs = scan_workflows_dir(args.workflows_dir)
    manifest = read_manifest(args.manifest)
    wt = worktree_branches(git, args.git_dir) if args.local_worktrees else set()
    return {"repo": repo, "default_branch": default_branch, "now": now, "branches": branches,
            "prs": prs, "workflow_literal": literal, "workflow_globs": globs,
            "manifest": manifest, "worktree_branches": wt}


def main(argv=None, *, api=None, git=None, now=None, out=None, environ=None):
    out = out or sys.stdout
    environ = os.environ if environ is None else environ
    api = api or gh_api
    git = git or run_git
    args = build_parser().parse_args(argv)
    if args.cmd == "notice":
        # Advisory only: whatever happens, exit 0 so this can never block a PR.
        try:
            if not args.pr.isdigit():
                raise ValueError("--pr must be an integer")
            cfg = make_cfg(args, environ)
            run_notice(api, args.repo, int(args.pr), cfg["caps"], args.dry_run, out, environ)
        except SystemExit as exc:
            out.write("notice: skipped (%s)\n" % exc)
        except Exception as exc:  # noqa: BLE001 - deliberate: never fail a PR
            out.write("notice: skipped (%s: %s)\n" % (type(exc).__name__, str(exc)[:200]))
        return 0

    cfg = make_cfg(args, environ)
    now = time.time() if now is None else now
    try:
        snap = collect(args, api, git, now)
        if args.cmd == "report":
            out.write(render_report(snap, cfg))
            return 0
        plan = build_plan(snap, cfg)
        allow = list(args.allow or [])
        if not allow and environ.get("PR_HYGIENE_ALLOW"):
            allow = [g.strip() for g in environ["PR_HYGIENE_ALLOW"].split(",") if g.strip()]
        out.write(render_plan(plan, snap, cfg, args.apply))
        if not args.apply:
            return 0
        return apply_plan(plan, snap, {"allow": allow, "remote_url": args.remote_url},
                          api, git, environ.get("GH_TOKEN") or environ.get("GITHUB_TOKEN"), out, environ)
    except IncompleteData as exc:
        out.write("ABORT (fail closed): %s\n" % exc)
        return 1


if __name__ == "__main__":
    sys.exit(main())
