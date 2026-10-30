\\ t5_hom.gp -- Tier T5: group-level Hom E(F_q)->E'(F_q) and F_q-rational isogeny search (odd p).
PMIN=ENVI("E0_PMIN",3); PMAX=ENVI("E0_PMAX",61);
{forprime(p=max(3,PMIN),PMAX,
  setfield(p,1);
  my(cl=classes_odd(p),t0=getabstime(),nviol=0,nbadh=0,nreach=0,nreach_t0=0,nt0=0);
  for(i=1,#cl,
    my(c=cl[i][2],w=cl[i][3],En=natcurve(1,c,0),Et=natcurve(1,c,1),G=ellgroup(En),Gt=ellgroup(Et));
    my(nE=ellcard(En),nEt=ellcard(Et),t=p+1-nE,gg=gcd(nE,nEt));
    my(hf=homformula(G,Gt),hb=homcount(G,Et,p),flags=List());
    if(hf!=hb,listput(flags,"HOMNEQ"));
    if((hf==1)!=(gg==1),listput(flags,"HOM_GCD"));
    if((2*gcd(p+1,t))%gg!=0,listput(flags,"GCDDIV"));
    my(pm=if(gg==1,1,vecmax(factor(gg)[,1])));
    if(t!=0&&pm>4*sqrt(p)+1e-9,listput(flags,"GCDPRIME"));
    my(b1="NA",b3="NA");
    if(p>=5,
      my(keyEt=canon((D_^2*c[2])%p,(D_^3*c[3])%p,p),keyE=canon(c[2],c[3],p));
      my(r1=bfs(c[2],c[3],p,[2,3,5,7],1));
      my(d1=-1); my(vv=r1[1]); for(k=1,#vv,if(vv[k]==keyEt,d1=1));
      if(keyEt==keyE,d1=0);
      b1=[#r1[1],r1[2],r1[3],r1[4],d1];
      if(r1[4]>0,listput(flags,"NBRCARD"));
      if(t!=0&&d1>=0,listput(flags,"F5_REACH"));
      if(p<=31,
        my(r3=bfs(c[2],c[3],p,[2,3],3));
        my(d3=-1); my(v3=r3[1]); for(k=1,#v3,if(v3[k]==keyEt,d3=3));
        if(keyEt==keyE,d3=0);
        b3=[#r3[1],r3[2],r3[3],r3[4],d3];
        if(r3[4]>0,listput(flags,"NBRCARD3"));
        if(t!=0&&d3>=0,listput(flags,"F5_REACH3"));
        if(t==0&&d3>=0,nreach_t0++));
      if(t==0,nt0++));
    my(fl=if(#flags==0,"OK",strjoin(Vec(flags),",")));
    if(fl!="OK",nviol++);
    emit(["T5",p,Str(c[2],",",c[3]),w,t,nE,nEt,gg,Str(G),Str(Gt),hf,hb,Str(b1),Str(b3),fl]));
  emit(["SUMMARY","T5",p,#cl,Str("viol=",nviol),Str("t0classes=",nt0),Str("t0_reach_Eprime_depth3=",nreach_t0),Str("ms=",getabstime()-t0)]));}
print("DONE|T5");
