# Generic Semaev summation polynomials S3,S4,S5 over Z[a,b,x...] via fmpz_mpoly resultants.
import flint, pickle, os, random
names = ('a','b','x1','x2','x3','x4','x5','u')
ctx = flint.fmpz_mpoly_ctx.get(names, 'lex')
a,b,x1,x2,x3,x4,x5,u = ctx.gens()
def S3(p,q,r):
    return (p-q)**2*r**2 - 2*((p+q)*(p*q+a)+2*b)*r + (p*q-a)**2 - 4*b*(p+q)
def build():
    S4 = S3(x1,x2,u).resultant(S3(x3,x4,u),'u')
    S5 = S3(x1,x2,u).resultant(S4.compose(x3,x4,x5,u, *[], ctx=ctx) if False else S4_sub(S4), 'u')
    return S4,S5
def S4_sub(S4):
    # S4(x3,x4,x5,u)
    return S4.compose(a,b,x3,x4,x5,u,x5,u)
if __name__=='__main__':
    S4 = S3(x1,x2,u).resultant(S3(x3,x4,u),'u')
    S4s = S4.compose(a,b,x3,x4,x5,u,x5,u)  # x1->x3,x2->x4,x3->x5,x4->u
    S5 = S3(x1,x2,u).resultant(S4s,'u')
    print('S4 terms',len(S4),'degrees',S4.degrees(),'total',S4.total_degree())
    print('S5 terms',len(S5),'degrees',S5.degrees(),'total',S5.total_degree())
    pickle.dump({'S4':S4.to_dict(),'S5':S5.to_dict()},open('semaev_polys.pkl','wb'))
