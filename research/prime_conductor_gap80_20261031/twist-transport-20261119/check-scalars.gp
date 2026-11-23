require(ok,why)={if(!ok,print("FAIL: ",why);quit(2))};
point_hex(P,modulus)={if(type(P)=="t_INT",[0],[Mod(eval(P[1]),modulus),Mod(eval(P[2]),modulus)])};
check_all()={
  my(a_lines=readstr(getenv("SCALAR_AUDIT_INPUT")));
  my(a_defs=eval(a_lines[1]));
  my(a_curves=vector(#a_defs),a_moduli=vector(#a_defs),a_orders=vector(#a_defs),a_checked=0);
  for(i=1,#a_defs,
    my(v=a_defs[i],p=eval(a_defs[i][1]),q=a_defs[i][4]);
    require(isprime(p)&&isprime(q),"prime field/order");
    a_curves[i]=ellinit([eval(v[2]),eval(v[3])],p);a_moduli[i]=p;a_orders[i]=q
  );
  for(i=2,#a_lines,
    my(v=eval(a_lines[i]));my(E=a_curves[v[1]],p=a_moduli[v[1]],k=eval(v[2]));
    my(P=point_hex(v[3],p),R=point_hex(v[4],p));
    require(ellisoncurve(E,P)&&P!=[0]&&ellmul(E,P,a_orders[v[1]])==[0],"base membership/order");
    require(ellisoncurve(E,R)&&ellmul(E,P,k)==R,"independent scalar result");a_checked++
  );
  require(a_checked==672&&#a_defs==3,"complete expected input coverage");
  print("{\"independent_PARI_scalar_samples\":",a_checked,",\"curve_models\":",#a_defs,",\"all_outputs_match\":true,\"timings_independently_remeasured\":false}")
};
iferr(check_all(),err,print("FAIL: ",err);quit(2));
quit(0);
