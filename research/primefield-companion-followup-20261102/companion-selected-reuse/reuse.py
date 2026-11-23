"""Restrict selected columns and certify compression before rebinding.

Internal deletion/selection provenance and weighted symmetry remain required.
The independent compression check certifies only the displacement equality.
Numerical and corruption validation of this implementation is pending.
"""
from recursive import Operator
from compress import compress, nonempty_generators
from compression_certificate import verify
from selection_delete import Selection
from metric_views import require_view


def restrict_and_compress(deletion, selection, view):
    if type(selection) is not Selection or not selection.consumed:
        raise ValueError('consumed internal deletion selection required')
    if deletion['kind']!='reduced' or deletion.get('selection_rebound',False):
        raise ValueError('successful proper deletion required')
    source=deletion['source']
    old=selection.source
    m=source.m
    n=len(old.G)
    j=selection.zero_prefix
    q=len(source.G)
    if (type(j) is not int or not 0<j<n or q!=n-j or old.m is not m or
            (source.a,source.b)!=(old.a,old.b) or
            selection.signature!=(n,old.a,old.b,m.r)):
        raise ValueError('matching unchanged deletion source required')
    k=len(selection.indices)
    if k not in (1,2) or len(selection.columns)!=k or any(
            type(i) is not int or not j<=i<n for i in selection.indices):
        raise ValueError('retained scalar/pair pivot required')
    if len(set(selection.indices))!=k:
        raise ValueError('distinct selected indices required')
    for column in selection.columns+(selection.first_column,):
        if len(column)!=n or any(type(v) is not int or not 0<=v<m.r for v in column):
            raise ValueError('canonical complete cached column required')
        if any(column[:j]):
            raise ValueError('cached column contradicts deleted zero rows')
        m.add('selected_reuse_column_entries_checked',n)
    if not any(selection.first_column[j:]):
        raise ValueError('retained first column must remain nonzero')
    d,di=selection.metric[j:],selection.inverse_metric[j:]
    require_view(view,d,di,m)
    deletion['selection_rebound']=True
    indices=tuple(i-j for i in selection.indices)
    columns=tuple(c[j:] for c in selection.columns)
    first=selection.first_column[j:]
    m.add('selected_reuse_column_entries_copied',(k+1)*q)
    result=compress(source.G,source.K,m)
    checked=verify(source.G,source.K,result,m.r)
    if checked['rank']==0:
        raise ArithmeticError('zero compressed source contradicts retained nonzero column')
    G,K=nonempty_generators(result)
    target=Operator(G,K,source.a,source.b,m,source.events,source.path+'/compressed')
    rebound=Selection(target,(q,source.a,source.b,m.r),tuple(d),tuple(di),
                      indices,columns,0,0,first)
    s=len(source.G[0]);g=checked['first_rank'];t=checked['rank']
    # Derived logical checker schedule; plain checker arithmetic is not Meter work.
    expected_products=2*q*g*(s+t)
    expected_reductions=q*(s+2*g+t)
    m.add('selected_reuse_compression_certificates_checked')
    m.add('selected_reuse_expected_checker_integer_products',expected_products)
    m.add('selected_reuse_expected_checker_modular_reductions',expected_reductions)
    # Same deterministic reduced selector: first diagonal uses one query,
    # secondary diagonal / pair branches use two.
    saved_queries=1 if first[0]!=0 else 2
    m.add('selected_reuse_source_queries_avoided',saved_queries)
    return dict(source=target,selection=rebound,certificate=result['certificate'],
        trace=dict(kind='selected_columns_rebound',dimension=q,pivot_indices=indices,
                   source_queries=0,avoided_selector_queries=saved_queries,
                   compressed_width=t,compression_identity_checked=True,
                   deletion_independently_checked=False,checker=checked,
                   checker_expected_integer_products=expected_products,
                   checker_expected_modular_reductions=expected_reductions,
                   checker_runtime_counts_verified=False))
