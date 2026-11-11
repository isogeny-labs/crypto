default(parisizemax,1000000000);
read(Str(getenv("GPDIR"),"/lib.gp"));
S=eval(getenv("S"));
recs=load_recs(S);
out=Str(OUTDIR,"/controls_",S,".csv");system(Str("rm -f ",out));
write(out,"test,n,pass,value1,value2,note");
rep(t,n,ps,v1,v2,note)=write(out,Str(t,",",n,",",ps,",",v1,",",v2,",",note));

\\ independent solver (per x1 quadratic in x2 over F_q), returns Set of sorted pairs
indep_pairs(X)={
  my(A,B,C,disc,r,out=Set(),x2);
  for(x1=0,PP-1,
    A=(X-x1)^2;
    B=-2*x1*X^2-2*X*(x1^2+AA)-2*AA*x1-4*BB;
    C=x1^2*X^2-2*X*(AA*x1+2*BB)+AA2-4*BB*x1;
    disc=B^2-4*A*C;
    if(issquare(disc,&r),
      x2=(-B+r)/(2*A);if(infp(x2),out=setunion(out,Set([vecsort([x1,polcoef(x2.pol,0,'tt)])])));
      x2=(-B-r)/(2*A);if(infp(x2),out=setunion(out,Set([vecsort([x1,polcoef(x2.pol,0,'tt)])])))));
  out;
}
S3f(x1,x2,x3)=(x1-x2)^2*x3^2-2*((x1+x2)*(x1*x2+AA)+2*BB)*x3+(x1*x2-AA)^2-4*BB*(x1+x2);

unit_tests()={
  my(rs,rec,ok,n,tg,pr,sp,mism,cnt0);
  \\ UT1 comb target = aG+bQ ; UT2 solver vs independent method
  foreach([1,4],ar,
    rs=recs_arm(recs,ar);rec=rs[1];inst_setup(rec);build_tables();comb_build();
    setrand(sbase(S,6,24)+ar);
    ok=0;n=200;
    for(t=1,n,tg=gen_target();if(tg[1]==elladd(EC,ellmul(EC,GG,tg[2]),ellmul(EC,QQ,tg[3])),ok++));
    rep(Str("UT1_comb_arm",ar),n,ok,"","","target equals aG+bQ");
    mism=0;n=0;cnt0=NSOLV;
    for(t=1,if(S<=24,300,100),
      tg=gen_target();if(tg[1]==[0]||infp(tg[1][1]),next);
      n++;sp=solve_pairs(tg[1][1]);
      if(sp!=indep_pairs(tg[1][1]),mism++));
    rep(Str("UT2_solver_vs_independent_arm",ar),n,n-mism,"","","Weil-restriction solver equals per-x1 quadratic method");
  );
  \\ UT3 C oracle vs exhaustive S3 (p small only)
  rs=recs_arm(recs,3);rec=rs[1];inst_setup(rec);build_tables();comb_build();
  if(PP<=700,
    c_build();setrand(sbase(S,6,25));
    ok=0;n=0;
    for(t=1,if(PP<=300,6,2),
      tg=gen_target();R=tg[1];if(R==[0]||infp(R[1]),next);
      for(l=1,MV,my(P1=YPT[VL[l]+1],T,key,v,ex,orc);
        for(sgn=0,1,
          T=if(sgn==0,elladd(EC,R,NEGV[l]),elladd(EC,R,P1));
          if(#T==1||infp(T[1]),next);
          ex=Set();
          for(i=1,MV,for(j=i,MV,if(S3f(VL[i],VL[j],T[1])==0,ex=setunion(ex,Set([[i,j]])))));
          key=xkey(T[1]);orc=Set();
          if(mapisdefined(CT,key,&v),for(z=1,#v,orc=setunion(orc,Set([[v[z][1],v[z][2]]]))));
          n++;if(ex==orc,ok++))));
    rep("UT3_C_oracle_vs_exhaustive_S3",n,ok,PP,MV,"table lookup equals exhaustive solution of S3 over all pairs in V");
  );
}
unit_tests();

\\ pre-registered stopping rule: first 1000 targets of the first instance of each curve, arm A
stopcheck()={
  my(rs=recs_arm(recs,1),rec,seen=Map(),viol=0,nt=0,tg,sp,key,ndeg=0);
  setrand(sbase(S,6,23));
  for(i=1,#rs,rec=rs[i];key=[rec[7],rec[10]];if(mapisdefined(seen,key),next);mapput(seen,key,1);
    inst_setup(rec);build_tables();comb_build();
    for(t=1,1000,tg=gen_target();if(tg[1]==[0]||infp(tg[1][1]),ndeg++;next);sp=solve_pairs(tg[1][1]);nt++;
      if(#sp!=1,viol++,
        if(CLS[sp[1][1]+1]==CLS[sp[1][2]+1],viol++))));
  rep("STOPCHECK_A_D(R)=1",nt,nt-viol,ndeg,"","first 1000 targets per curve; value1 = degenerate draws excluded (x(R) in F_p)");
  if(viol>0,write(Str(OUTDIR,"/STOP"),Str("stopping rule fired at s=",S)));
}
stopcheck();

pc_tests()={
  \\ PC2a planted wrong k'
  my(ns=0,nrej=0,nacc_true=0,rr=recs);
  for(t=1,1000,rec=rr[(t-1)%#rr+1];inst_setup(rec);
    kp=(KK+1)%LS;ns++;if(ellmul(EC,GS,kp)!=QS,nrej++);
    if(ellmul(EC,GS,KK%LS)==QS,nacc_true++));
  rep("PC2a_planted_wrong_k",ns,nrej,nacc_true,"","verifier rejects k+1; value1 = true k accepted");
  \\ PC2b synthetic systems
  my(Lsave=LS,ok=0,tot=0,cols,coefs,q,u,L,rows,got,w,a,b,M=30,qp);
  setrand(sbase(S,6,26));
  L=10007;LS=L;
  for(sy=1,200,
    w=1+(sy%3);la_reset();u=vector(M,i,random(L));qp=random(L);got=-1;rows=0;
    while(got<0 && rows<3000,
      rows++;cols=[];coefs=[];
      my(chosen=Set());
      while(#chosen<w,my(c=1+random(M));if(!setsearch(chosen,c),chosen=setunion(chosen,Set([c]))));
      chosen=Vec(chosen);
      my(sum=0);
      for(i=1,w,my(e=2*random(2)-1);cols=concat(cols,[chosen[i]]);coefs=concat(coefs,[e]);sum+=e*u[chosen[i]]);
      b=1+random(L-1);a=(sum-b*qp)%L;
      got=la_add(cols,coefs,-b,a));
    tot++;if(got==qp,ok++));
  LS=Lsave;
  rep("PC2b_synthetic_planted_q",tot,ok,"","","200 random systems of weight 1,2,3, exact recovery");

}

nc2()={
  my(rs=recs_arm(recs,1),n=#rs,per=ceil(1000/n),acc=0,tot=0,expc=0,vr=0,rec,tg,R,pairs,jc,mv,tot2=0,acc2=0,exp2=0,vr2=0,rv,q,nr,qh,c1,c2,col,e,sg,x1,x2,P1,P2,rels,ord,pp);
  setrand(sbase(S,7,2));
  for(i=1,n,rec=rs[i];inst_setup(rec);build_tables();comb_build();FB=vector(PP,j,CLS[j]!=0);
    for(r=1,per,
      cnt_reset();la_reset();rels=List();q=-1;
      while(q<0,tg=gen_target();R=tg[1];if(R==[0]||infp(R[1]),next);
        pairs=solve_pairs(R[1]);
        for(k=1,#pairs,x1=pairs[k][1];x2=pairs[k][2];P1=YPT[x1+1];P2=YPT[x2+1];
          sg=sign2(R,P1,P2);if(sg==0,next);
          c1=CLS[x1+1];c2=CLS[x2+1];
          if(c1==CSTAR&&c2!=CSTAR,col=x1;e=sg[1],if(c2==CSTAR&&c1!=CSTAR,col=x2;e=sg[2],next));
          listput(rels,[col,e,tg[2],tg[3]]);
          if(q<0,q=la_add([col],[e],-tg[3],tg[2]))));
      rv=Vec(rels);nr=#rv;
      ord=vecsort(vector(nr,j,[random(2^31),j]));pp=vector(nr,j,ord[j][2]);
      la_reset();qh=-1;
      for(j=1,nr,if(qh<0,qh=la_add([rv[j][1]],[rv[j][2]],-rv[pp[j]][4],rv[pp[j]][3])));
      jc=0;for(j=1,nr-1,if(rv[j][1]==rv[nr][1],jc=j));
      mv=(jc>0 && Set([pp[jc],pp[nr]])!=Set([jc,nr]));
      tot++;if(qh>=0&&ellmul(EC,GS,qh)==QS,acc++;if(mv,acc2++));
      expc+=1/LS;vr+=(1/LS)*(1-1/LS);
      if(mv,tot2++;exp2+=1/LS;vr2+=(1/LS)*(1-1/LS))));
  rep("NC2_permuted_targets",tot,tot-acc,expc,expc+3*sqrt(vr),"pass column = rejected; value1 = chance accepts expected sum(1/l*); value2 = chance + 3 sigma; literal criterion falsified if accepted > value2");
  rep("NC2b_permuted_targets_collision_rows_moved",tot2,tot2-acc2,exp2,exp2+3*sqrt(vr2),"subset where the shuffle gave at least one of the two colliding rows a foreign (a,b) (identity and the swap of the two rows excluded; the swap returns the true q whenever the two signs agree); same chance bound; falsified if accepted > value2");
}
if(S==20,pc_tests();nc2());
print("controls s=",S," done");
