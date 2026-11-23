"""Source-free scalar Schur unwind frames; derived, no numerical trials.

Borders and inverse have trusted internal provenance. No public certificate
is claimed. Retain no source, generators, Schur object, closure or full RHS.
"""
from pivot import permute
from pair import BorderPair


class LiftFrame:
    __slots__ = ('pair', 'inverse', 'rhs0', 'row', 'column',
                 'meter', 'profile', 'consumed')

    def __init__(self, schur, rhs, inverse, row, column):
        n = schur.dimension
        m = schur.m
        if schur.cut != 1 or schur.profile['released'] or len(rhs) != n:
            raise ValueError('live scalar borders and full RHS required')
        if not (0 <= row < n and 0 <= column < n):
            raise ValueError('permutation index')
        if type(inverse) is not int or not 0 < inverse < m.r:
            raise ValueError('canonical nonzero inverse required')
        if any(type(v) is not int or not 0 <= v < m.r for v in rhs):
            raise ValueError('canonical RHS required')
        self.pair = BorderPair(schur.first_row[1:], schur.first_column[1:], schur.a, m)
        self.inverse, self.rhs0 = inverse, rhs[0]
        self.row, self.column, self.meter = row, column, m
        self.consumed = False
        self.profile = dict(dimension=n, retained_field_entries=self.pair.profile['retained_field_entries']+2,
                            border_profile=self.pair.profile,
                            dot_products=0, dot_additions=0, source_queries=0,
                            consumed=False)
        m.add('compact_frames_created')

    def lift(self, tail, dual=False):
        if self.consumed:
            raise ValueError('frame already consumed')
        m = self.meter
        if len(tail) != self.pair.size or any(
                type(v) is not int or not 0 <= v < m.r for v in tail):
            raise ValueError('canonical complete lift tail required')
        if type(dual) is not bool:
            raise ValueError('boolean lift direction required')
        # Reserve this one-use frame before arithmetic, including failure paths.
        self.consumed = True
        self.profile['consumed'] = True
        value = self.pair.dot(tail, dual, m)
        self.profile['dot_products'] = self.pair.profile['dot_products']
        self.profile['dot_additions'] = self.pair.profile['dot_additions']
        m.add('dual_lift_calls' if dual else 'primal_lift_calls')
        m.add('leading_transpose_calls' if dual else 'leading_forward_calls')
        m.add('rank_solver_scalar_solve_calls')
        if dual:
            head = m.sub(0, m.mul(self.inverse, value))
        else:
            head = m.mul(self.inverse, m.sub(self.rhs0, value))
        result = permute([head] + list(tail), 0, self.row if dual else self.column)
        self.pair.profile['retained_field_entries'] = 0
        self.pair = None
        self.inverse = self.rhs0 = None
        self.profile['retained_field_entries'] = 0
        m.add('compact_frames_consumed')
        return result
