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


# Native solver.rs advances SplitMix64 twice per public query, once for
# each RHS label. A caller-supplied seed allows a read-only record consumer to
# detect skipped, reordered or relabelled attempts; the producer does not yet
# serialize this seed, so it is an EXTERNAL, opt-in reproducibility check.
U64 = 1 << 64


def splitmix_next(state):
    state = (state + 0x9e3779b97f4a7c15) % U64
    z = state
    z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) % U64
    z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) % U64
    return state, (z ^ (z >> 31)) % N


def verify_public_query_schedule(attempts, seed, max_queries, status):
    """Replay the native seed-to-(a,b) stream without a target secret.

    This authenticates schedule *consistency* with an independently specified
    seed and declared cap. It does not attest that F4 was executed, that
    deadlines were honest, or that the seed was fixed before the experiment.
    """
    state = need_int(seed, U64, "query seed")
    if type(max_queries) is not int or not 0 <= max_queries:
        raise ValueError("noncanonical maximum public-query count")
    if not isinstance(attempts, list) or len(attempts) > max_queries:
        raise ValueError("attempt count exceeds declared query cap")
    if status not in ("query_limit", "solved", "solved_after_deadline",
                      "deadline", "relation_rejected", "scalar_replay_failed"):
        raise ValueError("unsupported native terminal status")
    if status == "query_limit" and len(attempts) != max_queries:
        raise ValueError("query_limit omits public attempts")
    for i, attempt in enumerate(attempts):
        state, expected_a = splitmix_next(state)
        state, expected_b = splitmix_next(state)
        if (not isinstance(attempt, dict)
                or type(attempt.get("a")) is not int
                or type(attempt.get("b")) is not int
                or (attempt["a"], attempt["b"]) != (expected_a, expected_b)):
            raise ValueError(f"attempt {i}: mismatch with source-exact SplitMix64 query schedule")
    return len(attempts)


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


def canonicalized_extraction(candidate, a, b, base_size, label):
    """Replay producer RelationAdapter canonicalization of an extracted SignedRelation.

    An accepted receipt must match the *next* emitted candidate, not merely be
    any true elliptic-curve equation supplied independently by the producer.
    The producing code adds repeated signed leaves, removes cancellations,
    sorts indices and globally flips coefficients and RHS if the first is
    negative. This proves record-internal linkage, NOT that F4 actually ran.
    """
    if not isinstance(candidate, dict) or not isinstance(candidate.get("terms"), list):
        raise ValueError(f"{label}: missing signed extraction terms")
    coeffs = {}
    leaves = 0
    for term in candidate["terms"]:
        if not isinstance(term, list) or len(term) != 2:
            raise ValueError(f"{label}: malformed extracted term")
        ix, c = term
        ix = need_int(ix, base_size, f"{label} index")
        if type(c) is not int or not -4 <= c <= 4:
            raise ValueError(f"{label}: invalid extracted multiplicity")
        leaves += abs(c)
        if leaves > 4:
            raise ValueError(f"{label}: more than four extracted leaves")
        coeffs[ix] = coeffs.get(ix, 0) + c
    terms = [[ix, c] for ix, c in sorted(coeffs.items()) if c]
    if terms and terms[0][1] < 0:
        terms = [[ix, -c] for ix, c in terms]
        a, b = (-a) % N, (-b) % N
    return terms, a, b


def verify_dual_certificate(originals, weights, scalar, base_size):
    """Check a sparse separating combination against authenticated raw rows.

    Rows are [C|-b|a], and a certificate must sum to [0,...,0,1,d].
    No factor-base logarithm or secret target scalar is an input.
    """
    if not isinstance(weights, list) or not 0 < len(weights) <= base_size + 1:
        raise ValueError("dual target certificate: invalid support")
    scalar = need_int(scalar, N, "dual target scalar")
    total = [0] * (base_size + 2)
    previous = 0
    for term in weights:
        if not isinstance(term, list) or len(term) != 2:
            raise ValueError("dual target certificate: malformed term")
        number, weight = term
        if type(number) is not int or number <= previous or number not in originals:
            raise ValueError("dual target certificate: noncanonical or absent row reference")
        previous = number
        weight = need_int(weight, N, "dual target weight")
        if weight == 0:
            raise ValueError("dual target certificate: zero weight")
        row = originals[number]
        if len(row) != base_size + 2:
            raise ValueError("dual target certificate: malformed source row")
        total = [(v + weight * x) % N for v, x in zip(total, row)]
    if total != [0] * base_size + [1, scalar]:
        raise ValueError("dual target certificate: separating equation mismatch")
    return True


def validate(data, base, q, rows, producer=None, emit_dual_certificate=False):
    if not 0 < len(base) <= 4096 or len(rows) > 65536:
        raise ValueError("unsupported row/base size")
    # Enforce the original Rust adapter's base contract even when validate()
    # is invoked programmatically rather than through the CLI point parser.
    # Infinity is allowed as a public target, never as a factor-base leaf.
    for ix, pt in enumerate(base):
        if pt is None:
            raise ValueError(f"base[{ix}]: identity is not an admissible factor-base point")
        if (not isinstance(pt, tuple) or len(pt) != 2
                or point(list(pt), f"base[{ix}]") != pt):
            raise ValueError(f"base[{ix}]: noncanonical factor-base point")
    if q is not None and (not isinstance(q, tuple) or len(q) != 2
                          or point(list(q), "target Q") != q):
        raise ValueError("target Q is not a canonical curve point")
    if len(set(base)) != len(base):
        raise ValueError("duplicate factor-base points")
    bsize = len(base)
    if type(emit_dual_certificate) is not bool or (emit_dual_certificate and bsize > 1024):
        raise ValueError("dual target certificate requires a Boolean opt-in and B<=1024")
    full, without_q = (LinearSpan(bsize+1), LinearSpan(bsize))
    # For the scalar, eliminate base variables in a separate augmented system.
    base_pivots = {}
    # Opt-in sparse elimination provenance for a fully checkable target witness.
    # Each pivot references only the rows that created the base-column basis;
    # avoid retaining all failed/dependent input rows. Not a new extractor.
    pivot_sources, originals, dual_certificate = {}, {}, None
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
        witness = {number: 1} if emit_dual_certificate and recovered is None else None
        for col in sorted(base_pivots):
            if rest[col]:
                factor = rest[col]
                rest = [(x-factor*y) % N for x,y in zip(rest,base_pivots[col])]
                if witness is not None:
                    for rid, weight in pivot_sources[col].items():
                        updated = (witness.get(rid, 0) - factor * weight) % N
                        if updated:
                            witness[rid] = updated
                        else:
                            witness.pop(rid, None)
        candidate_col = next((j for j in range(bsize) if rest[j]), None)
        if candidate_col is not None:
            inv = pow(rest[candidate_col], N-2, N)
            base_pivots[candidate_col] = [(v*inv) % N for v in rest]
            if witness is not None:
                pivot_sources[candidate_col] = {
                    rid: weight * inv % N for rid, weight in witness.items()
                    if weight * inv % N
                }
                originals[number] = augmented
        elif rest[bsize]:
            target_inverse = pow(rest[bsize], N-2, N)
            candidate = rest[-1] * target_inverse % N
            if recovered is not None and recovered != candidate:
                raise ValueError(f"row {number}: contradictory target scalar")
            if recovered is None:
                recovered, first = candidate, number
                if scale(candidate, G) != q:
                    raise ValueError(f"row {number}: recovered scalar does not replay")
                if witness is not None:
                    originals[number] = augmented
                    weights = [[rid, weight * target_inverse % N]
                               for rid, weight in sorted(witness.items())
                               if weight * target_inverse % N]
                    verify_dual_certificate(originals, weights, candidate, bsize)
                    dual_certificate = {
                        "relation_weights": weights,
                        "target_coefficient": 1,
                        "scalar": candidate,
                        "first_target_prefix": number,
                    }
        elif rest[-1]:
            raise ValueError(f"row {number}: contradictory augmented system")
        if (full.rank-without_q.rank == 1) != (recovered is not None):
            # For the identity Q=O, target d=0 is known without equations.
            if q is not None:
                raise ValueError(f"row {number}: rank-gap/target-visibility mismatch")
        if producer and relation.get("recovered_scalar") != recovered:
            raise ValueError(f"row {number}: producer recovered_scalar mismatch")
    report = {"curve":"ordinary_prime31","p":P,"n":N,"base_size":bsize,
              "authenticated_rows":len(rows),"independent_rank":full.rank,"dependent_rows":dependent,
              "rank_without_target_column":without_q.rank,"first_target_prefix":first,
              "recovered":recovered,"scalar_replay_passed":recovered is not None,
              "source":"coordinator-authored Python development replay; no independent PCC or native Rust pass"}
    if emit_dual_certificate:
        report["dual_target_certificate"] = dual_certificate
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receipt", help="direct public_rr5_sample.json-style input")
    parser.add_argument("--record", help="native RR5 SolveRecord JSON")
    parser.add_argument("--base", help="public base manifest with base array for --record")
    parser.add_argument("--seed", type=lambda x: int(x, 0),
                        help="optional native u64 SplitMix64 query seed; requires --max-queries")
    parser.add_argument("--max-queries", type=int,
                        help="optional external native query cap; requires --seed")
    parser.add_argument("--emit-dual-certificate", action="store_true",
                        help="opt-in sparse separating weights for independently replaying a unique target")
    args = parser.parse_args()
    if bool(args.receipt) == bool(args.record) or bool(args.record) != bool(args.base):
        parser.error("provide --receipt OR both --record and --base")
    if (args.seed is None) != (args.max_queries is None):
        parser.error("--seed and --max-queries must be provided together")
    if args.receipt and args.seed is not None:
        parser.error("the public query schedule is available only with --record")
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
        checked_schedule = None
        if args.seed is not None:
            checked_schedule = verify_public_query_schedule(
                attempts, args.seed, args.max_queries, raw.get("status"))
        extracted = 0
        for i,attempt in enumerate(attempts):
            aa=need_int(attempt["a"],N,f"attempt {i} a")
            bb=need_int(attempt["b"],N,f"attempt {i} b")
            pt=point(attempt["point"],"producer query point")
            if pt != add(scale(aa,G),scale(bb,q)):
                raise ValueError(f"attempt {i}: public query point mismatch")
            extraction=attempt.get("extraction")
            candidates=extraction.get("relations") if isinstance(extraction,dict) else None
            accepted=attempt.get("receipts")
            if not isinstance(candidates,list) or not isinstance(accepted,list):
                raise ValueError(f"attempt {i}: missing extractor/receipt arrays")
            if len(accepted)>len(candidates):
                raise ValueError(f"attempt {i}: accepted receipts exceed extracted rows")
            # The native solver consumes each candidate in order and stops
            # on rejection, deadline or recovered scalar. Accepted rows must
            # therefore be the canonicalized PREFIX of that attempt's output.
            for j,row in enumerate(accepted):
                if not isinstance(row,dict):
                    raise ValueError(f"attempt {i} accepted row {j}: malformed receipt")
                expected=canonicalized_extraction(candidates[j],aa,bb,len(base),
                                                  f"attempt {i} candidate {j}")
                actual=(row.get("canonical_terms"),row.get("a"),row.get("b"))
                if actual!=expected:
                    raise ValueError(f"attempt {i} accepted row {j}: not canonical extraction prefix")
                rows.append(row)
            extracted += len(candidates)
        # Unaccepted/failed/censored extractor rows are preserved as reported
        # denominator only. They never enter authenticated rank or recovery.
        extras={"input_sha256":rawhash,"base_sha256":basehash,
                "input_type":"producer_SolveRecord","attempts_included":len(attempts),
                "attempts_without_accepted_rows":sum(not a["receipts"] for a in attempts),
                "producer_status":raw.get("status"),"producer_final_replay":raw.get("final_replay_passed"),
                "producer_error":raw.get("terminal_error"),
                "producer_deadline_exceeded":raw.get("deadline_exceeded"),
                "extracted_candidate_rows":extracted,
                "accepted_rows":len(rows),"extracted_but_not_accepted_rows":extracted-len(rows),
                "query_schedule_checked":checked_schedule is not None,
                "source_exact_scheduled_queries":checked_schedule,
                "declared_max_queries":args.max_queries}
    result=validate(raw,base,q,rows,producer=bool(args.record),
                    emit_dual_certificate=args.emit_dual_certificate)
    if args.record:
        if raw.get("rank") != result["independent_rank"] or raw.get("scalar") != result["recovered"]:
            raise ValueError("producer terminal rank/scalar mismatch")
        if raw.get("final_replay_passed") != result["scalar_replay_passed"]:
            raise ValueError("producer final_replay_passed mismatch")
        solved_status = raw.get("status") in ("solved", "solved_after_deadline")
        if solved_status != result["scalar_replay_passed"]:
            raise ValueError("producer terminal status/scalar contradiction")
    print(json.dumps({**result,**extras},sort_keys=True,indent=2))


if __name__ == "__main__":
    try:
        main()
    except (ValueError,KeyError,TypeError,IndexError) as exc:
        raise SystemExit(f"FAIL: {exc}")
