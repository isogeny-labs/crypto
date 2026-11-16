"""Exact conditional sharing of row/column borders; derived, unexecuted."""
class BorderPair:
    __slots__ = ('column', 'row', 'scale', 'mode', 'size', 'profile')

    def __init__(self, row, column, scale, meter):
        q = len(column)
        if q == 0 or len(row) != q or type(scale) is not int or not 0 <= scale < meter.r:
            raise ValueError('nonempty equal tails and canonical scale required')
        if any(type(x) is not int or not 0 <= x < meter.r for v in (row, column) for x in v):
            raise ValueError('canonical border entries required')
        self.size = q
        self.profile = dict(relation_tests=0, relation_products=0,
                            dot_products=0, dot_additions=0, scale_products=0)
        # General scaling stores one additional field value. Skip if no gain.
        eligible = scale in (0, 1) or q > 1
        matched = eligible
        if eligible:
            for i in range(q):
                if scale == 0:
                    expected = 0
                elif scale == 1:
                    expected = column[q-1-i]
                else:
                    expected = meter.mul(scale, column[q-1-i])
                    self.profile['relation_products'] += 1
                meter.add('border_relation_entry_tests')
                self.profile['relation_tests'] += 1
                if row[i] != expected:
                    matched = False
                    break
        self.column = tuple(column)
        if not matched:
            self.mode, self.row, self.scale = 'plain', tuple(row), None
            fields = 2*q
        elif scale == 0:
            self.mode, self.row, self.scale = 'zero_row', None, None
            fields = q
        elif scale == 1:
            self.mode, self.row, self.scale = 'reverse', None, None
            fields = q
        else:
            self.mode, self.row, self.scale = 'scaled_reverse', None, scale
            fields = q+1
        self.profile.update(mode=self.mode, retained_field_entries=fields,
                            original_field_entries=2*q)
        meter.add('shared_border_copied_entries', q if self.row is None else 2*q)

    def dot(self, vector, dual, meter):
        if type(dual) is not bool or len(vector) != self.size or any(
                type(x) is not int or not 0 <= x < meter.r for x in vector):
            raise ValueError('canonical vector and boolean direction required')
        if not dual and self.mode == 'zero_row':
            return 0
        out = 0
        for i, entry in enumerate(vector):
            if dual:
                coefficient = self.column[i]
            elif self.mode == 'plain':
                coefficient = self.row[i]
            else:
                coefficient = self.column[self.size-1-i]
            out = meter.plus(out, meter.mul(coefficient, entry))
            self.profile['dot_products'] += 1
            self.profile['dot_additions'] += 1
        if not dual and self.mode == 'scaled_reverse':
            out = meter.mul(self.scale, out)
            self.profile['scale_products'] += 1
        return out
