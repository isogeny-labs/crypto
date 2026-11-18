from research.residue_quotient_20261116_v99b.kernel import rem
from research.local_split_circuit_20261116_v100.kernel import BF

def reduce_factor(factor,d,s,modulus,C):
    degree=len(factor)-1;out=[]
    for j,coefficient in enumerate(factor):
        bound=(degree-j)*(d-1);bits=[0]*s
        for n,wire in enumerate(coefficient[:bound+1]):
            image=rem(1<<(bound-n),modulus)
            for bit in range(s):
                if image>>bit&1:bits[bit]=C.xor(bits[bit],wire)
        out.append(bits)
    assert out[-1]==[1]+[0]*(s-1);return out

def square_mod(v,H,f):
    degree=len(H)-1;out=[f.const(0)for _ in range(2*degree-1)]
    for i,a in enumerate(v):out[2*i]=f.square(a)
    for k in range(2*degree-2,degree-1,-1):
        c=out[k]
        for j in range(degree):out[k-degree+j]=f.add(out[k-degree+j],f.mul(c,H[j]))
    return out[:degree]

def split_output(H,f):
    degree=len(H)-1;assert degree in [2,3];v=[f.const(0),f.const(1)]+[f.const(0)for _ in range(degree-2)]
    for _ in range(f.s):v=square_mod(v,H,f)
    v[1]=f.add(v[1],f.const(1))
    for _ in range((degree-1).bit_length()):v=square_mod(v,H,f)
    return v

def append_support(C,outputs,A,B,d,fields):
    result=list(outputs)
    for s,modulus in fields:
        f=BF(s,modulus,C)
        for label,factor in [('A',A),('B',B)]:
            H=reduce_factor(factor,d,s,modulus,C);result.append({'factor':label,'s':s,'m':modulus,'wires':split_output(H,f)})
    return C,result
