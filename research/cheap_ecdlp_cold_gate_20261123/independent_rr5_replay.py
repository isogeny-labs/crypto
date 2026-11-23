#!/usr/bin/env python3
"""Development-only independent group/linear replay of public Prime31 RR5 receipts.

Accepts a synthetic direct receipt or the *native RR5 SolveRecord JSON* plus an
explicit factor-base manifest. It never reads factor-base logs or a target secret.
This is not native-Rust execution, a signed independent validator, or a benchmark.
"""
import argparse
import hashlib
import json
from pathlib import Path

P, N = 2_147_483_647, 2_147_446_831
A, B = 847_514_846, 2_087_444_724
G = (625_037_715, 2_100_627_251)


def need_int(value, upper, label):
    if type(value) is not int or not 0 <= value < upper:
        raise ValueError(f"{label}: noncanonical integer")
    return value


def point(value, label):
    if value is None:
        return None
    if not isinstance(value, list) or len(value) != 2:
        raise ValueError(f"{label}: expected [x,y] or null")
    x, y = (need_int(v, P, label) for v in value)
    if (y*y - x*x*x - A*x - B) % P:
        raise ValueError(f"{label}: off curve")
    return x, y


def add(x, y):
    if x is None:
        return y
    if y is None:
        return x
    u, v = x
    w, z = y
    if u == w and (v + z) % P == 0:
        return None
    if u == w:
        numerator, denominator = (3*u*u + A) % P, (2*v) % P
    else:
        numerator, denominator = (z-v) % P, (w-u) % P
    if denominator == 0:
        raise ValueError("undefined group-law denominator")
    slope = numerator * pow(denominator, P-2, P) % P
    new_x = (slope*slope-u-w) % P
    return new_x, (slope*(u-new_x)-v) % P


def scale(k, pt):
    out = None
    while k:
        if k & 1:
            out = add(out, pt)
        pt = add(pt, pt)
        k >>= 1
    return out


def load(path):
    raw = Path(path).read_bytes()
    return json.loads(raw), hashlib.sha256(raw).hexdigest()


class LinearSpan:
    """Independent forward elimination over variable coefficients (no RHS)."""
    def __init__(self, width):
        self.width = width
        self.pivots = {}

    def insert(self, values):
        v = list(values)
        for j in sorted(self.pivots):
            if v[j]:
                base = self.pivots[j]
                factor = v[j]
                v = [(a-factor*b) % N for a, b in zip(v, base)]
        pivot = next((j for j, x in enumerate(v) if x), None)
        if pivot is None:
            return False
        inv = pow(v[pivot], N-2, N)
        self.pivots[pivot] = [(x*inv) % N for x in v]
        return True

    @property
    def rank(self):
        return len(self.pivots)


def canonical_key(row):
    p = next((x for x in row[:-1] if x), None)
    if p is None:
        if row[-1]:
            raise ValueError("contradictory zero coefficient equation")
        return tuple(row)
    inverse = pow(p, N-2, N)
    return tuple(x*inverse % N for x in row)


def validate(data, base, q, rows, producer=None):
    if not 0 < len(base) <= 4096 or len(rows) > 65536:
        raise ValueError("unsupported row/base size")
    if len(set(base)) != len(base):
        raise ValueError("duplicate factor-base points")
    bsize = len(base)
    full, without_q = (LinearSpan(bsize+1), LinearSpan(bsize))
    # For the scalar, eliminate base variables in a separate augmented system.
    base_pivots = {}
    first, recovered, dependent = None, (0 if q is None else None), 0
    if q is None:
        first = 0
    seen_keys = set()
    for number, relation in enumerate(rows, 1):
        if not isinstance(relation, dict):
            raise ValueError(f"row {number}: not a record")
        a = need_int(relation.get("a"), N, f"row {number} a")
        b = need_int(relation.get("b"), N, f"row {number} b")
        terms = relation.get("terms", relation.get("canonical_terms"))
        if not isinstance(terms, list) or len(terms) > 4:
            raise ValueError(f"row {number}: invalid term list")
        lhs = None
        coeffs = [0] * bsize
        leaves = 0
        for term in terms:
            if not isinstance(term, list) or len(term) != 2:
                raise ValueError(f"row {number}: malformed term")
            ix, c = term
            ix = need_int(ix, bsize, "factor-base index")
            if type(c) is not int or c == 0 or abs(c) > 4:
                raise ValueError(f"row {number}: invalid multiplicity")
            leaves += abs(c)
            coeffs[ix] = (coeffs[ix] + c) % N
            lhs = add(lhs, scale(abs(c), base[ix] if c > 0 else (base[ix][0], (-base[ix][1]) % P)))
        if leaves > 4:
            raise ValueError(f"row {number}: more than four signed leaves")
        rhs = add(scale(a, G), scale(b, q))
        if lhs != rhs:
            raise ValueError(f"row {number}: invalid public EC equality")
        vector = coeffs + [(-b) % N]
        augmented = vector + [a]
        key = canonical_key(augmented)
        is_duplicate = key in seen_keys
        seen_keys.add(key)
        previous_rank = full.rank
        independent = full.insert(vector)
        without_q.insert(coeffs)
        if not independent:
            dependent += 1
        if producer:
            if relation.get("rank_before") != previous_rank or relation.get("rank_after") != full.rank:
                raise ValueError(f"row {number}: producer rank disagreement")
            expected = "Duplicate" if is_duplicate else ("Independent" if independent else "Dependent")
            if relation.get("disposition") != expected:
                raise ValueError(f"row {number}: producer disposition mismatch {expected}")
        # Separate base-only reduction identifies a leftover q equation.
        rest = augmented[:]
        for col in sorted(base_pivots):
            if rest[col]:
                factor = rest[col]
                rest = [(x-factor*y) % N for x,y in zip(rest,base_pivots[col])]
        candidate_col = next((j for j in range(bsize) if rest[j]), None)
        if candidate_col is not None:
            inv = pow(rest[candidate_col], N-2, N)
            base_pivots[candidate_col] = [(v*inv) % N for v in rest]
        elif rest[bsize]:
            candidate = rest[-1] * pow(rest[bsize], N-2, N) % N
            if recovered is not None and recovered != candidate:
                raise ValueError(f"row {number}: contradictory target scalar")
            if recovered is None:
                recovered, first = candidate, number
                if scale(candidate, G) != q:
                    raise ValueError(f"row {number}: recovered scalar does not replay")
        elif rest[-1]:
            raise ValueError(f"row {number}: contradictory augmented system")
        if (full.rank-without_q.rank == 1) != (recovered is not None):
            # For the identity Q=O, target d=0 is known without equations.
            if q is not None:
                raise ValueError(f"row {number}: rank-gap/target-visibility mismatch")
        if producer and relation.get("recovered_scalar") != recovered:
            raise ValueError(f"row {number}: producer recovered_scalar mismatch")
    return {"curve":"ordinary_prime31","p":P,"n":N,"base_size":bsize,
            "authenticated_rows":len(rows),"independent_rank":full.rank,"dependent_rows":dependent,
            "rank_without_target_column":without_q.rank,"first_target_prefix":first,
            "recovered":recovered,"scalar_replay_passed":recovered is not None,
            "source":"coordinator-authored Python development replay; no independent PCC or native Rust pass"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receipt", help="direct public_rr5_sample.json-style input")
    parser.add_argument("--record", help="native RR5 SolveRecord JSON")
    parser.add_argument("--base", help="public base manifest with base array for --record")
    args = parser.parse_args()
    if bool(args.receipt) == bool(args.record) or bool(args.record) != bool(args.base):
        parser.error("provide --receipt OR both --record and --base")
    raw, rawhash = load(args.receipt or args.record)
    if args.receipt:
        base = [point(v,"base") for v in raw["base"]]
        q = point(raw["target_q"], "target_q")
        rows = raw["relations"]
        extras = {"input_sha256":rawhash, "input_type":"synthetic_or_public_receipt"}
    else:
        manifest, basehash = load(args.base)
        if manifest.get("curve") != "ordinary_prime31" or any(manifest.get(k)!=v for k,v in (("p",P),("n",N),("a",A),("b",B),("g",list(G)))):
            raise ValueError("base manifest does not identify the exact ordinary Prime31 fixture")
        base = [point(v,"base") for v in manifest["base"]]
        q = point(raw["target"],"producer target")
        rows=[]
        attempts=raw["attempts"]
        for i,attempt in enumerate(attempts):
            aa=need_int(attempt["a"],N,f"attempt {i} a")
            bb=need_int(attempt["b"],N,f"attempt {i} b")
            pt=point(attempt["point"],"producer query point")
            if pt != add(scale(aa,G),scale(bb,q)):
                raise ValueError(f"attempt {i}: public query point mismatch")
            for row in attempt["receipts"]:
                rows.append(row)
        extras={"input_sha256":rawhash,"base_sha256":basehash,
                "input_type":"producer_SolveRecord","attempts_included":len(attempts),
                "producer_status":raw.get("status"),"producer_final_replay":raw.get("final_replay_passed"),
                "producer_error":raw.get("terminal_error"),
                "producer_deadline_exceeded":raw.get("deadline_exceeded"),
                "extracted_candidate_rows":sum(len(a.get("extraction",{}).get("relations",[])) for a in attempts)}
    result=validate(raw,base,q,rows,producer=bool(args.record))
    if args.record:
        if raw.get("rank") != result["independent_rank"] or raw.get("scalar") != result["recovered"]:
            raise ValueError("producer terminal rank/scalar mismatch")
        if raw.get("final_replay_passed") != result["scalar_replay_passed"]:
            raise ValueError("producer final_replay_passed mismatch")
    print(json.dumps({**result,**extras},sort_keys=True,indent=2))


if __name__ == "__main__":
    try:
        main()
    except (ValueError,KeyError,TypeError,IndexError) as exc:
        raise SystemExit(f"FAIL: {exc}")
