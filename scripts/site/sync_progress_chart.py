#!/usr/bin/env python3
"""Synchronize the scoreboard chart and its accessible table from one JSON source.

    python3 scripts/site/sync_progress_chart.py
    python3 scripts/site/sync_progress_chart.py --check
"""
from __future__ import annotations

import argparse
import html
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TIMELINE = ROOT / "docs/ic/progress-timeline.json"
SCOREBOARD = ROOT / "docs/index-calculus-scoreboard.html"
SCRIPT = re.compile(r'(<script type="application/json" id="progress-data">).*?(</script>)', re.S)
TABLE = re.compile(r'(<summary>Every plotted point as a table</summary>.*?<tbody>).*?(</table>)', re.S)


def point_rows(timeline: dict) -> str:
    rows = []
    for series in timeline["series"]:
        for point in series["points"]:
            e = lambda value: html.escape(str(value), quote=True)
            unit = f' <small>{e(point["unit"])}</small>' if "unit" in point else ""
            rows.append(
                f'<tr><td>{e(point["date"])}</td>'
                f'<td>{e(series["label"])}<br><small>{e(series["note"])}</small></td>'
                f'<td>{e(point["label"])}<br><small>{e(point["detail"])}</small></td>'
                f'<td class="n">{e(point["value"])}&times;{unit}</td>'
                f'<td>{e(point["reference"])}</td>'
                f'<td><span class="chip">{e(point["class"])}</span></td>'
                f'<td><a href="{e(point["anchor"])}">panel</a> &middot; '
                f'<a href="{e(point["source"])}">source</a></td></tr>'
            )
    return "\n" + "\n".join(rows) + "\n"


def synchronized(page: str, raw: str) -> str:
    if "</script" in raw.lower():
        raise ValueError("timeline JSON contains an HTML script terminator")
    timeline = json.loads(raw)
    if len(SCRIPT.findall(page)) != 1 or len(TABLE.findall(page)) != 1:
        raise ValueError("scoreboard must have one progress script and one point table")
    page = SCRIPT.sub(lambda m: m.group(1) + raw.rstrip() + m.group(2), page)
    # Include the table terminator so rows left outside an older <tbody> are
    # removed along with the stale body. Those rows are still counted by the
    # site test and would otherwise duplicate canonical timeline points.
    return TABLE.sub(lambda m: m.group(1) + point_rows(timeline) + "</tbody>" + m.group(2), page)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail if the scoreboard is stale")
    args = parser.parse_args()
    page = SCOREBOARD.read_text()
    fresh = synchronized(page, TIMELINE.read_text())
    if args.check:
        if page != fresh:
            parser.exit(1, "scoreboard progress chart is stale; run scripts/site/sync_progress_chart.py\n")
    elif page != fresh:
        SCOREBOARD.write_text(fresh)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
