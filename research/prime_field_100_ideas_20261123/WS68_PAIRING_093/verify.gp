\\ Independent small-group enumeration and point check for published #93 result.
verify()=
{
  my(p=43,r=11,count=1,v,E,G,H,matches=0,found=-1);
  for(a=0,p-1,
    v=(a^3+a)%p;
    if(v==0,count++,if(kronecker(v,p)==1,count+=2))
  );
  if(count!=44,error("independent curve count mismatch"));
  E=ellinit([0,0,0,Mod(1,p),Mod(0,p)]);
  G=[Mod(31,p),Mod(18,p)]; H=[Mod(13,p),Mod(24,p)];
  if(!ellisoncurve(E,G) || !ellisoncurve(E,H) || ellmul(E,G,r)!=[0],error("point/group mismatch"));
  for(a=0,r-1,if(ellmul(E,G,a)==H,matches++;found=a));
  if(matches!=1 || found!=7,error("unique scalar mismatch"));
  print("VERIFY_OK=1 curve_count=",count," subgroup=",r," unique_scalar=",found," matches=",matches);
};
verify();
