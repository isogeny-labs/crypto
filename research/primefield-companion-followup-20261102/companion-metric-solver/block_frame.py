"""Compact weighted block lifts; no source or Schur object retained."""
from transport import undo_vector


class BlockFrame:
    __slots__ = ('columns','rhs_head','inverse','root_metric','root_inverse_metric',
                 'labels','order','meter','cut','size','consumed','profile')

    def __init__(self, columns, rhs, inverse, root_metric, root_inverse_metric,
                 labels, order, meter):
        self.cut, self.size = len(columns), len(rhs)
        if self.cut not in (1,2) or self.cut >= self.size or inverse.size != self.cut:
            raise ValueError('nonterminal scalar/pair frame required')
        if len(labels) != self.size or len(order) != self.size:
            raise ValueError('complete metric labels and permutation required')
        if any(len(c) != self.size for c in columns):
            raise ValueError('complete leading columns required')
        self.columns = tuple(tuple(c[self.cut:]) for c in columns)
        self.rhs_head = tuple(rhs[:self.cut])
        self.inverse = inverse
        # These are the same two immutable root tuples in all frames.
        self.root_metric, self.root_inverse_metric = root_metric, root_inverse_metric
        self.labels, self.order = tuple(labels), tuple(order)
        self.meter = meter
        self.consumed = False
        self.profile = dict(cut=self.cut, dimension=self.size,
                            retained_field_entries=self.cut*(self.size-self.cut)+2*self.cut,
                            retained_index_entries=2*self.size, dot_products=0,
                            dot_additions=0, metric_products=0, source_queries=0,
                            consumed=False)
        meter.add('metric_frame_border_copy_entries',self.cut*(self.size-self.cut))

    def lift(self, tail, dual=False):
        m, k, q = self.meter, self.cut, self.size-self.cut
        if self.consumed or type(dual) is not bool or len(tail)!=q or any(
                type(v) is not int or not 0 <= v < m.r for v in tail):
            raise ValueError('unused frame and canonical complete tail required')
        self.consumed = True
        self.profile['consumed'] = True
        if dual:
            weighted = tail
        else:
            weighted = [m.mul(self.root_metric[self.labels[k+i]], tail[i]) for i in range(q)]
            self.profile['metric_products'] += q
        correction = []
        for j, column in enumerate(self.columns):
            value = 0
            for coefficient, entry in zip(column, weighted):
                value = m.plus(value, m.mul(coefficient, entry))
            if not dual:
                value = m.mul(self.root_inverse_metric[self.labels[j]], value)
                self.profile['metric_products'] += 1
            correction.append(value)
        self.profile['dot_products'] += k*q
        self.profile['dot_additions'] += k*q
        m.add('dual_lift_calls' if dual else 'primal_lift_calls')
        m.add('leading_transpose_calls' if dual else 'leading_forward_calls')
        if dual:
            head = [m.sub(0,x) for x in self.inverse.apply(correction,True)]
        else:
            head = self.inverse.apply([m.sub(x,y) for x,y in zip(self.rhs_head,correction)])
        out = undo_vector(head+list(tail),self.order)
        self.columns = self.rhs_head = self.inverse = None
        self.root_metric = self.root_inverse_metric = self.labels = self.order = None
        self.profile['retained_field_entries'] = self.profile['retained_index_entries'] = 0
        return out
