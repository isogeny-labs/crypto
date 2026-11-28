#!/usr/bin/env python3
"""Agent autonomy ledger (schema crypto.autonomy/v1): do agent sessions finish
tasks without the owner's supervision?

An episode is one task given to an agent session, from the first prompt to its
PR's merge or close (or abandonment). Entries are JSON files under
benchmarks/agent-autonomy/episodes/, named by the SHA-256 of their canonical
content, so they never merge-conflict and an edited entry no longer matches its
id. Entries are never edited or deleted; a later fact is a new entry.

    autonomy_report.py add-episode --session ID --task "..." --task-class C [--pr N]
                       --started ISO --ended ISO --result R [--ci-green-at-merge] [--merged]
                       [--incident KIND:SELF|HUMAN ...] [--tokens N] [--usd X] --evidence E [...]
    autonomy_report.py add-intervention --episode E-... --ts ISO --class C
                       --text-sha256 HEX --classifier haiku|sonnet|human|rule [--audited-by human]
    autonomy_report.py set-check --episode E-... --check reverted|validator_blocking
                       --value true|false --evidence E
    autonomy_report.py verify [--base REF]
    autonomy_report.py report [--since YYYY-MM-DD] [--until YYYY-MM-DD] [--out DIR]
    autonomy_report.py classify-prompt
    autonomy_report.py audit-sample --rate 0.1 --seed N

Intervention classes (each human message after the first prompt):
  redirect  agent was wrong, stuck, off-scope, or the human rescued it; counts against autonomy
  nudge     agent stopped with work left and the human restarted it ("continue", "keep going,
            do X next"); counts against autonomy, reported apart (owner decision 2026-11-15)
  unblock   access, credentials, budget, an owner decision the agent lacked authority for
  approve, new_task, question   neutral
  hook      automated hook feedback (for example a stop hook); neutral, reported apart

Episode state (derived, never stored):
  fail                 any redirect or nudge; result fail/blocked/abandoned; reverted true;
                       validator_blocking true; merged with CI not green
  unsupervised_success result success, no redirect, PR merged, CI green at merge,
                       reverted checked false, validator_blocking not true
  pending              everything else: PR still open, no reverted check yet, window not elapsed
The text of a message is never stored, only its SHA-256.
Every entry is [pcc:unverified] until a pcc claim cites it.
"""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import math
import re
import statistics
import subprocess
import sys
from pathlib import Path

SCHEMA = "crypto.autonomy/v1"
ROOT = Path(__file__).resolve().parents[1]
DEFAULT_DIR = ROOT / "benchmarks" / "agent-autonomy" / "episodes"
DEFAULT_OUT = ROOT / "benchmarks" / "agent-autonomy" / "report"
KINDS = ("episode", "intervention", "check")
PREFIX = {"episode": "E", "intervention": "I", "check": "C"}
TASK_CLASSES = ("ci-repair", "measurement", "lean", "docs-tooling", "research", "blocked", "other")
RESULTS = ("success", "partial", "fail", "blocked", "abandoned")
CLASSES = ("redirect", "nudge", "unblock", "approve", "new_task", "question", "hook")
# Later labels from a higher-ranked classifier override lower ones for the same message.
CLASSIFIER_RANK = {"rule": 0, "haiku": 1, "sonnet": 2, "opus": 3, "human": 4}
CLASSIFIERS = ("haiku", "sonnet", "opus", "human", "rule")
CHECKS = ("reverted", "validator_blocking")
RECOVERY = ("SELF", "HUMAN")
REVERT_WINDOW_DAYS = 14
REQUIRED = {
    "episode": ("created", "session", "task", "task_class", "started", "ended", "result",
                "ci_green_at_merge", "merged", "incidents", "evidence"),
    "intervention": ("created", "episode", "ts", "class", "text_sha256", "classifier"),
    "check": ("created", "episode", "check", "value", "evidence"),
}


def canonical(body: dict) -> bytes:
    return json.dumps(body, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def entry_id(body: dict) -> str:
    return f"{PREFIX[body['kind']]}-{hashlib.sha256(canonical(body)).hexdigest()[:12]}"


def now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="microseconds").replace("+00:00", "Z")


def parse_ts(s: str) -> dt.datetime:
    t = dt.datetime.fromisoformat(s.strip())
    return t if t.tzinfo else t.replace(tzinfo=dt.timezone.utc)


def load(directory: Path) -> dict[str, dict]:
    entries: dict[str, dict] = {}
    if not directory.is_dir():
        return entries
    for path in sorted(directory.glob("*.json")):
        try:
            entries[path.stem] = json.loads(path.read_text())
        except (OSError, json.JSONDecodeError) as exc:
            entries[path.stem] = {"_error": str(exc)}
    return entries


def write(directory: Path, body: dict) -> str:
    body = {"schema": SCHEMA, **body}
    eid = entry_id(body)
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / f"{eid}.json"
    if path.exists():
        print(f"{eid} already recorded", file=sys.stderr)
        return eid
    path.write_text(json.dumps(body, indent=2, sort_keys=True, ensure_ascii=False) + "\n")
    return eid


def need_evidence(evidence: list[str]) -> list[str]:
    ev = [e.strip() for e in evidence if e.strip()]
    if not ev:
        sys.exit("error: at least one --evidence (path, commit, PR, job or receipt) is required")
    return ev


def wilson(k: int, n: int, z: float = 1.96) -> tuple[float, float]:
    if n == 0:
        return (0.0, 1.0)
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, c - h), min(1.0, c + h))


def require_episode(eid: str, entries: dict[str, dict]) -> None:
    if entries.get(eid, {}).get("kind") != "episode":
        sys.exit(f"error: {eid} is not a recorded episode")


# ---------------------------------------------------------------------------
# Derivation
# ---------------------------------------------------------------------------

def effective_interventions(entries: dict[str, dict], eid: str) -> list[dict]:
    """One row per (episode, message hash). The highest-ranked classifier wins
    (human > opus > sonnet > haiku > rule); within a rank, the latest label wins."""
    groups: dict[str, list[tuple[str, dict]]] = {}
    for iid, e in entries.items():
        if e.get("kind") == "intervention" and e.get("episode") == eid:
            groups.setdefault(e["text_sha256"], []).append((iid, e))
    out = []
    for rows in groups.values():
        rows.sort(key=lambda r: (CLASSIFIER_RANK.get(r[1].get("classifier"), 0),
                                 r[1].get("created", ""), r[0]))
        out.append(rows[-1][1])
    return out


def latest_checks(entries: dict[str, dict], eid: str) -> dict[str, bool]:
    # Latest by `created` wins. Two checks with the same timestamp resolve to
    # the adverse value (true), so a tie can never manufacture a success.
    best: dict[str, tuple[str, int, str]] = {}
    for cid, e in entries.items():
        if e.get("kind") == "check" and e.get("episode") == eid:
            key = (e.get("created", ""), int(bool(e["value"])), cid)
            if e["check"] not in best or key > best[e["check"]]:
                best[e["check"]] = key
    return {k: bool(v[1]) for k, v in best.items()}


def episode_state(eid: str, entries: dict[str, dict], at: dt.datetime | None = None) -> dict:
    ep = entries[eid]
    ivs = effective_interventions(entries, eid)
    counts = {c: 0 for c in CLASSES}
    for i in ivs:
        counts[i["class"]] += 1
    checks = latest_checks(entries, eid)
    at = at or dt.datetime.now(dt.timezone.utc)
    window_over = at >= parse_ts(ep["ended"]) + dt.timedelta(days=REVERT_WINDOW_DAYS)
    reverted = checks.get("reverted")
    if counts["redirect"] > 0:
        state, why = "fail", "redirect"
    elif counts["nudge"] > 0:
        state, why = "fail", "nudge"
    elif reverted is True:
        state, why = "fail", "reverted"
    elif checks.get("validator_blocking") is True:
        state, why = "fail", "validator blocking"
    elif ep["result"] in ("fail", "blocked", "abandoned"):
        state, why = "fail", f"result {ep['result']}"
    elif ep["merged"] and not ep["ci_green_at_merge"]:
        state, why = "fail", "CI not green at merge"
    elif ep["result"] == "partial":
        state, why = ("fail", "partial and merged") if ep["merged"] else ("pending", "partial, PR not merged")
    elif not ep["merged"]:
        state, why = "pending", "PR not merged"
    elif reverted is None:
        state, why = "pending", ("no reverted check recorded (window over)" if window_over
                                 else "revert window not elapsed")
    else:
        state, why = "unsupervised_success", "all conditions met"
    return {"state": state, "why": why, "counts": counts, "checks": checks}


def week_start(ts: str) -> str:
    d = parse_ts(ts).date()
    return (d - dt.timedelta(days=d.weekday())).isoformat()


def rate_row(states: list[str]) -> dict:
    k = states.count("unsupervised_success")
    f = states.count("fail")
    p = states.count("pending")
    lo, hi = wilson(k, k + f)
    return {"success": k, "fail": f, "pending": p, "n": len(states),
            "rate": (k / (k + f)) if k + f else None, "ci": (lo, hi)}


def summarize(entries: dict[str, dict], since: str | None = None, at: dt.datetime | None = None,
              until: str | None = None) -> dict:
    superseded = {e.get("supersedes") for e in entries.values()
                  if e.get("kind") == "episode" and e.get("supersedes")}
    eps = {eid: e for eid, e in entries.items() if e.get("kind") == "episode"
           and eid not in superseded
           and (since is None or e["ended"][:10] >= since)
           and (until is None or e["ended"][:10] <= until)}
    st = {eid: episode_state(eid, entries, at) for eid in eps}
    overall = rate_row([s["state"] for s in st.values()])
    by_class: dict[str, dict] = {}
    for tc in sorted({e["task_class"] for e in eps.values()}):
        by_class[tc] = rate_row([st[i]["state"] for i, e in eps.items() if e["task_class"] == tc])
    redirects = [s["counts"]["redirect"] for s in st.values()]
    nudges = [s["counts"]["nudge"] for s in st.values()]
    unblocks = [s["counts"]["unblock"] for s in st.values()]
    hooks = sum(s["counts"]["hook"] for s in st.values())
    inc_total = inc_self = 0
    for e in eps.values():
        for inc in e["incidents"]:
            inc_total += 1
            inc_self += inc["recovery"] == "SELF"
    n_ok = overall["success"]
    cost: dict = {}
    toks = [e["tokens"] for e in eps.values() if e.get("tokens") is not None]
    usds = [e["usd"] for e in eps.values() if e.get("usd") is not None]
    if toks:
        cost["tokens_total"] = sum(toks)
        cost["tokens_per_success"] = sum(toks) / n_ok if n_ok else None
    if usds:
        cost["usd_total"] = sum(usds)
        cost["usd_per_success"] = sum(usds) / n_ok if n_ok else None
    weeks: dict[str, list[str]] = {}
    for eid, e in eps.items():
        weeks.setdefault(week_start(e["ended"]), []).append(st[eid]["state"])
    series = [{"week": w, **rate_row(s)} for w, s in sorted(weeks.items())]
    acc = classifier_accuracy(entries, eps)
    return {"episodes": len(eps), "states": {i: s for i, s in st.items()}, "overall": overall,
            "by_class": by_class, "redirects": redirects, "nudges": nudges, "unblocks": unblocks, "hooks": hooks,
            "incidents": {"total": inc_total, "self": inc_self, "ci": wilson(inc_self, inc_total)},
            "cost": cost, "series": series, "classifier_accuracy": acc}


def classifier_accuracy(entries: dict[str, dict], eps: dict[str, dict]) -> dict:
    agree = total = 0
    for eid in eps:
        by_hash: dict[str, list[dict]] = {}
        for e in entries.values():
            if e.get("kind") == "intervention" and e.get("episode") == eid:
                by_hash.setdefault(e["text_sha256"], []).append(e)
        for rows in by_hash.values():
            rows.sort(key=lambda r: r.get("created", ""))
            human = [r for r in rows if r["classifier"] == "human"]
            machine = [r for r in rows if r["classifier"] != "human"]
            if human and machine:
                total += 1
                agree += human[-1]["class"] == machine[-1]["class"]
    return {"audited": total, "agree": agree, "ci": wilson(agree, total)}


# ---------------------------------------------------------------------------
# Report
# ---------------------------------------------------------------------------

def pct(x: float | None) -> str:
    return "n/a" if x is None else f"{100 * x:.0f}%"


def ci_str(ci: tuple[float, float]) -> str:
    return f"{100 * ci[0]:.0f}-{100 * ci[1]:.0f}%"


def mean_median(xs: list[int]) -> str:
    if not xs:
        return "n/a"
    return f"mean {statistics.mean(xs):.2f}, median {statistics.median(xs):g}"


def render_markdown(s: dict, since: str | None, until: str | None = None) -> str:
    o = s["overall"]
    lines = [f"# Agent autonomy report ({SCHEMA})", "",
             f"Episodes: {s['episodes']}" + (f" ended since {since}" if since else "") + (f" until {until}" if until else "") +
             f". Generated {now()}. All figures are [pcc:unverified] until a pcc claim cites them.", "",
             "## Unsupervised success", "",
             "Rate = unsupervised successes / (successes + failures). Pending episodes are counted "
             "apart and left out of the rate. Interval: Wilson 95%.", "",
             "| task class | episodes | unsupervised success | fail | pending | rate | 95% CI |",
             "|:--|--:|--:|--:|--:|--:|:--|"]
    for name, r in [("all", o), *s["by_class"].items()]:
        lines.append(f"| {name} | {r['n']} | {r['success']} | {r['fail']} | {r['pending']} | "
                     f"{pct(r['rate'])} | {ci_str(r['ci'])} |")
    inc = s["incidents"]
    lines += ["", "## Interventions and recovery", "",
              f"- Redirects per episode: {mean_median(s['redirects'])}",
              f"- Nudges per episode (agent stopped, human restarted it; counted against): {mean_median(s['nudges'])}",
              f"- Unblocks per episode: {mean_median(s['unblocks'])} (reported apart, not counted against)",
              f"- Hook messages: {s['hooks']} (neutral)",
              f"- Self-recovery: {inc['self']}/{inc['total']} incidents fixed without a human message"
              + (f" ({pct(inc['self'] / inc['total'])}, 95% CI {ci_str(inc['ci'])})" if inc["total"] else ""),
              ]
    a = s["classifier_accuracy"]
    lines.append(f"- Classifier accuracy on the human-audited sample: "
                 + (f"{a['agree']}/{a['audited']} (95% CI {ci_str(a['ci'])})" if a["audited"]
                    else "no audited messages yet"))
    lines += ["", "## Cost", ""]
    c = s["cost"]
    if not c:
        lines.append("No tokens or USD recorded.")
    else:
        if "tokens_total" in c:
            per = c["tokens_per_success"]
            lines.append(f"- Tokens: {c['tokens_total']:,} total; per unsupervised success: "
                         + (f"{per:,.0f}" if per is not None else "n/a (no success)"))
        if "usd_total" in c:
            per = c["usd_per_success"]
            lines.append(f"- USD: {c['usd_total']:.2f} total; per unsupervised success: "
                         + (f"{per:.2f}" if per is not None else "n/a (no success)"))
        lines.append("- Cost covers every episode with a recorded value, failures included.")
    lines += ["", "## Weekly series (by week the episode ended, Monday start)", "",
              "| week | episodes | unsupervised success | fail | pending | rate | 95% CI |",
              "|:--|--:|--:|--:|--:|--:|:--|"]
    for w in s["series"]:
        lines.append(f"| {w['week']} | {w['n']} | {w['success']} | {w['fail']} | {w['pending']} | "
                     f"{pct(w['rate'])} | {ci_str(w['ci'])} |")
    lines += ["", "## Episodes", "", "| episode | state | reason |", "|:--|:--|:--|"]
    for eid, st in sorted(s["states"].items()):
        lines.append(f"| {eid} | {st['state']} | {st['why']} |")
    return "\n".join(lines) + "\n"


def render_svg(series: list[dict]) -> str:
    W, H = 760, 460
    L, R = 70, 30
    pw = W - L - R
    top0, top1 = 60, 250       # rate panel
    bot0, bot1 = 300, 400      # episode-count panel
    n = max(len(series), 1)

    def x(i: int) -> float:
        return L + pw * (i + 0.5) / n

    def yr(v: float) -> float:
        return top1 - (top1 - top0) * v

    maxc = max([w["n"] for w in series] + [1])

    def yc(v: float) -> float:
        return bot1 - (bot1 - bot0) * v / maxc

    o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" '
         f'role="img" aria-labelledby="t d">',
         '<title id="t">Weekly unsupervised-success rate and episodes per week</title>',
         '<desc id="d">Points show unsupervised successes over resolved episodes per week with Wilson 95% '
         'whiskers. Bars show episodes per week. Each label gives n, the resolved episodes.</desc>',
         "<style>:root{--bg:#ffffff;--fg:#1f2328;--mut:#6b7280;--grid:#d8dee4;--pt:#0b6bcb;--bar:#9db7d5}"
         "@media (prefers-color-scheme:dark){:root{--bg:#0d1117;--fg:#e6edf3;--mut:#9ba7b4;"
         "--grid:#30363d;--pt:#58a6ff;--bar:#35516f}}"
         "text{font-family:system-ui,sans-serif;font-size:12px;fill:var(--fg)}"
         ".m{fill:var(--mut)}.g{stroke:var(--grid);stroke-width:1}.w{stroke:var(--pt);stroke-width:2}"
         ".p{fill:var(--pt)}.b{fill:var(--bar)}</style>",
         f'<rect width="{W}" height="{H}" fill="var(--bg)"/>',
         f'<text x="{L}" y="28" style="font-size:16px;font-weight:600">'
         f'Agent autonomy: weekly unsupervised-success rate</text>',
         f'<text class="m" x="{L}" y="46">Point = successes / resolved episodes; whiskers = Wilson 95% CI; '
         f'n = resolved episodes</text>']
    for v in (0, 0.25, 0.5, 0.75, 1):
        o.append(f'<line class="g" x1="{L}" x2="{W - R}" y1="{yr(v):.1f}" y2="{yr(v):.1f}"/>')
        o.append(f'<text class="m" x="{L - 8}" y="{yr(v) + 4:.1f}" text-anchor="end">{int(v * 100)}%</text>')
    o.append(f'<text class="m" x="16" y="{(top0 + top1) / 2:.0f}" text-anchor="middle" '
             f'transform="rotate(-90 16 {(top0 + top1) / 2:.0f})">unsupervised success rate</text>')
    for i, w in enumerate(series):
        resolved = w["success"] + w["fail"]
        if resolved:
            lo, hi = w["ci"]
            o.append(f'<line class="w" x1="{x(i):.1f}" x2="{x(i):.1f}" y1="{yr(lo):.1f}" y2="{yr(hi):.1f}"/>')
            for yy in (lo, hi):
                o.append(f'<line class="w" x1="{x(i) - 6:.1f}" x2="{x(i) + 6:.1f}" '
                         f'y1="{yr(yy):.1f}" y2="{yr(yy):.1f}"/>')
            o.append(f'<circle class="p" cx="{x(i):.1f}" cy="{yr(w["rate"]):.1f}" r="5"/>')
            o.append(f'<text x="{x(i) + 10:.1f}" y="{yr(w["rate"]) - 8:.1f}">n={resolved}</text>')
        else:
            o.append(f'<text class="m" x="{x(i):.1f}" y="{(top0 + top1) / 2:.0f}" text-anchor="middle">'
                     f'no resolved episodes</text>')
    o.append(f'<text x="{L}" y="{bot0 - 14}" style="font-weight:600">Episodes per week</text>')
    o.append(f'<line class="g" x1="{L}" x2="{W - R}" y1="{bot1}" y2="{bot1}"/>')
    for v in sorted({0, maxc}):
        o.append(f'<text class="m" x="{L - 8}" y="{yc(v) + 4:.1f}" text-anchor="end">{v}</text>')
    o.append(f'<text class="m" x="16" y="{(bot0 + bot1) / 2:.0f}" text-anchor="middle" '
             f'transform="rotate(-90 16 {(bot0 + bot1) / 2:.0f})">episodes</text>')
    bw = min(60, pw / n * 0.5)
    for i, w in enumerate(series):
        o.append(f'<rect class="b" x="{x(i) - bw / 2:.1f}" y="{yc(w["n"]):.1f}" width="{bw:.1f}" '
                 f'height="{bot1 - yc(w["n"]):.1f}"/>')
        o.append(f'<text x="{x(i):.1f}" y="{yc(w["n"]) - 5:.1f}" text-anchor="middle">{w["n"]}</text>')
        o.append(f'<text class="m" x="{x(i):.1f}" y="{bot1 + 18}" text-anchor="middle">{w["week"]}</text>')
    o.append(f'<text class="m" x="{(L + W - R) / 2:.0f}" y="{H - 16}" text-anchor="middle">'
             f'week starting (Monday, UTC), by the day the episode ended</text>')
    o.append("</svg>")
    return "\n".join(o) + "\n"


def cmd_report(a, d: Path) -> int:
    s = summarize(load(d), a.since, until=a.until)
    md = render_markdown(s, a.since, a.until)
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    (out / "AUTONOMY.md").write_text(md)
    (out / "autonomy.svg").write_text(render_svg(s["series"]))
    print(md, end="")
    return 0


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------

def parse_incident(v: str) -> dict:
    kind, _, rec = v.rpartition(":")
    if not kind or rec not in RECOVERY:
        sys.exit(f"error: --incident wants KIND:SELF or KIND:HUMAN, got {v!r}")
    return {"kind": kind.strip(), "recovery": rec}


def cmd_add_episode(a, d: Path) -> int:
    started, ended = parse_ts(a.started), parse_ts(a.ended)
    if ended < started:
        sys.exit("error: --ended is before --started")
    body = {"kind": "episode", "created": now(), "session": a.session.strip(), "task": a.task.strip(),
            "task_class": a.task_class, "pr": a.pr, "started": a.started, "ended": a.ended,
            "result": a.result, "ci_green_at_merge": bool(a.ci_green_at_merge),
            "merged": bool(a.merged), "incidents": [parse_incident(i) for i in a.incident],
            "tokens": a.tokens, "usd": a.usd, "evidence": need_evidence(a.evidence)}
    if a.supersedes:
        require_episode(a.supersedes, load(d))
        body["supersedes"] = a.supersedes
    print(write(d, body))
    return 0


def cmd_add_intervention(a, d: Path) -> int:
    require_episode(a.episode, load(d))
    parse_ts(a.ts)
    if not re.fullmatch(r"[0-9a-f]{64}", a.text_sha256.lower()):
        sys.exit("error: --text-sha256 must be 64 hex characters (the hash of the message, never the text)")
    body = {"kind": "intervention", "created": now(), "episode": a.episode, "ts": a.ts,
            "class": a.cls, "text_sha256": a.text_sha256.lower(), "classifier": a.classifier,
            "audited_by": a.audited_by or ""}
    print(write(d, body))
    return 0


def cmd_set_check(a, d: Path) -> int:
    require_episode(a.episode, load(d))
    body = {"kind": "check", "created": now(), "episode": a.episode, "check": a.check,
            "value": a.value == "true", "evidence": need_evidence(a.evidence)}
    print(write(d, body))
    return 0


def verify(d: Path, base: str | None) -> list[str]:
    errors: list[str] = []
    entries = load(d)
    for eid, e in entries.items():
        if "_error" in e:
            errors.append(f"{eid}: unreadable ({e['_error']})")
            continue
        if e.get("schema") != SCHEMA or e.get("kind") not in KINDS:
            errors.append(f"{eid}: wrong schema or kind")
            continue
        kind = e["kind"]
        if entry_id(e) != eid:
            errors.append(f"{eid}: content does not match id (edited entry)")
        for f in REQUIRED[kind]:
            if f not in e or e[f] in (None, "") and f not in ("ci_green_at_merge", "merged"):
                errors.append(f"{eid}: missing field {f}")
        if "evidence" in REQUIRED[kind] and not e.get("evidence"):
            errors.append(f"{eid}: no evidence")
        if kind == "episode":
            if e.get("task_class") not in TASK_CLASSES:
                errors.append(f"{eid}: bad task_class")
            if e.get("result") not in RESULTS:
                errors.append(f"{eid}: bad result")
            for inc in e.get("incidents", []):
                if not isinstance(inc, dict) or inc.get("recovery") not in RECOVERY or not inc.get("kind"):
                    errors.append(f"{eid}: bad incident {inc}")
            for f in ("started", "ended"):
                try:
                    parse_ts(e[f])
                except (KeyError, ValueError):
                    errors.append(f"{eid}: bad timestamp {f}")
        else:
            if entries.get(e.get("episode"), {}).get("kind") != "episode":
                errors.append(f"{eid}: references unknown episode {e.get('episode')}")
        if kind == "intervention":
            if e.get("class") not in CLASSES:
                errors.append(f"{eid}: bad class")
            if e.get("classifier") not in CLASSIFIERS:
                errors.append(f"{eid}: bad classifier")
            if not re.fullmatch(r"[0-9a-f]{64}", str(e.get("text_sha256", ""))):
                errors.append(f"{eid}: bad text_sha256")
            if "text" in e:
                errors.append(f"{eid}: message text must not be stored")
        if kind == "check":
            if e.get("check") not in CHECKS:
                errors.append(f"{eid}: bad check")
            if not isinstance(e.get("value"), bool):
                errors.append(f"{eid}: check value must be boolean")
    if base:
        rel = d.resolve().relative_to(ROOT)
        try:
            listing = subprocess.run(["git", "-C", str(ROOT), "ls-tree", "--name-only", f"{base}:{rel}"],
                                     capture_output=True, text=True, check=True).stdout.split()
        except subprocess.CalledProcessError:
            listing = []
        for name in listing:
            old = subprocess.run(["git", "-C", str(ROOT), "show", f"{base}:{rel}/{name}"],
                                 capture_output=True, check=False).stdout
            path = d / name
            if not path.exists():
                errors.append(f"{name}: removed (entries are append-only)")
            elif path.read_bytes() != old:
                errors.append(f"{name}: changed since {base} (entries are append-only)")
    return errors


def cmd_verify(a, d: Path) -> int:
    errors = verify(d, a.base)
    for e in errors:
        print(f"FAIL {e}")
    print(f"autonomy verify: {len(load(d))} entries, {len(errors)} error(s)")
    return 1 if errors else 0


CLASSIFY_PROMPT = """\
You label the human messages that follow the first prompt of one agent episode.
An episode is one task given to an agent session, from the first prompt to its
PR's merge or close. For each human message, choose exactly one class:

redirect  The agent was wrong, stuck or off-scope, or the human had to rescue or
          correct it (for example "that is not what I asked", "undo that",
          "you missed the failing test", "stop doing X"). Counts against autonomy.
nudge     The agent stopped while work remained and the human restarted it
          ("continue", "resume", "keep going, do X next"). Counts against autonomy.
unblock   The agent lacked access, credentials, budget, or an owner decision it
          had no authority to make (for example "here is the token", "yes, you
          may spend up to N", "use option B"). Reported apart; not against autonomy.
approve   The human accepts or confirms work with no correction ("looks good", "go ahead and merge").
new_task  The human starts a different task in the same session.
question  The human asks for information and gives no correction.
hook      Automated hook feedback, for example a stop hook demanding a commit.
          Not typed by a person.

Rules:
- When a message both corrects the agent and asks something, choose redirect.
- When unsure between redirect and another class, choose redirect and say why.
- Judge only what the message and its surrounding turns show. Do not guess intent.
- Never copy message text into the output.

Output: one JSON object per line, nothing else:
{"ts": "<message timestamp, ISO 8601>", "text_sha256": "<sha256 hex of the message text>", "class": "<one of the six>", "reason": "<at most 20 words>"}
"""


def cmd_classify_prompt(a, d: Path) -> int:
    print(CLASSIFY_PROMPT, end="")
    return 0


def audit_sample(ids: list[str], rate: float, seed: int) -> list[str]:
    scored = sorted((int(hashlib.sha256(f"{seed}:{i}".encode()).hexdigest()[:16], 16) / 16 ** 16, i)
                    for i in ids)
    picked = [i for score, i in scored if score < rate]
    if not picked and scored:
        picked = [scored[0][1]]
    return sorted(picked)


def cmd_audit_sample(a, d: Path) -> int:
    ids = sorted(i for i, e in load(d).items() if e.get("kind") == "intervention")
    for i in audit_sample(ids, a.rate, a.seed):
        print(i)
    return 0


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--dir", type=Path, default=DEFAULT_DIR, help="entries directory")
    sub = p.add_subparsers(dest="cmd", required=True)

    s = sub.add_parser("add-episode"); s.add_argument("--session", required=True)
    s.add_argument("--task", required=True)
    s.add_argument("--task-class", required=True, choices=TASK_CLASSES)
    s.add_argument("--pr", type=int); s.add_argument("--started", required=True)
    s.add_argument("--ended", required=True); s.add_argument("--result", required=True, choices=RESULTS)
    s.add_argument("--ci-green-at-merge", action="store_true"); s.add_argument("--merged", action="store_true")
    s.add_argument("--incident", action="append", default=[])
    s.add_argument("--tokens", type=int); s.add_argument("--usd", type=float)
    s.add_argument("--evidence", action="append", default=[])
    s.add_argument("--supersedes", help="episode id this one replaces (e.g. a partial record of a PR that later merged)")

    s = sub.add_parser("add-intervention"); s.add_argument("--episode", required=True)
    s.add_argument("--ts", required=True)
    s.add_argument("--class", dest="cls", required=True, choices=CLASSES)
    s.add_argument("--text-sha256", required=True)
    s.add_argument("--classifier", required=True, choices=CLASSIFIERS)
    s.add_argument("--audited-by", choices=("human",))

    s = sub.add_parser("set-check"); s.add_argument("--episode", required=True)
    s.add_argument("--check", required=True, choices=CHECKS)
    s.add_argument("--value", required=True, choices=("true", "false"))
    s.add_argument("--evidence", action="append", default=[])

    s = sub.add_parser("verify"); s.add_argument("--base")
    s = sub.add_parser("report"); s.add_argument("--since"); s.add_argument("--until")
    s.add_argument("--out", type=Path, default=DEFAULT_OUT)
    sub.add_parser("classify-prompt")
    s = sub.add_parser("audit-sample"); s.add_argument("--rate", type=float, default=0.1)
    s.add_argument("--seed", type=int, default=0)

    a = p.parse_args(argv)
    return {"add-episode": cmd_add_episode, "add-intervention": cmd_add_intervention,
            "set-check": cmd_set_check, "verify": cmd_verify, "report": cmd_report,
            "classify-prompt": cmd_classify_prompt,
            "audit-sample": cmd_audit_sample}[a.cmd](a, a.dir)


if __name__ == "__main__":
    sys.exit(main())
