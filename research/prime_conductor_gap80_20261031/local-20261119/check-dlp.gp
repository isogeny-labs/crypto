require(ok,why)={if(!ok,print("FAIL: ",why);quit(2))};
point_hex(P,p)=[Mod(eval(P[1]),p),Mod(eval(P[2]),p)];
check_all()={
 my(a_lines=readstr("research/prime_conductor_gap80_20261031/local-20261119/dlp-audit.gpvec"),a_checked=0);
 for(i=1,#a_lines,
  my(v=eval(a_lines[i]));my(p=eval(v[1]),q=v[4],k=eval(v[7]));
  require(isprime(p)&&isprime(q)&&k>=0&&k<q,"field/subgroup/recovery range");
  my(E=ellinit([eval(v[2]),eval(v[3])],p),G=point_hex(v[5],p),Q=point_hex(v[6],p));
  require(G!=[0]&&ellisoncurve(E,G)&&ellisoncurve(E,Q)&&ellmul(E,G,q)==[0],"exact prime generator order and target membership");
  require(ellmul(E,G,k)==Q,"independent recovered logarithm verification");a_checked++
 );
 require(a_checked==24,"complete expected recovery coverage");
 print("{\"independent_PARI_verified_recoveries\":",a_checked,",\"logarithms_re_solved\":false,\"all_outputs_match\":true}")
};
iferr(check_all(),err,print("FAIL: ",err);quit(2));quit(0);
