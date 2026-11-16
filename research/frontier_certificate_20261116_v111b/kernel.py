from research.residue_quotient_20261116_v99b.kernel import basis,preimage,apply

def solve(rows,n):
    cols=[sum(((mask>>j)&1)<<i for i,(mask,rhs)in enumerate(rows))for j in range(n)]
    target=sum(rhs<<i for i,(mask,rhs)in enumerate(rows));piv,K=basis(cols)
    try:particular=preimage(target,piv)
    except ValueError:return {'feasible':False,'rank':len(piv),'rows':rows}
    assert all((mask&particular).bit_count()%2==rhs for mask,rhs in rows)
    assert all(not((mask&k).bit_count()%2)for mask,rhs in rows for k in K)
    return {'feasible':True,'rank':len(piv),'dimension':len(K),'particular':particular,'columns':K,'rows':rows}

def guarded_charts(rows,n,u,v,w,q1,q2):
    # flag tuples are (linear-mask, constant), evaluated constant XOR parity.
    mu,bu=u;mv,bv=v;mw,bw=w;m1,b1=q1;m2,b2=q2
    common=rows+[(mu^mv,bu^bv),(m2^mw,b2^bw)]
    return [solve(common+[(mw,bw),(m1,b1)],n),
            solve(common+[(mw,bw^1),(mu,bu)],n)]
