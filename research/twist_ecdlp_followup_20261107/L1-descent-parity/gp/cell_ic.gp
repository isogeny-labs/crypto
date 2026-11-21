default(parisizemax,1000000000);
read(Str(getenv("GPDIR"),"/lib.gp"));
S=eval(getenv("S"));ARM=getenv("ARM");REPS=eval(getenv("REPS"));
recs=load_recs(S);
spec=Map([ "A",[1,1,"all",0,sbase(S,1,2)]; "B",[1,2,"B",sbase(S,2,3),sbase(S,2,2)]; "Bp",[1,12,"B",sbase(S,2,13),sbase(S,2,12)]; "D",[4,4,"V",0,sbase(S,4,2)]; "Dh",[4,14,"D1",sbase(S,6,3),sbase(S,6,2)]; "Dq",[4,24,"D2",sbase(S,6,13),sbase(S,6,12)]; "C",[3,3,"C",0,sbase(S,3,2)] ]);
sp=mapget(spec,ARM);
rs=recs_arm(recs,sp[1]);nm=getenv("NMAX");if(nm!=0,rs=vector(min(#rs,eval(nm)),i,rs[i]));n=#rs;id=sp[2];kind=sp[3];
if(kind=="B"||kind=="D1"||kind=="D2",FBL=fb_precompute(rs,kind,sp[4]));
setrand(sp[5]);seeds=vector(n*REPS,i,random(2^31));
out=Str(OUTDIR,"/ic_",ARM,"_",S,".csv");system(Str("rm -f ",out));
write(out,"armid,s,inst,rep,seed,success,att,ndeg,krel,ncols,nsolv,nres,ngcd,nroot,sroot,nsq,GAE,NAT,LANAT,LAROW,LAENT,anom,wall_ms,nrel,mcol");
t00=getabstime();
main()={
  for(i=1,n,
    rec=rs[i];inst_setup(rec);build_tables();comb_build();
    if(kind=="all",FB=vector(PP,j,CLS[j]!=0));
    if(kind=="V",FB=vector(PP,j,YPT[j]!=0));
    if(kind=="B"||kind=="D1"||kind=="D2",FB=FBL[i]);
    if(kind=="C",c_build());
    mcol=if(kind=="C",MV,if(sp[1]==1,sum(j=1,PP,FB[j]&&CLS[j]==CSTAR),sum(j=1,PP,FB[j]!=0)));
    for(r=1,REPS,
      sd=seeds[(i-1)*REPS+r];setrand(sd);
      cap=capatt(id,PP);
      res=if(kind=="C",ic_run_c(cap),ic_run_pairs(if(sp[1]==1,1,2),cap));
      wl(out,concat(concat([id,S,rec[3],r,sd],res),[mcol]))));
}
main();
print("cell_ic ",ARM," s=",S," instances=",n," reps=",REPS," total_ms=",getabstime()-t00);
