"""W2: CryptoMiniSat (native XOR) vs Z3 (And/Xor) on n=13 phase-aware systems, total-cost accounting.

Uses ../../../frobenius_quotient_sat_20260930/encode.py and run.py unchanged (imported).
Targets are the four *random* (natural) targets that `run.py --n 13 --s 4 --phases` draws;
they are compared with the recorded meta of results/n13_phase_b300.jsonl before use.
Z3 receives exactly the clause set and XOR constraints the encoder produced for CMS.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import random
import resource
import sys
import time

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
FQ = REPO / "research" / "frobenius_quotient_sat_20260930"
sys.path.insert(0, str(HERE))
sys.path.append(str(FQ))

import tcommon as tc
import encode as enc                       # frozen encoder (also imports the followup verify_claims)
import run as fq                           # frozen harness: factor_space, targets

N, S, SEED = 13, 4, 20260930
FORM = ("specialized", True, True)         # subgroup_xor: final, subgroup certificate, native XOR
ENGINES = ("sat-cms-xor", "sat-z3")
DEADLINE, AS_LIMIT, CHILD_WALL = 120.0, 2 << 30, 200
SCHEMA = "solver_total_cost/w2/v1"
RECORDED = FQ / "results" / "n13_phase_b300.jsonl"


def xor_tree(z3, values):
    values = list(values)
    while len(values) > 1:
        values = [z3.Xor(values[i], values[i + 1]) if i + 1 < len(values) else values[i]
                  for i in range(0, len(values), 2)]
    return values[0]


class LazyModel:
    """Index like pycryptosat's solution tuple, evaluating Z3 variables on demand."""

    def __init__(self, z3, model, variables):
        self.z3, self.model, self.variables = z3, model, variables

    def __getitem__(self, v):
        return bool(self.z3.is_true(self.model.eval(self.variables[v], model_completion=True)))


def fb_and_targets(st):
    with st.stage("fb_setup"):
        rng = random.Random(SEED * 1000 + N)
        E = enc.Curve(N)
        basis, valid = fq.factor_space(E, S, rng)
        r = E.h
        pts = [E.lift(x) for x in valid]
        roots = [v for v in range(r) if (v * v + v + 2) % r == 0]
        lam, = [v for v in roots if E.mul(v, pts[0]) == E.frob(pts[0])]
        lookup, columns = tc.build_lookup(E, pts, lam, N, r)
    with st.stage("input_gen"):
        planted = fq.targets(E, valid, "planted", 4, rng, True)      # consumed to reproduce the RNG stream
        natural = fq.targets(E, valid, "random", 4, rng, True)
    return E, basis, valid, r, lam, lookup, columns, [R for R, _ in natural], [R for R, _ in planted]


def child(index, engine, prior_rows):
    resource.setrlimit(resource.RLIMIT_AS, (AS_LIMIT, AS_LIMIT))
    st = tc.Stages()
    t_main = time.perf_counter()
    out = {"schema": SCHEMA, "workload": "W2", "engine": engine, "index": index, "n": N,
           "payload_bits": S, "seed": SEED, "formulation": "subgroup_xor", "phases": True,
           "kind": "natural", "deadline_s": DEADLINE}
    try:
        E, basis, valid, r, lam, lookup, columns, natural, planted = fb_and_targets(st)
        R = natural[index]
        out.update(target=[hex(R[0]), hex(R[1])], odd_subgroup_order=r, frobenius_eigenvalue=lam,
                   basis=[hex(b) for b in basis], valid_x_in_space=len(valid),
                   columns=len(columns), orbit_entries=len(lookup))
        final, sub, nx = FORM
        with st.stage("eq_build"):
            M, cvars, evars = enc.build(N, basis, R, final, sub, nx, True)
        out["model_size"] = M.size()
        deadline = time.perf_counter() + DEADLINE
        block_vars = [v for ci in cvars for v in ci] + [e for ev in (evars or []) for e in ev]
        rejected, found, xs = 0, None, None
        if engine == "sat-cms-xor":
            with st.stage("engine_load"):
                import pycryptosat
            with st.stage("solver_setup"):
                solver = pycryptosat.Solver(threads=1)
                M.load(solver)
        else:
            with st.stage("engine_load"):
                import z3
            with st.stage("solver_setup"):
                bv = [None] + [z3.Bool(f"b{i}") for i in range(1, M.nv + 1)]
                solver = z3.Solver()
                solver.set(random_seed=13)
                for clause in M.clauses:
                    solver.add(z3.Or([bv[l] if l > 0 else z3.Not(bv[-l]) for l in clause]))
                for vs, rhs in M.xors:
                    x = xor_tree(z3, [bv[v] for v in vs])
                    solver.add(x if rhs else z3.Not(x))
        pivots = {}
        for row in prior_rows:
            tc.rank_add(pivots, row, r)
        rank_before = len(pivots)
        while True:
            remaining = deadline - time.perf_counter()
            if remaining <= 0:
                raise TimeoutError(f"{engine} aggregate deadline after {rejected} rejected tuples")
            with st.stage("solve"):
                if engine == "sat-cms-xor":
                    sat, sol = solver.solve(time_limit=remaining)
                else:
                    solver.set(timeout=max(1, int(remaining * 1000)))
                    result = solver.check()
                    sat = True if result == z3.sat else (False if result == z3.unsat else None)
            if sat is None:
                raise TimeoutError(f"{engine} timed out after {rejected} rejected tuples")
            if not sat:
                break
            with st.stage("model_extraction"):
                if engine == "sat-z3":
                    sol = LazyModel(z3, solver.model(), bv)
                xs = enc.decode(sol, cvars, basis, evars, E.f)
            with st.stage("lifting"):
                witness = enc.verify(E, xs, R)
            if witness:
                found = (xs, witness)
                break
            rejected += 1
            with st.stage("model_extraction"):
                lits = [-v if sol[v] else v for v in block_vars]
                if engine == "sat-cms-xor":
                    solver.add_clause(lits)
                else:
                    solver.add(z3.Or([bv[l] if l > 0 else z3.Not(bv[-l]) for l in lits]))
        out["rejected_tuples"] = rejected
        out["statistics"] = {"solver": engine, "version": (pycryptosat.__version__ if engine == "sat-cms-xor"
                             else z3.get_version_string())}
        if found is None:
            out.update(status="complete", result="unsat",
                       relation={"natural_verified": 0, "planted_verified": 0, "new_independent_natural": 0,
                                 "rank_before": rank_before, "rank_after": len(pivots),
                                 "columns": len(columns), "modulus_r": r, "relation_rows": []})
        else:
            xs, witness = found
            with st.stage("rank_update"):
                row = tc.relation_row(lookup, len(columns), witness, r)
            with st.stage("verification"):
                assert len({tuple(p) for p in witness}) == 4
                assert all(E.on(p) and E.mul(E.h, p) is None for p in witness)
                total = None
                for p in witness:
                    total = E.add(total, p)
                assert total == R
                assert len({p[0] for p in witness}) == 4
                recon = None
                for i, a in enumerate(row):
                    recon = E.add(recon, E.mul(a, columns[i]) if a else None)
                assert recon == R
            with st.stage("rank_update"):
                independent = tc.rank_add(pivots, row, r)
            out.update(status="complete", result="verified", found_x=[hex(x) for x in sorted(xs)],
                       relation={"natural_verified": 1, "planted_verified": 0,
                                 "new_independent_natural": int(independent), "rank_before": rank_before,
                                 "rank_after": len(pivots), "columns": len(columns), "modulus_r": r,
                                 "relation_rows": [{"row": row, "independent_of_natural_matrix": independent}]})
    except (TimeoutError, MemoryError) as error:
        out.update(status="timeout" if isinstance(error, TimeoutError) else "oom",
                   type=type(error).__name__, message=str(error), result=None,
                   relation={"natural_verified": 0, "planted_verified": 0, "new_independent_natural": 0})
    out["stages"] = st.dump()
    out["child_main_wall_s"] = time.perf_counter() - t_main
    out["peak_rss_bytes"] = tc.peak_rss_bytes()
    out["python"], out["platform"] = sys.version, tc.versions()["platform"]
    return out


def recorded_targets():
    meta = json.loads(RECORDED.open().readline())["meta"]
    return meta["targets"]["random"], meta["basis"]


def parent(out_dir):
    out_dir.mkdir(parents=True, exist_ok=False)
    st = tc.Stages()
    E, basis, valid, r, lam, lookup, columns, natural, planted = fb_and_targets(st)
    mine = [[hex(R[0]), hex(R[1])] for R in natural]
    rec_targets, rec_basis = recorded_targets()
    check = {"targets_equal_recorded_meta": mine == rec_targets,
             "basis_equal_recorded_meta": [hex(b) for b in basis] == rec_basis,
             "targets": mine, "recorded_file": str(RECORDED.relative_to(REPO))}
    assert check["targets_equal_recorded_meta"] and check["basis_equal_recorded_meta"], check
    env = dict(os.environ, PYTHONHASHSEED="0")
    files = [FQ / n for n in ("encode.py", "run.py", "brute_check.py", "test_encode.py")] + \
        [FQ.parent / "frobenius_quotient_followup_20260926" / "verify_claims.py", RECORDED] + \
        [HERE / n for n in ("tcommon.py", "run_w2.py", "PROTOCOL.md")]
    manifest = {"type": "manifest", "schema": SCHEMA, "protocol": "solver_total_cost_20261011",
                "workload": "W2", "engines": list(ENGINES), "targets_check": check,
                "limits": {"address_space_bytes": AS_LIMIT, "child_wall_seconds": CHILD_WALL,
                           "solve_deadline_seconds_aggregate": DEADLINE},
                "env": {"PYTHONHASHSEED": "0", "PYTHONPYCACHEPREFIX": os.environ.get("PYTHONPYCACHEPREFIX")},
                "versions": tc.versions(), "git": tc.git_state(REPO),
                "source_sha256": tc.source_hashes(files, REPO), "loadavg_at_start": os.getloadavg(),
                "started_unix": time.time()}
    path = out_dir / "receipts.jsonl"
    with path.open("x") as stream:
        stream.write(json.dumps(manifest, sort_keys=True) + "\n")
        for engine in ENGINES:
            prior = []
            for index in range(4):
                key = f"W2-n13-s4-t{index}-{engine}"
                cmd = [sys.executable, str(HERE / "run_w2.py"), "--child", "--index", str(index),
                       "--engine", engine, "--prior-rows", json.dumps(prior)]
                rec = tc.run_child(cmd, CHILD_WALL, env)
                result = rec["result"]
                if result and result.get("status") == "complete":
                    prior.extend(x["row"] for x in result["relation"]["relation_rows"]
                                 if x["independent_of_natural_matrix"])
                rec.update({"type": "child", "key": key, "workload": "W2", "index": index, "engine": engine})
                stream.write(json.dumps(rec, sort_keys=True) + "\n")
                stream.flush()
                status = (result.get("result") or result.get("status")) if result else \
                    ("wall_killed" if rec["timeout_killed"] else "crashed")
                print(key, status, round(rec["parent_wall_s"], 1), flush=True)
    print("receipts:", path)


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=Path)
    ap.add_argument("--child", action="store_true")
    ap.add_argument("--index", type=int)
    ap.add_argument("--engine", choices=ENGINES)
    ap.add_argument("--prior-rows", default="[]")
    a = ap.parse_args()
    if a.child:
        try:
            print(json.dumps(child(a.index, a.engine, json.loads(a.prior_rows)), sort_keys=True), flush=True)
        except Exception as error:
            print(json.dumps({"status": "exception", "type": type(error).__name__, "message": str(error),
                              "workload": "W2", "index": a.index, "engine": a.engine}), flush=True)
            raise
    else:
        parent(a.out)
