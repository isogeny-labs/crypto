\\ t3_allab.gp -- Tier T3: all (a4,a6) with nonzero discriminant, no class reduction (odd p>=5).
PMIN=ENVI("E0_PMIN",5); PMAX=ENVI("E0_PMAX",23);
{forprime(p=max(5,PMIN),PMAX,
  setfield(p,1);
  my(cl=curves_odd_all(p),t0=getabstime(),nviol=0,ng=0,nev=0);
  if(#cl!=p^2-p,error("curve count"));
  for(i=1,#cl,
    my(r=t2class(cl[i][1],cl[i][2],cl[i][3]));
    r[1]="T3"; emit(r);
    if(r[16]!="OK",nviol++); if(r[9]>1,ng++); if(r[7]%2==0,nev++));
  emit(["SUMMARY","T3",p,#cl,#cl,Str("viol=",nviol),Str("g2gt1=",ng),Str("nE_even=",nev),Str("ms=",getabstime()-t0)]));}
print("DONE|T3");
