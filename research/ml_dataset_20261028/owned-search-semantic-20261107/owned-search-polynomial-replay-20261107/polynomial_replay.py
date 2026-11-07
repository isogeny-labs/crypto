"""Bounded independent GF(2)[x]/(x_i^2+x_i) row-identity replay.

Equality is relative to the supplied node equations, not original-map admission,
GB completeness, a signed relation or a fall-degree certificate.
"""
from dataclasses import dataclass, asdict

@dataclass(frozen=True)
class Limits:
    variables: int = 65536
    generators: int = 4096
    terms: int = 65536
    degree: int = 4096
    proof: int = 65536
    work: int = 5_000_000
    allocation_units: int = 2_000_000

class Refusal(Exception):
    def __init__(self, status, reason):
        self.status, self.reason = status, reason

class Meter:
    def __init__(self, limits):
        self.limits = limits
        self.work = self.allocation_units = 0
        hard = Limits(1_000_000, 65536, 1_000_000, 65536, 1_000_000,
                      100_000_000, 20_000_000)
        if type(limits) is not Limits:
            raise Refusal('invalid', 'limits must be Limits')
        for key, value in asdict(limits).items():
            if type(value) is not int or value < 0 or value > getattr(hard, key):
                raise Refusal('invalid', 'limits outside hard bounds')

    def pay(self, work=1, literals=0):
        if self.work + work > self.limits.work:
            raise Refusal('inconclusive', 'work cap')
        if self.allocation_units + literals > self.limits.allocation_units:
            raise Refusal('inconclusive', 'cumulative allocation units cap')
        self.work += work
        self.allocation_units += literals

    def sequence(self, value, cap, name):
        self.pay()
        if type(value) is not list:
            raise Refusal('invalid', name + ' must be an array')
        if len(value) > cap:
            raise Refusal('inconclusive', name + ' cap')

    def monomial(self, value, n):
        self.sequence(value, self.limits.degree, 'degree')
        previous = -1
        for variable in value:
            self.pay()
            if type(variable) is not int or variable < 0 or variable >= n:
                raise Refusal('invalid', 'variable out of range or non-integer')
            if variable <= previous:
                raise Refusal('invalid', 'monomial must be strictly increasing')
            previous = variable
        self.pay(len(value) + 1, len(value) + 1)
        return tuple(value)

    def polynomial(self, value, n):
        self.sequence(value, self.limits.terms, 'polynomial terms')
        result, previous = [], None
        for raw in value:
            term = self.monomial(raw, n)
            key = (len(term), term)
            self.pay(len(term) + (len(previous[1]) if previous else 0) + 1)
            if previous is not None and key <= previous:
                raise Refusal('invalid', 'polynomial must be strict degree-lex')
            previous = key
            result.append(term)
        return result

    def union(self, a, b):
        # Boolean quotient: multiply squarefree monomials by variable union.
        self.pay(len(a) + len(b) + 1, len(a) + len(b) + 1)
        result, i, j = [], 0, 0
        while i < len(a) or j < len(b):
            if j == len(b) or (i < len(a) and a[i] < b[j]):
                result.append(a[i]); i += 1
            elif i == len(a) or b[j] < a[i]:
                result.append(b[j]); j += 1
            else:
                result.append(a[i]); i += 1; j += 1
        if len(result) > self.limits.degree:
            raise Refusal('inconclusive', 'product degree cap')
        return tuple(result)

    def toggle(self, acc, term):
        self.pay(len(term) + 1)
        if term in acc:
            acc.remove(term)
        else:
            if len(acc) >= self.limits.terms:
                raise Refusal('inconclusive', 'accumulator term cap')
            acc.add(term)

    def equations(self, n, generators):
        if type(n) is not int or n < 0 or n > 2**32 - 1:
            raise Refusal('invalid', 'invalid variable count')
        if n > self.limits.variables:
            raise Refusal('inconclusive', 'variable cap')
        self.sequence(generators, self.limits.generators, 'generators')
        return [self.polynomial(g, n) for g in generators]

    def row(self, n, generators, row):
        self.pay()
        if type(row) is not dict or len(row) != 2 or set(row) != {'polynomial', 'proof'}:
            raise Refusal('invalid', 'row keys')
        polynomial = self.polynomial(row['polynomial'], n)
        proof = row['proof']
        self.sequence(proof, self.limits.proof, 'proof terms')
        previous, acc = None, set()
        for raw in proof:
            self.sequence(raw, 2, 'proof pair')
            if len(raw) != 2 or type(raw[0]) is not int:
                raise Refusal('invalid', 'proof pair must be [generator, multiplier]')
            generator = raw[0]
            if generator < 0 or generator >= len(generators):
                raise Refusal('invalid', 'generator index out of range')
            multiplier = self.monomial(raw[1], n)
            key = (generator, multiplier)
            self.pay(len(multiplier) + (len(previous[1]) if previous else 0) + 1)
            if previous is not None and key <= previous:
                raise Refusal('invalid', 'proof must be strict generator then lex order')
            previous = key
            for term in generators[generator]:
                self.toggle(acc, self.union(multiplier, term))
        self.pay(sum(len(t) + 1 for t in polynomial) + 1)
        if acc != set(polynomial):
            raise Refusal('mismatch', 'row differs from generator combination')
        return polynomial


def _verify(n, generators, row, limits, field=None):
    meter = None
    try:
        meter = Meter(limits)
        equations = meter.equations(n, generators)
        polynomial = meter.row(n, equations, row)
        if field is not None:
            basis, variable = field
            if type(variable) is not int or not 0 <= variable < n:
                raise Refusal('invalid', 'field-task variable out of range')
            basis_poly = meter.row(n, equations, basis)
            meter.pay(len(basis_poly[-1]) + 1 if basis_poly else 1)
            if not basis_poly or variable not in basis_poly[-1]:
                raise Refusal('invalid', 'field variable must divide basis leading monomial')
            acc = set()
            for term in basis_poly:
                meter.toggle(acc, meter.union(term, (variable,)))
            meter.pay(sum(len(t) + 1 for t in polynomial) + 1)
            if set(polynomial) != acc:
                raise Refusal('mismatch', 'field row differs from x_v times basis row')
        status, reason = 'verified', 'Boolean quotient polynomial equality'
    except Refusal as failure:
        status, reason = failure.status, failure.reason
    return {'schema': 'owned-search-row-replay/v1', 'status': status,
            'reason': reason, 'work': meter.work if meter else 0,
            'allocation_units': meter.allocation_units if meter else 0,
            'field_task_checked': field is not None and status == 'verified',
            'scope': 'supplied node generators only',
            'source_authenticated': False, 'groebner_basis_complete': False,
            'fall_degree_certified': False}


def verify_row(variables, generators, row, limits=Limits()):
    return _verify(variables, generators, row, limits)


def verify_field_row(variables, generators, basis_row, variable, row, limits=Limits()):
    """Actual Field(i,v) task is x_v*basis[i] in the Boolean quotient.

    No extra generator index encodes x_v^2+x_v. Both input and output row
    derivations are checked against the supplied node generator array.
    """
    return _verify(variables, generators, row, limits, (basis_row, variable))
