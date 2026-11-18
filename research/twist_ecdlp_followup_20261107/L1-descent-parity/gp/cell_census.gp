default(parisizemax,1000000000);
read(Str(getenv("GPDIR"),"/lib.gp"));
S=eval(getenv("S"));
recs=load_recs(S);
out=Str(OUTDIR,"/census_",S,".csv");system(Str("rm -f ",out));
write(out,"armrec,s,inst,fbvariant,ntargets,ndegenerate,h0,h1,h2,h3,h4,h5plus,valid");
t00=getabstime();
main()={
  my(rs,n,per,F2,F3,H1,H2,seeds,rec,FA,FV,c);
  rs=recs_arm(recs,1);n=#rs;per=ceil(20000/n);
  F2=fb_precompute(rs,"B",sbase(S,2,3));F3=fb_precompute(rs,"B",sbase(S,2,13));
  setrand(sbase(S,6,21));seeds=vector(n,i,random(2^31));
  for(i=1,n,rec=rs[i];inst_setup(rec);build_tables();comb_build();
    FA=vector(PP,j,CLS[j]!=0);setrand(seeds[i]);
    c=census_inst(per,[FA,F2[i],F3[i]]);
    for(f=1,3,wl(out,concat([1,S,rec[3],f,c[1],c[2]],concat(c[3],[c[4][f]])))));
  rs=recs_arm(recs,4);n=#rs;per=ceil(20000/n);
  H1=fb_precompute(rs,"D1",sbase(S,6,3));H2=fb_precompute(rs,"D2",sbase(S,6,13));
  setrand(sbase(S,6,22));seeds=vector(n,i,random(2^31));
  for(i=1,n,rec=rs[i];inst_setup(rec);build_tables();comb_build();
    FV=vector(PP,j,YPT[j]!=0);setrand(seeds[i]);
    c=census_inst(per,[FV,H1[i],H2[i]]);
    for(f=1,3,wl(out,concat([4,S,rec[3],f,c[1],c[2]],concat(c[3],[c[4][f]])))));
}
main();
print("cell_census s=",S," total_ms=",getabstime()-t00);
