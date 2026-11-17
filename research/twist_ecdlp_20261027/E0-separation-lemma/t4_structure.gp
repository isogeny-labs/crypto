\\ t4_structure.gp -- Tier T4: abstract group structure E(F_{q^2}) vs E(F_q) x E'(F_q).
PMIN=ENVI("E0_PMIN",3); PMAX=ENVI("E0_PMAX",101); MMIN=ENVI("E0_MMIN",2); MMAX=ENVI("E0_MMAX",6);
t4run(cl)={
  my(n=#cl,n1=0,n1iso=0,n2=0,n2iso=0,nmis=0);
  for(i=1,n, my(r=t4class(cl[i][1],cl[i][2],cl[i][3])); emit(r);
    if(r[9]==1,n1++;if(r[15],n1iso++),n2++;if(r[15],n2iso++)); if(r[16]=="MISMATCH",nmis++));
  emit(["SUMMARY","T4",Q_,n,Str("g2eq1=",n1),Str("g2eq1_abstract_iso=",n1iso),Str("g2gt1=",n2),Str("g2gt1_abstract_iso=",n2iso),Str("native_mismatch=",nmis)]);
};
{forprime(p=max(3,PMIN),PMAX, setfield(p,1); t4run(classes_odd(p)));
if(ENVI("E0_NOBIN",0)==0,
  for(m=MMIN,MMAX, setfield(2,m); t4run(classes_ord());
    if(m<=5, t4run(classes_ss(m<=4)))));}
print("DONE|T4");
