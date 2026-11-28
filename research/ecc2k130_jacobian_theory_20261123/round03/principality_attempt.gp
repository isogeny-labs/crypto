default(parisize,268435456);
default(parisizemax,536870912);
read("field.gp");
attempt() = {
  my(B,P,V);
  print("BEGIN fixed ideal certificate attempt; any class-group nonprincipality is provisional without bnfcertify");
  B=bnfinit(H,1);
  print("BNFINIT returned; GRH certification still outstanding");
  P=idealadd(B,263,Mod(x+1,H));
  if(idealnorm(B,P)!=263,error("wrong ideal norm"));
  V=bnfisprincipal(B,P,4);
  print("ordinary_class_vector=",V[1]);
  print("compact_generator=",V[2]);
};
iferr(attempt(),E,print("PARI_ERROR=",E);quit(2));
quit;
