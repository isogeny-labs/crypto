# Build generic S_5 over Z[a,b,x1..x5] = Res_u(S3(x1,x2,u), S4(x3,x4,x5,u)) and pickle it.  [pcc:unverified]
import flint, pickle, time, sys
t0 = time.time()
ctx = flint.fmpz_mpoly_ctx.get(('a','b','x1','x2','x3','x4','x5','u','v'), 'lex')
a,b,x1,x2,x3,x4,x5,u,v = ctx.gens()
def S3(p_,q_,r_): return (p_-q_)**2*r_**2 - 2*((p_+q_)*(p_*q_+a)+2*b)*r_ + (p_*q_-a)**2 - 4*b*(p_+q_)
S4 = S3(x3, x4, v).resultant(S3(x5, u, v), 'v')      # S4(x3,x4,x5,u)
S5 = S3(x1, x2, u).resultant(S4, 'u')                 # S5(x1..x5)
d = {tuple(int(t) for t in e[:7]): int(c) for e, c in S5.to_dict().items()}
pickle.dump(d, open(sys.argv[1], 'wb'))
print('S5 terms', len(d), 'sec', round(time.time()-t0,1))
