#!/usr/bin/env python3
"""Complete one-target n23 cofactor-cleared group-index IC and rho control."""

from sage.all import *
from collections import Counter
import hashlib
import json
from pathlib import Path
import platform
import random
import resource
import signal
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
SOURCE = ROOT / SPEC["source_instance_result"]
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_instance_sha256"]
SOURCE_RESULT = json.loads(SOURCE.read_text())
R = SPEC["subgroup_order"]
HCOF = SPEC["cofactor"]

def canonical(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()

def digest(obj):
    return hashlib.sha256(canonical(obj)).hexdigest()

def to_int(e):
    return sum(int(c) << i for i, c in enumerate(e.polynomial().list()))

def point_record(P):
    return [to_int(P[0]), to_int(P[1])]

def save(name, obj):
    (HERE / name).write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n")

def timeout(*_):
    raise TimeoutError("frozen 180-second full-pipeline process cap")

class RowSpace:
    def __init__(self, columns):
        self.columns = columns
        self.pivots = {}
        self.rows = []

    def add(self, coeffs, rhs):
        self.rows.append((coeffs, rhs))
        row = [x % R for x in coeffs] + [rhs % R]
        for pivot in sorted(self.pivots):
            factor = row[pivot]
            if factor:
                basis = self.pivots[pivot]
                for j in range(pivot, self.columns + 1):
                    row[j] = (row[j] - factor * basis[j]) % R
        pivot = next((j for j in range(self.columns) if row[j]), None)
        if pivot is None:
            assert row[-1] == 0, "inconsistent relation"
            return False
        inverse = pow(row[pivot], -1, R)
        self.pivots[pivot] = [x * inverse % R for x in row]
        return True

    @property
    def rank(self):
        return len(self.pivots)

    def solve(self):
        assert self.rank == self.columns
        logs = [0] * self.columns
        for pivot in sorted(self.pivots, reverse=True):
            row = self.pivots[pivot]
            logs[pivot] = (row[-1] - sum(row[j] * logs[j]
                                            for j in range(pivot + 1, self.columns))) % R
        assert all(sum(c * logs[i] for i, c in enumerate(coeffs)) % R == rhs % R
                   for coeffs, rhs in self.rows)
        return logs

def build_context():
    n = SPEC["field_degree"]
    poly = SOURCE_RESULT["field_modulus"]
    F2 = GF(2)
    PR = PolynomialRing(F2, "z")
    z = PR.gen()
    mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
    K = GF(1 << n, "z", modulus=mod)
    def elt(v):
        return K([(v >> i) & 1 for i in range(n)])
    E = EllipticCurve(K, [1, elt(SOURCE_RESULT["curve_a2"]), 0, 0,
                          elt(SOURCE_RESULT["curve_a6"])])
    assert int(E.cardinality()) == R * HCOF
    return E, elt

def build_base(E, elt):
    basis = SOURCE_RESULT["subspace_basis"]
    raw = []
    for bits in range(1 << len(basis)):
        x = 0
        for j, v in enumerate(basis):
            if (bits >> j) & 1:
                x ^= v
        raw.extend(E.lift_x(elt(x), all=True))
    assert len(raw) == SOURCE_RESULT["raw_lifted_points"]
    assert digest([point_record(P) for P in raw]) == SOURCE_RESULT["raw_points_sha256"]
    zero = E(0)
    cleared_all = [HCOF * P for P in raw]
    base = sorted(set(cleared_all) - {zero}, key=point_record)
    assert len(base) == SOURCE_RESULT["cleared_nonidentity_points"]
    assert digest([point_record(P) for P in base]) == SOURCE_RESULT["cleared_points_sha256"]
    G = next(P for P in cleared_all if P != zero)
    assert point_record(G) == SOURCE_RESULT["generator"]
    assert all(R * P == zero for P in base)
    reps = sorted({min((P, -P), key=point_record) for P in base}, key=point_record)
    assert len(reps) * 2 == len(base)
    rep_index = {P: i for i, P in enumerate(reps)}
    encoding = {}
    for P in base:
        rep = min((P, -P), key=point_record)
        encoding[P] = (rep_index[rep], 1 if P == rep else -1)
    return base, reps, encoding, G

def build_index(base):
    sums = {}
    for i, A in enumerate(base):
        for j, B in enumerate(base):
            sums.setdefault(A + B, []).append((i, j))
    return sums

def lookup(index, base, target, rng):
    start = rng.randrange(len(base))
    attempts = 0
    for offset in range(len(base)):
        c = (start + offset) % len(base)
        attempts += 1
        pairs = index.get(target - base[c])
        if pairs:
            a, b = pairs[rng.randrange(len(pairs))]
            return (a, b, c), attempts
    return None, attempts

def row_for(indices, base, encoding):
    row = [0] * (len(base) // 2)
    for i in indices:
        col, sign = encoding[base[i]]
        row[col] = (row[col] + sign) % R
    return row

def manifest(base, reps, G):
    source_hash = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    field = {"p": 2, "n": 23, "representation": "polynomial_basis",
             "defining_polynomial_int": SOURCE_RESULT["field_modulus"],
             "element_encoding": "nonnegative_polynomial_coefficient_bitmask"}
    curve = {"model": "y^2+x*y=x^3+a2*x^2+a6",
             "coefficients": {"a1": 1, "a2": SOURCE_RESULT["curve_a2"],
                              "a3": 0, "a4": 0, "a6": SOURCE_RESULT["curve_a6"]},
             "curve_order": R * HCOF, "trace": (1 << 23) + 1 - R * HCOF,
             "r": R, "cofactor": HCOF, "G": point_record(G),
             "target_group": "prime_order_r_subgroup"}
    curve_id = "EC1N23Ch4randh" + digest({"field": field, "curve": curve})[:12]
    curve["curve_id"] = curve_id
    record = {"schema_version": 1, "field": field, "curve": curve,
              "isogeny": "none",
              "endomorphism": {"endomorphism_order_conductor": None,
                               "frobenius_order_conductor": None,
                               "volcano_levels": None, "status": "unknown_not_used"},
              "factor_base": {"construction": "all_curve_lifts_above_frozen_x_subspace_then_clear_by_36",
                              "subspace_basis": SOURCE_RESULT["subspace_basis"],
                              "encoded_points": [point_record(P) for P in base],
                              "enumerated_set_digest": digest([point_record(P) for P in base]),
                              "nominal_dimension": 8, "geometric_point_count": SOURCE_RESULT["raw_lifted_points"],
                              "actual_usable_points": len(base), "sign_quotient_rule": "lexicographic_min_of_P_and_neg_P",
                              "effective_columns": len(reps), "subgroup_filter": "nonidentity_after_cofactor_36"},
              "point_decomposition": {"m": 3, "family": "mitm", "variant": "ordered_pair_sum_index",
                                      "summation_polynomial": "none_group_law_index",
                                      "weil_descent_encoding": "none",
                                      "index_entries": len(base)**2,
                                      "witness_policy": "random_start_C_first_nonempty_pair_bucket_random_pair",
                                      "internal_matrix_kernel": "none", "cache_policy": "reusable_pair_index",
                                      "query_cap": len(base), "source_digest": source_hash},
              "relation_collection": {"query_distribution": "uniform_nonzero_known_scalar_times_G",
                                      "target_exclusion": "exclude_one_frozen_final_target_scalar",
                                      "witness_rule": "first_verified_index_witness",
                                      "rank_stop": len(reps), "query_cap": SPEC["query_cap"],
                                      "duplicate_dependency_policy": "retain_all_rows_rank_increment_only",
                                      "source_digest": source_hash},
              "relation_linear_algebra": {"modulus": R, "columns": len(reps),
                                          "row_rule": "signed_inverse_folded_three_point_coefficients",
                                          "solver": "gauss", "implementation": "incremental_modular_pivots_backsub",
                                          "rank_criterion": len(reps), "source_digest": source_hash},
              "target_descent": {"policy": "direct_pair_index_lookup",
                                 "success_rule": "verified_group_sum_then_scalar_replay",
                                 "source_digest": source_hash},
              "implementation": {"source_sha256": {"this_runner": source_hash,
                                                       "source_instance": SPEC["source_instance_sha256"]},
                                 "algorithm_flags": {"one_cpu_worker": True, "factor_base_sign_fold": True}}}
    candidate_id = (f"IC1N23Ch4randfb{len(base)}PDP3mitmRCsampleLAgaussTDdirectISO0h"
                    + digest(record)[:12])
    return curve_id, candidate_id, record

class OnlineClock:
    phases = ("target_query", "target_pdp", "target_relation_check",
              "target_descent", "target_recovery_check")

    def __init__(self):
        self.times = {phase: 0 for phase in self.phases}
        self.start = self.last = time.perf_counter_ns()
        self.phase = "target_query"

    def switch(self, phase):
        now = time.perf_counter_ns()
        self.times[self.phase] += now - self.last
        self.phase = phase
        self.last = now

    def finish(self):
        now = time.perf_counter_ns()
        self.times[self.phase] += now - self.last
        assert sum(self.times.values()) == now - self.start
        return now - self.start

def ic_online(base, reps, encoding, G, index, logs, target):
    rng = random.Random(73023)
    clock = OnlineClock()
    attempts = 0
    start = rng.randrange(len(base))
    for offset in range(len(base)):
        c = (start + offset) % len(base)
        attempts += 1
        residual = target - base[c]
        clock.switch("target_pdp")
        pairs = index.get(residual)
        if not pairs:
            clock.switch("target_query")
            continue
        a, b = pairs[rng.randrange(len(pairs))]
        clock.switch("target_relation_check")
        assert base[a] + base[b] + base[c] == target
        row = row_for((a, b, c), base, encoding)
        assert sum((row[j] * reps[j] for j in range(len(reps))), G.curve()(0)) == target
        clock.switch("target_descent")
        scalar = sum(v * logs[j] for j, v in enumerate(row)) % R
        clock.switch("target_recovery_check")
        assert scalar * G == target
        elapsed = clock.finish()
        return {"status": "verified", "scalar": scalar, "wall_ns": elapsed,
                "phase_wall_ns": clock.times, "third_point_attempts": attempts,
                "witness_indices": [a, b, c], "witness": [point_record(base[i]) for i in (a, b, c)],
                "row_sha256": digest(row)}
    elapsed = clock.finish()
    return {"status": "zero_yield", "scalar": None, "wall_ns": elapsed,
            "phase_wall_ns": clock.times, "third_point_attempts": attempts,
            "witness_indices": None, "witness": None, "row_sha256": None}

def rho_online(G, target):
    rng = random.Random(92023)
    start = time.perf_counter_ns()
    adds = doubles = restarts = 0

    def step(state):
        nonlocal adds, doubles
        P, a, b = state
        bucket = 0 if P == G.curve()(0) else to_int(P[0]) % 3
        if bucket == 0:
            adds += 1
            return P + G, (a + 1) % R, b
        if bucket == 1:
            doubles += 1
            return P + P, (2 * a) % R, (2 * b) % R
        adds += 1
        return P + target, a, (b + 1) % R

    for _ in range(100):
        a, b = rng.randrange(R), rng.randrange(R)
        initial = (a * G + b * target, a, b)
        tortoise, hare = step(initial), step(step(initial))
        for _ in range(20000):
            if tortoise[0] == hare[0]:
                denominator = (hare[2] - tortoise[2]) % R
                if denominator:
                    scalar = (tortoise[1] - hare[1]) * pow(denominator, -1, R) % R
                    if scalar * G == target:
                        return {"status": "verified", "scalar": scalar,
                                "wall_ns": time.perf_counter_ns() - start,
                                "adds": adds, "doubles": doubles, "restarts": restarts}
                break
            tortoise, hare = step(tortoise), step(step(hare))
        restarts += 1
    return {"status": "restart_cap", "scalar": None,
            "wall_ns": time.perf_counter_ns() - start,
            "adds": adds, "doubles": doubles, "restarts": restarts}

def main():
    result = {"schema": "h4-n23-full-pipeline-result-v1", "status": "running",
              "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
              "source_instance_sha256": SPEC["source_instance_sha256"],
              "host": {"node": platform.node(), "system": platform.system(),
                       "machine": platform.machine(), "tier": "ordinary_exploratory"}}
    save("result.json", result)
    signal.signal(signal.SIGALRM, timeout)
    signal.alarm(SPEC["process_cap_seconds"])
    cold_start = time.perf_counter_ns()
    phases = {name: 0 for name in ("setup", "isogeny", "factor_base", "precompute",
                                  "queries", "pdp", "relation_check", "matrix_build",
                                  "relation_la", "target_descent", "recovery_check")}
    records = []
    try:
        t = time.perf_counter_ns()
        E, elt = build_context()
        phases["setup"] += time.perf_counter_ns() - t
        t = time.perf_counter_ns()
        base, reps, encoding, G = build_base(E, elt)
        phases["factor_base"] += time.perf_counter_ns() - t
        curve_id, candidate_id, candidate = manifest(base, reps, G)
        result.update(curve_id=curve_id, candidate_id=candidate_id,
                      base_points=len(base), folded_columns=len(reps))
        t = time.perf_counter_ns()
        index = build_index(base)
        phases["precompute"] += time.perf_counter_ns() - t
        result.update(pair_entries=len(base)**2, distinct_pair_sums=len(index))
        if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 8 * 1024**3:
            raise MemoryError("frozen 8 GiB observed RSS cap")

        fixture_start = time.perf_counter_ns()
        target_label = b"h4-n23-full-target-0"
        fixture_scalar = int.from_bytes(hashlib.sha256(target_label).digest(), "big") % (R - 1) + 1
        target = fixture_scalar * G
        fixture_wall_ns = time.perf_counter_ns() - fixture_start
        workload_record = {"curve_id": curve_id, "subgroup_order": R,
                           "G": point_record(G), "target": point_record(target),
                           "target_count": 1, "target_law": "sha256_nonzero_scalar_times_G",
                           "target_label": target_label.decode(), "precomputation_state": "ready"}
        workload_id = "W" + digest(workload_record)[:12]
        run_id = candidate_id + workload_id + "R1"
        result.update(workload_id=workload_id, run_id=run_id,
                      target=point_record(target), target_fixture_scalar=fixture_scalar)

        space = RowSpace(len(reps))
        anchor = [0] * len(reps)
        col, sign = encoding[G]
        anchor[col] = sign
        assert space.add(anchor, 1)
        rng = random.Random(61023)
        for query_number in range(1, SPEC["query_cap"] + 1):
            if space.rank == len(reps):
                break
            t = time.perf_counter_ns()
            scalar = rng.randrange(1, R)
            while scalar == fixture_scalar:
                scalar = rng.randrange(1, R)
            query_point = scalar * G
            phases["queries"] += time.perf_counter_ns() - t
            t = time.perf_counter_ns()
            indices, attempts = lookup(index, base, query_point, rng)
            pdp_ns = time.perf_counter_ns() - t
            phases["pdp"] += pdp_ns
            record = {"query_number": query_number, "known_scalar": scalar,
                      "third_point_attempts": attempts, "pdp_wall_ns": pdp_ns,
                      "status": "zero_yield" if indices is None else "verified",
                      "rank_before": space.rank, "witness_indices": list(indices) if indices else None}
            if indices is not None:
                t = time.perf_counter_ns()
                assert sum((base[i] for i in indices), E(0)) == query_point
                phases["relation_check"] += time.perf_counter_ns() - t
                t = time.perf_counter_ns()
                row = row_for(indices, base, encoding)
                phases["matrix_build"] += time.perf_counter_ns() - t
                t = time.perf_counter_ns()
                gained = space.add(row, scalar)
                phases["relation_la"] += time.perf_counter_ns() - t
                record.update(rank_gain=int(gained), row_sha256=digest(row))
            else:
                record.update(rank_gain=0, row_sha256=None)
            record["rank_after"] = space.rank
            records.append(record)
            if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 8 * 1024**3:
                raise MemoryError("frozen 8 GiB observed RSS cap")
        result.update(relation_attempts=len(records),
                      relation_status_counts=dict(Counter(x["status"] for x in records)),
                      novel_rows=sum(x["rank_gain"] for x in records), final_rank=space.rank)
        if space.rank != len(reps):
            result["status"] = "censored_rank_incomplete"
            return
        t = time.perf_counter_ns()
        logs = space.solve()
        phases["relation_la"] += time.perf_counter_ns() - t
        t = time.perf_counter_ns()
        assert all(logs[i] * G == reps[i] for i in range(len(reps)))
        phases["recovery_check"] += time.perf_counter_ns() - t
        cold_observed = time.perf_counter_ns() - cold_start - fixture_wall_ns
        phases["setup"] += cold_observed - sum(phases.values())
        assert sum(phases.values()) == cold_observed
        result.update(cold_wall_ns=cold_observed, fixture_wall_ns=fixture_wall_ns,
                      cold_phase_wall_ns=phases,
                      matrix_rows=len(space.rows), matrix_columns=len(reps),
                      matrix_rank=space.rank, factor_logs=logs,
                      factor_logs_sha256=digest(logs),
                      relation_matrix_sha256=digest(space.rows),
                      factor_logs_verified=True,
                      relation_seed=61023, target_descent_seed=73023, rho_seed=92023)
        save("candidate.json", {"candidate_id": candidate_id, "record": candidate})
        save("workload.json", {"workload_id": workload_id, "record": workload_record})
        ic = ic_online(base, reps, encoding, G, index, logs, target)
        rho = rho_online(G, target)
        assert ic["status"] == rho["status"] == "verified"
        assert ic["scalar"] == rho["scalar"] == fixture_scalar
        result.update(status="complete", ic=ic, rho=rho,
                      online_wall_ns=ic["wall_ns"], rho_online_wall_ns=rho["wall_ns"],
                      verified_scalar=ic["scalar"], rho_verified_scalar=rho["scalar"],
                      target_point_sha256=digest(point_record(target)),
                      peak_rss_bytes=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss)
    except TimeoutError as exc:
        result.update(status="censored_timeout", error=str(exc))
    except MemoryError as exc:
        result.update(status="censored_memory_limit", error=str(exc))
    except Exception as exc:
        result.update(status="error", error=repr(exc))
        raise
    finally:
        signal.alarm(0)
        result["relation_trace"] = records
        result["peak_rss_bytes"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        save("result.json", result)
    print(json.dumps({k:v for k,v in result.items() if k != "relation_trace"}, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
