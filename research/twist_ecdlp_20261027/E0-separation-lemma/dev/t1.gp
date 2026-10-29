default(parisizemax,2000000000);
p=101; g=ffprimroot(ffgen(p^2,'w)); one=g^0;
z=(5+7*g);
print(lift(z)," ",type(lift(z)));
print(lift(3*one)," ",type(lift(3*one)));
print(type(lift(0*one)), " ", lift(0*one));
print(z.pol, " ", z.p, " ", z.mod);
print((3*one).pol, " ", type((3*one).pol));
E=ellinit([0,0,0,3*one,5*one]);
P=random(E);Q=random(E);
my(t=getabstime());for(i=1,100000,S=elladd(E,P,Q));print("elladd 1e5: ",getabstime()-t," ms");
t=getabstime();for(i=1,100000,u=S[1]^p==S[1]&&S[2]^p==S[2]);print("frob test 1e5: ",getabstime()-t);
t=getabstime();for(i=1,100000,u=type(lift(S[1]))=="t_INT");print("lift test 1e5: ",getabstime()-t);
t=getabstime();for(i=1,100000,u=ellordinate(E,i*one));print("ellordinate 1e5: ",getabstime()-t);
b=ffprimroot(ffgen(2^20,'z)); h=b^(2^10+1); print(fforder(h));
print(lift(h)," ",type(lift(h)), " ", h.pol);
print(fflog(h,b));
\\ test ffembed
print(version());
