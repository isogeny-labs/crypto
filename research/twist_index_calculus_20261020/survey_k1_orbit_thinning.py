#!/usr/bin/env python3
"""Frozen exact-oracle survey of orbit-complete L2 subsets, not polynomial cost."""
from __future__ import annotations

import collections
import hashlib
import json
import sys
from pathlib import Path
from typing import Optional

from diagnose_k1_missing import exact_set

ROOT = Path(__file__).resolve().parent
Point = Optional[tuple[tuple[int, ...], tuple[int, ...]]]


class Curve:
    def __init__(self, p: int, b0: int):
        self.p = p
        self.b0 = b0
        self.d = next(d for d in range(2, p) if pow(d, (p - 1) // 2, p) == p - 1)
        self.zero = (0, 0, 0, 0)
        self.one = (1, 0, 0, 0)

    def addf(self, a, b):
        return tuple((x + y) % self.p for x, y in zip(a, b))

    def subf(self, a, b):
        return tuple((x - y) % self.p for x, y in zip(a, b))

    def mulf(self, a, b):
        out = [0] * 7
        for i, x in enumerate(a):
            for j, y in enumerate(b):
                out[i + j] += x * y
        for i in range(6, 3, -1):
            out[i - 4] += self.d * out[i]
        return tuple(x % self.p for x in out[:4])

    def powf(self, a, exponent: int):
        result = self.one
        while exponent:
            if exponent & 1:
                result = self.mulf(result, a)
            a = self.mulf(a, a)
            exponent >>= 1
        return result

    def invf(self, a):
        assert a != self.zero
        return self.powf(a, self.p**4 - 2)

    def negp(self, pt: Point) -> Point:
        return None if pt is None else (pt[0], self.subf(self.zero, pt[1]))

    def addp(self, a: Point, b: Point) -> Point:
        if a is None:
            return b
        if b is None:
            return a
        x1, y1 = a
        x2, y2 = b
        if x1 == x2 and self.addf(y1, y2) == self.zero:
            return None
        if x1 == x2:
            numerator = self.mulf((3, 0, 0, 0), self.mulf(x1, x1))
            denominator = self.mulf((2, 0, 0, 0), y1)
        else:
            numerator = self.subf(y2, y1)
            denominator = self.subf(x2, x1)
        slope = self.mulf(numerator, self.invf(denominator))
        x3 = self.subf(self.subf(self.mulf(slope, slope), x1), x2)
        y3 = self.subf(self.mulf(slope, self.subf(x1, x3)), y1)
        return x3, y3

    def mulp(self, pt: Point, k: int) -> Point:
        out = None
        while k:
            if k & 1:
                out = self.addp(out, pt)
            pt = self.addp(pt, pt)
            k >>= 1
        return out

    def oncurve(self, pt: Point) -> bool:
        if pt is None:
            return True
        x, y = pt
        return self.mulf(y, y) == self.addf(self.mulf(self.mulf(x, x), x),
                                               (0, 0, 0, self.b0))


def point(value) -> Point:
    return tuple(tuple(component) for component in value)  # type: ignore[return-value]


def rank_solve(rows: list[tuple[list[int], int]], n: int, columns: int) -> tuple[int, list[int] | None]:
    matrix = [[*coeff, rhs] for coeff, rhs in rows]
    rank = 0
    for col in range(columns):
        pivot = next((i for i in range(rank, len(matrix)) if matrix[i][col]), None)
        if pivot is None:
            continue
        matrix[rank], matrix[pivot] = matrix[pivot], matrix[rank]
        inverse = pow(matrix[rank][col], -1, n)
        matrix[rank] = [(v * inverse) % n for v in matrix[rank]]
        for i in range(len(matrix)):
            if i == rank:
                continue
            factor = matrix[i][col]
            matrix[i] = [(v - factor * w) % n for v, w in zip(matrix[i], matrix[rank])]
        rank += 1
        if rank == len(matrix):
            break
    assert all(row[columns] == 0 for row in matrix[rank:])
    if rank == columns:
        assert len(matrix) == columns
        return rank, [row[columns] for row in matrix]
    return rank, None


def row_from_terms(terms, old_to_new: dict[int, int], old_cols: list[int],
                   coefficients: list[int], anchor: int, a: int, b: int,
                   n: int, columns: int) -> tuple[list[int], int]:
    sums = [0] * columns
    for index, sign in terms:
        new_col = old_to_new[old_cols[index]]
        sums[new_col] = (sums[new_col] + sign * coefficients[index]) % n
    return sums[1:] + [(-b) % n], (a - anchor * sums[0]) % n


def run_seed(p: int, seed: int) -> list[dict]:
    rec_path = ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_v1.json"
    rec = json.loads(rec_path.read_text())
    audit = json.loads((ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_audit_v1.json").read_text())
    cen = json.loads((ROOT / f"quartic_twist_k1_census_p{p}_seed{seed}_v1.json").read_text())
    caudit = json.loads((ROOT / f"quartic_twist_k1_census_p{p}_seed{seed}_audit_v1.json").read_text())
    assert audit["base_logs_verified"] == cen["physical_base"]
    assert caudit["per_query_signed_set_digests_verified"] == cen["queries"]
    n = rec["n"]
    logs = rec["physical_factor_logs"]
    base = [point(v) for v in rec["base_coordinates"]]
    line0 = cen["line0"]
    assert len(base) == len(logs) == cen["physical_base"]
    old_cols = rec["orbit_column"]
    coefficients = rec["orbit_coefficient"]
    reps = rec["orbit_representatives"]
    l0_cols = sorted(set(old_cols[:line0]), key=lambda col: reps[col])
    l2_cols = sorted(set(old_cols[line0:]), key=lambda col: reps[col])
    assert len(l0_cols) == line0 // 12
    assert len(l2_cols) == cen["line2"] // 12
    assert all(sum(old_cols[i] == col for i in range(line0, len(base))) == 12 for col in l2_cols)
    curve = Curve(p, rec["b0"])
    generator = (tuple(rec["generator_x"]), tuple(rec["generator_y"]))
    target_point = (tuple(rec["target_x"]), tuple(rec["target_y"]))
    assert curve.oncurve(generator) and curve.oncurve(target_point)
    assert curve.mulp(generator, 271) == target_point
    assert curve.mulp(generator, n) is None
    pairs = collections.defaultdict(list)
    for i in range(line0):
        for j in range(i + 1, line0):
            for si in (-1, 1):
                for sj in (-1, 1):
                    pairs[(si * logs[i] + sj * logs[j]) % n].append(((i, si), (j, sj)))
    exact_stream = []
    for item in cen["trace"]:
        a, b = item["a"], item["b"]
        signed_set = exact_set(logs, line0, n, pairs, (a + 271 * b) % n)
        serialized = json.dumps(sorted(signed_set), separators=(",", ":")).encode()
        assert hashlib.sha256(serialized).hexdigest() == item["signed_set_sha256"]
        assert len(signed_set) == item["k1_signed_relations"]
        exact_stream.append(sorted(signed_set))
    records = []
    for orbit_count in range(1, len(l2_cols) + 1):
        chosen_l2 = l2_cols[:orbit_count]
        all_cols = l0_cols + chosen_l2
        old_to_new = {col: i for i, col in enumerate(all_cols)}
        selected_ids = list(range(line0)) + [i for i in range(line0, len(base))
                                              if old_cols[i] in chosen_l2]
        assert len(selected_ids) == line0 + 12 * orbit_count
        columns = len(all_cols)
        rows: list[tuple[list[int], int]] = []
        selected_relations = []
        rank = 0
        stop = None
        attempted_systems = 0
        prefix_relations = 0
        cumulative = 0
        first_hit = None
        zeros = 0
        trace = []
        for index, (item, complete) in enumerate(zip(cen["trace"], exact_stream)):
            a, b = item["a"], item["b"]
            residual_log = (a + 271 * b) % n
            subset = [terms for terms in complete if old_cols[terms[3][0]] in chosen_l2]
            assert all(sum(t[0] >= line0 for t in terms) == 1 for terms in subset)
            if not subset:
                zeros += 1
            elif first_hit is None:
                first_hit = index
            cumulative += len(subset)
            if stop is None:
                attempted_systems += sum(
                    residual_log != (sign * logs[i]) % n
                    for i in selected_ids[line0:] for sign in (-1, 1))
                prefix_relations += len(subset)
                for terms in subset:
                    if rank == columns:
                        break
                    assert sum(sign * logs[i] for i, sign in terms) % n == residual_log
                    row, rhs = row_from_terms(terms, old_to_new, old_cols,
                                              coefficients, rec["anchor_log"], a, b, n, columns)
                    candidate_rank, solution = rank_solve(rows + [(row, rhs)], n, columns)
                    if candidate_rank > rank:
                        rows.append((row, rhs))
                        rank = candidate_rank
                        selected_relations.append({"query_index": index,
                                                   "terms": [list(t) for t in terms],
                                                   "row": row, "rhs": rhs, "rank": rank})
                if rank == columns:
                    stop = index + 1
                    actual_rank, solution = rank_solve(rows, n, columns)
                    assert actual_rank == columns and solution is not None
                    expected_logs = [logs[reps[col]] for col in all_cols[1:]] + [271]
                    assert solution == expected_logs
                    for col, log in zip(all_cols[1:], solution[:-1]):
                        assert curve.mulp(generator, log) == base[reps[col]]
                    assert curve.mulp(generator, solution[-1]) == target_point
            trace.append({"query_index": index, "signed_relations": len(subset),
                          "cumulative_signed_relations": cumulative,
                          "rank_after": rank})
        if stop is None:
            stop = len(trace)
            status = "censored"
        else:
            status = "pass"
        records.append({"p": p, "seed": seed, "orbit_count": orbit_count,
                        "chosen_l2_full_columns": chosen_l2,
                        "chosen_l2_representative_ids": [reps[col] for col in chosen_l2],
                        "line0": line0, "line2": 12 * orbit_count,
                        "physical_base": len(selected_ids), "folded_columns": columns,
                        "recorded_stream_cap": len(trace), "processed_queries": stop,
                        "status": status, "anchored_rank": rank,
                        "selected_relations": selected_relations,
                        "prefix_signed_relations": prefix_relations,
                        "full_stream_signed_relations": cumulative,
                        "first_hit": first_hit, "zero_queries": zeros,
                        "attempted_s4_systems_to_stop": attempted_systems,
                        "trace": trace,
                        "source_sha256": hashlib.sha256(rec_path.read_bytes()).hexdigest()})
    assert records[-1]["full_stream_signed_relations"] == cen["total_k1_signed_relations"]
    return records


if __name__ == "__main__":
    assert len(sys.argv) == 2, "usage: survey_k1_orbit_thinning OUTPUT_JSON"
    all_records = [item for p in (73, 97) for seed in range(3)
                   for item in run_seed(p, seed)]
    assert len(all_records) == 21
    out = {"status": "pass", "method": "ideal exact scalar-keyed group oracle, not a polynomial cost",
           "protocol_commit": "b1bd9996dc", "runs": all_records,
           "full_rank_runs": sum(r["status"] == "pass" for r in all_records),
           "censored_runs": sum(r["status"] == "censored" for r in all_records)}
    Path(sys.argv[1]).write_text(json.dumps(out, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: value for key, value in out.items() if key != "runs"}, sort_keys=True))
