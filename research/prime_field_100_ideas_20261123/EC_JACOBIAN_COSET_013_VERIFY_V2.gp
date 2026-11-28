\\ Function-wrapped independent PARI/GP replay of frozen V5 accepted rows.
verify()={
  my(rows=readvec("/tmp/ec13_coset_rows_20261128.gpdat"),n,e,G,Q,base,k,A,t,c,h1,h2,j1,j2,source,BA,BT,N,chosen,y,yt,yc,d,idx,weights,aa,bb,cert);
  n=#rows;
  if(n!=72,error("accepted row count mismatch"));
  e=ellinit([0,210,0,186,126],211); G=[165,19]; Q=[178,46];
  base=Set(concat(vector(n,i,[rows[i][3],rows[i][4]]),vector(n,i,[rows[i][5],rows[i][6]])));
  k=#base; A=matrix(n,k,i,j,Mod(0,109));
  t=vector(n,i,Mod(-4*rows[i][2],109));
  c=vector(n,i,Mod(4*rows[i][1],109));
  for(i=1,n,
    h1=[rows[i][3],rows[i][4]]; h2=[rows[i][5],rows[i][6]];
    j1=setsearch(base,h1); j2=setsearch(base,h2);
    if(!j1||!j2,error("missing base coordinate"));
    A[i,j1]+=Mod(1,109); A[i,j2]+=Mod(1,109);
    source=elladd(e,ellmul(e,G,rows[i][1]),ellmul(e,Q,rows[i][2]));
    if(elladd(e,h1,h2)!=ellmul(e,source,4),error("point identity mismatch at row ",i));
  );
  BA=matrank(A); BT=matrank(matconcat([A,Mat(t~)]));
  if(BA!=42||BT!=43,error("rank mismatch"));
  N=matker(A~); chosen=0;
  for(j=1,matsize(N)[2],y=N[,j]; yt=sum(i=1,n,y[i]*t[i]); if(yt!=Mod(0,109),chosen=j;break));
  if(!chosen,error("target coefficient is in base column span"));
  y=N[,chosen]; yt=sum(i=1,n,y[i]*t[i]); yc=sum(i=1,n,y[i]*c[i]); d=yc/yt;
  if(ellmul(e,G,lift(d))!=Q,error("independent scalar verification failed"));
  print("verify rows=",n," observed_base=",k," base_rank=",BA," augmented_rank=",BT," kernel_dimension=",matsize(N)[2]);
  print("generic_certificate_kernel_column=",chosen," y_dot_target=",lift(yt)," y_dot_constant=",lift(yc)," d=",lift(d));
  \\ Compact four-row witness from independently found trial IDs.
  idx=[12,51,55,70]; weights=[9,100,9,100]; cert=vector(k,i,Mod(0,109)); aa=Mod(0,109); bb=Mod(0,109);
  \\ Accepted rows are ordered by trial but gaps require original trial IDs.
  \\ Four rows are embedded below from the frozen raw, so this certificate
  \\ can be audited independently of the extracted accepted-row ordering.
  my(four=[[90,62,3,51,115,204],[73,88,3,51,201,72],[42,1,3,160,201,72],[83,81,3,160,115,204]]);
  for(z=1,4,
    my(w=Mod(weights[z],109),f=four[z],matches=0);
    for(i=1,n,if(rows[i]==f,matches++));
    if(matches!=1,error("four-row tuple absent or duplicated in raw",z));
    cert[setsearch(base,[f[3],f[4]])]+=w;
    cert[setsearch(base,[f[5],f[6]])]+=w;
    aa+=w*f[1]; bb+=w*f[2];
  );
  if(sum(i=1,k,cert[i]!=Mod(0,109))!=0,error("four-row base cancellation failed"));
  if(lift(aa)!=2||lift(bb)!=27,error("four-row source coefficients mismatch"));
  if(lift(-4*bb)!=1||lift(4*aa)!=8,error("four-row target certificate mismatch"));
  print("four_trial_ids=",idx," weights=",weights," base_residual=0 weighted_a=",lift(aa)," weighted_b=",lift(bb)," target_coefficient=",lift(-4*bb)," constant=",lift(4*aa));
  print("all_72_group_identities=true scalar_group_check=true");
};
verify();
