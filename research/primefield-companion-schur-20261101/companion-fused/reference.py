"""Unexecuted independent coefficient-list reference for frozen carry checks.

No packed extraction, masking or chunk reversal. Mathematical integer lists
are evaluated at beta only to compare against the candidate trace.
"""
def convolution(a,b):
    out=[0]*(len(a)+len(b)-1)
    for i,u in enumerate(a):
        for j,v in enumerate(b):out[i+j]+=u*v
    return out

def reference(G,K,a,b,x,r,transpose=False):
    n=len(x);s=len(G[0]);R=r-1
    bound=s*n*n*R**4*(1+a)*(1+b)
    d=max(1,(bound.bit_length()+7)//8);beta=256**d
    inv=pow((a-b)%r,-1,r)
    def packed(coefficients):
        assert all(0<=v<=bound<beta for v in coefficients)
        return sum(v*beta**i for i,v in enumerate(coefficients))
    trace=[dict(stage='setup',bound=bound,digit_bytes=d,inverse=inv)]
    shared=list(x) if transpose else list(reversed(x))
    acc=[0]*n;maxima=dict(P=0,T=0,Q=0,Y=0,acc=0,final=0)
    u,v=(a,b) if transpose else (b,a)
    for j in range(s):
        g=[row[j] for row in G];k=[row[j] for row in K]
        first=list(reversed(g)) if transpose else k
        second=list(reversed(k)) if transpose else g
        P=convolution(first,shared)
        T=[P[n-1+i]+(u*P[i-1] if i else 0) for i in range(n)]
        Q=convolution(second,list(reversed(T)))
        Y=[Q[i]+(v*Q[n+i] if n+i<len(Q) else 0) for i in range(n)]
        acc=[z+w for z,w in zip(acc,Y)]
        arrays=dict(P=P,T=T,Q=Q,Y=Y,acc=acc)
        for name,values in arrays.items():maxima[name]=max(maxima[name],max(values))
        trace.append(dict(stage='column',column=j,**{name:packed(values) for name,values in arrays.items()}))
    final=[inv*z for z in acc];maxima['final']=max(final)
    trace.append(dict(stage='final',packed=packed(final)))
    y=[v%r for v in final]
    if transpose:y.reverse()
    return y,trace,maxima


def expected_fixed_counts(r,n,s,a,b,transpose=False):
    d=max(1,((s*n*n*(r-1)**4*(a+1)*(b+1)).bit_length()+7)//8)
    return dict(field_products=0,field_additions=1,field_inversions=1,
        coefficient_reductions=n,integer_multiplications=4*s+8,
        bound_integer_multiplications=7,convolution_integer_multiplications=2*s,
        fold_integer_multiplications=2*s,scale_integer_multiplications=1,
        input_integer_encodings=3*s+1,packing_bytes=(3*s+1)*n*d,
        intermediate_serialization_bytes=s*n*d,output_serialization_bytes=n*d,
        byte_reversal_input_bytes=s*n*d,byte_reversal_output_bytes=s*n*d,
        output_integer_decodings=n,decoded_bytes=n*d,
        integer_shifts=3*s+2,integer_masks=2*s,integer_additions=3*s+2,
        integer_subtractions=3,coordinate_reversal_slots=(2*s+1)*n if transpose else n)
