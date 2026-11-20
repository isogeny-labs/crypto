default(parisizemax,4000000000);
twist_higher_quote(s)={Str(Strchr(34),s,Strchr(34))};
twist_higher_point(P)={if(P==[0],"null",Str("[",twist_higher_quote(P[1]),",",twist_higher_quote(P[2]),"]"))};
twist_higher_construct(p,a,b,n,d,l,k,source)={
 my(m=ffinit(p,k,'t),z=ffgen(m,'z),E=ellinit([a,b],z),t=p+1-n,s0=2,s1=t,sk,N,c,valuation=0,P=[0],Q=[0],attempts=0,w=sqrt(d*z^0));
 for(j=2,k,sk=t*s1-p*s0;s0=s1;s1=sk);N=p^k+1-s1;c=N;
 while(c%l==0,c=c/l;valuation++);if(valuation<2,error("extension order lacks full torsion"));
 setrand(202610230020+p*l);
 for(j=1,128,
  attempts++;my(xx=sum(i=0,k-1,random(p)*z^i),ys=ellordinate(E,xx),R);
  if(#ys==0,next);R=ellmul(E,[xx,ys[1]],c);
  while(R!=[0]&&ellmul(E,R,l)!=[0],R=ellmul(E,R,l));if(R==[0],next);
  if(P==[0],P=R;next);if(ellweilpairing(E,P,R,l)!=1,Q=R;break);
 );
 if(Q==[0],error("bounded torsion basis search exhausted"));
 print("CERTIFICATE {\"source\":",twist_higher_quote(source),",\"ell\":",l,",\"degree\":",k,",\"p\":",p,",\"modulus\":",Vecrev(lift(m)),",\"w\":",twist_higher_quote(w),",\"basis\":[",twist_higher_point(P),",",twist_higher_point(Q),"],\"pairing\":",twist_higher_quote(ellweilpairing(E,P,Q,l)),",\"extension_order\":",twist_higher_quote(N),",\"cofactor\":",twist_higher_quote(c),",\"ell_valuation\":",valuation,",\"point_trials\":",attempts,"}");
};
