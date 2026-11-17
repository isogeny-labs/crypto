"""Complete conditional weighted-symmetric solve/witness driver; unexecuted.

Caller establishes D A = A^T D over a prime >3. Inverse-metric validation
and local reciprocal checks do not establish this global invariant. Caller
provides sibling module paths as listed in DERIVATION.md; no solve oracle.
"""
from recursive import Operator
from compress import compress, nonempty_generators
from metric_views import MetricRoot
from selection_view import select
from transport import permute_rhs, undo_vector
from paired import move
from lift import reduced_rhs
from block_cache_view import BlockCache
from block_frame import BlockFrame


def solve_or_witness(source, rhs, metric, inverse_metric):
    if not __debug__:
        raise ValueError('inherited numerical assertions must remain enabled')
    n, meter = len(source.G), source.m
    if (n == 0 or meter.r <= 3 or source.a == source.b or
            not (0 <= source.a < meter.r and 0 <= source.b < meter.r)):
        raise ValueError('square distinct-wrap source over caller-certified prime > 3 required')
    if len(rhs) != n or any(type(x) is not int or not 0 <= x < meter.r for x in rhs):
        raise ValueError('canonical RHS required')
    if len(metric)!=n or len(inverse_metric)!=n:
        raise ValueError('complete source metric required')
    metric_root=MetricRoot(metric,inverse_metric,meter)
    root_metric,root_inverse_metric=metric_root.d,metric_root.di
    view=metric_root.view()
    labels=tuple(range(n))
    original, original_rhs = source, list(rhs)
    b = list(rhs)
    frames, trace = [], []
    pivots = 0
    while True:
        size = len(source.G)
        reduced = compress(source.G, source.K, meter)
        trace.append(dict(kind='compression', dimension=size,
                          certificate=reduced['certificate']))
        rank = reduced['certificate']['rank']
        if rank == 0:
            index = None
            for i, value in enumerate(b):
                meter.add('rank_solver_rhs_entry_tests')
                if value != 0:
                    index = i
                    break
            vector = [0] * size
            if index is None:
                kind = 'solution'
            else:
                kind = 'exclusion_witness'
                vector[index] = meter.inv(b[index])
            trace.append(dict(kind='zero_source_terminal', dimension=size,
                              rhs_nonzero_index=index))
            break
        G, K = nonempty_generators(reduced)
        source = Operator(G, K, source.a, source.b, meter, source.events,
                          source.path + '/compressed')
        d=tuple(root_metric[i] for i in labels)
        di=tuple(root_inverse_metric[i] for i in labels)
        selection=select(source,d,di,view)
        if not selection.indices:
            raise ArithmeticError('nonzero displacement but all source columns zero')
        moved=move(source,selection)
        order,k=moved['order'],moved['cut']
        labels=tuple(labels[i] for i in order)
        view=view.permuted(order)
        b=permute_rhs(b,order)
        pivots+=k
        trace.append(dict(kind='metric_pivot',dimension=size,cut=k,
                          indices=selection.indices,order=order,
                          search_queries=selection.source_queries,
                          transport=moved['trace'],transport_source_queries=moved['transport_source_queries'],
                          transported_width=moved['output_width']))
        if k==size:
            vector=undo_vector(moved['inverse'].apply(b),order)
            kind='solution'
            break
        schur=BlockCache(moved['source'],k,moved['inverse'],moved['columns'],
                         moved['metric'],moved['inverse_metric'],view)
        trace.append(dict(kind='block_cache',dimension=size,profile=schur.profile))
        G,K=schur.generators(2,1)
        h=reduced_rhs(schur,b)
        frame=BlockFrame(moved['columns'],b,moved['inverse'],root_metric,
                         root_inverse_metric,labels,order,meter)
        trace.append(dict(kind='block_frame',dimension=size,profile=frame.profile))
        frames.append(frame)
        schur.release()
        source=Operator(G,K,2,1,meter,moved['source'].events,
                        moved['source'].path+'/metric-schur')
        b=h
        labels=labels[k:]
        view=view.tail(k)
        # Do not keep parent source/cache/selection references across descent.
        del schur,moved,selection,frame,d,di
    for frame in reversed(frames):
        vector=frame.lift(vector,dual=(kind!='solution'))
    if kind == 'solution':
        product = original.apply(vector)
        meter.add('rank_solver_output_entry_tests', n)
        if product != original_rhs:
            raise ArithmeticError('root solution check failed')
    else:
        product = original.apply(vector, True)
        pairing = meter.dot(vector, original_rhs)
        meter.add('rank_solver_output_entry_tests', n+1)
        if any(product) or pairing != 1:
            raise ArithmeticError('root normalized witness check failed')
    return dict(kind=kind, vector=vector, root_equation_checked=True,
                derived_rank=pivots, rank_independently_checked=False,
                trace=trace, feature_annihilation_checked=False,
                weighted_symmetry_assumed=True,shared_metric_field_entries=2*n)
