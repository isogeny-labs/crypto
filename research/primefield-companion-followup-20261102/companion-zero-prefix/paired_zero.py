"""Combined simultaneous swap using selected columns; derived, unexecuted.

Internal cache provenance and validated inverse metric come from selection.
No source mutation, forged selection, or independently certified cache is assumed.
"""
from recursive import Operator
from pivot import permute
from transport import SmallInverse
from selection_zero import Selection


def _swap(source, i, j, d, di, cache, zeros):
    n, m = len(source.G), source.m
    if i == j or not (0 <= i < n and 0 <= j < n):
        raise ValueError('nontrivial in-range swap required')
    missing = 0
    reused = 0
    for index in (i,j):
        if index not in cache:
            m.add('zero_prefix_membership_tests')
            if index in zeros:
                cache[index]=(0,)*n
                reused+=1
                m.add('zero_prefix_columns_reused')
                m.add('zero_prefix_materialized_entries',n)
            else:
                unit=[0]*n
                unit[index]=1
                cache[index]=tuple(source.apply(unit))
                missing+=1
                m.add('paired_transport_missing_column_queries')
    ci,cj=cache[i],cache[j]
    if any(len(c)!=n or any(type(v) is not int or not 0<=v<m.r for v in c)
           for c in (ci,cj)):
        raise ValueError('canonical cached columns required')
    u=[0]*n
    u[i],u[j]=1,m.sub(0,1)
    zau=[m.mul(source.a,u[-1])]+u[:-1]
    zatu=u[1:]+[m.mul(source.a,u[0])]
    zbu=[m.mul(source.b,u[-1])]+u[:-1]
    zbtu=u[1:]+[m.mul(source.b,u[0])]
    au=[m.sub(x,y) for x,y in zip(ci,cj)]
    atu=[m.mul(d[k],m.sub(m.mul(di[i],ci[k]),m.mul(di[j],cj[k]))) for k in range(n)]
    atzatu=source.apply(zatu,True)
    azbu=source.apply(zbu)
    pg,pk=permute(source.G,i,j),permute(source.K,i,j)
    pau,pazbu=permute(au,i,j),permute(azbu,i,j)
    patu,patzatu=permute(atu,i,j),permute(atzatu,i,j)
    G=[list(pg[k])+[u[k],m.sub(0,zau[k]),pau[k],m.sub(0,pazbu[k])] for k in range(n)]
    K=[list(pk[k])+[patzatu[k],patu[k],zbtu[k],u[k]] for k in range(n)]
    target=Operator(G,K,source.a,source.b,m,source.events,
                    source.path+'/paired-swap-'+str(i)+'-'+str(j))
    def swap_index(key):
        return j if key==i else i if key==j else key
    updated={swap_index(key):tuple(permute(c,i,j)) for key,c in cache.items()}
    zeros={swap_index(key) for key in zeros}
    m.add('zero_prefix_indices_transported',len(zeros))
    receipt=dict(zero_columns_reused=reused,zero_prefix_size=len(zeros),kind='paired_swap',rows_and_columns=[i,j],missing_columns=missing,
                 source_queries=2+missing,transpose_queries=1,forward_queries=1+missing,
                 input_width=len(source.G[0]),output_width=len(G[0]),
                 field_products_outside_queries=3*n+4,
                 field_additions_outside_queries=4*n+1,
                 retained_cache_columns=len(updated),cache_independently_verified=False)
    return target,permute(d,i,j),permute(di,i,j),updated,zeros,receipt


def move(source, selection):
    n,m=len(source.G),source.m
    if (type(selection) is not Selection or selection.source is not source or selection.consumed or
            selection.signature!=(n,source.a,source.b,m.r)):
        raise ValueError('wrong, changed or consumed selection')
    if not selection.indices:
        raise ValueError('zero-source selection has no pivot to move')
    prefix=selection.zero_prefix
    if (type(prefix) is not int or not 0<=prefix<n or
            any(type(i) is not int or not prefix<=i<n for i in selection.indices)):
        raise ValueError('invalid zero-prefix provenance')
    # Internal producer provenance, not an independent certificate of zero columns.
    zeros=set(range(prefix))
    m.add('zero_prefix_indices_retained',prefix)
    selection.consumed=True
    order=list(range(n))
    moved=source
    d,di=list(selection.metric),list(selection.inverse_metric)
    cache=dict(zip(selection.indices,selection.columns))
    receipts=[]
    for front,original_index in enumerate(selection.indices):
        position=order.index(original_index)
        if position==front:
            continue
        # Selected columns remain in the transformed cache through every swap.
        if position not in cache:
            raise ArithmeticError('selected pivot column lost from cache')
        moved,d,di,cache,zeros,receipt=_swap(moved,front,position,d,di,cache,zeros)
        if receipt['missing_columns']>1:
            raise ArithmeticError('selected-column provenance violated')
        order[front],order[position]=order[position],order[front]
        receipts.append(receipt)
    columns=tuple(tuple(c[i] for i in order) for c in selection.columns)
    k=len(columns)
    block=tuple(tuple(columns[j][i] for j in range(k)) for i in range(k))
    inverse=SmallInverse(block,m)
    return dict(source=moved,metric=tuple(d),inverse_metric=tuple(di),
                order=tuple(order),cut=k,columns=columns,inverse=inverse,
                trace=receipts,transport_source_queries=sum(x['source_queries'] for x in receipts),
                output_width=len(moved.G[0]))
