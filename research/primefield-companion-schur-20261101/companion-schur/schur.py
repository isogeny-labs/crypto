"""Unexecuted Schur prototype; leading-block solves are explicit supplied oracles.

Uses companion-blocks/operators.py. It never constructs the full matrix or
assigns a full-matrix singularity verdict to a leading-block failure.
"""
from operators import apply as full_apply

class LeadingBlockObstruction(Exception):
    pass

class Schur:
    def __init__(self,G,K,a,b,cut,solve,solve_transpose,m):
        assert 0<cut<len(G) and len(G)==len(K) and a!=b
        self.G=G;self.K=K;self.a=a;self.b=b;self.cut=cut
        self.size=len(G)-cut;self.solve=solve;self.solve_transpose=solve_transpose;self.m=m
    def _source(self,x,transpose=False):
        self.m.add('source_transpose_calls' if transpose else 'source_forward_calls')
        return full_apply(self.G,self.K,self.a,self.b,x,self.m,transpose)
    def _solve(self,x,transpose=False):
        assert len(x)==self.cut
        self.m.add('leading_transpose_calls' if transpose else 'leading_forward_calls')
        try:y=(self.solve_transpose if transpose else self.solve)(x)
        except ZeroDivisionError as e:
            raise LeadingBlockObstruction('Leading-block oracle failed; full matrix rank remains undetermined') from e
        assert len(y)==self.cut and all(type(v)is int and 0<=v<self.m.r for v in y)
        return y
    def apply(self,x,transpose=False):
        assert len(x)==self.size
        self.m.add('schur_transpose_calls' if transpose else 'schur_forward_calls')
        y=self._source([0]*self.cut+list(x),transpose)
        z=self._solve(y[:self.cut],transpose)
        w=self._source(list(z)+[0]*self.size,transpose)
        return [self.m.sub(u,v) for u,v in zip(y[self.cut:],w[self.cut:])]
    def generators(self,c,d):
        assert 0<=c<self.m.r and 0<=d<self.m.r and c!=d
        n=self.cut;q=self.size;s=len(self.G[0]);m=self.m
        GS=[[] for _ in range(q)];KS=[[] for _ in range(q)]
        for j in range(s):
            g=[row[j] for row in self.G];k=[row[j] for row in self.K]
            cg=self._source(self._solve(g[:n])+[0]*q)[n:]
            bk=self._source(self._solve(k[:n],True)+[0]*q,True)[n:]
            for i in range(q):
                GS[i].append(m.sub(g[n+i],cg[i]));KS[i].append(m.sub(k[n+i],bk[i]))
        ce=self._source(self._solve([1]+[0]*(n-1))+[0]*q)[n:]
        be=self._source(self._solve([0]*(n-1)+[1],True)+[0]*q,True)[n:]
        first=self.apply([1]+[0]*(q-1));last=self.apply([0]*(q-1)+[1],True)
        for i in range(q):
            GS[i].extend([m.plus(m.mul(self.a,ce[i]),c if i==0 else 0),m.sub(0,first[i])])
            KS[i].extend([last[i],m.plus(be[i],d if i==q-1 else 0)])
        return GS,KS
