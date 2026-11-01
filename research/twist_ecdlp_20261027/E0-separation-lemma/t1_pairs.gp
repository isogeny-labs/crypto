\\ t1_pairs.gp -- Tier T1: full pair enumeration (explicit curve additions in E(F_{q^2})).
\\ env: E0_PMIN,E0_PMAX (odd primes) or E0_MMIN,E0_MMAX + E0_FAMILY=ord|ss (binary).
PMIN=ENVI("E0_PMIN",0); PMAX=ENVI("E0_PMAX",0); MMIN=ENVI("E0_MMIN",0); MMAX=ENVI("E0_MMAX",0); FAM=ENVS("E0_FAMILY","");
t1run(cl)={
  my(t0=getabstime(),nrow=#cl,nviol=0,wt=0,ng=0,ncop=0,ncopg=0,nlit=0,nbij=0,nmis=0);
  for(i=1,nrow,
    my(r=t1class(cl[i][1],cl[i][2],cl[i][3]));
    emit(r);
    wt+=r[5]; if(r[24]!="OK",nviol++); if(r[9]>1,ng++); if(r[11],ncop++); if(r[11]&&r[9]>1,ncopg++);
    if(r[22]==0,nlit++); if(r[23],nbij++); if((r[22]!=0)!=(r[9]==1),nmis++));
  emit(["SUMMARY","T1",Q_,nrow,wt,Str("viol=",nviol),Str("g2gt1=",ng),Str("coprime=",ncop),Str("coprime_and_g2gt1=",ncopg),Str("literal_fails=",nlit),Str("bijective=",nbij),Str("offdiag_M2=",nmis),Str("ms=",getabstime()-t0)]);
};
{if(PMAX>0, forprime(p=max(3,PMIN),PMAX, setfield(p,1); t1run(classes_odd(p))));
if(MMAX>0,
  for(m=MMIN,MMAX, setfield(2,m);
    if(FAM=="ord", t1run(classes_ord()), t1run(classes_ss(m<=4)))));}
print("DONE|T1");
