"""GAE pricing for run records (AGENTS.md §5a).

Unpriced work is priced at 0, 0.1 and 1 GAE per operation; an arm is ranked only
where the order of arms is the same across the band.  Every experiment emits
one JSON record with the raw counters and the three priced totals.
"""
from __future__ import annotations

import json
from dataclasses import dataclass, asdict, field

RATES = (0.0, 0.1, 1.0)


@dataclass
class Record:
    experiment: str
    config: dict = field(default_factory=dict)
    counts: dict = field(default_factory=dict)      # primitive -> int
    metrics: dict = field(default_factory=dict)     # measured quantities
    seconds: float = 0.0

    def priced(self) -> dict:
        total = sum(self.counts.values())
        return {f"gae_{r:g}": total*r for r in RATES}

    def to_dict(self) -> dict:
        d = {"experiment": self.experiment, "config": self.config,
             "counts": self.counts, "metrics": self.metrics,
             "seconds": round(self.seconds, 4)}
        d.update(self.priced())
        return d


def write_jsonl(path, records):
    with open(path, "w", encoding="utf-8") as fh:
        for r in records:
            fh.write(json.dumps(r.to_dict() if isinstance(r, Record) else r) + "\n")
