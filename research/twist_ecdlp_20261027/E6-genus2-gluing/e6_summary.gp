\\ e6_summary.gp -- tally the raw outputs.  Env: E6_FILES = comma-separated list of main outputs,
\\ E6_CFILES = comma-separated list of controls outputs.  Reads only; prints SUMMARY lines.
default(parisizemax,2000000000);
fld(s,k)={my(v=Vecsmall(s),key=Vecsmall(concat(" ",concat(k,"="))),n=#v,m=#key,i,j);
  for(i=1,n-m+1,if(v[i..i+m-1]==key,j=i+m;my(e=j);while(e<=n&&v[e]!=32,e++);return(Strchr(v[j..e-1]))));""};
rate(k,n)={my(z=1.959964,ph=if(n>0,k/n,0.),den=1+z^2/n,ctr=(ph+z^2/(2*n))/den,hw=z*sqrt(ph*(1-ph)/n+z^2/(4*n^2))/den);
  if(n==0,return([0.,0.,0.]));[ph,max(0.,ctr-hw),min(1.,ctr+hw)]};
lines(f)=readstr(f);
tally(files)={my(L=List(),ps=Set());
  foreach(strsplit(files,","),f,foreach(readstr(f),s,listput(L,s)));Vec(L)};
pre(s,t)=(#s>=#t&&Vec(Vecsmall(s)[1..#t])==Vec(Vecsmall(t)));
GP=[];
main2()={my(L=tally(getenv("E6_FILES")),pset=Set());
  foreach(L,s,if(pre(s,"A "),pset=setunion(pset,Set([eval(fld(s,"p"))]))));
  foreach(pset,p,
    my(nA=0,aok=0,b0=0,b0ok=0,b1id=0,b1g2=0,b1sing=0,b1nsf=0,cT=0,cTm=0,cTp=0,cTq=0,cTd=0,cTi=0,cC=0,cCm=0,cCe=0,c2=0,c2ok=0,c2p1=0,
       ct=List(),njr=List(),cf=List(),np=0);
    foreach(L,s,
      if(fld(s,"p")!=Str(p),next);
      if(pre(s,"A "),nA++;
        if(fld(s,"sum_ok")=="1"&&fld(s,"twistcard_ok")=="1"&&fld(s,"divpat_eq")=="1"&&fld(s,"rank2_eq")=="1"&&fld(s,"rootscale_ok")=="1"&&fld(s,"jeq")=="1",aok++));
      if(pre(s,"B0 "),b0++;if(fld(s,"equiv_eq_interp")=="1"&&fld(s,"n_equiv")==fld(s,"n_equiv_expected"),b0ok++));
      if(pre(s,"B1 "),
        if(#strsplit(s,"status=identity_g2=0")>1,b1id++);
        if(#strsplit(s,"status=g2zero")>1,b1g2++);
        if(#strsplit(s,"status=singular")>1,b1sing++);
        if(#strsplit(s,"status=sextic_not_squarefree")>1,b1nsf++));
      if(pre(s,"C "),
        if(fld(s,"arm")=="target",cT++;
          if(fld(s,"match_target")=="1",cTm++);
          if(fld(s,"nJ_eq_prod")=="1",cTp++);
          if(fld(s,"n1_eq_nE")=="1"&&fld(s,"n2_eq_nEt")=="1",cTq++);
          if(fld(s,"nE_div_nJ")=="1",cTd++);
          if(fld(s,"ident_ok")=="1",cTi++));
        if(fld(s,"arm")=="control",cC++;
          if(fld(s,"match_control")=="1",cCm++);
          if(fld(s,"nJ_eq_prod")=="1",cCe++)));
      if(pre(s,"C2 "),c2++;
        if(fld(s,"brute_eq_pari")=="1",c2ok++);
        if(fld(s,"arm")=="target"&&fld(s,"N1_eq_p+1")=="1",c2p1++));
      if(pre(s,"D "),listput(ct,eval(fld(s,"ctie")));listput(cf,eval(fld(s,"ctie_full")));listput(njr,eval(fld(s,"nJ_over_p2")));if(fld(s,"nE_isprime")=="1",np++)));
    ct=vecsort(Vec(ct));njr=vecsort(Vec(njr));cf=vecsort(Vec(cf));
    print("SUMMARY p=",p," curves=",nA," A_all_ok=",aok," B0_n=",b0," B0_ok=",b0ok,
          " B1_identity_nosol=",b1id," B1_g2zero=",b1g2," B1_singular=",b1sing," B1_sextic_nsf=",b1nsf);
    print("SUMMARY p=",p," C_target_n=",cT," match_target=",cTm," nJ_eq_prod=",cTp," quotients_ok=",cTq," nE_div_nJ=",cTd," ident_ok=",cTi,
          " C_control_n=",cC," match_control=",cCm," control_nJ_eq_prod=",cCe);
    print("SUMMARY p=",p," C2_n=",c2," brute_eq_pari=",c2ok," N1_eq_p+1_target=",c2p1);
    if(#ct,GP=concat(GP,[[p,ct[(#ct+1)\2],cf[(#cf+1)\2]]]);print("SUMMARY p=",p," nE_prime_count=",np," ctie_full_min=",cf[1]," ctie_full_med=",cf[(#cf+1)\2]," ctie_full_max=",cf[#cf]," bound_full=",1.2533141373155*(sqrt(p)+1)/p);
      print("SUMMARY p=",p," ctie_min=",ct[1]," ctie_med=",ct[(#ct+1)\2]," ctie_max=",ct[#ct]," nJ_over_p2_min=",njr[1]," nJ_over_p2_max=",njr[#njr]));
  );
  if(#GP>=2,my(n=#GP,X=vector(n,i,log(GP[i][1])),Y1=vector(n,i,log(GP[i][2])),Y2=vector(n,i,log(GP[i][3])),sx=sum(i=1,n,X[i]),sxx=sum(i=1,n,X[i]^2));
    print("SUMMARY slope_logctie_med_vs_logp=",(n*sum(i=1,n,X[i]*Y1[i])-sx*sum(i=1,n,Y1[i]))/(n*sxx-sx^2),
          " slope_logctie_full_med_vs_logp=",(n*sum(i=1,n,X[i]*Y2[i])-sx*sum(i=1,n,Y2[i]))/(n*sxx-sx^2)))};
mainc()={my(L=tally(getenv("E6_CFILES")));
  foreach(L,s,
    if(pre(s,"POS "),my(r=rate(eval(fld(s,"hits")),eval(fld(s,"n"))));print("CSUMMARY ",s," wilson95=",r));
    if(pre(s,"NEGGAL "),my(r=rate(eval(fld(s,"flagged_isomorphic")),eval(fld(s,"pairs"))));print("CSUMMARY ",s," wilson95=",r));
    if(pre(s,"NULL "),my(r=rate(eval(fld(s,"hits")),eval(fld(s,"n"))),r2=rate(eval(fld(s,"even_charpoly")),eval(fld(s,"n"))));print("CSUMMARY ",s," hit_wilson95=",r," even_wilson95=",r2)))};
if(getenv("E6_FILES")!=0&&getenv("E6_FILES")!="",main2());
if(getenv("E6_CFILES")!=0&&getenv("E6_CFILES")!="",mainc());
quit;
