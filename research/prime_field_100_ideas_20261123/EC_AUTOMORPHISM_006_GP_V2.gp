\\ Frozen valid finite EC automorphism quotient for proposal 6.
\\ Full signed relation set on one public CM curve and one fixed target.
p=1009; r=79; B=12; E=ellinit([Mod(0,p),Mod(3,p)]);
if(ellcard(E)!=948 || !isprime(r),error("curve order guard"));
cmroot=0;for(k=2,p-1,if(cmroot==0 && Mod(k,p)^3==1,cmroot=Mod(k,p)));
if(cmroot==0,error("cube root guard"));
phi(P)=if(P==[0],[0],[cmroot*P[1],P[2]]);
find_generator()={my(q,P,H);for(xx=1,p-1,q=Mod(xx^3+3,p);if(issquare(q),P=[Mod(xx,p),sqrt(q)];H=ellmul(E,P,12);if(H!=[0] && ellorder(E,H)==r,return(H))));error("subgroup generator guard")};
G=find_generator();
lam=0;for(k=1,r-1,if(lam==0 && ellmul(E,G,k)==phi(G),lam=k));
if(lam==0 || (lam^3)%r!=1 || lam==1,error("eigenvalue guard"));
setup0=getabstime();wall0=getwalltime();
choose_orbits()={my(reps=List(),P,seen);
  for(k=1,r-1,
    P=ellmul(E,G,k);seen=0;
    for(j=1,#reps,if(P==reps[j] || P==phi(reps[j]) || P==phi(phi(reps[j])),seen=1));
    if(!seen && #reps<5,listput(reps,P));
  );
  if(#reps!=5,error("orbit selection guard"));
  rep=vector(4,j,reps[j]);R=reps[5];
  fb=vector(B);negfb=vector(B);fbx=vector(B);
  for(j=1,4,fb[3*j-2]=rep[j];fb[3*j-1]=phi(rep[j]);fb[3*j]=phi(phi(rep[j])));
  for(i=1,B,
    if(!ellisoncurve(E,fb[i]) || ellorder(E,fb[i])!=r,error("factor base guard"));
    negfb[i]=ellsub(E,[0],fb[i]);fbx[i]=lift(fb[i][1]);
  );
  if(!ellisoncurve(E,R) || ellorder(E,R)!=r,error("target guard"));
  for(i=1,B,if(R==fb[i] || R==negfb[i],error("target in signed base")));
  targetPre=[R,phi(phi(R)),phi(R)];
  if(phi(targetPre[2])!=R || phi(phi(targetPre[3]))!=R,error("target orbit guard"));
};
choose_orbits();
setupCpu=getabstime()-setup0;setupWall=getwalltime()-wall0;
print("header p=",p," curve_card=",ellcard(E)," subgroup=",r," cmroot=",lift(cmroot)," lambda=",lam," generator=",G," target=",R);
print("base_x=",fbx," base_points=",fb," orbit_reps=",rep," common_setup_cpu_ms=",setupCpu," common_setup_wall_ms=",setupWall);
id(i,j,k,mask)=8*((i-1)*B*B+(j-1)*B+k-1)+mask+1;
phidx(i)=if(i%3==0,i-2,i+1);
row_for(i,j,k,mask)={my(v=vector(B+1,h,0),e1=if(mask%2,-1,1),e2=if((mask\2)%2,-1,1),e3=if((mask\4)%2,-1,1));v[i]+=e1;v[j]+=e2;v[k]+=e3;v[B+1]=-1;v};
sum_for(i,j,k,mask)={my(P1=if(mask%2,negfb[i],fb[i]),P2=if((mask\2)%2,negfb[j],fb[j]),P3=if((mask\4)%2,negfb[k],fb[k]));elladd(E,elladd(E,P1,P2),P3)};
rank_rows(rows,ncols)={my(M);if(#rows==0,return(0));M=matrix(ncols,#rows,ii,jj,Mod(rows[jj][ii],r));matrank(M)};
project_row(v)={my(q=vector(5,h,0),pow);for(i=1,B,pow=lift(Mod(lam,r)^((i-1)%3));q[1+(i-1)\3]+=v[i]*pow);q[5]=v[B+1];vector(5,j,centerlift(Mod(q[j],r)))};
check_project(q)={my(S=[0]);for(j=1,4,S=elladd(E,S,ellmul(E,rep[j],q[j])));S==R};
baseline()={my(t0=getabstime(),w0=getwalltime(),Q,v,rows=List(),proj=List(),hit=0,adds=0,checks=0);
  baselineMask=vector(B^3*8,h,0);
  for(i=1,B,for(j=1,B,for(k=1,B,for(mask=0,7,
    Q=sum_for(i,j,k,mask);adds+=2;checks++;
    if(Q==R,hit++;baselineMask[id(i,j,k,mask)]=1;v=row_for(i,j,k,mask);listput(rows,v);listput(proj,project_row(v)));
  ))));
  baselineRows=Vec(rows);baselineProjected=Vec(proj);
  print("baseline candidates=",B^3*8," point_adds=",adds," target_comparisons=",checks,
        " hits=",hit," full_rank=",rank_rows(baselineRows,B+1)," projected_rank=",rank_rows(baselineProjected,5),
        " collect_cpu_ms=",getabstime()-t0," collect_wall_ms=",getwalltime()-w0);
};
quotient()={my(t0=getabstime(),w0=getwalltime(),rows=List(),proj=List(),Q,v,
  canon=0,transformChecks=0,adds=0,checks=0,hits=0,ii,jj,kk,key,key1,key2,maskOut,idx,
  orbitSetupCpu,orbitSetupWall,collectCpu,collectWall);
  quotientMask=vector(B^3*8,h,0);
  for(i=1,B,for(j=1,B,for(k=1,B,
    key=(i-1)*B*B+(j-1)*B+k-1;
    ii=phidx(i);jj=phidx(j);kk=phidx(k);
    key1=(ii-1)*B*B+(jj-1)*B+kk-1;
    ii=phidx(ii);jj=phidx(jj);kk=phidx(kk);
    key2=(ii-1)*B*B+(jj-1)*B+kk-1;
    transformChecks+=2;
    if(key<=key1 && key<=key2,listput(rows,[i,j,k]));
  )));
  candidateReps=Vec(rows);canon=#candidateReps;
  if(canon!=B^3/3,error("orbit representative guard"));
  orbitSetupCpu=getabstime()-t0;orbitSetupWall=getwalltime()-w0;
  rows=List();
  for(h=1,canon,
    i=candidateReps[h][1];j=candidateReps[h][2];k=candidateReps[h][3];
    for(mask=0,7,
      Q=sum_for(i,j,k,mask);adds+=2;
      ii=i;jj=j;kk=k;
      for(e=1,3,
        checks++;
        if(Q==targetPre[e],
          hits++;idx=id(ii,jj,kk,mask);
          if(quotientMask[idx]!=0,error("duplicate quotient witness"));
          quotientMask[idx]=1;v=row_for(ii,jj,kk,mask);listput(rows,v);
        );
        ii=phidx(ii);jj=phidx(jj);kk=phidx(kk);
      );
    );
  );
  quotientRows=Vec(rows);
  if(quotientMask!=baselineMask || hits!=#baselineRows,error("full signed witness set mismatch"));
  proj=vector(#quotientRows,h,project_row(quotientRows[h]));
  for(h=1,#proj,if(!check_project(proj[h]),error("projected row point certificate mismatch")));
  collectCpu=getabstime()-t0-orbitSetupCpu;collectWall=getwalltime()-w0-orbitSetupWall;
  print("quotient orbit_reps=",canon," candidate_sign_patterns=",canon*8," orbit_transform_checks=",transformChecks,
        " point_adds=",adds," target_orbit_comparisons=",checks," hits=",hits,
        " full_rank=",rank_rows(quotientRows,B+1)," projected_rank=",rank_rows(proj,5),
        " orbit_setup_cpu_ms=",orbitSetupCpu," orbit_setup_wall_ms=",orbitSetupWall,
        " collect_cpu_ms=",collectCpu," collect_wall_ms=",collectWall,
        " total_cpu_ms=",getabstime()-t0," total_wall_ms=",getwalltime()-w0,
        " witness_mask_equal=true projected_certificates=true");
};
baseline();quotient();
print("complete finite EC automorphism orbit quotient");
