"""Immutable inverse-metric provenance; not a global symmetry certificate."""
from dataclasses import dataclass
from metric import validate_metric


@dataclass(frozen=True, init=False)
class MetricRoot:
    d: tuple
    di: tuple
    prime: int
    meter: object

    def __init__(self, d, di, meter):
        d,di=tuple(d),tuple(di)
        validate_metric(d,di,meter)
        object.__setattr__(self,'d',d)
        object.__setattr__(self,'di',di)
        object.__setattr__(self,'prime',meter.r)
        object.__setattr__(self,'meter',meter)
        meter.add('metric_roots_validated')

    def view(self):
        return MetricView(self,tuple(range(len(self.d))))


@dataclass(frozen=True, init=False)
class MetricView:
    root: MetricRoot
    labels: tuple

    def __init__(self, root, labels):
        if type(root) is not MetricRoot or root.meter.r!=root.prime:
            raise ValueError('validated root in unchanged field required')
        labels=tuple(labels)
        if not labels:
            raise ValueError('nonempty metric view required')
        for i in labels:
            root.meter.add('metric_view_label_tests')
            if type(i) is not int or not 0<=i<len(root.d):
                raise ValueError('root coordinate out of range')
        if len(set(labels))!=len(labels):
            raise ValueError('duplicate metric coordinates')
        object.__setattr__(self,'root',root)
        object.__setattr__(self,'labels',labels)
        root.meter.add('metric_views_created')

    def permuted(self, order):
        order=tuple(order)
        n=len(self.labels)
        if len(order)!=n:
            raise ValueError('complete permutation required')
        for i in order:
            self.root.meter.add('metric_view_permutation_tests')
            if type(i) is not int or not 0<=i<n:
                raise ValueError('permutation index out of range')
        if len(set(order))!=n:
            raise ValueError('duplicate permutation index')
        return MetricView(self.root,tuple(self.labels[i] for i in order))

    def tail(self, cut):
        if type(cut) is not int or not 0<=cut<len(self.labels):
            raise ValueError('nonempty trailing view required')
        return MetricView(self.root,self.labels[cut:])


def require_view(view, d, di, meter):
    if (type(view) is not MetricView or type(view.root) is not MetricRoot or
            view.root.meter is not meter or view.root.prime!=meter.r):
        raise ValueError('same-meter inverse-metric view required')
    if len(d)!=len(view.labels) or len(di)!=len(view.labels):
        raise ValueError('metric view dimensions differ')
    for j,i in enumerate(view.labels):
        meter.add('metric_view_coordinate_pairs_checked')
        if (type(d[j]) is not int or type(di[j]) is not int or
                d[j]!=view.root.d[i] or di[j]!=view.root.di[i]):
            raise ValueError('metric values differ from inherited view')
    meter.add('metric_inverse_products_reused',len(view.labels))
