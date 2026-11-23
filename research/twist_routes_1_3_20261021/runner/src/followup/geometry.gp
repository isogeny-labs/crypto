\\ BHLS arXiv:1403.6911 Algorithm 5.4, both generic charts, Appendix A.
\\ This is a constructor. The separate Rust checker replays the map identities.
default(parisizemax,4000000000);
twist_scale(p,aa,bb,A,B)={
  if(aa==0||bb==0,return([]));
  my(u2=B*aa/(A*bb));
  if(u2^2*aa!=A||u2^3*bb!=B||kronecker(lift(u2),p)!=1,return([]));
  [sqrt(u2)];
};
twist_emit(name,a,b,c,d,t,u,v)={
  print("CANDIDATE {\"source\":\"",name,"\",\"a\":\"",lift(a),"\",\"b\":\"",lift(b),"\",\"c\":\"",lift(c),"\",\"d\":\"",lift(d),"\",\"t\":\"",lift(t),"\",\"u1\":\"",lift(u),"\",\"u2\":\"",lift(v),"\"}");
};
twist_models(p,A,B,D,name,a,b,c,d)={
  my(da=a^3+b^2,dc=c^3+d^2);
  if(da==0||dc==0||12*a*c+16*b*d!=1,return(0));
  my(e12=12*(2*a^2*d-b*c),e14=12*(16*a*d^2+3*c^2)*da,e16=512*da^2*d^3);
  my(e22=12*(2*b*c^2-a*d),e24=12*(16*b^2*c+3*a^2)*dc,e26=512*dc^2*b^3);
  my(a1=e14-e12^2/3,b1=e16-e12*e14/3+2*e12^3/27);
  my(a2=e24-e22^2/3,b2=e26-e22*e24/3+2*e22^3/27,n=0);
  for(i=1,2,
    my(t=if(i==1,Mod(1,p),D),u=twist_scale(p,t^2*a1,t^3*b1,A,B));
    if(#u,
      my(v=twist_scale(p,t^2*a2,t^3*b2,D^2*A,D^3*B));
      if(#v,twist_emit(name,a,b,c,d,t,u[1],v[1]);n++);
    );
  );n;
};
twist_construct(p,A,B,D,name)={
  A=Mod(A,p);B=Mod(B,p);D=Mod(D,p);
  my(j=1728*4*A^3/(4*A^3+27*B^2),aa='y,cc='x,dd=(1-12*aa*cc)/(16*aa));
  if(j==0||j==Mod(1728,p),error("special j requires the other published branches"));
  my(g1=numerator(1728*(aa^2*cc+4*aa^2*dd-4*aa^2*cc^2)^3-j*(aa^3+aa^2)^2*(cc^3+dd^2)));
  my(g2=numerator(1728*(aa*cc^2+4*aa*cc*dd-4*aa^2*dd^2)^3-j*(aa^3+aa^2)*(cc^3+dd^2)^2));
  my(R=polresultant(g1,g2,'x));
  if(R==0,error("zero resultant: saturation or chart issue"));
  my(roots=polrootsmod(R,p),n=0,quads=0);
  print("RESULTANT ",name," degree=",poldegree(R)," rational_a_roots=",#roots);
  for(i=1,#roots,
    my(a=roots[i]);if(a==0||a==-1,next);
    my(h=gcd(subst(g1,'y,a),subst(g2,'y,a)));
    if(h==0,error("zero specialized gcd"));
    if(poldegree(h,'x)<1,next);
    my(cs=polrootsmod(h,p));
    for(k=1,#cs,
      my(c=cs[k],d=(1-12*a*c)/(16*a));
      if((a^3+a^2)*(c^3+d^2)==0,next);
      quads++;
      n+=twist_models(p,A,B,D,name,a,a,c,d);
      n+=twist_models(p,A,B,D,name,c,d,a,a);
    );
  );
  print("GEOMETRY_SEARCH_DONE ",name," generic_quads=",quads," matched_candidates=",n);
};
