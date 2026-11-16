"""Structural precheck and bounded supplied-point witness selection; no field sweep."""
from dataclasses import dataclass
from itertools import combinations
from reference_coefficients import Basis, canonical


def parameters(r, q):
    if type(r) is not int or type(q) is not int or r <= 3 or not 1 <= q < r:
        raise ValueError('r must be caller-certified prime >3, with 1<=q<r')


def signature_bound(r, q):
    """Derived exclusion certificate, distinct from an explicit sparse witness."""
    parameters(r, q)
    # Avoid constructing a q-bit integer when the bound is already vacuous.
    cap = r if q >= r.bit_length() else min(r, (1 << q) + q)
    return dict(schema='character-signature-bound/v1', r=r, q=q,
                signature_cap=cap, agreement_cap=cap,
                status='analytic_excluded' if cap < r else 'inconclusive',
                prime_certification='caller precondition',
                domain='all functions of the q shifted quadratic characters')


@dataclass(frozen=True)
class Row:
    pivot: int
    feature: tuple
    target: int
    combination: tuple


class Stream:
    """Retain independent feature rows only; each offer performs fully charged work."""
    def __init__(self, r, q, meter):
        parameters(r, q)
        if meter.r != r:
            raise ValueError('meter field')
        labels = tuple(S for d in range(4) for S in combinations(range(q), d))
        self.basis = Basis(r, q, labels)
        self.meter = meter
        self.rows = []
        self.points = []
        self.offers = 0
        self.finished = False

    def offer(self, x):
        """Return independent, consistent_dependent, or a normalized witness."""
        r, meter = self.basis.r, self.meter
        if self.finished or meter.r != r or not canonical(x, r):
            raise ValueError('finished stream, field, or noncanonical coordinate')
        self.offers += 1
        v = list(self.basis.evaluate(x, meter))
        target = x
        t = len(self.rows)
        coefficients = [0]*t + [1]
        # Insertion order is the echelon order: every row is zero at all
        # earlier pivots, even when numerical pivot indices are not sorted.
        for row in self.rows:
            factor = v[row.pivot]
            v = [meter.sub(a, meter.mul(factor, b)) for a, b in zip(v, row.feature)]
            target = meter.sub(target, meter.mul(factor, row.target))
            for j, c in enumerate(row.combination):
                coefficients[j] = meter.sub(coefficients[j], meter.mul(factor, c))
        pivot = next((j for j, value in enumerate(v) if value), None)
        if pivot is not None:
            inverse = meter.inv(v[pivot])
            feature = tuple(meter.mul(a, inverse) for a in v)
            value = meter.mul(target, inverse)
            combination = tuple(meter.mul(a, inverse) for a in coefficients)
            self.rows.append(Row(pivot, feature, value, combination))
            self.points.append(x)
            return dict(status='independent', feature_rank=len(self.rows), offers=self.offers)
        if not target:
            return dict(status='consistent_dependent', feature_rank=t, offers=self.offers)
        inverse = meter.inv(target)
        normalized = tuple(meter.mul(a, inverse) for a in coefficients)
        support = tuple(zip(tuple(self.points)+(x,), normalized))
        self.finished = True
        return dict(status='witness', support=support, feature_rank=t, offers=self.offers,
                    verification='requires independent_check.verify on public support')


def collect(r, q, points, limit, meter):
    """Bounded caller-supplied stream. Exhaustion never means represented/excluded."""
    if type(limit) is not int or limit < 0:
        raise ValueError('nonnegative finite sample limit')
    stream = Stream(r, q, meter)
    iterator = iter(points)
    for _ in range(limit):
        try:
            x = next(iterator)
        except StopIteration:
            break
        result = stream.offer(x)
        if result['status'] == 'witness':
            return result
    return dict(status='inconclusive', offers=stream.offers,
                feature_rank=len(stream.rows), reason='sample budget or stream exhausted')
