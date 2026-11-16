"""Leading block cache using compiler-derived weighted symmetry; unexecuted."""
from recursive import SourceSchur
from rows_view import transpose_from_columns


class BlockCache(SourceSchur):
    def __init__(self, source, cut, inverse, columns, d, di, view):
        super().__init__(source, cut, inverse)
        self.n = len(source.G)
        if len(columns) != cut or any(len(c) != self.n for c in columns):
            raise ValueError('complete leading column block required')
        self.columns = tuple(tuple(c) for c in columns)
        self.rows = transpose_from_columns(self.columns, d, di, self.m, view)
        self.profile = dict(cut=cut, dimension=self.n, cache_calls=0,
                            cache_products=0, cache_additions=0, source_queries=0,
                            support_tests=0, retained_border_entries=2*self.n*cut,
                            row_setup_products=2*self.n*cut, released=False)

    def _source(self, vector, transpose=False):
        if self.profile['released']:
            raise ValueError('block cache already released')
        if len(vector) != self.n or type(transpose) is not bool or any(
                type(x) is not int or not 0 <= x < self.m.r for x in vector):
            raise ValueError('canonical complete source vector required')
        for x in vector[self.cut:]:
            self.profile['support_tests'] += 1
            self.m.add('metric_cache_support_tests')
            if x != 0:
                self.profile['source_queries'] += 1
                return super()._source(vector, transpose)
        border = self.rows if transpose else self.columns
        out = []
        for i in range(self.n):
            value = 0
            for j in range(self.cut):
                value = self.m.plus(value, self.m.mul(border[j][i], vector[j]))
            out.append(value)
        products = self.n*self.cut
        self.profile['cache_calls'] += 1
        self.profile['cache_products'] += products
        self.profile['cache_additions'] += products
        self.m.add('metric_cache_calls')
        return out

    def release(self):
        if self.profile['released']:
            raise ValueError('block cache already released')
        self.columns = self.rows = None
        self.profile['released'] = True
        self.profile['retained_border_entries'] = 0
