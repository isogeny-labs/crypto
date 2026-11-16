default(parisizemax,1000000000);
read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/jac.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/glue.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/ic.gp"); read("research/twist_ecdlp_followup_20261107/L3-genus2-measured-arm/rho.gp");
RHOMODE=0; RHOCHECK(xp) = everify(RG,RH,xp);
Q=1000003; QH=(Q-1)\2; 
test() =
{
  my(A, B, E, n, fa, l, G, H, x, res, tot=[0,0,0,0,0,0], ok=0, EAs);
  setrand(5);
  while(1, A=1+random(Q-1);B=1+random(Q-1); EA=A;EB=B; E=ellinit([A,B],Q); n=ellcard(E); fa=factor(n); l=fa[#fa~,1]; if(l>n/4,break));
  G=erandpt(); G=emul(n/l,G); x=1+random(l-1); H=emul(x,G); RG=G; RH=H;
  print("l=",l," sqrt(l/2)=",sqrt(l/2.));
  for(s=1,30, setrand(s); ezero(); res=rhorun(EGR,G,H,l,0); tot+=[res[1],res[3],res[4],res[5],res[6],res[7]]);
  print("solved=",tot[1]," mean steps/sqrt(l/2)=",tot[2]/30./sqrt(l/2.)," trails=",tot[3]/30.," dps=",tot[4]/30.," esc2/step=",tot[5]/tot[2]*1.," escB/step=",tot[6]/tot[2]*1.);
}
test();
quit
