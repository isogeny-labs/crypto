#!/usr/bin/env python3
"""Complete n35 pipeline with triangular pair index and sparse target check."""

from sage.all import *
from collections import Counter
from sage.groups.generic import discrete_log_rho
import inspect
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
assert hashlib.sha256((ROOT / SPEC["triangular_component_result"]).read_bytes()).hexdigest() == SPEC["triangular_component_sha256"]
assert hashlib.sha256((ROOT / SPEC["sparse_component_source"]).read_bytes()).hexdigest() == SPEC["sparse_component_source_sha256"]
assert hashlib.sha256((ROOT / SPEC["sparse_component_result"]).read_bytes()).hexdigest() == SPEC["sparse_component_result_sha256"]
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest() == SPEC["sage_rho_source_sha256"]
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
    raise TimeoutError("frozen 900-second full-pipeline process cap")

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
    assert len(raw) == SOURCE_RESULT["raw_count"]
    zero = E(0)
    cleared_all = [HCOF * P for P in raw]
    base = sorted(set(cleared_all) - {zero}, key=point_record)
    assert len(base) == SOURCE_RESULT["base_count"]
    assert digest([point_record(P) for P in base]) == SOURCE_RESULT["base_sha256"]
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

def point_key(P):
    if P == P.curve()(0):
        return 1 << 70
    return int(P[0].to_integer()) | (int(P[1].to_integer()) << 35)

def build_index(base):
    sums = {}
    size = len(base)
    for i, A in enumerate(base):
        for j in range(i, size):
            sums.setdefault(point_key(A + base[j]), i * size + j)
    return sums

def lookup(index, base, target, rng):
    start = rng.randrange(len(base))
    attempts = 0
    for offset in range(len(base)):
        c = (start + offset) % len(base)
        attempts += 1
        packed_pair = index.get(point_key(target - base[c]))
        if packed_pair is not None:
            a, b = divmod(packed_pair, len(base))
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
    field = {"p": 2, "n": 35, "representation": "polynomial_basis",
             "defining_polynomial_int": SOURCE_RESULT["field_modulus"],
             "element_encoding": "nonnegative_polynomial_coefficient_bitmask"}
    curve = {"model": "y^2+x*y=x^3+a2*x^2+a6",
             "coefficients": {"a1": 1, "a2": SOURCE_RESULT["curve_a2"],
                              "a3": 0, "a4": 0, "a6": SOURCE_RESULT["curve_a6"]},
             "curve_order": R * HCOF, "trace": (1 << 35) + 1 - R * HCOF,
             "r": R, "cofactor": HCOF, "G": point_record(G),
             "target_group": "prime_order_r_subgroup"}
    curve_id = "EC1N35Ch4randh" + digest({"field": field, "curve": curve})[:12]
    curve["curve_id"] = curve_id
    record = {"schema_version": 1, "field": field, "curve": curve,
              "isogeny": "none",
              "endomorphism": {"endomorphism_order_conductor": None,
                               "frobenius_order_conductor": None,
                               "volcano_levels": None, "status": "unknown_not_used"},
              "factor_base": {"construction": "all_curve_lifts_above_frozen_x_subspace_then_clear_by_66",
                              "subspace_basis": SOURCE_RESULT["subspace_basis"],
                              "encoded_points": [point_record(P) for P in base],
                              "enumerated_set_digest": digest([point_record(P) for P in base]),
                              "nominal_dimension": 12, "geometric_point_count": SOURCE_RESULT["raw_count"],
                              "actual_usable_points": len(base), "sign_quotient_rule": "lexicographic_min_of_P_and_neg_P",
                              "effective_columns": len(reps), "subgroup_filter": "nonidentity_after_cofactor_66"},
              "point_decomposition": {"m": 3, "family": "mitm", "variant": "triangular_pair_sum_direct_integer_key_index_sparse_target_check",
                                      "summation_polynomial": "none_group_law_index",
                                      "weil_descent_encoding": "none",
                                      "index_entries": len(base)*(len(base)+1)//2,
                                      "stored_pair_keys": SOURCE_RESULT["distinct_pair_sums"],
                                      "key_rule": "x_to_integer_or_y_to_integer_shift35_identity_1shift70",
                                      "payload_rule": "first_ordered_pair_i_times_base_size_plus_j",
                                      "witness_policy": "random_start_C_first_nonempty_first_pair",
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
              "target_descent": {"policy": "up_to_16_known_scalar_target_shifts_then_pair_index_lookup_sparse_signed_check",
                                 "max_shifts": SPEC["target_shift_cap"],
                                 "shift_seed": 73035,
                                 "success_rule": "verified_shifted_group_sum_subtract_shift_then_original_scalar_replay",
                                 "source_digest": source_hash},
              "implementation": {"source_sha256": {"this_runner": source_hash,
                                                       "source_instance": SPEC["source_instance_sha256"]},
                                 "algorithm_flags": {"one_cpu_worker": True, "factor_base_sign_fold": True,
                                                     "direct_integer_pair_index": True,
                                                     "triangular_pair_enumeration": True,
                                                     "sparse_target_relation_check": True}}}
    candidate_id = (f"IC1N35Ch4randfb{len(base)}PDP3mitmRCsampleLAgaussTDdescentISO0h"
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
    rng = random.Random(73035)
    clock = OnlineClock()
    shifts = []
    for shift_number in range(SPEC["target_shift_cap"]):
        shift = 0 if shift_number == 0 else rng.randrange(1, R)
        shifted = target + shift * G if shift else target
        clock.switch("target_pdp")
        indices, attempts = lookup(index, base, shifted, rng)
        shifts.append({"shift_number": shift_number, "shift_scalar": shift,
                       "third_point_attempts": attempts,
                       "status": "exhausted" if indices is None else "verified"})
        if indices is None:
            clock.switch("target_query")
            continue
        a, b, c = indices
        clock.switch("target_relation_check")
        assert base[a] + base[b] + base[c] == shifted
        row = row_for(indices, base, encoding)
        signed = {}
        for i in indices:
            col, sign = encoding[base[i]]
            signed[col] = signed.get(col, 0) + sign
        assert len(signed) <= 3
        assert all(row[j] == signed.get(j, 0) % R for j in range(len(reps)))
        assert sum((v * reps[j] for j, v in signed.items()), G.curve()(0)) == shifted
        clock.switch("target_descent")
        scalar = (sum(v * logs[j] for j, v in enumerate(row)) - shift) % R
        clock.switch("target_recovery_check")
        assert scalar * G == target
        elapsed = clock.finish()
        return {"status": "verified", "scalar": scalar, "wall_ns": elapsed,
                "phase_wall_ns": clock.times, "target_shifts": shifts,
                "witness_indices": [a, b, c], "witness": [point_record(base[i]) for i in indices],
                "row_sha256": digest(row),
                "nonzero_signed_columns": sum(v != 0 for v in signed.values()),
                "signed_coefficients": [[j, signed[j]] for j in sorted(signed)]}
    elapsed = clock.finish()
    return {"status": "censored_shift_cap", "scalar": None, "wall_ns": elapsed,
            "phase_wall_ns": clock.times, "target_shifts": shifts,
            "witness_indices": None, "witness": None, "row_sha256": None,
            "nonzero_signed_columns": None, "signed_coefficients": None}

def rho_online(G, target):
    def coord_hash(P):
        if P == G.curve()(0):
            return 0
        x, y = point_record(P)
        return ((x * 0x9e3779b97f4a7c15) ^
                (y * 0xbf58476d1ce4e5b9)) & ((1 << 64) - 1)
    set_random_seed(92035)
    start = time.perf_counter_ns()
    scalar = int(discrete_log_rho(target, G, ord=R, operation="+",
                                  hash_function=coord_hash))
    assert scalar * G == target
    return {"status": "verified", "scalar": scalar,
            "wall_ns": time.perf_counter_ns() - start,
            "sage_rho_source_sha256": SPEC["sage_rho_source_sha256"]}

def main():
    result = {"schema": "h4-n35-triangular-sparse-full-pipeline-result-v1", "status": "running",
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
        result.update(pair_entries=len(base)*(len(base)+1)//2, distinct_pair_sums=len(index))
        assert len(base) == SOURCE_RESULT["base_count"]
        assert len(index) == SOURCE_RESULT["distinct_pair_sums"]
        save("candidate.json", {"candidate_id": candidate_id, "record": candidate})
        if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 8 * 1024**3:
            raise MemoryError("frozen 8 GiB observed RSS cap")

        fixture_start = time.perf_counter_ns()
        target_label = b"h4-n35-triangular-sparse-full-target-0"
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
        save("workload.json", {"workload_id": workload_id, "record": workload_record})

        space = RowSpace(len(reps))
        anchor = [0] * len(reps)
        col, sign = encoding[G]
        anchor[col] = sign
        assert space.add(anchor, 1)
        rng = random.Random(61035)
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
            if query_number % 100 == 0:
                result.update(relation_attempts=len(records), final_rank=space.rank,
                              relation_status_counts=dict(Counter(x["status"] for x in records)))
                save("result.json", result)
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
                      relation_seed=61035, target_descent_seed=73035, rho_seed=92035)
        ic = ic_online(base, reps, encoding, G, index, logs, target)
        result.update(ic=ic, online_wall_ns=ic["wall_ns"])
        save("result.json", result)
        if ic["status"] != "verified":
            result["status"] = "censored_target_shift_cap"
            return
        try:
            signal.alarm(SPEC["rho_cap_seconds"])
            rho = rho_online(G, target)
        except TimeoutError as exc:
            result.update(status="censored_rho_timeout", rho={"status": "censored_timeout", "error": str(exc)})
            return
        finally:
            signal.alarm(0)
        result.update(rho=rho, rho_online_wall_ns=rho["wall_ns"])
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
        import gzip
        trace_bytes = canonical(records)
        import base64
        trace_gzip = gzip.compress(trace_bytes, compresslevel=9, mtime=0)
        (HERE / "relation-trace.gzip.b64.txt").write_text(base64.b64encode(trace_gzip).decode() + "\n")
        result["relation_trace"] = {"file": "relation-trace.gzip.b64.txt", "sha256": hashlib.sha256(trace_bytes).hexdigest(), "rows": len(records), "encoding": "canonical_json_gzip_base64", "compressed_sha256": hashlib.sha256(trace_gzip).hexdigest()}
        result["partial_cold_phase_wall_ns"] = phases
        result["peak_rss_bytes"] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        save("result.json", result)
    print(json.dumps({k:v for k,v in result.items() if k not in ("factor_logs",)}, indent=2, sort_keys=True))

if __name__ == "__main__":
    main()
