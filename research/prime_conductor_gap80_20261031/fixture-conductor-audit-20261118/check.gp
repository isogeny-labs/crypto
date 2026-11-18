\\ Native bounded proof qualification. No parameter search, elllog or timers.
require(ok,why)={if(!ok,print("FAIL: ",why);quit(2))};
point(P,p)={if(#P==1,[0],[Mod(P[1],p),Mod(P[2],p)])};
plain(P)={if(#P==1,"null",Str("[\"",lift(P[1]),"\",\"",lift(P[2]),"\"]"))};
direct(E,P,V)={
  if(P==[0],return(P));
  my(x=P[1],y=P[2],U);
  for(i=1,#V,
    U=elladd(E,P,V[i]);if(U==[0],return([0]));
    x+=U[1]-V[i][1];y+=U[2]-V[i][2];
  );[x,y]
};
audit(v)={
  my(label=v[1],p=v[2],ab=v[3],ell=v[4],K=point(v[5],p),cd=v[6],N=v[7],q=v[8],G=point(v[9],p),H=point(v[10],p),panel=v[11],scalars=v[12],expected=v[13],mode=v[14]);
  require(isprime(p)&&isprime(ell)&&isprime(q)&&ell!=p,"prime/characteristic guards");
  my(E=ellinit(ab,p),F=ellinit(cd,p),fpi=0,D=0,t=0,gap=0,fs=0,fd=0,counted=0,cmendo=0);
  require(E.disc!=0&&F.disc!=0,"nonsingular endpoints");
  if(mode=="small",
    my(n0=ellcard(E),n1=ellcard(F));t=p+1-n0;
    require(gcd(t,p)==1,"ordinary source");
    require(n0==N&&n1==N,"independent endpoint point counts");
    D=quaddisc(t^2-4*p);require(issquare((t^2-4*p)/D,&fpi)&&fpi>0,"Frobenius order discriminant");
    require((t^2-4*p)==D*fpi^2,"exact discriminant reconstruction");
    require(fpi%ell!=0,"horizontal certificate requires ell not dividing fpi");
    gap=1;counted=1;if(fpi==1,fs=1;fd=1),
    if(mode=="cm2",
      require(ell==2&&E.j==Mod(-3375,p)&&kronecker(-7,p)==1,"ordinary CM -7 source");
      my(s=sqrt(Mod(-7,p)),u=(1-s)/2,J=ellisogeny(E,[7+s,Mod(0,p)],1));
      require(u^2-u+2==0&&J[4]==u^4*E.a4&&J[5]==u^6*E.a6,"explicit degree2 CM endomorphism and differential");
      require(qfbclassno(-7)==1&&F.j!=E.j,"exclude maximal-order target using class number one");
      fs=1;fd=2;gap=2;D=-7;cmendo=2,
      require(mode=="cm5","known bounded certification mode");
      require(ell==5&&E.j==Mod(-884736,p)&&kronecker(-19,p)==1&&F.j==E.j,"ordinary CM -19 and geometric endpoint isomorphism");
      my(c=F.a6*E.a4/(E.a6*F.a4),u=sqrt(c));
      require(c^2==F.a4/E.a4&&c^3==F.a6/E.a6,"explicit codomain scaling");
      require((u+5/u)^2==1,"degree5 CM endomorphism has absolute trace1");
      fs=1;fd=1;gap=1;D=-19;cmendo=5
    )
  );
  require(gap==expected,"isogeny degree is not the certified conductor gap");
  require(K!=[0]&&ellisoncurve(E,K)&&ellmul(E,K,ell)==[0],"exact prime kernel order");
  require(gcd(q,ell)==1&&G!=[0]&&H!=[0]&&ellisoncurve(E,G)&&ellisoncurve(F,H)&&ellmul(E,G,q)==[0]&&ellmul(F,H,q)==[0],"prime subgroup, disjoint kernel and endpoints");
  my(V=vector(ell-1),Q=K,S=Mod(0,p),T=Mod(0,p));
  for(i=1,#V,
    require(Q!=[0],"kernel enumerated without early closure");V[i]=Q;
    S+=3*Q[1]^2+E.a4;T+=5*Q[1]^3+3*E.a4*Q[1]+2*E.a6;
    Q=elladd(E,Q,K)
  );
  require(Q==[0],"complete kernel coverage");
  require(F.a4==E.a4-5*S&&F.a6==E.a6-7*T,"independent direct Velu codomain");
  for(i=1,#panel,
    my(P=point(panel[i][1],p),want=point(panel[i][2],p));
    require(ellisoncurve(E,P),"source input membership");
    my(got=direct(E,P,V));
    require(got==want&&ellisoncurve(F,got),"independent direct Velu image")
  );
  for(i=1,#scalars,
    my(k=scalars[i][1],P=point(scalars[i][2],p),R=point(scalars[i][3],p));
    require(ellmul(E,G,k)==P&&ellmul(F,H,k)==R,"matched known scalar verification")
  );
  print("{\"case\":\"",label,"\",\"p\":\"",p,"\",\"degree\":\"",ell,"\",\"ordinary\":true,\"D_K\":\"",D,"\",\"point_counts_recomputed\":",if(counted,"true","false"),",\"trace\":",if(counted,Str("\"",t,"\""),"null"),",\"frobenius_conductor\":",if(counted,Str("\"",fpi,"\""),"null"),",\"source_end_conductor\":",if(fs,Str("\"",fs,"\""),"null"),",\"destination_end_conductor\":",if(fd,Str("\"",fd,"\""),"null"),",\"conductor_ratio\":\"",gap,"\",\"cm_endomorphism_degree\":",cmendo,",\"direct_kernel_points\":",#V,",\"direct_map_inputs_verified\":",#panel,",\"matched_scalars_verified\":",#scalars,",\"subgroup_order\":\"",q,"\",\"kernel_intersection_trivial\":true,\"source_j\":\"",lift(E.j),"\",\"destination_j\":\"",lift(F.j),"\",\"runtime_measurements\":null,\"ECDLP_solved\":false}");
};
rows=readstr(getenv("CONDUCTOR_FIXTURES"));
for(row=1,#rows,audit(eval(rows[row])));
quit(0);
