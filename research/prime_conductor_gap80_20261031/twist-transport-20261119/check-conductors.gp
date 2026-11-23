\\ Ordinary endomorphism-order certificate arithmetic, independent of native map.
must(v,msg)=if(!v,error(msg));
{
 iferr(
 my(cv=read("case0.gpvec"),pr=cv[1],degree=cv[4],subgroup=cv[11],E,C,trace,disc,hilbert);
 must(isprime(pr)&&isprime(degree)&&isprime(subgroup),"primes");
 E=ellinit([0,0,0,cv[7],cv[8]],pr);C=ellinit([0,0,0,cv[9],cv[10]],pr);
 must(ellcard(E)==subgroup&&ellcard(C)==subgroup,"fresh endpoint counts");
 trace=pr+1-subgroup;must(gcd(trace,pr)==1,"ordinary criterion");
 disc=trace^2-4*pr;must(disc==-19*degree^2,"Frobenius discriminant");
 hilbert=polclass(-19);must(hilbert==x+884736&&qfbclassno(-19)==1,"maximal CM class polynomial");
 must(subst(hilbert,x,E.j)==0&&subst(hilbert,x,C.j)!=0,"source maximal and target nonmaximal j");
 write("cm-certificate.gpvec",lift([pr,degree,subgroup,trace,disc,-19,E.j,C.j,Vec(hilbert),1,degree]));
 print("PASS ordinary CM-order certificate arithmetic: source conductor1; target conductor",degree,"; prime bit length",exponent(degree)+1);
 print("Hilbert class polynomial: ",hilbert,"; log2(conductor ratio)=",log(degree)/log(2));
 ,failure,print(failure);quit(2));
}
quit(0);
