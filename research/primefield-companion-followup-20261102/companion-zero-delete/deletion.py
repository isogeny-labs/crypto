"""Conditional zero-prefix deletion. Derived identity; numerically unexecuted.

Internal selection provenance and caller-established weighted symmetry required.
No independent source/cache certificate is provided by these entry guards.
"""
from recursive import Operator
from selection_delete import Selection


def delete_prefix(source, selection, rhs):
    n,m=len(source.G),source.m
    if (type(selection) is not Selection or selection.source is not source or
            selection.consumed or selection.signature!=(n,source.a,source.b,m.r)):
        raise ValueError('unchanged source and unused internal selection required')
    j=selection.zero_prefix
    if type(j) is not int or not 0<j<n or not selection.indices:
        raise ValueError('proper nonempty zero prefix required')
    c=selection.first_column
    if len(c)!=n or any(type(v) is not int or not 0<=v<m.r for v in c):
        raise ValueError('canonical first nonzero column required')
    if any(c[:j]) or not any(c[j:]):
        raise ValueError('first column contradicts zero-block provenance')
    if len(rhs)!=n or any(type(v) is not int or not 0<=v<m.r for v in rhs):
        raise ValueError('canonical full RHS required')
    selection.consumed=True
    for t in range(j):
        m.add('zero_delete_rhs_tests')
        if rhs[t]!=0:
            vector=[0]*n
            vector[t]=m.inv(rhs[t])
            return dict(kind='exclusion_witness',vector=vector,
                trace=dict(kind='zero_prefix_witness',dimension=n,zero_prefix=j,
                           witness_index=t,search_queries=selection.source_queries,
                           source_queries=0,inversions=1,
                           remaining_rank_known=False))
    q=n-j
    unit=[0]*n
    unit[-1]=1
    last=source.apply(unit,True)
    if len(last)!=n or any(type(v) is not int or not 0<=v<m.r for v in last):
        raise ArithmeticError('malformed last source row')
    if any(last[:j]):
        raise ArithmeticError('last row contradicts zero-column provenance')
    neg_b=m.sub(0,source.b)
    G=[list(source.G[j+i])+[source.a if i==0 else 0,m.mul(neg_b,c[j+i])]
       for i in range(q)]
    K=[list(source.K[j+i])+[last[j+i],1 if i==q-1 else 0] for i in range(q)]
    m.add('zero_delete_generator_entries_copied',2*q*len(source.G[0]))
    m.add('zero_delete_generator_entries_appended',4*q)
    target=Operator(G,K,source.a,source.b,m,source.events,source.path+'/zero-delete')
    return dict(kind='reduced',source=target,
        trace=dict(kind='zero_prefix_delete',dimension=n,zero_prefix=j,
                   target_dimension=q,search_queries=selection.source_queries,
                   input_width=len(source.G[0]),
                   output_width=len(G[0]),source_queries=1,transpose_queries=1,
                   field_products_outside_queries=q,field_subtractions_outside_queries=1,
                   source_symmetry_assumed=True,cache_independently_verified=False))


class ZeroFrame:
    """Reinsert free zero coordinates, with no source or field array retained."""
    __slots__=('size','cut','meter','consumed')

    def __init__(self,size,cut,meter):
        if type(size) is not int or type(cut) is not int or not 0<cut<size:
            raise ValueError('proper nonempty deleted prefix required')
        self.size,self.cut,self.meter=size,cut,meter
        self.consumed=False
        meter.add('zero_delete_frames_created')

    def lift(self,tail,dual=False):
        m=self.meter
        if self.consumed or type(dual) is not bool or len(tail)!=self.size-self.cut or any(
                type(v) is not int or not 0<=v<m.r for v in tail):
            raise ValueError('unused frame and canonical trailing vector required')
        self.consumed=True
        m.add('zero_delete_reinserted_entries',self.cut)
        m.add('zero_delete_tail_copy_entries',len(tail))
        return [0]*self.cut+list(tail)
