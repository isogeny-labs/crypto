default(parisizemax,1000000000);
read(Str(getenv("GPDIR"),"/lib.gp"));
S=eval(getenv("S"));
recs=load_recs(S);
out=Str(OUTDIR,"/rho_",S,".csv");system(Str("rm -f ",out));
write(out,"armrec,s,inst,mode,j,seed,success,steps,esc,rst,GAE,NAT,wall_ms,GAEsetup,L,pred");
t00=getabstime();
main()={
  my(rs,n,nper,seeds,c,rec,pr,cap,sd,r,Qr,pr2,cap2);
  foreach([1,3,4],ar,
    rs=recs_arm(recs,ar);n=#rs;
    nper=if(ar==1,11,6);
    setrand(sbase(S,5,4)+ar-1);
    seeds=vector(n*nper,i,random(2^31));
    c=0;
    for(i=1,n,
      rec=rs[i];inst_setup(rec);
      pr=0.886*sqrt(LS);cap=40*ceil(max(pr,2000));
      for(j=1,5,c++;sd=seeds[c];setrand(sd);r=rho_run(GS,QS,LS,cap);wl(out,concat([ar,S,rec[3],1,j,sd],concat(r,[LS,pr]))));
      c++;sd=seeds[c];setrand(sd);Qr=ellmul(EC,GS,random(LS));r=rho_run(GS,Qr,LS,cap);wl(out,concat([ar,S,rec[3],3,1,sd],concat(r,[LS,pr])));
      if(ar==1,
        pr2=0.886*sqrt(NN);cap2=40*ceil(max(pr2,2000));
        for(j=1,5,c++;sd=seeds[c];setrand(sd);r=rho_run(GG,QQ,NN,cap2);wl(out,concat([ar,S,rec[3],2,j,sd],concat(r,[NN,pr2])))))));
}
main();
print("cell_rho s=",S," total_ms=",getabstime()-t00);
