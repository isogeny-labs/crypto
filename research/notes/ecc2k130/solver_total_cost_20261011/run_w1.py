"""W1: total verified-relation cost on the frozen m83 fixed-phase S4 workloads.

Reuses the frozen m83 sources unchanged (imported, not copied) and re-implements only
the per-input driver with stage timers.  See PROTOCOL.md.  Parent mode spawns one
child per (workload, seed, index, engine) sequentially; child mode runs one cell.
"""
from __future__ import annotations

import argparse
import itertools
import json
import os
from pathlib import Path
import random
import resource
import sys
import time

HERE = Path(__file__).resolve().parent
M83 = HERE.parent / "m83_solver_matrix_20260927"
REPO = HERE.parents[3]
sys.path.append(str(M83))
sys.path.insert(0, str(HERE))

import run as base                      # frozen m83 run.py (also puts source/ dirs on sys.path)
import run_extended as ext              # frozen PlainSetup / archived Setup
import core as c                        # archived Semaev/Boolean code
import tcommon as tc

N, MODULUS, SEEDS = base.N, base.MODULUS, base.SEEDS
ENGINES = ("fes", "sat-cms-xor", "sat-z3", "f4-boolean", "f5b-boolean")
LIMIT, BUILD_LIMIT, CHILD_WALL, AS_LIMIT = 5.0, 25.0, 70, 1 << 30
SCHEMA = "solver_total_cost/w1/v1"


# ----------------------------------------------------------------- inputs
def factor_base_lifts(curve, setup, s):
    """First half of base.make_inputs: usable subgroup lifts of the nonzero payloads."""
    used = []
    for payload in range(1, 1 << s):
        x = setup.normal.to_field(setup.space.encode(payload))
        for point in curve.lift(x):
            if curve.scale(point, curve.r) is None:
                used.append((payload, point))
    return used


def planted_and_natural(curve, seed, s, used, generator):
    """Second half of base.make_inputs, line for line."""
    planted = []
    for picks in itertools.product(used, repeat=3):
        points = tuple(curve.frob(p, phase) for phase, (_, p) in enumerate(picks))
        if not c.proper(curve, points):
            continue
        target = c.group_sum(curve, points)
        middle = curve.add(points[0], points[1])
        if target is None or middle is None or not target[0] or not middle[0]:
            continue
        if target not in [q["target"] for q in planted]:
            planted.append({"kind": "planted", "index": len(planted), "target": target,
                            "payloads": [p for p, _ in picks], "witness": points})
        if len(planted) == 2:
            break
    rng = random.Random(seed * 1000 + 830 + s)
    natural = [{"kind": "natural", "index": i, "scalar": (scalar := rng.randrange(1, curve.r)),
                "target": curve.scale(generator, scalar)} for i in range(4)]
    return planted, natural


# ----------------------------------------------------------------- engines
def solve_engine(engine, equations, nvars, st, deadline):
    """Return (sorted roots, stats).  Mirrors run.py / run_extended.py with stage timers."""
    stats = {}
    if engine == "fes":
        with st.stage("solve"):
            roots = [bits for bits in range(1 << nvars)
                     if all(base.evaleq(eq, bits) == 0 for eq in equations)]
        stats = {"assignments_tested": 1 << nvars}
    elif engine == "f4-boolean":
        with st.stage("solve"):
            basis, stats = base.f4_rows(equations, nvars, deadline)
        with st.stage("model_extraction"):
            roots = [bits for bits in range(1 << nvars)
                     if all(base.evaleq(eq, bits) == 0 for eq in basis)]
        stats["root_extraction_assignments"] = 1 << nvars
    elif engine == "f5b-boolean":
        with st.stage("engine_load"):
            from boolean_f5b import BooleanF5B
        with st.stage("solver_setup"):
            solver = BooleanF5B(nvars, timeout=LIMIT, max_pairs=250000)
            generators = [solver.from_terms(eq) for eq in equations]
        with st.stage("solve"):
            basis = solver.basis(generators)
        with st.stage("model_extraction"):
            roots = [bits for bits in range(1 << nvars)
                     if all(solver.evaluate(eq, bits) == 0 for eq in basis)]
        stats = dict(solver.stats, root_extraction_assignments=1 << nvars)
    elif engine == "sat-z3":
        with st.stage("engine_load"):
            import z3
        with st.stage("solver_setup"):
            variables = [z3.Bool(f"v{i}") for i in range(nvars)]
            all_monomials = set().union(*equations)
            monomials = {mask: z3.And(*[variables[i] for i in range(nvars) if mask >> i & 1])
                         if mask else z3.BoolVal(True) for mask in all_monomials}
            solver = z3.Solver()
            solver.set(random_seed=13)
            for eq in equations:
                solver.add(z3.Not(c.xor_tree(z3, [monomials[mask] for mask in sorted(eq)])))
        roots = []
        while True:
            remaining = deadline - time.perf_counter()
            if remaining <= 0:
                raise TimeoutError(f"SAT enumeration deadline after {len(roots)} models")
            with st.stage("solve"):
                solver.set(timeout=max(1, int(remaining * 1000)))
                result = solver.check()
            if result == z3.unsat:
                break
            if result != z3.sat:
                raise TimeoutError(f"SAT {solver.reason_unknown()} after {len(roots)} models")
            with st.stage("model_extraction"):
                model = solver.model()
                bits = sum(int(z3.is_true(model.eval(v, model_completion=True))) << i
                           for i, v in enumerate(variables))
                assert all(base.evaleq(eq, bits) == 0 for eq in equations)
                roots.append(bits)
                solver.add(z3.Or(*[v != z3.BoolVal(bool(bits >> i & 1))
                                   for i, v in enumerate(variables)]))
        stats = {"z3_version": z3.get_version_string(), "models_enumerated": len(roots),
                 "statistics": {k: v for k, v in solver.statistics()}}   # untimed diagnostics
    elif engine == "sat-cms-xor":
        with st.stage("engine_load"):
            import pycryptosat
        with st.stage("solver_setup"):
            solver = pycryptosat.Solver(threads=1)
            monomials = sorted(set().union(*equations))
            auxiliary = {mask: nvars + i + 1 for i, mask in enumerate(
                mask for mask in monomials if mask.bit_count() > 1)}
            clauses = xors = 0
            for mask, out in auxiliary.items():
                inputs = [i + 1 for i in range(nvars) if mask >> i & 1]
                solver.add_clause([out] + [-variable for variable in inputs])
                clauses += 1
                for variable in inputs:
                    solver.add_clause([-out, variable])
                    clauses += 1
            for eq in equations:
                literals = [auxiliary[mask] if mask in auxiliary else mask.bit_length()
                            for mask in sorted(eq) if mask]
                rhs = (0 in eq)
                if not literals:
                    if rhs:
                        solver.add_clause([])
                        clauses += 1
                    continue
                solver.add_xor_clause(literals, rhs)
                xors += 1
            if time.perf_counter() > deadline:
                raise TimeoutError("CryptoMiniSat setup deadline")
        roots = []
        while True:
            remaining = deadline - time.perf_counter()
            if remaining <= 0:
                raise TimeoutError(f"CryptoMiniSat enumeration deadline after {len(roots)} models")
            with st.stage("solve"):
                sat, model = solver.solve(time_limit=remaining)
            if sat is None:
                raise TimeoutError(f"CryptoMiniSat timed out after {len(roots)} models")
            if not sat:
                break
            with st.stage("model_extraction"):
                bits = sum(int(bool(model[i + 1])) << i for i in range(nvars))
                assert all(base.evaleq(eq, bits) == 0 for eq in equations)
                roots.append(bits)
                solver.add_clause([-(i + 1) if bits >> i & 1 else i + 1 for i in range(nvars)])
                clauses += 1
        stats = {"cms_version": pycryptosat.__version__, "seed_configurable": False,
                 "threads": 1, "auxiliary_variables": len(auxiliary), "cnf_clauses": clauses,
                 "xor_clauses": xors, "models_enumerated": len(roots), "decisions": None}
    else:
        raise ValueError(engine)
    with st.stage("model_extraction"):
        roots = sorted(roots)
        assert all(all(base.evaleq(eq, bits) == 0 for eq in equations) for bits in roots)
    return roots, stats


# ----------------------------------------------------------------- child
def child(workload, policy, s, seed, index, engine, prior_rows):
    resource.setrlimit(resource.RLIMIT_AS, (AS_LIMIT, AS_LIMIT))
    st = tc.Stages()
    t_main = time.perf_counter()
    ident = {"schema": SCHEMA, "workload": workload, "factor_base_policy": policy,
             "payload_bits": s, "seed": seed, "index": index, "engine": engine}
    out = dict(ident)
    base.S = s
    nvars = 3 * s
    try:
        with st.stage("fb_setup"):
            setup = (ext.archived_setup if policy == "quotient" else ext.PlainSetup)(
                N, s, seed, "ternary_inline", MODULUS)
            curve = setup.curve
            assert curve.order == 4 * 2417851639230796216685689 and curve.cofactor == 4
            g = base.field_generator(curve)
            root = base.sqrt_mod(-7 % curve.r, curve.r)
            lambdas = [((-1 + root) * pow(2, -1, curve.r)) % curve.r,
                       ((-1 - root) * pow(2, -1, curve.r)) % curve.r]
            lam, = [v for v in lambdas if curve.scale(g, v) == curve.frob(g)]
            assert pow(lam, N, curve.r) == 1 and lam != 1
            used = factor_base_lifts(curve, setup, s)
            reps = sorted(set(min(p, curve.neg(p)) for _, p in used))
            lookup, columns = tc.build_lookup(curve, reps, lam, N, curve.r)
        with st.stage("input_gen"):
            planted, natural = planted_and_natural(curve, seed, s, used, g)
        inputs = (planted + [None] * (2 - len(planted))) + natural
        out["factor_base"] = {"usable_lifts": len(used), "representatives": reps,
                              "columns": len(columns), "orbit_entries": len(lookup),
                              "planted_available": len(planted), "generator": g,
                              "frobenius_eigenvalue": lam, "normal_generator": setup.normal.beta,
                              "phases": list(setup.phases)}
        if inputs[index] is None:
            out.update(status="setup_failed",
                       reason="no proper planted triple in frozen factor base")
            return finish(out, st, t_main)
        record = inputs[index]
        target = record["target"]
        out["kind"] = record["kind"]
        assert curve.scale(target, curve.r) is None and curve.valid(target)
        maps = [setup.phase_maps[i][i] for i in range(3)]
        with st.stage("eq_build"):
            field_poly = c.Algebra(curve.f, setup.counts, time.perf_counter() + BUILD_LIMIT).s4(
                *maps, target[0])
            equations = c.boolean_equations(field_poly, N)
        out.update(input=record, input_sha256=tc.digest(record),
                   equation_sha256=tc.digest([sorted(eq) for eq in equations]),
                   nvars=nvars, field_polynomial_terms=len(field_poly),
                   boolean_equations=len(equations), boolean_terms=sum(map(len, equations)))
        if record["kind"] == "planted":       # harness sanity check, not charged
            bits = sum(payload << (i * s) for i, payload in enumerate(record["payloads"]))
            assert all(base.evaleq(eq, bits) == 0 for eq in equations)
            assert c.lift(curve, [c.evaluate(m, bits) for m in maps], target) is not None
        deadline = time.perf_counter() + LIMIT        # aggregate solver deadline, as in m83
        roots, stats = solve_engine(engine, equations, nvars, st, deadline)
        out.update(algebraic_roots=roots, statistics=stats)
        pivots = {}
        for row in prior_rows:                        # carried-over natural state, not charged
            tc.rank_add(pivots, row, curve.r)
        rank_before = len(pivots)
        verified, false_positive, witnesses, relation_rows = [], [], {}, []
        for bits in roots:
            with st.stage("lifting"):
                xs = [c.evaluate(poly, bits) for poly in maps]
                witness = c.lift(curve, xs, target)
            if witness is None:
                false_positive.append(bits)
                continue
            with st.stage("rank_update"):
                row = tc.relation_row(lookup, len(columns), witness, curve.r)
            with st.stage("verification"):
                assert all(curve.valid(p) and curve.scale(p, curve.r) is None for p in witness)
                assert c.group_sum(curve, witness) == target
                assert c.group_sum(curve, [curve.scale(columns[i], a) for i, a in enumerate(row)]) \
                    == c.group_sum(curve, witness)
            with st.stage("rank_update"):
                if record["kind"] == "natural":
                    independent = tc.rank_add(pivots, row, curve.r)
                else:                                   # planted: charged on a scratch copy
                    independent = tc.rank_add({k: list(v) for k, v in pivots.items()}, row, curve.r)
            verified.append(bits)
            witnesses[str(bits)] = witness
            relation_rows.append({"payload_bits_assignment": bits, "row": row,
                                  "independent_of_natural_matrix": independent})
        out["group_verification"] = {"verified": verified, "false_positive": false_positive,
                                     "witnesses": witnesses}
        natural_new = sum(r["independent_of_natural_matrix"] for r in relation_rows) \
            if record["kind"] == "natural" else 0
        out["relation"] = {"columns": len(columns), "modulus_r": curve.r,
                           "verified_candidates": len(verified), "relation_rows": relation_rows,
                           "natural_verified": len(verified) if record["kind"] == "natural" else 0,
                           "planted_verified": len(verified) if record["kind"] == "planted" else 0,
                           "new_independent_natural": natural_new,
                           "rank_before": rank_before, "rank_after": len(pivots)}
        out["status"] = "complete"
        return finish(out, st, t_main)
    except (TimeoutError, c.Budget) as error:
        out.update(status="timeout", type=type(error).__name__, message=str(error))
    except MemoryError as error:
        out.update(status="oom", type=type(error).__name__, message=str(error))
    return finish(out, st, t_main)


def finish(out, st, t_main):
    out["stages"] = st.dump()
    out["child_main_wall_s"] = time.perf_counter() - t_main
    out["peak_rss_bytes"] = tc.peak_rss_bytes()
    out["python"], out["platform"] = sys.version, tc.versions()["platform"]
    return out


# ----------------------------------------------------------------- parent
def cells():
    for seed in SEEDS:
        for index in range(6):
            for engine in ENGINES:
                yield "W1a", "quotient", 2, seed, index, engine
    for s in (2, 3, 4):
        for seed in SEEDS:
            for index in (0, 2):
                for engine in ENGINES:
                    yield "W1b", "normal_subspace", s, seed, index, engine


def parent(out_dir, reps, only):
    out_dir.mkdir(parents=True, exist_ok=False)
    env = dict(os.environ, PYTHONHASHSEED="0")
    files = sorted((M83 / "source").rglob("*.py")) + [M83 / n for n in (
        "run.py", "run_extended.py", "preflight_engines.py", "PROTOCOL.md", "PROTOCOL_EXTENSION.md",
        "PROTOCOL_PLAIN_SUBSPACE.md", "AMENDMENT.md")] + [HERE / n for n in (
        "tcommon.py", "run_w1.py", "PROTOCOL.md")]
    manifest = {"type": "manifest", "schema": SCHEMA, "protocol": "solver_total_cost_20261011",
                "workloads": ["W1a", "W1b"], "reps": reps, "seeds": list(SEEDS),
                "engines": list(ENGINES),
                "limits": {"address_space_bytes": AS_LIMIT, "child_wall_seconds": CHILD_WALL,
                           "equation_build_seconds": BUILD_LIMIT, "solver_seconds": LIMIT},
                "env": {"PYTHONHASHSEED": "0",
                        "PYTHONPYCACHEPREFIX": os.environ.get("PYTHONPYCACHEPREFIX")},
                "versions": tc.versions(), "git": tc.git_state(REPO),
                "source_sha256": tc.source_hashes(files, REPO),
                "loadavg_at_start": os.getloadavg(), "started_unix": time.time()}
    warm = tc.run_child([sys.executable, "-c",
                         f"import sys; sys.path[:0]=[{str(HERE)!r}]; import run_w1, z3, pycryptosat; "
                         "from boolean_f5b import BooleanF5B"], 120, env)
    manifest["warmup"] = {"returncode": warm["returncode"], "parent_wall_s": warm["parent_wall_s"],
                          "purpose": "page-cache and bytecode warm-up so the first z3 import is not cold"}
    path = out_dir / "receipts.jsonl"
    with path.open("x") as stream:
        stream.write(json.dumps(manifest, sort_keys=True) + "\n")
        prior = {}
        for rep in range(1, reps + 1):
            for workload, policy, s, seed, index, engine in cells():
                if only and not any(tok in f"{workload}-{engine}-s{s}-{seed}-i{index}" for tok in only):
                    continue
                key = f"{workload}-r{rep}-s{s}-seed{seed}-i{index}-{engine}"
                prior_key = (rep, workload, policy, s, seed, engine)
                cmd = [sys.executable, str(HERE / "run_w1.py"), "--child", "--workload", workload,
                       "--policy", policy, "--s", str(s), "--seed", str(seed), "--index", str(index),
                       "--engine", engine, "--prior-rows", json.dumps(prior.get(prior_key, []))]
                rec = tc.run_child(cmd, CHILD_WALL, env)
                result = rec["result"]
                if result and result.get("status") == "complete" and \
                        result["relation"]["new_independent_natural"]:
                    prior.setdefault(prior_key, []).extend(
                        r["row"] for r in result["relation"]["relation_rows"]
                        if r["independent_of_natural_matrix"])
                rec.update({"type": "child", "key": key, "rep": rep, "workload": workload,
                            "policy": policy, "payload_bits": s, "seed": seed, "index": index,
                            "engine": engine})
                stream.write(json.dumps(rec, sort_keys=True) + "\n")
                stream.flush()
                status = result["status"] if result else ("wall_killed" if rec["timeout_killed"] else "crashed")
                print(key, status, round(rec["parent_wall_s"], 2), flush=True)
    print("receipts:", path)


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", type=Path)
    ap.add_argument("--reps", type=int, default=3)
    ap.add_argument("--only", nargs="*", default=None, help="smoke filter: substrings of the cell key")
    ap.add_argument("--child", action="store_true")
    ap.add_argument("--workload")
    ap.add_argument("--policy", choices=("quotient", "normal_subspace"))
    ap.add_argument("--s", type=int)
    ap.add_argument("--seed", type=int)
    ap.add_argument("--index", type=int)
    ap.add_argument("--engine", choices=ENGINES)
    ap.add_argument("--prior-rows", default="[]")
    a = ap.parse_args()
    if a.child:
        try:
            print(json.dumps(child(a.workload, a.policy, a.s, a.seed, a.index, a.engine,
                                   json.loads(a.prior_rows)), sort_keys=True), flush=True)
        except Exception as error:
            print(json.dumps({"status": "exception", "type": type(error).__name__,
                              "message": str(error), "workload": a.workload, "seed": a.seed,
                              "index": a.index, "engine": a.engine, "payload_bits": a.s}), flush=True)
            raise
    else:
        parent(a.out, a.reps, a.only)
