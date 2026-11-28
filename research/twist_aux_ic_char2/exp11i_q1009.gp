\\ EXP11i at q=1009: exact mean decomps/target (64*N0/#E) for the case-P line vs K random lines per curve;
\\ searched baseline = best of the first k random lines (by yield). Also trials per full relation set ~ F/yield.
read("exp11i_yield.gp");
{ my(q=1009, t=ffgen(q^4,'t), one=t^0, done=0, K=2000, ks=[1,10,100,1000,2000]);
  setrand(100901);
  while(done<3, my(e=vector(3,i,random(t)), line, N, P, Y=vector(K), F=vector(K));
    if(#Set(e)<3, next);
    line=caseP(e,q,t); if(line==0, next); done++;
    my(cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]]); N=ellcard(ellinit([0,cf[1],0,cf[2],cf[3]],t));
    P=predict(e,line[1],line[2],q,one,N);
    for(i=1,K, my(r=predict(e,random(t),random(t),q,one,N)); F[i]=r[1]; Y[i]=r[2]);
    my(srt=vecsort(Y));
    printf("curve %d: v2(N)=%d | P: F=%d yield %.4f | random median: F=%d yield %.5f (ratio P/R %.1f; F/yield ratio %.1f)\n", done, valuation(N,2), P[1], P[2], vecsort(F)[K\2], srt[K\2], P[2]/srt[K\2], (vecsort(F)[K\2]/srt[K\2])/(P[1]/P[2]));
    foreach(ks, k, my(b=1); for(i=2,k, if(Y[i]>Y[b], b=i));
      printf("   best of %4d: F=%d yield %.5f | yield ratio P/best %.2f | F/yield ratio best/P %.2f | regime pred 2(2q/F)^3=%.1f, F_best~q+sqrt(2q ln k)=%.0f\n",
        k, F[b], Y[b], P[2]/Y[b], (F[b]/Y[b])/(P[1]/P[2]), 2*(2.*q/F[b])^3, q+sqrt(2.*q*log(max(k,2))))) ) }
