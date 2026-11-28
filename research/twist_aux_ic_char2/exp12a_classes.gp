\\ EXP12a: 2-descent class counts per line type (pool sizes for a balanced attack), and
\\ brute-force check at q=43 that a single-class subset of size S has yield 64*C(S+3,4)/#E on either line type.
read("exp11i_yield.gp");
cls(e,x) = 1+(!issquare(x-e[1]))+2*(!issquare(x-e[2]));
{ my(q=1009, t=ffgen(q^4,'t), one=t^0, done=0); setrand(100901);
  while(done<3, my(e=vector(3,i,random(t)), line); if(#Set(e)<3, next);
    line=caseP(e,q,t); if(line==0, next); done++;
    printf("q=%d curve %d P-line classes %s\n", q, done, lineclasses(e,line[1],line[2],q,one));
    my(M=matrix(20,4)); for(i=1,20, my(n=lineclasses(e,random(t),random(t),q,one)); for(c=1,4, M[i,c]=n[c]));
    my(mx=vector(20,i,vecmax(M[i,])), mn=vector(20,i,vecmin(M[i,])));
    printf("   20 random lines: total median %d, largest class median %d (max %d), smallest class median %d\n",
      vecsort(vector(20,i,vecsum(M[i,])))[10], vecsort(mx)[10], vecmax(mx), vecsort(mn)[10])) }
\\ q=43 check: restrict pts to one class, brute-force decomps per target in 2E
{ my(q=43, n=4, t=ffgen(q^4,'t), one=t^0, done=0, NT=200); setrand(4301);
  while(done<2, my(e=vector(3,i,random(t)), line, cf, E, N); if(#Set(e)<3, next);
    line=caseP(e,q,t); if(line==0, next); done++;
    cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]]; E=ellinit([0,cf[1],0,cf[2],cf[3]],t); N=ellcard(E);
    my(L=[["P",line[1],line[2]], ["R",random(t),random(t)], ["R",random(t),random(t)]]);
    for(l=1,#L, my(pts=linepts(E,L[l][2],L[l][3],q,one), cnt=vector(4), best, S, sub, s=0);
      for(w=1,q, if(#pts[w], cnt[cls(e,pts[w][1][1])]++)); best=1; for(c=2,4, if(cnt[c]>cnt[best], best=c)); S=cnt[best];
      sub=vector(q,w, if(#pts[w] && cls(e,pts[w][1][1])==best, pts[w], []));
      for(r=1,NT, my(R=ellmul(E,random(E),2)); if(R==[0], next); s+=bf2(E,sub,q,R));
      printf("q=43 curve %d %s-line classes %s | single class S=%d: measured %.4f predicted %.4f\n", done, L[l][1], cnt, S, s*1./NT, 64.*binomial(S+3,4)/N))) }
