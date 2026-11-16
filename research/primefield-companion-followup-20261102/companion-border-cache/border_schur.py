"""Leading-border reuse for cut-one Schur reduction; derived, unexecuted.

The supplied first column comes from the immediate pivot search/transport.
It is trusted internal provenance, not a public certificate. Source generators
must remain unchanged during this object's lifetime.
"""
from recursive import SourceSchur


class BorderSchur(SourceSchur):
    def __init__(self, source, inverse, first_column):
        n = len(source.G)
        if n < 2 or len(first_column) != n:
            raise ValueError('nonterminal scalar Schur and complete column required')
        if (any(type(x) is not int or not 0 <= x < source.m.r for x in first_column)
                or first_column[0] == 0):
            raise ValueError('canonical column with nonzero scalar pivot required')
        super().__init__(source, 1, inverse)
        self.dimension = n
        self.first_column = tuple(first_column)
        self.profile = dict(build_queries=0, forward_requests=0, transpose_requests=0,
                            forward_hits=0, transpose_hits=0, forward_queries=0,
                            transpose_queries=0, scaling_products=0,
                            support_entry_tests=0, retained_border_entries=2*n,
                            peak_border_entries=2*n, zero_after_release=0,
                            released=False)
        unit = [1] + [0] * (n-1)
        self.first_row = tuple(self._full_source(unit, True))
        self.profile['build_queries'] = 1
        self.m.add('border_build_queries')
        if (len(self.first_row) != n or
                any(type(x) is not int or not 0 <= x < self.m.r for x in self.first_row)
                or self.first_row[0] != self.first_column[0]):
            raise ArithmeticError('cached border corner mismatch or malformed row')

    def release_borders(self):
        """Release border vectors after generator setup and RHS reduction."""
        if self.profile['released']:
            raise ValueError('borders already released')
        self.first_column = None
        self.first_row = None
        self.profile['released'] = True
        self.profile['retained_border_entries'] = 0

    def _full_source(self, x, transpose):
        direction = 'transpose' if transpose else 'forward'
        self.profile[direction + '_queries'] += 1
        self.m.add('source_' + direction + '_calls')
        return self.source.apply(x, transpose)

    def _source(self, x, transpose=False):
        if (len(x) != self.dimension or
                any(type(v) is not int or not 0 <= v < self.m.r for v in x)):
            raise ValueError('canonical source argument required')
        direction = 'transpose' if transpose else 'forward'
        self.profile[direction + '_requests'] += 1
        self.m.add('border_' + direction + '_requests')
        for value in x[1:]:
            self.profile['support_entry_tests'] += 1
            self.m.add('border_support_entry_tests')
            if value != 0:
                return self._full_source(x, transpose)
        if self.profile['released']:
            if x[0] != 0:
                # Preserve the full operator interface after cache release.
                return self._full_source(x, transpose)
            self.profile['zero_after_release'] += 1
            self.m.add('border_zero_after_release')
            return [0] * self.dimension
        self.profile[direction + '_hits'] += 1
        self.profile['scaling_products'] += self.dimension
        self.m.add('border_' + direction + '_hits')
        border = self.first_row if transpose else self.first_column
        # Full length is charged even when x[0] is zero or one.
        return [self.m.mul(x[0], value) for value in border]
