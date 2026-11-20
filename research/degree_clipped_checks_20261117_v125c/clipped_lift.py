from research.coprime_factor_checks_20261116_v119b.circuit import coefficient

def lift(F,d,C):
    assert [v[0]for v in F]==[0,0,1,1,1,1]
    A=[[0],[0],[1]];B=[[1],[1],[1],[1]];m=d-1
    for n in range(1,3*m+1):
        e=[C.xor(F[j][n]if n<len(F[j])else 0,coefficient(A,B,j,n,C))for j in range(5)]
        a1=C.xor(e[0],e[1]);a=[e[0],a1];b=[C.xor(e[1],e[2]),C.xor(e[1],e[3]),C.xor(a1,e[4])]
        for factor,degree,values in [(A,2,a),(B,3,b)]:
            for j,value in enumerate(values):
                if n<=(degree-j)*m:factor[j].append(value)
    return A,B,[]
