from research.residue_quotient_20261116_v99b.kernel import pmul

def solve(a,b,m):
    assert m>=1 and 0<=a<(1<<m)and 0<=b<(1<<(2*m-1))
    if not a:
        if any(b>>j&1 for j in range(1,2*m-1,2)):return {'roots':[],'complete':True}
        root=sum(((b>>(2*j))&1)<<j for j in range(m));assert pmul(root,root)==b
        return {'roots':[(root,2)],'complete':True}
    k=a.bit_length()-1;residual=b;root=0
    for j in list(range(m-1,k,-1))+list(range(k-1,-1,-1)):
        degree=2*j if j>k else k+j
        if residual>>degree&1:root^=1<<j;residual^=(1<<(2*j))^(a<<j)
    assert not(root>>k&1)
    if residual:return {'roots':[],'complete':True}
    other=root^a;assert root<other and other<(1<<m)
    assert pmul(root,root)^pmul(a,root)^b==0 and pmul(other,other)^pmul(a,other)^b==0
    return {'roots':[(root,1),(other,1)],'complete':True}
