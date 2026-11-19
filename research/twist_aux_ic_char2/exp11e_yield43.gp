\\ EXP11e: relation yield per decomposition trial at n=4, q=43 (same construction as exp11d_gen.gp):
\\ mean number of decompositions (bf2 ground truth) per target R = 2*random, case-P line vs random lines.
read("exp11d_gen.gp");
yield(q, ncurves, NT, NRL, seed) = {
  setrand(seed); my(n=4, t=ffgen(q^4,'t), one=t^0, bas=vector(n,i,t^(i-1)), gam=ffprimroot(t)^((q^4-1)/(q^2-1)), done=0, SP=0, SR=0, nP=0, nR=0, hitP=0, hitR=0);
  while(done < ncurves,
    my(e=vector(3,i,random(t)), cf, E, got=0, line=0);
    if(#Set(e)<3, next);
    cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]]; E=ellinit([0,cf[1],0,cf[2],cf[3]],t);
    for(i=1,3, if(got,break); for(j=1,3, if(i==j||got, next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n), sol, z, c1, c0);
      for(b=1,n, M[,b] = tovec(B*bas[b] - A^q*bas[b]^q, n)~);
      sol = iferr(matsolve(M, tovec(gam^q-gam,n)~), err, 0); if(sol==0, next);
      z = sum(b=1,n, lift(sol[b])*bas[b]); if(z==0, next);
      c1=1/z; c0=e[k]-c1*gam; if(!issquare(c1), next); got=1; line=[c0,c1]));
    if(!got, next); done++;
    my(L=concat([["P",line[1],line[2]]], vector(NRL,i,["R",random(t),random(t)])));
    for(l=1,#L, my(pts=linepts(E,L[l][2],L[l][3],q,one), F=sum(w=1,q,#pts[w]>0), s=0, h=0);
      for(r=1,NT, my(R=ellmul(E,random(E),2), c); if(R==[0], next); c=bf2(E,pts,q,R); s+=c; h+=(c>0));
      if(L[l][1]=="P", SP+=s; nP+=NT; hitP+=h, SR+=s; nR+=NT; hitR+=h);
      print("q=",q," curve ",done," ",L[l][1],"-line F=",F," decomps/target ",s*1./NT," hit-rate ",h*1./NT)));
  printf("SUMMARY q=%d: P decomps/target %.4f hit %.4f | R decomps/target %.4f hit %.4f | trials-per-relation ratio R/P %.2f\n", q, SP/nP, hitP/nP, SR/nR, hitR/nR, (SP/nP)/(SR/nR));
}
yield(43, 3, 150, 3, 4301);
