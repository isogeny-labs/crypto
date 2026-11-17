"""Independent small-field audit of fast_trace/kernel.py.

Expected values use elementary integer polynomial arithmetic and a line/cubic
intersection group law implemented here. No production arithmetic or input
root extraction is reused in the expected-value path. The implementation
under test is imported only by the runner. Inputs are deliberately small:
this is a correctness audit and not an independent performance measurement.
"""

from __future__ import annotations

from collections import Counter
from copy import deepcopy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import random
import time


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
KERNEL = ROOT / "fast_trace" / "kernel.py"


def require(condition, description):
    if not condition:
        raise ValueError(description)


def clean(poly, p):
    out = [int(x) % p for x in poly]
    while out and not out[-1]:
        out.pop()
    return out


def evaluate(poly, x, p):
    total = 0
    for coefficient in reversed(poly):
        total = (total * x + coefficient) % p
    return total


def multiply(left, right, p):
    if not left or not right:
        return []
    out = [0] * (len(left) + len(right) - 1)
    for i, a in enumerate(left):
        for j, b in enumerate(right):
            out[i+j] = (out[i+j] + a*b) % p
    return clean(out, p)


def quotient(dividend, divisor, p):
    dividend, divisor = clean(dividend, p), clean(divisor, p)
    require(bool(divisor), "division by zero")
    out = [0] * max(0, len(dividend) - len(divisor) + 1)
    inverse = pow(divisor[-1], -1, p)
    while len(dividend) >= len(divisor):
        degree = len(dividend) - len(divisor)
        value = dividend[-1] * inverse % p
        out[degree] = value
        for i, coefficient in enumerate(divisor):
            dividend[degree+i] = (dividend[degree+i] - value*coefficient) % p
        dividend = clean(dividend, p)
    require(not dividend, "polynomial division is not exact")
    return clean(out, p)


def vanish(xs, p):
    polynomial = [1]
    for x in xs:
        polynomial = multiply(polynomial, [-x % p, 1], p)
    return polynomial


def interpolate(xs, ys, p):
    require(len(set(xs)) == len(xs), "repeated interpolation coordinate")
    out = [0] * len(xs)
    for i, (x, y) in enumerate(zip(xs, ys)):
        other_xs = xs[:i] + xs[i+1:]
        basis = vanish(other_xs, p)
        denominator = evaluate(basis, x, p)
        scale = y * pow(denominator, -1, p) % p
        for j, coefficient in enumerate(basis):
            out[j] = (out[j] + scale*coefficient) % p
    return clean(out, p)


def on_curve(P, p, a, b):
    return P is None or (P[1]*P[1] - P[0]**3 - a*P[0] - b) % p == 0


def negative(P, p):
    return None if P is None else (P[0], -P[1] % p)


def geometric_add(P, Q, p, a, b):
    """Find the third line/cubic intersection, then negate it.

    In contrast to the production translation's rational coordinate formula,
    this constructs the intersection cubic and divides out the two known
    linear factors, including a repeated tangent factor for P=Q.
    """
    require(on_curve(P,p,a,b) and on_curve(Q,p,a,b), "oracle point off curve")
    if P is None:
        return Q
    if Q is None:
        return P
    x,y = P
    xx,yy = Q
    if x == xx and (y+yy) % p == 0:
        return None
    if x == xx:
        require(y == yy, "same abscissa with incompatible ordinates")
        slope = (3*x*x+a) * pow(2*y,-1,p) % p
    else:
        slope = (yy-y) * pow(xx-x,-1,p) % p
    intercept = (y-slope*x) % p
    cubic = [(b-intercept*intercept) % p,
             (a-2*slope*intercept) % p, -slope*slope % p, 1]
    linear = quotient(quotient(cubic,[-x % p,1],p),[-xx % p,1],p)
    require(len(linear) == 2, "third intersection is not linear")
    final_x = -linear[0] * pow(linear[1],-1,p) % p
    final_y = -(slope*final_x+intercept) % p
    result = final_x,final_y
    require(on_curve(result,p,a,b), "oracle output off curve")
    return result


def points(p,a,b):
    return [(x,y) for x in range(p) for y in range(p)
            if (y*y-x*x*x-a*x-b) % p == 0]


def encode(selected, p):
    xs = [P[0] for P in selected]
    require(len(set(xs)) == len(xs), "simple input has repeated abscissa")
    return vanish(xs,p),interpolate(xs,[P[1] for P in selected],p)


def split_expected(expected, p):
    """Encode expected point multiset solely to exercise the checker itself."""
    layers = [[],[]]
    by_x = {}
    infinity = expected[None] != 0
    for P,count in expected.items():
        if P is None:
            require(count <= 1, "expected infinity repeated")
            continue
        require(count == 1, "expected affine point repeated")
        index = by_x.get(P[0],0)
        layers[index].append(P)
        by_x[P[0]] = index+1
    components = []
    for layer in layers:
        if layer:
            u,v = encode(layer,p)
            components.append({"u":u,"v":v})
    return {"components":components,"infinity":infinity,"stats":{}}


def decode_and_check(output, expected, p, a, b, input_degree):
    require(isinstance(output,dict), "result must be an object")
    require(isinstance(output.get("components"),list), "components must be a list")
    require(type(output.get("infinity")) is bool, "infinity flag must be Boolean")
    actual = Counter()
    if output["infinity"]:
        actual[None] += 1
    declared = int(output["infinity"])
    for component in output["components"]:
        u = clean(component["u"],p)
        v = clean(component["v"],p)
        require(u and u[-1] == 1, "component u must be monic and nonzero")
        degree = len(u)-1
        if not degree:
            require(not v, "empty component has nonzero ordinate")
            continue
        require(len(v)-1 < degree, "ordinate is not reduced modulo u")
        roots = [x for x in range(p) if evaluate(u,x,p) == 0]
        require(len(roots) == degree, "component u is not split and squarefree")
        derivative = [(i*u[i]) % p for i in range(1,len(u))]
        for x in roots:
            require(evaluate(derivative,x,p) != 0, "repeated root in component")
            P = x,evaluate(v,x,p)
            require(on_curve(P,p,a,b), "component point is off curve")
            actual[P] += 1
        declared += degree
    require(declared == input_degree, "translation changes divisor degree")
    require(actual == expected, "translated point multiset differs from geometry")
    return actual


def make_case(label,p,a,b,selected,Q,input_infinity=False,tags=()):
    require((4*a**3+27*b*b) % p != 0, "singular test curve")
    u,v = encode(selected,p)
    expected = Counter(geometric_add(P,Q,p,a,b) for P in selected)
    if input_infinity:
        expected[Q] += 1
    degree = len(selected)+int(input_infinity)
    fake = split_expected(expected,p)
    decode_and_check(fake,expected,p,a,b,degree)
    return {"label":label,"p":p,"a":a,"b":b,"u":u,"v":v,
            "Q":Q,"input_infinity":input_infinity,"selected":selected,
            "expected":expected,"degree":degree,"tags":list(tags)}


def fixtures():
    rng = random.Random(2026111604)
    cases = []
    for p in (5,7,11,13,17,19,29,43,83,101):
        curves = []
        if p == 5:
            curves.append((3,0))
        if p == 83:
            curves.append((31,21))
        while len(curves) < 3:
            a,b = rng.randrange(p),rng.randrange(p)
            if (4*a**3+27*b*b) % p and (a,b) not in curves:
                curves.append((a,b))
        for curve_index,(a,b) in enumerate(curves):
            ps = points(p,a,b)
            by_x = {}
            for P in ps:
                by_x.setdefault(P[0],[]).append(P)
            curve_key = f"p{p}-a{a}-b{b}"
            Q = rng.choice(ps)
            for q_value in (None,Q):
                cases.append(make_case(curve_key+"-empty",p,a,b,[],q_value,tags=["empty"]))
                cases.append(make_case(curve_key+"-only-infinity",p,a,b,[],q_value,True,tags=["input_infinity"]))
            # Singletons include both exceptional signs whenever distinct.
            singleton_points = {Q,negative(Q,p),rng.choice(ps)}
            singleton_points.update(P for P in ps if P[1] == 0)
            for P in sorted(singleton_points):
                tags = ["singleton"]
                if P == Q:tags.append("plus_Q")
                if P == negative(Q,p):tags.append("minus_Q")
                if not P[1]:tags.append("two_torsion")
                cases.append(make_case(curve_key+f"-single-{P}",p,a,b,[P],Q,tags=tags))
                cases.append(make_case(curve_key+f"-single-infty-{P}",p,a,b,[P],Q,True,tags=tags+["input_infinity"]))
            # Random simple divisors include all available x at maximum size.
            available = sorted(by_x)
            degrees = sorted({1,min(2,len(available)),min(4,len(available)),
                              min(8,len(available)),min(16,len(available)),len(available)})
            for degree in degrees:
                for repeat in range(2):
                    xs = rng.sample(available,degree)
                    selected = [rng.choice(by_x[x]) for x in xs]
                    q_value = None if repeat == 0 else Q
                    tags = ["random","identity_translation" if q_value is None else "finite_translation"]
                    if degree == p:tags.append("degree_equals_characteristic")
                    if any(P[1] == 0 for P in selected):tags.append("two_torsion")
                    cases.append(make_case(curve_key+f"-random-d{degree}-{repeat}",p,a,b,selected,q_value,repeat==1,tags))
            # Construct all-colliding translates by choosing opposite outputs.
            # Input x remain distinct, so collisions arise solely after Q-add.
            chosen = []
            used_input_x = set()
            used_output_x = set()
            for R in ps:
                if not R[1] or R[0] in used_output_x:
                    continue
                A = geometric_add(R,negative(Q,p),p,a,b)
                B = geometric_add(negative(R,p),negative(Q,p),p,a,b)
                if A is None or B is None or A[0] == B[0]:
                    continue
                if A[0] in used_input_x or B[0] in used_input_x:
                    continue
                chosen.extend([A,B]);used_input_x.update((A[0],B[0]));used_output_x.add(R[0])
                if len(chosen) in (2,4,8):
                    cases.append(make_case(curve_key+f"-all-collisions-d{len(chosen)}",p,a,b,chosen[:],Q,tags=["all_collisions"]))
                if len(chosen) >= 8:
                    break
            if chosen:
                extra = next((P for P in ps if P[0] not in used_input_x),None)
                if extra is not None:
                    cases.append(make_case(curve_key+"-mixed-collisions",p,a,b,chosen+[extra],Q,True,tags=["mixed_collisions","input_infinity"]))
            # Force the image x coordinate to zero whenever possible.
            zero_x = [P for P in ps if P[0] == 0]
            if zero_x:
                A = geometric_add(zero_x[0],negative(Q,p),p,a,b)
                if A is not None:
                    cases.append(make_case(curve_key+"-image-x-zero",p,a,b,[A],Q,tags=["zero_image"]))
    return cases


def mutator_checks(cases):
    outcomes = []
    good = next(c for c in cases if c["degree"] == 1 and
                next(iter(c["expected"])) is not None and
                next(iter(c["expected"]))[1] != 0)
    signed = split_expected(good["expected"],good["p"])
    wrong = deepcopy(signed)
    wrong["components"][0]["v"] = [(-x) % good["p"] for x in wrong["components"][0]["v"]]
    colliding = next(c for c in cases if "all_collisions" in c["tags"])
    missed_pair = split_expected(colliding["expected"],colliding["p"])
    missed_pair["components"].pop()
    exceptional = next(c for c in cases if c["expected"][None] == 1)
    missed_infinity = split_expected(exceptional["expected"],exceptional["p"])
    missed_infinity["infinity"] = False
    duplicate = deepcopy(signed)
    duplicate["components"].append(deepcopy(duplicate["components"][0]))
    wrong_root = deepcopy(signed)
    wrong_root["components"][0]["u"][0] = (wrong_root["components"][0]["u"][0]+1) % good["p"]
    for name,case,output in (("wrong_ordinate_sign",good,wrong),
                             ("missing_collision_branch",colliding,missed_pair),
                             ("missing_infinity",exceptional,missed_infinity),
                             ("duplicate_component",good,duplicate),
                             ("wrong_abscissa",good,wrong_root)):
        try:
            decode_and_check(output,case["expected"],case["p"],case["a"],case["b"],case["degree"])
        except ValueError as exc:
            outcomes.append({"mutation":name,"detected":True,"reason":str(exc)})
        else:
            outcomes.append({"mutation":name,"detected":False})
    require(all(x["detected"] for x in outcomes),"checker missed a deliberate mutation")
    return outcomes


def import_kernel():
    spec = importlib.util.spec_from_file_location("fast_trace_under_test",KERNEL)
    require(spec is not None and spec.loader is not None,"kernel cannot be loaded")
    module = importlib.util.module_from_spec(spec)
    # Dataclasses and similar decorators look up their declaring module.
    import sys
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def internal_checks(implementation):
    """Check projection and inversion identities against scalar point sums.

    Large-prime tests audit the 128-bit native modular arithmetic without
    enumerating the ambient field. Synthetic image maps have multiplicity at
    most two, which is the exact promise needed by the reconstruction API.
    They need not be a curve translation; weighted projection is more general.
    """
    rng = random.Random(2026111614)
    rows = []
    for p in (5,7,17,97,2147483647,2305843009213693951):
        for d in sorted({1,2,3,min(p,5),min(p,8),min(p,13)}):
            xs = list(range(d))
            for pattern in ("injective","paired","mixed"):
                if pattern == "injective":
                    images = list(range(d))
                elif pattern == "paired":
                    images = [i//2 for i in range(d)]
                else:
                    images = [i//2 if i < d//2 else i for i in range(d)]
                weights = [rng.randrange(p) for _ in xs]
                u = vanish(xs,p)
                phi = interpolate(xs,images,p)
                g = interpolate(xs,weights,p)
                pu,pp,pg = (implementation.Poly(z,p) for z in (u,phi,g))
                row = {"p":p,"degree":d,"image_pattern":pattern}
                try:
                    series = implementation._trace_series(pu,p,2*d-1)
                    expected_series = [sum(pow(x,k,p) for x in xs) % p
                                       for k in range(2*d-1)]
                    require([int(series[k]) for k in range(2*d-1)] == expected_series,
                            "logarithmic-derivative trace series differs from scalar sum")
                    dual = implementation._dual(pg,series,d,p)
                    expected_dual = [sum(w*pow(x,k,p) for x,w in zip(xs,weights)) % p
                                     for k in range(d)]
                    require(dual == expected_dual,"Hankel convolution differs from scalar sum")
                    aa,bb = implementation.power_project(pu,pp,pg,p)
                    expected_a = [sum(pow(y,k,p) for y in images) % p for k in range(2*d)]
                    expected_b = [sum(w*pow(y,k,p) for y,w in zip(images,weights)) % p
                                  for k in range(2*d)]
                    require(aa == expected_a,"unweighted projection differs from scalar sum")
                    require(bb == expected_b,"weighted projection differs from scalar sum")
                    distinct = sorted(set(images))
                    q = vanish(distinct,p)
                    require(clean(list(implementation._recurrence(aa,p)),p) == q,
                            "native recurrence differs from independently formed image polynomial")
                    # Independently form numerator of sum_i weight_i/(T-image_i).
                    for values,moments in (([1]*d,aa),(weights,bb)):
                        numerator = [0]*len(distinct)
                        for image,weight in zip(images,values):
                            term = quotient(q,[-image % p,1],p)
                            for j,c in enumerate(term):
                                numerator[j] = (numerator[j]+weight*c) % p
                        actual = implementation._trace_numerator(implementation.Poly(q,p),moments,p)
                        require(clean(list(actual),p) == clean(numerator,p),
                                "trace numerator differs from direct partial-fraction sum")
                    counts = Counter(images)
                    expected_single = vanish([x for x in distinct if counts[x] == 1],p)
                    expected_double = vanish([x for x in distinct if counts[x] == 2],p)
                    for method in (implementation.reconstruct_trace,implementation.reconstruct_dense):
                        single,double,ordinate = method(pu,pp,pg,p)
                        require(clean(list(single),p) == expected_single,"singleton image polynomial mismatch")
                        require(clean(list(double),p) == expected_double,"double image polynomial mismatch")
                        for y,weight in zip(images,weights):
                            if counts[y] == 1:
                                require(int(ordinate(y)) == weight,"inverse interpolation mismatch")
                    row["status"] = "passed"
                except Exception as exc:
                    row.update(status="failed",error=f"{type(exc).__name__}: {exc}")
                rows.append(row)
    return rows


def rejected_input_checks(implementation,cases):
    """Test promised runtime rejection, without assuming prime/split certification."""
    good = next(c for c in cases if len(c["selected"]) == 1 and c["degree"] == 1 and c["u"][0] != 0)
    args = [good[k] for k in ("p","a","b","u","v","Q")]
    p,a,b = args[:3]
    invalid = []
    def add(name,index,value,**kwargs):
        changed = deepcopy(args)
        changed[index] = value
        invalid.append((name,changed,kwargs))
    add("zero_u",3,[])
    add("nonmonic_u",3,[(2*c)%p for c in args[3]])
    add("nonsquarefree_u",3,multiply(args[3],args[3],p))
    # A chosen constant v must fail the curve identity for this singleton.
    bad_y = next(y for y in range(p) if (y*y-evaluate([b,a,0,1],good["selected"][0][0],p))%p)
    add("off_curve_divisor",4,[bad_y])
    bad_Q = next((x,y) for x in range(p) for y in range(p)
                 if not on_curve((x,y),p,a,b))
    add("off_curve_Q",5,bad_Q)
    add("characteristic_two",0,2)
    add("characteristic_three",0,3)
    add("field_too_large",0,1<<63)
    singular = deepcopy(args);singular[1:3] = [0,0]
    invalid.append(("singular_curve",singular,{}))
    invalid.append(("unknown_method",deepcopy(args),{"method":"unknown"}))
    # A square beta is rejected only if general-p collision lifting needs it.
    colliding = next(c for c in cases if "all_collisions" in c["tags"] and c["p"]%4 == 1)
    invalid.append(("square_supplied_as_nonsquare",
                    [colliding[k] for k in ("p","a","b","u","v","Q")],
                    {"nonsquare":1}))
    rows = []
    for name,changed,kwargs in invalid:
        try:
            implementation.translate(*changed,**kwargs)
        except ValueError as exc:
            rows.append({"input":name,"status":"passed","error":str(exc)})
        except Exception as exc:
            rows.append({"input":name,"status":"failed","error":f"unexpected {type(exc).__name__}: {exc}"})
        else:
            rows.append({"input":name,"status":"failed","error":"invalid input was accepted"})
    return rows


def run():
    if hasattr(os,"sched_setaffinity"):
        os.sched_setaffinity(0,{4})
    cases = fixtures()
    mutations = mutator_checks(cases)
    if not KERNEL.exists():
        result = {"status":"checker_ready_waiting_for_kernel","fixtures":len(cases),
                  "mutations":mutations}
        (HERE/"oracle_preflight.json").write_text(json.dumps(result,indent=2)+"\n")
        print(json.dumps(result,indent=2))
        return
    implementation = import_kernel()
    implementation.initialize()
    rows = []
    started = time.perf_counter()
    for case in cases:
        for method in ("trace","dense"):
            row = {"label":case["label"],"method":method,"p":case["p"],
                   "degree":case["degree"],"tags":case["tags"]}
            try:
                output = implementation.translate(case["p"],case["a"],case["b"],
                            case["u"],case["v"],case["Q"],method=method,
                            input_infinity=case["input_infinity"])
                decode_and_check(output,case["expected"],case["p"],case["a"],
                                 case["b"],case["degree"])
                row["status"] = "passed"
            except Exception as exc:
                row["status"] = "failed"
                row["error"] = f"{type(exc).__name__}: {exc}"
            rows.append(row)
    by_tag = {}
    for row in rows:
        for tag in row["tags"]:
            summary = by_tag.setdefault(tag,{"passed":0,"failed":0})
            summary[row["status"]] += 1
    internals = internal_checks(implementation)
    rejected = rejected_input_checks(implementation,cases)
    all_rows = rows+internals+rejected
    result = {"status":"passed" if all(x["status"] == "passed" for x in all_rows) else "failed",
              "scope":"Independent small-field geometric oracle; no production arithmetic reused in expected output. Correctness testing is not a formal proof or performance benchmark.",
              "source_sha256":hashlib.sha256(KERNEL.read_bytes()).hexdigest(),
              "native_source_sha256":hashlib.sha256((KERNEL.parent/"recurrence.cpp").read_bytes()).hexdigest(),
              "auditor_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              "case_count":len(cases),"translation_checks":len(rows),
              "elapsed_seconds_nonbenchmark":time.perf_counter()-started,
              "by_tag":by_tag,"mutations":mutations,"rows":rows,
              "internal_identity_checks":internals,"invalid_input_checks":rejected}
    (HERE/"audit_results.json").write_text(json.dumps(result,indent=2)+"\n")
    summary = {k:v for k,v in result.items() if k not in ("rows","internal_identity_checks","invalid_input_checks")}
    summary["internal_identity_checks"] = dict(Counter(x["status"] for x in internals))
    summary["invalid_input_checks"] = dict(Counter(x["status"] for x in rejected))
    summary["failures"] = [x for x in all_rows if x["status"] != "passed"]
    print(json.dumps(summary,indent=2))


if __name__ == "__main__":
    run()
