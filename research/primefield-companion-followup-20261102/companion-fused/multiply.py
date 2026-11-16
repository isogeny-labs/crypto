"""Unexecuted companion product with deferred coefficient reductions.

Prime coefficient field, arbitrary positive dimension, distinct wraps.
All intermediate polynomial arithmetic uses nonnegative integer lifts.
"""
def apply(G,K,a,b,x,m,transpose=False,trace=None):
    r=m.r;n=len(x);assert r>3 and n>0 and len(G)==len(K)==n and G[0]
    assert type(a)is int and type(b)is int and 0<=a<r and 0<=b<r and a!=b
    s=len(G[0]);assert all(len(row)==s for row in list(G)+list(K))
    assert all(type(v)is int and 0<=v<r for row in list(G)+list(K) for v in row)
    assert all(type(v)is int and 0<=v<r for v in x)
    def mul(u,v,kind):
        z=u*v;m.add('integer_multiplications');m.add(kind+'_integer_multiplications')
        m.add('operand_bitlength_product_sum',u.bit_length()*v.bit_length())
        m.add('product_result_bitlength_sum',z.bit_length())
        m.blocks.append(dict(kind=kind,a_bits=u.bit_length(),b_bits=v.bit_length(),product_bits=z.bit_length()))
        return z
    def native(op,u,v):
        if op=='add':z=u+v;counter='integer_additions'
        elif op=='subtract':z=u-v;counter='integer_subtractions'
        elif op=='left_shift':z=u<<v;counter='integer_shifts'
        elif op=='right_shift':z=u>>v;counter='integer_shifts'
        elif op=='mask':z=u&v;counter='integer_masks'
        else:raise ValueError(op)
        m.add(counter)
        m.add(op+'_input_bitlength_sum',u.bit_length())
        if op not in ('left_shift','right_shift'):
            m.add(op+'_input_bitlength_sum',v.bit_length())
        else:m.add(op+'_distance_sum',v)
        m.add(op+'_result_bitlength_sum',z.bit_length())
        return z
    def emit(stage,**data):
        if trace is not None:trace(dict(stage=stage,**data))
    R=native('subtract',r,1)
    R2=mul(R,R,'bound');R4=mul(R2,R2,'bound');N2=mul(n,n,'bound')
    bound=mul(mul(mul(mul(s,N2,'bound'),R4,'bound'),native('add',a,1),'bound'),native('add',b,1),'bound')
    d=max(1,(bound.bit_length()+7)//8);bits=8*d;lowbits=(n-1)*bits;fullbits=n*bits
    lowmask=native('subtract',native('left_shift',1,lowbits),1)
    fullmask=native('subtract',native('left_shift',1,fullbits),1)
    assert (1<<bits)>bound
    inv=m.inv(m.sub(a,b));m.add('field_products',0)
    emit('setup',bound=bound,digit_bytes=d,inverse=inv)
    def pack(values):
        raw=b''.join(v.to_bytes(d,'little') for v in values)
        m.add('input_integer_encodings');m.add('packing_bytes',len(raw))
        return int.from_bytes(raw,'little')
    shared=pack(list(x) if transpose else list(x)[::-1])
    m.add('coordinate_reversal_slots',0 if transpose else n)
    acc=0;u=a if transpose else b;v=b if transpose else a
    for j in range(s):
        g=[row[j] for row in G];k=[row[j] for row in K]
        first=g[::-1] if transpose else k;second=k[::-1] if transpose else g
        if transpose:m.add('coordinate_reversal_slots',2*n)
        P=mul(pack(first),shared,'convolution')
        low=native('mask',P,lowmask)
        T=native('add',native('right_shift',P,lowbits),
                 native('left_shift',mul(u,low,'fold'),bits))
        assert T<=fullmask
        raw=T.to_bytes(n*d,'little');m.add('intermediate_serialization_bytes',len(raw))
        reversed_raw=b''.join(raw[i*d:(i+1)*d] for i in range(n-1,-1,-1))
        m.add('byte_reversal_input_bytes',len(raw));m.add('byte_reversal_output_bytes',len(reversed_raw))
        m.add('input_integer_encodings');m.add('packing_bytes',len(reversed_raw))
        Q=mul(pack(second),int.from_bytes(reversed_raw,'little'),'convolution')
        term=native('add',native('mask',Q,fullmask),
                    mul(v,native('right_shift',Q,fullbits),'fold'))
        acc=native('add',acc,term);assert acc<=fullmask
        emit('column',column=j,P=P,T=T,Q=Q,Y=term,acc=acc)
    result=mul(inv,acc,'scale');assert result<=fullmask
    emit('final',packed=result)
    raw=result.to_bytes(n*d,'little');m.add('output_serialization_bytes',len(raw))
    y=[int.from_bytes(raw[i*d:(i+1)*d],'little')%r for i in range(n)]
    m.add('output_integer_decodings',n);m.add('decoded_bytes',n*d);m.add('coefficient_reductions',n)
    if transpose:m.add('coordinate_reversal_slots',n);y.reverse()
    return y
