"""Root-only compact-diagonal pivot proposal with checked constructor binding.

Numerical/corruption validation pending. Caller certifies prime and Meter;
internal bindings must not be forged or mutated through reflection.
"""
from dataclasses import dataclass
from diagonal import build as build_diagonal
from reference_constructor import build as build_source
from recursive import Operator
from compression_certificate import verify
from metric_views import require_view
from selection_delete import select


def _freeze(matrix):
    if any(type(v) is not int for row in matrix for v in row):
        raise ValueError('integer generator entries required')
    return tuple(tuple(row) for row in matrix)


@dataclass
class RootSelection:
    source: object
    signature: tuple
    metric: tuple
    inverse_metric: tuple
    indices: tuple
    columns: tuple
    source_queries: int = 1
    zero_prefix: int = 0  # Empty proof set; no skipped column is claimed zero.
    first_column: object = None  # First nonzero column was not searched for.
    consumed: bool = False


class RootBinding:
    def __init__(self,source,diagonal,metric,inverse_metric):
        self.root=source
        self.meter=source.m
        self.signature=(len(source.G),source.a,source.b,source.m.r)
        self.left,self.right=_freeze(source.G),_freeze(source.K)
        self.diagonal=diagonal
        self.metric,self.inverse_metric=tuple(metric),tuple(inverse_metric)
        self.phase='fresh'
        self.trace=dict(kind='root_diagonal_proposal',phase='fresh',source_queries=0,
                        diagonal_tests=0,skipped_columns_certified_zero=False,
                        first_nonzero_column_known=False)
        source.m.add('root_binding_snapshot_entries',sum(map(len,source.G))+sum(map(len,source.K)))

    def _release(self):
        self.root=self.left=self.right=self.diagonal=None
        self.metric=self.inverse_metric=None

    def prepare(self,source,result):
        m=self.meter
        if (self.phase!='fresh' or source is not self.root or source.m is not m or
                type(source.a) is not int or type(source.b) is not int or
                (len(source.G),source.a,source.b,m.r)!=self.signature or
                _freeze(source.G)!=self.left or _freeze(source.K)!=self.right):
            raise ValueError('unchanged fresh constructor source required')
        m.add('root_binding_generator_entries_checked',sum(map(len,source.G))+sum(map(len,source.K)))
        self.phase='reserved'
        checked=verify(source.G,source.K,result,m.r)
        n,s=self.signature[0],checked['input_width']
        g,t=checked['first_rank'],checked['rank']
        self.trace.update(compression_identity_checked=True,
                          checker_expected_integer_products=2*n*g*(s+t),
                          checker_expected_modular_reductions=n*(s+2*g+t),
                          checker_runtime_counts_verified=False)
        m.add('root_binding_compression_certificates_checked')
        m.add('root_binding_expected_checker_integer_products',2*n*g*(s+t))
        m.add('root_binding_expected_checker_modular_reductions',n*(s+2*g+t))
        self.root=None
        if t==0:
            self.phase='consumed'
            self.trace['phase']='zero_source_terminal'
            self._release()
        else:
            self.left,self.right=_freeze(result['G']),_freeze(result['K'])
            m.add('root_binding_snapshot_entries',2*n*t)
            self.phase='ready'
            self.trace['phase']='compression_checked'

    def _candidate(self):
        packet,m=self.diagonal,self.meter
        # A-order exceptional indices 0..q-2, then the bulk, then r-1.
        for z in range(1,packet.window):
            value=m.sub(packet.background,packet.corrections[z])
            self.trace['diagonal_tests']+=1
            m.add('root_diagonal_candidate_tests')
            if value!=0:
                return z-1,value
        self.trace['diagonal_tests']+=1
        m.add('root_diagonal_candidate_tests')
        if packet.background!=0:
            return packet.window-1,packet.background
        value=m.sub(packet.background,packet.corrections[0])
        self.trace['diagonal_tests']+=1
        m.add('root_diagonal_candidate_tests')
        if value!=0:
            return packet.prime-1,value
        return None

    def choose(self,source,d,di,view):
        n,a,b,p=self.signature
        m=self.meter
        if (self.phase!='ready' or source.m is not m or m.r!=p or
                type(source.a) is not int or type(source.b) is not int or
                (len(source.G),source.a,source.b)!=(n,a,b) or
                _freeze(source.G)!=self.left or _freeze(source.K)!=self.right or
                tuple(d)!=self.metric or tuple(di)!=self.inverse_metric or
                view.labels!=tuple(range(n))):
            raise ValueError('exact compressed root and original metric coordinates required')
        require_view(view,d,di,m)
        m.add('root_binding_generator_entries_checked',sum(map(len,source.G))+sum(map(len,source.K)))
        self.phase='consumed'
        candidate=self._candidate()
        self._release()
        if candidate is None:
            self.trace['phase']='all_diagonals_zero_fallback'
            result=select(source,d,di,view)
            self.trace['source_queries']=result.source_queries
            return result
        index,b_diagonal=candidate
        expected=m.mul(di[index],b_diagonal)
        unit=[0]*n
        unit[index]=1
        column=tuple(source.apply(unit))
        m.add('root_diagonal_column_queries')
        if len(column)!=n or any(type(v) is not int or not 0<=v<p for v in column):
            raise ArithmeticError('malformed proposed pivot column')
        if expected==0 or column[index]!=expected:
            raise ArithmeticError('compact diagonal disagrees with source column')
        self.trace.update(phase='scalar_pivot',candidate_index=index,
                          source_queries=1,diagonal_entry_checked=True)
        return RootSelection(source,self.signature,tuple(d),tuple(di),(index,),(column,))


def build_bound(r,q,gamma,weights,meter,events):
    # Both constructors consume the same immutable fixed input values.
    fixed_weights=tuple(weights)
    packet=build_diagonal(r,q,gamma,fixed_weights,meter)
    raw=build_source(r,q,gamma,fixed_weights,meter)
    source=Operator(raw['G'],raw['K'],raw['left_wrap'],raw['right_wrap'],
                    meter,events,'public-feature-root')
    metric=[1]
    for i in range(1,r): metric.append(meter.mul(metric[-1],gamma))
    inverse=tuple(raw['inverse_diagonal'])
    metric=tuple(metric)
    binding=RootBinding(source,packet,metric,inverse)
    return dict(source=source,rhs=raw['rhs'],metric=metric,inverse_metric=inverse,
                root_binding=binding)
