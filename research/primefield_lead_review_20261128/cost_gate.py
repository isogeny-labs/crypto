"""Recompute conditional fixed-B677 cost gates from public PR reports.

These are arithmetic checks of producer-reported medians, not validated timings.
"""

from dataclasses import dataclass


@dataclass(frozen=True)
class Cost:
    fixture_ms: float
    table_ms: float
    scan_ms: float
    graph_ms: float
    rho_ms: float

    def fixed_overhead_ms(self) -> float:
        return self.fixture_ms + self.table_ms + self.graph_ms

    def max_scan_only_speedup(self) -> float:
        return (self.fixed_overhead_ms() + self.scan_ms) / self.fixed_overhead_ms()


def main() -> None:
    b = Cost(1.2, 72.8, 178.7, 31.0, 4.6)
    total = b.fixed_overhead_ms() + b.scan_ms
    assert abs(total - 283.7) < 1e-9
    print(f"B677 reported phase sum: {total:.1f} ms; reported cold total: 287.6 ms")
    print(f"Unattributed difference: {287.6 - total:.1f} ms")
    print(f"Fixed measured phases without query scan: {b.fixed_overhead_ms():.1f} ms")
    print(f"Conditional floor / equal-engineering rho: {b.fixed_overhead_ms()/b.rho_ms:.2f}x")
    print(f"Maximum scan-only speedup on reported phase sum: {b.max_scan_only_speedup():.2f}x")
    print(f"Scan fraction of reported cold total: {b.scan_ms/287.6:.3f}")
    # Check the companion Semaev report's library-vs-reference comparison.
    print(f"Semaev default/reference rank-throughput ratio: {1007/2050:.3f}")
    print(f"Semaev default/reference cold-time ratio: {0.63/0.32:.3f}x")


if __name__ == "__main__":
    main()
