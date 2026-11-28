import json,sys
def rank(M,r):
    M=[row[:] for row in M]; rk=0; n=len(M[0])
    for c in range(n):
        p=next((i for i in range(rk,len(M)) if M[i][c]),None)
        if p is None: continue
        M[rk],M[p]=M[p],M[rk]; inv=pow(M[rk][c],r-2,r); M[rk]=[x*inv%r for x in M[rk]]
        for i in range(len(M)):
            if i!=rk and M[i][c]:
                f=M[i][c]; M[i]=[(a-f*b)%r for a,b in zip(M[i],M[rk])]
        rk+=1
    return rk
def _main():
  for line in open(sys.argv[1]):
      d=json.loads(line); r,cc,ct=d['r'],d['cc'],d['ct']
      A=[];T=[]
      for ents,rhs in d['rows']:
          v=[0]*(cc+1)
          for c,x in ents: v[c]=x%r
          v[cc]=rhs%r; A.append(v)
          if v[ct]: T.append((v[cc],v[ct]))
      rA=rank([a[:cc] for a in A],r); rAb=rank(A,r)
      nz_rhs_no_t=sum(1 for a in A if a[ct]==0 and a[cc]!=0)
      print(d['q'],d['seed'],d['attempt'],'cc',cc,'rankA',rA,'rank[A|b]',rAb,'t_rows',len(T),'rhs!=0 w/o t',nz_rhs_no_t, 'ratios', len({ (b*pow(t,r-2,r))%r for b,t in T}))

if __name__=='__main__': _main()