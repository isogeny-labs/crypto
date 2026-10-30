g=ffgen(2^8,'z); h=g^(2^4+1); print("subfield gen order ", fforder(h));
f=factor(x^2+x+h); print("factor rows ", matsize(f)[1]);
print("ffprimroot ", fforder(ffprimroot(g)));
print("env: ", getenv("E0_TEST"));
E=ellinit([0,0,0,1,1]*Mod(1,5)); print("p=5 depressed ok ", ellcard(E));
E3=ellinit([0,2,0,1,1]*Mod(1,3)); print("p=3 general ok ", ellcard(E3));
print(ellgroup(ellinit([0,0,0,1,1]*ffgen(5^2,'w))));
