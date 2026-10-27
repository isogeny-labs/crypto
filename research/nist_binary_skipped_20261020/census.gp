\\ Census of one FIPS 186-2 binary-curve run: GP does the basis change and
\\ the point counting; the candidates come from the Rust enumerator.
\\ Inputs (prepended by census.sh): M, FPOLY (in 'z), CNB_PUB, B_PUB, CAND, NAME.
\\ Output lines:
\\   calib <name> <conjugate index> <bit order> <matches>
\\   cand <i> <seed> <b_pb hex> <N> <n prime 0/1> <mov ok 0/1> <checks 0/1>
\\ checks: N = 2 mod 4, |2^M + 1 - N| <= 2^(M/2+1), and [N]P = O at 3 random points.
default(parisizemax, 4000000000);
gnbtype(m)={for(T=1,60,my(p=T*m+1);if(isprime(p),my(k=znorder(Mod(2,p)));if(gcd(T*m/k,m)==1,return(T))));}
gnbminpoly(m)={
  my(T=gnbtype(m),p=T*m+1,k=znorder(Mod(2,p)),G=ffgen([2,k],'g),gam,u,th);
  gam=G^((2^k-1)/p); u=Mod(znprimroot(p),p)^((p-1)/T);
  th=sum(j=0,T-1,gam^lift(u^j));
  minpoly(th,'x)
}
F=ffgen(FPOLY*Mod(1,2),'z);
pb2int(e)=subst(lift(e.pol),'z,2);

\\ normal-basis integer (MSB = coefficient of beta^(2^0) when ord=0) to F
nb2pb(c,beta,ord)={my(s=0*F,b=beta,bits=binary(c)); bits=concat(vector(M-#bits),bits);
  for(i=0,M-1, if(bits[if(ord,M-i,i+1)], s+=b); b=b^2); s};
movok(n)={for(k=1,2^10-1, if(Mod(2,n)^(M*k)==1, return(0))); 1};
{
R=polrootsmod(gnbminpoly(M),[2,FPOLY]);
R=vector(#R,r,subst(lift(lift(R[r])),'z,F)+0*F);
hits=[]; for(r=1,#R, for(o=0,1, if(pb2int(nb2pb(CNB_PUB,R[r],o))==B_PUB, hits=concat(hits,[[r,o]]))));
if(#hits==0, print("calib ",NAME," none none 0"); quit);
print("calib ",NAME," ",hits[1][1]," ",hits[1][2]," ",#hits);
BETA=R[hits[1][1]]; ORD=hits[1][2];
if(CAND!="",
  L=externstr(Str("cat ",CAND));
  for(l=1,#L,
    my(w=strsplit(L[l]," "),i=eval(w[1]),c=eval(Str("0x",w[3])),b,N,n,pr,mv);
    b=nb2pb(c,BETA,ORD);
    my(E=ellinit([1,1,0,0,b]),ck);
    N=ellcard(E);
    n=N/2; pr=(N%2==0 && ispseudoprime(n)); mv=if(pr,movok(n),0);
    ck=(N%4==2) && (abs(2^M+1-N)^2<=4*2^M) && prod(j=1,3,ellmul(E,random(E),N)==[0]);
    print("cand ",i," ",w[2]," ",Strprintf("%x",pb2int(b))," ",N," ",pr," ",mv," ",ck)));
}
