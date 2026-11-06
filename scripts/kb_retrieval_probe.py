#!/usr/bin/env python3
"""Measure research-kb retrieval on a fixed set of real questions.

`docs/kb-retrieval-queries.json` lists queries phrased the way a session asks
them, each with the `repo:path` a good answer should contain. This probe runs
each query through the CLI wrapper, records the rank of the expected source
(1-based, or none in the top k), and reports top-1, top-5 and MRR.

It reads the KB; it never writes to it and never edits a source record. The
point is to decide index or ranking changes by measurement, not by feel.

Usage:
    python3 scripts/kb_retrieval_probe.py
    python3 scripts/kb_retrieval_probe.py --top-k 8 --json --out /tmp/probe.json
    python3 scripts/kb_retrieval_probe.py --runner /path/to/research-kb

Exit status is 0 unless --fail-under-top5 is given and the rate is below it.
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

SCHEMA = "kb-retrieval-result/v1"
HERE = Path(__file__).resolve().parent
DEFAULT_QUERIES = HERE.parent / "docs" / "kb-retrieval-queries.json"
DEFAULT_RUNNER = HERE / "research-kb"


def rank_of(results: list[dict], repo: str, path: str) -> int | None:
    """1-based rank of the first result for repo:path, or None."""
    for index, result in enumerate(results):
        for origin in result.get("origins") or []:
            if origin.get("repo") == repo and origin.get("path") == path:
                return index + 1
    return None


def summarize(ranks: list[int | None], top_k: int = 5) -> dict:
    total = len(ranks)
    top1 = sum(1 for r in ranks if r == 1)
    hits = sum(1 for r in ranks if r is not None and r <= top_k)
    mrr = sum(1.0 / r for r in ranks if r) / total if total else 0.0
    return {
        "queries": total,
        "top1": top1,
        "top1_rate": (top1 / total) if total else 0.0,
        "hits": hits,
        "hit_rate": (hits / total) if total else 0.0,
        "mrr": mrr,
    }


def search(runner: Path, query: str, top_k: int) -> list[dict]:
    proc = subprocess.run(
        [str(runner), "search", query, "--top-k", str(top_k)],
        capture_output=True,
        text=True,
        timeout=120,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip()[:200] or f"runner exited {proc.returncode}")
    try:
        payload = json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"runner did not return JSON: {exc}") from exc
    return payload.get("results") or []


def probe(queries: list[dict], runner: Path, top_k: int) -> list[dict]:
    rows = []
    for item in queries:
        results = search(runner, item["query"], top_k)
        rows.append(
            {
                "query": item["query"],
                "expect": f"{item['repo']}:{item['path']}",
                "rank": rank_of(results, item["repo"], item["path"]),
                "top": [
                    f"{(r.get('origins') or [{}])[0].get('repo', '?')}:"
                    f"{(r.get('origins') or [{}])[0].get('path', '?')}"
                    for r in results[:3]
                ],
            }
        )
    return rows


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--queries", type=Path, default=DEFAULT_QUERIES)
    parser.add_argument("--runner", type=Path, default=DEFAULT_RUNNER)
    parser.add_argument("--top-k", type=int, default=None, help="default: top_k in the query file, else 5")
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--out", type=Path, default=None)
    parser.add_argument("--fail-under-top5", type=float, default=None, help="exit 1 below this rate (0..1)")
    args = parser.parse_args(argv)

    data = json.loads(args.queries.read_text(encoding="utf-8"))
    top_k = args.top_k or int(data.get("top_k") or 5)
    runner = args.runner.resolve()
    if not runner.is_file():
        print(f"kb_retrieval_probe: runner not found at {runner}", file=sys.stderr)
        return 2
    try:
        rows = probe(data["queries"], runner, top_k)
    except (RuntimeError, subprocess.TimeoutExpired) as exc:
        print(f"kb_retrieval_probe: {exc}", file=sys.stderr)
        return 2

    stats = summarize([row["rank"] for row in rows], top_k)
    result = {
        "schema": SCHEMA,
        "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "top_k": top_k,
        "queries_file": str(args.queries),
        "runner": str(runner),
        "stats": stats,
        "rows": rows,
    }
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print(f"{'rank':>4}  {'expected':60}  query")
        for row in rows:
            rank = str(row["rank"]) if row["rank"] else "-"
            print(f"{rank:>4}  {row['expect'][:60]:60}  {row['query'][:60]}")
        print(
            f"top-1 {stats['top1']}/{stats['queries']} "
            f"({stats['top1_rate']:.1%}), top-{top_k} {stats['hits']}/{stats['queries']} "
            f"({stats['hit_rate']:.1%}), MRR {stats['mrr']:.3f}"
        )
        print("Rank counts the first result whose origin matches repo:path; a score is relevance, not truth.")
    if args.fail_under_top5 is not None and stats["hit_rate"] < args.fail_under_top5:
        print(
            f"kb_retrieval_probe: top-{top_k} {stats['hit_rate']:.1%} below "
            f"--fail-under-top5 {args.fail_under_top5:.1%}",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
