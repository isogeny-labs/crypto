\\ t2_bonly.gp -- Tier T2: B-only linear check on E'(F_q) (+ all pair-free prefilters).
PMIN=ENVI("E0_PMIN",0); PMAX=ENVI("E0_PMAX",0); MMIN=ENVI("E0_MMIN",0); MMAX=ENVI("E0_MMAX",0); FAM=ENVS("E0_FAMILY","");
NSAMP=ENVI("E0_NSAMP",2000);
t2run(cl,tag)={
  my(t0=getabstime(),nrow=#cl,nviol=0,wt=0,ng=0,ncop=0,ncopg=0,nlit=0,nmis=0,wev=0);
  for(i=1,nrow,
    my(r=t2class(cl[i][1],cl[i][2],cl[i][3]));
    r[1]=tag; emit(r);
    wt+=r[5]; if(r[16]!="OK",nviol++); if(r[9]>1,ng++); if(r[11],ncop++); if(r[11]&&r[9]>1,ncopg++);
    if(r[15],nlit++); if(r[15]!=(r[9]>1),nmis++); if(r[7]%2==0,wev+=r[5]));
  emit(["SUMMARY",tag,Q_,nrow,wt,Str("viol=",nviol),Str("g2gt1=",ng),Str("coprime=",ncop),Str("coprime_and_g2gt1=",ncopg),Str("literal_fails=",nlit),Str("offdiag_M2=",nmis),Str("weight_nE_even=",wev),Str("ms=",getabstime()-t0)]);
};
{if(PMAX>0, forprime(p=max(3,PMIN),PMAX, setfield(p,1); t2run(classes_odd(p),"T2")));
if(MMAX>0,
  for(m=MMIN,MMAX, setfield(2,m);
    if(FAM=="ord", t2run(classes_ord(),"T2"),
      if(m<=7, t2run(classes_ss(0),"T2"),
        setrand(20261021);
        my(cl=vector(NSAMP,i,[3,[SUBF_[2],SUBF_[1+random(Q_)],SUBF_[1+random(Q_)]],1]));
        t2run(cl,"T2S")))));}
print("DONE|T2");
