import cypari2,random,math
pari=cypari2.Pari()
def run(p,seed):
  random.seed(seed)
  while True:
    a=random.randrange(p);b=random.randrange(p)
    if (4*a**3+27*b*b)%p==0: continue
    E=pari.ellinit([a,b],p); r=int(pari.ellcard(E))
    if pari.isprime(r) and math.gcd(r,p-1)==1: break
  while True:
    x=random.randrange(p); y2=(x**3+a*x+b)%p
    if pow(y2,(p-1)//2,p)==1: y=int(pari.sqrt(pari.Mod(y2,p)).lift()); break
  P=[x,y]
  # division values psi_n(P) via EDS recurrence
  N=r+3; W=[0]*(N+2)
  W[0]=0;W[1]=1;W[2]=2*y%p
  W[3]=(3*x**4+6*a*x*x+12*b*x-a*a)%p
  W[4]=4*y*(x**6+5*a*x**4+20*b*x**3-5*a*a*x*x-4*a*b*x-8*b*b-a**3)%p
  inv2y=pow(2*y,-1,p)
  for n in range(5,N+1):
    m=n//2
    if n%2: W[n]=(W[m+2]*W[m]**3-W[m-1]*W[m+1]**3)%p
    else: W[n]=(W[m]*(W[m+2]*W[m-1]**2-W[m-2]*W[m+1]**2)*inv2y)%p
  bad=0
  for j in range(2,min(r-1,3000)):
    Q=pari.ellmul(E,P,j); xq=int(Q[0].lift())
    if (xq-x)*W[j]**2 % p != (-W[j-1]*W[j+1])%p: bad+=1
  # U(k)=W(k)/W(k-1), k=2..r-1 ; quasi-period check
  U=[None,None]+[W[k]*pow(W[k-1],-1,p)%p for k in range(2,r)]
  res={'p':p,'r':r,'id_fail':bad,'Wr':W[r]}
  for l in [q for q in range(2,14) if pari.isprime(q) and (p-1)%q==0]:
    g=int(pari.znprimroot(p).lift()); 
    def chi(v): 
      t=pow(v,(p-1)//l,p); 
      return next(i for i in range(l) if pow(g,i*(p-1)//l,p)==t)
    c=[chi(U[k]) for k in range(2,r)]
    best=0
    for A in range(l):
      for B in range(l):
        for C in range(l):
          ag=sum(1 for i,k in enumerate(range(2,r)) if c[i]==(A*k*k+B*k+C)%l)/len(c)
          best=max(best,ag)
    res['l%d'%l]=[round(best,4),round(1/l+5/math.sqrt(r),4)]
  return res
for i,p in enumerate([65537+0,70001,100003,131101,262147]):
  p=int(pari.nextprime(p)); print(run(p,i))
