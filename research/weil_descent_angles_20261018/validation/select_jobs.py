"""Nonce-driven replay sampling for the Weil-descent round-3/4 validation.

Usage: python3 -I select_jobs.py <nonce-hex>

Rule (fixed before the nonce was drawn):
  For each source run, enumerate its job list exactly as n2_routes.rs `main`
  builds it for the recorded command (sizes / cofactors / thetas / instances).
  Score each job by sha256("<nonce>|<run-dir>|<job-id>") (hex, ascending).
  Walk the jobs in score order and select a job if its size (bits) has not
  been selected yet for that run, until the run's quota is reached; if the
  run has fewer distinct sizes than its quota, a second pass admits jobs from
  new replay cells regardless of size.  Quotas: itemOJ 3, itemF 2, H1 3,
  H2 3, H4 2, H6 1.
  The replay unit is the smallest cell the tool can be restricted to with
  N2_SIZES / N2_HS (/ N2_CS for H6): every row of that cell is compared, and
  the selected job's rows are reported separately.
"""
import hashlib
import json
import sys

RUNS = {
    "itemOJ-run-20261018T2132Z": dict(mode="run", quota=3,
        jobs=[dict(bits=b, instance=i) for b in (20, 24, 28, 32, 36, 40) for i in range(3)],
        cell=lambda j: dict(N2_SIZES=str(j["bits"]))),
    "itemF-control-run-20261018T2128Z": dict(mode="control", quota=2,
        jobs=[dict(bits=b, instance=i) for b in (20, 24, 28, 32, 36, 40) for i in range(3)],
        cell=lambda j: dict(N2_SIZES=str(j["bits"]))),
    "H1-hsweep-run-20261018T2240Z": dict(mode="hsweep", quota=3,
        jobs=[dict(bits=b, h=h, instance=i) for b in (32, 36, 40)
              for h in (4, 8, 12, 16, 24, 32, 48, 64, 96, 128) for i in range(4)],
        cell=lambda j: dict(N2_SIZES=str(j["bits"]), N2_HS=str(j["h"]))),
    "H2-glue-run-20261018T2240Z": dict(mode="glue", quota=3,
        jobs=[dict(bits=b, h=h, instance=i) for b in (28, 32, 36, 40) for h in (1, 4) for i in range(4)],
        cell=lambda j: dict(N2_SIZES=str(j["bits"]), N2_HS=str(j["h"]))),
    "H4-memsweep-run-20261018T2209Z": dict(mode="memsweep", quota=2,
        jobs=[dict(bits=b, theta=t, instance=i) for b in (32, 36) for t in ("0.5", "0.25", "0.125") for i in range(4)],
        cell=lambda j: dict(N2_SIZES=str(j["bits"]))),
    "H6-large-run-20261018T2210Z": dict(mode="run", quota=1,
        jobs=[dict(bits=b, instance=i) for b in (44, 48) for i in range(3)],
        cell=lambda j: dict(N2_SIZES=str(j["bits"]), N2_CS="2")),
}


def job_id(j):
    return ",".join(f"{k}={j[k]}" for k in ("bits", "h", "theta", "instance") if k in j)


def select(nonce):
    out = {}
    for run, spec in RUNS.items():
        scored = sorted(spec["jobs"], key=lambda j: hashlib.sha256(f"{nonce}|{run}|{job_id(j)}".encode()).hexdigest())
        chosen, sizes, cells = [], set(), set()
        for j in scored:
            if len(chosen) == spec["quota"]:
                break
            if j["bits"] not in sizes:
                chosen.append(j); sizes.add(j["bits"]); cells.add(json.dumps(spec["cell"](j), sort_keys=True))
        for j in scored:
            if len(chosen) == spec["quota"]:
                break
            c = json.dumps(spec["cell"](j), sort_keys=True)
            if c not in cells:
                chosen.append(j); cells.add(c)
        out[run] = [dict(job=job_id(j), score=hashlib.sha256(f"{nonce}|{run}|{job_id(j)}".encode()).hexdigest(),
                         mode=spec["mode"], env=spec["cell"](j)) for j in chosen]
    return out


if __name__ == "__main__":
    print(json.dumps(select(sys.argv[1]), indent=1))
