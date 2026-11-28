\\ Native PARI/GP bounded construction. This is charged preprocessing, not
\\ a general efficient datum-construction theorem. No target DLP labels enter.
default(parisizemax, 512000000);
emit_factor(F) = {
  my(n=matsize(F)[1]); print(n);
  for(i=1,n, print(F[i,1]," ",F[i,2]));
};
emit_basis(E,G) = {
  my(P=G[3][1],Q=[0],d1=G[2][1],d2=1,k=0);
  if(#G[2]==2,
    d2=G[2][2]; Q=G[3][2];
    if(d1/d2>1,k=elllog(E,ellmul(E,Q,d2),ellmul(E,P,d2),d1/d2));
    Q=ellsub(E,Q,ellmul(E,P,k));
    if(ellorder(E,Q)!=d2,error("direct-product correction failed"));
  );
  print("0 ",lift(P[1])," ",lift(P[2])," ",d1);
  if(d2==1,print("1 0 0 1"),print("0 ",lift(Q[1])," ",lift(Q[2])," ",d2));
};
{
  my(p=eval(getenv("TRACE_AUX_P")),seed=eval(getenv("TRACE_AUX_SEED")),
     fixed=eval(getenv("TRACE_AUX_FIXED")),beta=2,a,b,E,Et,N,Nt,F,Ft,G,H,tries=0,okay=0);
  if(!isprime(p)||p<=3||p>=2^62,error("unsupported field"));
  setrand(seed);
  while(kronecker(beta,p)!=-1,beta++);
  for(attempt=1,128,
    tries=attempt;
    if(fixed,a=eval(getenv("TRACE_AUX_A"));b=eval(getenv("TRACE_AUX_B")),a=random(p);b=random(p));
    if((4*a^3+27*b^2)%p==0,if(fixed,break,next));
    E=ellinit([a,b],p); N=ellcard(E); Nt=2*p+2-N;
    if(!fixed && N%2,next);
    F=factor(N);Ft=factor(Nt);
    if(!fixed && max(F[matsize(F)[1],1],Ft[matsize(Ft)[1],1])>1024,next);
    Et=ellinit([a/Mod(beta,p)^2,b/Mod(beta,p)^3],p);
    G=ellgroup(E,,1); H=ellgroup(Et,,1);
    if(G[1]!=N||H[1]!=Nt,error("point-count disagreement"));
    okay=1;break;
  );
  if(!okay,print("MISS ",tries);quit(3));
  print(p," ",lift(a)," ",lift(b)," ",beta," ",N," ",Nt);
  emit_factor(F);emit_factor(Ft);emit_basis(E,G);emit_basis(Et,H);
  print(tries);
  quit(0);
}
