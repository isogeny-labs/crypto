#!/usr/bin/env python3
"""Transparent portable controls, not an exact replay of the upstream Rust binary.

The m2 quotient oracle follows prime31_s3_quotient_gcd.rs at the pinned commit
recorded in source_provenance.json.  Rectangular pools are an explicit extension.
The m3 control uses the same batched-inversion, x-only pair technique as
prime_fast.rs, but streams two pools against an O(B) index of the third pool.
It therefore has O(B^2) field operations and O(B) auxiliary storage.  Its
Python/FLINT timings must not be reported as native-Rust timings.
"""
from __future__ import annotations

import argparse
from collections import defaultdict
import json
import os
from pathlib import Path
import resource
import subprocess
import sys
import time

sys.path.insert(0, "/workspace/ecdlp_research/math_vendor")
from flint import nmod_poly, ctx

ctx.threads = 1
Point = tuple[int, int] | None


def neg(p: int, point: Point) -> Point:
    return None if point is None else (point[0], -point[1] % p)


def add(p: int, a: int, u: Point, v: Point) -> Point:
    """Unbatched affine group law used for independent witness validation."""
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, w = v
    if x == z:
        if (y + w) % p == 0:
            return None
        slope = (3*x*x+a) * pow(2*y, -1, p) % p
    else:
        slope = (w-y) * pow(z-x, -1, p) % p
    xx = (slope*slope-x-z) % p
    return xx, (slope*(x-xx)-y) % p


def scalar(p: int, a: int, k: int, point: Point) -> Point:
    if k < 0:
        return scalar(p, a, -k, neg(p, point))
    acc = None
    while k:
        if k & 1:
            acc = add(p, a, acc, point)
        point = add(p, a, point, point)
        k >>= 1
    return acc


def verify_witness(fixture: dict, witness: list[dict]) -> dict:
    """No secret input: check actual points, relation coefficients, and kP=Q."""
    c = fixture["curve"]
    p, a, r = c["p"], c["a"], c["r"]
    acc = None
    A = B = 0
    for term in witness:
        e = fixture["pools"][term["pool"]][term["index"]]
        sign = term["sign"]
        if sign not in (-1, 1):
            raise ValueError("witness sign must be ±1")
        point = e["x"], e["y"]
        acc = add(p, a, acc, point if sign == 1 else neg(p, point))
        A = (A + sign*e["alpha"]) % r
        B = (B + sign*e["beta"]) % r
    geometric = acc is None
    coefficient_point = add(p, a, scalar(p, a, A, tuple(c["P"])),
                            scalar(p, a, B, tuple(c["Q"])))
    informative = geometric and coefficient_point is None and B != 0
    k = (-A*pow(B, -1, r)) % r if informative else None
    recovered = informative and scalar(p, a, k, tuple(c["P"])) == tuple(c["Q"])
    return {"geometric_zero": geometric,
            "label_relation_zero": coefficient_point is None,
            "informative": informative,
            "recovered_scalar_verified": bool(recovered),
            "A": A, "B": B, "k": k}


def batch_inverses(values: list[int], p: int) -> list[int]:
    """Nonzero inputs; 1 inverse and 3(n-1) modular multiplications."""
    if not values:
        return []
    prefix = [values[0]]
    for v in values[1:]:
        prefix.append(prefix[-1]*v % p)
    u = pow(prefix[-1], -1, p)
    out = [0]*len(values)
    for j in range(len(values)-1, 0, -1):
        out[j] = prefix[j-1]*u % p
        u = u*values[j] % p
    out[0] = u
    return out


def solve_pair(fixture: dict, signed: bool = True) -> dict:
    """First informative signed zero-sum from three labelled pools.

    Global negation permits fixing sign(pool0)=+1.  With signed=True, try
    P0+P1 and P0-P1; x-only matching against pool2 resolves its sign.  No
    target scalar or planted-witness metadata is read.  The complete input
    index is built even on an early success.  All misses and exceptional
    denominators are charged.  Returns at most one informative relation.
    """
    t0, w0 = time.process_time(), time.perf_counter()
    c = fixture["curve"]
    p, a = c["p"], c["a"]
    pools = fixture["pools"]
    if len(pools) != 3:
        raise ValueError("exactly three pools required")
    index = defaultdict(list)
    for k, e in enumerate(pools[2]):
        index[e["x"]].append((k, e["y"]))
    tp, wp = time.process_time(), time.perf_counter()
    counts = {"pair_positions_scanned": 0, "signed_pair_candidates": 0,
              "x_lookups": 0, "x_hits": 0, "candidate_witnesses": 0,
              "uninformative_relations": 0, "exceptional_denominators": 0,
              "pair_sums_at_infinity": 0, "modular_inversions_search": 0,
              "fp_multiplications_explicit_search": 0,
              "batch_rows": 0, "batch_inputs": 0}
    witness = recovery = None
    recovery_cpu = recovery_wall = 0.0
    signs = (1, -1) if signed else (1,)
    stop = False
    for i, e0 in enumerate(pools[0]):
        x, y = e0["x"], e0["y"]
        ordinary = [(j, (e1["x"]-x) % p)
                    for j, e1 in enumerate(pools[1]) if e1["x"] != x]
        invs = batch_inverses([d for _, d in ordinary], p)
        invmap = {j: v for (j, _), v in zip(ordinary, invs)}
        if ordinary:
            counts["batch_rows"] += 1
            counts["batch_inputs"] += len(ordinary)
            counts["modular_inversions_search"] += 1
            counts["fp_multiplications_explicit_search"] += 3*(len(ordinary)-1)
        for j, e1 in enumerate(pools[1]):
            z, w = e1["x"], e1["y"]
            counts["pair_positions_scanned"] += 1
            if x == z:
                counts["exceptional_denominators"] += 1
            for sign1 in signs:
                counts["signed_pair_candidates"] += 1
                if x == z:
                    # Rare branch, charged separately.  Does not assume that
                    # disjoint pool indices imply disjoint x coordinates.
                    point = add(p, a, (x,y), (z, sign1*w % p))
                    if point is None:
                        counts["pair_sums_at_infinity"] += 1
                        continue
                    xx, yy = point
                    counts["modular_inversions_search"] += 1
                    counts["fp_multiplications_explicit_search"] += 4
                    slope = None
                else:
                    slope = (sign1*w-y)*invmap[j] % p
                    xx = (slope*slope-x-z) % p
                    counts["fp_multiplications_explicit_search"] += 2
                    yy = None
                counts["x_lookups"] += 1
                hits = index.get(xx, ())
                if not hits:
                    continue
                counts["x_hits"] += 1
                if yy is None:
                    yy = (slope*(x-xx)-y) % p
                    counts["fp_multiplications_explicit_search"] += 1
                for k, target_y in hits:
                    for sign2 in signs:
                        if (yy+sign2*target_y) % p:
                            continue
                        candidate = [{"pool":0,"index":i,"sign":1},
                                     {"pool":1,"index":j,"sign":sign1},
                                     {"pool":2,"index":k,"sign":sign2}]
                        counts["candidate_witnesses"] += 1
                        tr, wr = time.process_time(), time.perf_counter()
                        result = verify_witness(fixture, candidate)
                        recovery_cpu += time.process_time()-tr
                        recovery_wall += time.perf_counter()-wr
                        if not result["geometric_zero"]:
                            raise AssertionError("pair witness failed full group check")
                        if result["recovered_scalar_verified"]:
                            witness, recovery, stop = candidate, result, True
                            break
                        counts["uninformative_relations"] += 1
                    if stop:
                        break
                if stop:
                    break
            if stop:
                break
        if stop:
            break
    te, we = time.process_time(), time.perf_counter()
    return {"algorithm":"streamed-batched-pair-enumeration",
            "implementation":"portable Python; native source timing not replayed",
            "fixture_id":fixture.get("id"), "B":fixture.get("B"),
            "status":"relation" if witness is not None else "none",
            "witness":witness, "recovery":recovery, "counters":counts,
            "phase_cpu_seconds":{"preprocessing":tp-t0,
                "search":te-tp-recovery_cpu,"witness_recovery":recovery_cpu},
            "phase_wall_seconds":{"preprocessing":wp-w0,
                "search":we-wp-recovery_wall,"witness_recovery":recovery_wall},
            "total_cpu_seconds":te-t0,"total_wall_seconds":we-w0,
            "generation_cpu_seconds":fixture.get("generation",{}).get("cpu_seconds"),
            "peak_rss_kib":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
            "auxiliary_space":"O(B); x index and one inversion batch, no B^2 pair table"}


def solve_pair_cpp(fixture: dict, cpu: int = 2, repetitions: int = 1) -> dict:
    """Call the separately compiled portable C++ control, accounting for driver.

    Native phases include parsing, index build, search, exceptional branches,
    group/label checks and scalar recovery. Process/driver overhead is recorded
    separately. No compiler invocation is hidden in a trial.
    """
    td, wd = time.process_time(), time.perf_counter()
    c = fixture["curve"]
    lines = [" ".join(map(str,[c["p"],c["a"],c["b"],c["r"],*c["P"],*c["Q"],
                              *map(len,fixture["pools"])]))]
    for pool in fixture["pools"]:
        lines.extend(" ".join(str(e[key]) for key in ("x","y","alpha","beta"))
                     for e in pool)
    packed="\n".join(lines)+"\n"
    binary=Path(__file__).resolve().parent/"pair_control"
    before=resource.getrusage(resource.RUSAGE_CHILDREN)
    ws=time.perf_counter()
    run=subprocess.run(["taskset","-c",str(cpu),str(binary),"--repeats",str(repetitions)],input=packed,
                       text=True,capture_output=True,check=True)
    process_wall=time.perf_counter()-ws
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    result=json.loads(run.stdout)
    result.update({"implementation":"portable C++ control; upstream Rust binary not replayed",
        "fixture_id":fixture.get("id"),"B":fixture.get("B"),
        "cpu_affinity":[cpu],"subprocess_wall_seconds":process_wall,
        "child_cpu_seconds_including_startup_and_output":
            after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,
        "driver_cpu_seconds":time.process_time()-td,
        "driver_wall_seconds_including_subprocess":time.perf_counter()-wd,
        "generation_cpu_seconds":fixture.get("generation",{}).get("cpu_seconds")})
    return result


def base_poly(values: list[int], p: int):
    h = nmod_poly([1], p)
    for v in values:
        h *= nmod_poly([-v % p,1],p)
    return h


def s3_scalar(p: int, a: int, b: int, x: int, t: int):
    """S3(x,Y,t), constant coefficient first, including degenerate branches."""
    A = (x-t)**2 % p
    B = -2*((x+t)*(x*t+a)+2*b) % p
    C = ((x*t-a)**2-4*b*(x+t)) % p
    return nmod_poly([C,B,A], p)


def quotient_pairs(p: int, a: int, b: int, xs: list[int],
                   ys: list[int], t: int, polys=None) -> dict:
    """Coordinate-only m2 quotient/norm/GCD oracle, with exceptional x=t.

    The finite pools need not coincide.  Known pool roots are scanned for
    witness extraction; generic factoring/root finding is not being hidden.
    The output includes coordinate candidates, not signed EC relations.
    """
    t0, w0 = time.process_time(), time.perf_counter()
    if len(set(xs)) != len(xs) or len(set(ys)) != len(ys):
        raise ValueError("quotient pools must have distinct coordinates internally")
    h,hy = (base_poly(xs,p),base_poly(ys,p)) if polys is None else polys
    X = nmod_poly([0,1],p)
    exceptional = [i for i,x in enumerate(xs) if x == t]
    hr = h//(X-t) if exceptional else h
    retained = list(exceptional)
    first_degree = 0
    ring_mults = 0
    tp, wp = time.process_time(), time.perf_counter()
    if hr.degree() > 0:
        A = (X-t)**2
        B = -2*((X+t)*(X*t+a)+2*b)
        C = (X*t-a)**2-4*b*(X+t)
        d, ai, _ = A.xgcd(hr)
        if d.degree() != 0:
            raise ArithmeticError("regular branch leading coefficient not a unit")
        ai = ai*pow(int(d[0]),-1,p) % hr
        beta, gamma = B*ai % hr, C*ai % hr
        ring_mults += 2
        r0 = nmod_poly([],p)
        r1 = nmod_poly([],p)
        for c in reversed(list(hy.coeffs())):
            r0, r1 = (int(c)-gamma*r1) % hr, (r0-beta*r1) % hr
            ring_mults += 2
        norm = (r0*r0-beta*r0*r1+gamma*r1*r1) % hr
        ring_mults += 5
        eliminant = hr.gcd(norm)
        first_degree = eliminant.degree()
        retained += [i for i,x in enumerate(xs)
                     if x != t and int(eliminant(x)) == 0]
    tm, wm = time.process_time(), time.perf_counter()
    pairs=[]
    for i in retained:
        q = s3_scalar(p,a,b,xs[i],t)
        d = hy if q.is_zero() else hy.gcd(q)
        for j,y in enumerate(ys):
            if int(d(y)) == 0:
                pairs.append((i,j))
    pairs = sorted(set(pairs))
    te, we = time.process_time(), time.perf_counter()
    return {"pairs":pairs,"first_eliminant_degree":first_degree,
            "retained_first_indices":retained,
            "exceptional_x_equals_target_indices":exceptional,
            "quotient_ring_mults":ring_mults,
            "known_root_evaluations":len(xs)+len(ys)*len(retained),
            "phase_cpu_seconds":{"preprocessing":tp-t0,
                "selector":tm-tp,"coordinate_witness_recovery":te-tm},
            "phase_wall_seconds":{"preprocessing":wp-w0,
                "selector":wm-wp,"coordinate_witness_recovery":we-wm},
            "total_cpu_seconds":te-t0,"total_wall_seconds":we-w0}


def solve_quotient_scan(fixture: dict) -> dict:
    """Apply the existing m2 quotient oracle to successive third-pool roots.

    This is a complete m3 control that charges all tried targets, including
    misses. The two pool polynomials are shared and charged once. This routine
    does not claim the m2 primitive by itself selects an m3 incidence root.
    """
    t0,w0=time.process_time(),time.perf_counter()
    c=fixture['curve'];p,a,b=c['p'],c['a'],c['b']
    pools=fixture['pools']
    xs=[e['x'] for e in pools[0]];ys=[e['x'] for e in pools[1]]
    polys=base_poly(xs,p),base_poly(ys,p)
    tp,wp=time.process_time(),time.perf_counter()
    selector_cpu=coordinate_cpu=recovery_cpu=0.0
    selector_wall=coordinate_wall=recovery_wall=0.0
    counts={'targets_processed':0,'quotient_ring_mults':0,'known_root_evaluations':0,
            'coordinate_candidate_pairs':0,'candidate_witnesses':0,
            'uninformative_relations':0,'exceptional_x_equals_target':0}
    witness=recovery=None
    stop=False
    for k,e2 in enumerate(pools[2]):
        q=quotient_pairs(p,a,b,xs,ys,e2['x'],polys=polys)
        counts['targets_processed']+=1
        counts['quotient_ring_mults']+=q['quotient_ring_mults']
        counts['known_root_evaluations']+=q['known_root_evaluations']
        counts['coordinate_candidate_pairs']+=len(q['pairs'])
        counts['exceptional_x_equals_target']+=len(q['exceptional_x_equals_target_indices'])
        selector_cpu+=q['phase_cpu_seconds']['preprocessing']+q['phase_cpu_seconds']['selector']
        selector_wall+=q['phase_wall_seconds']['preprocessing']+q['phase_wall_seconds']['selector']
        coordinate_cpu+=q['phase_cpu_seconds']['coordinate_witness_recovery']
        coordinate_wall+=q['phase_wall_seconds']['coordinate_witness_recovery']
        tr,wr=time.process_time(),time.perf_counter()
        for i,j in q['pairs']:
            e0,e1=pools[0][i],pools[1][j]
            for sj in (1,-1):
                point=add(p,a,(e0['x'],e0['y']),(e1['x'],sj*e1['y']%p))
                if point is None:
                    continue
                for sk in (1,-1):
                    if point!=(e2['x'],-sk*e2['y']%p):
                        continue
                    candidate=[{'pool':0,'index':i,'sign':1},
                               {'pool':1,'index':j,'sign':sj},
                               {'pool':2,'index':k,'sign':sk}]
                    counts['candidate_witnesses']+=1
                    result=verify_witness(fixture,candidate)
                    if not result['geometric_zero']:
                        raise AssertionError('quotient scan witness failed group check')
                    if result['recovered_scalar_verified']:
                        witness,recovery,stop=candidate,result,True
                        break
                    counts['uninformative_relations']+=1
                if stop:
                    break
            if stop:
                break
        recovery_cpu+=time.process_time()-tr
        recovery_wall+=time.perf_counter()-wr
        if stop:
            break
    te,we=time.process_time(),time.perf_counter()
    return {'algorithm':'existing-m2-quotient-oracle-target-scan',
        'implementation':'portable Python-FLINT quotient/GCD control; upstream Rust not replayed',
        'fixture_id':fixture.get('id'),'B':fixture.get('B'),
        'status':'relation' if witness else 'none','witness':witness,'recovery':recovery,
        'counters':counts,'phase_cpu_seconds':{'preprocessing':tp-t0,
            'selector':selector_cpu,'coordinate_witness_recovery':coordinate_cpu,
            'signed_witness_recovery':recovery_cpu,
            'orchestration':te-tp-selector_cpu-coordinate_cpu-recovery_cpu},
        'phase_wall_seconds':{'preprocessing':wp-w0,'selector':selector_wall,
            'coordinate_witness_recovery':coordinate_wall,'signed_witness_recovery':recovery_wall,
            'orchestration':we-wp-selector_wall-coordinate_wall-recovery_wall},
        'total_cpu_seconds':te-t0,'total_wall_seconds':we-w0,
        'generation_cpu_seconds':fixture.get('generation',{}).get('cpu_seconds'),
        'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
        'auxiliary_space':'O(B) polynomial count, each degree O(B); no B^2 pair table'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fixture",type=Path)
    parser.add_argument("--cpu",type=int,default=2)
    args=parser.parse_args()
    os.sched_setaffinity(0,{args.cpu})
    fixture=json.loads(args.fixture.read_text())
    print(json.dumps(solve_pair(fixture),sort_keys=True))


if __name__ == "__main__":
    main()
