\\ Public fixture generation, independent of solver outcomes and arm.
\\ First prime-order nonsingular a=-3 curve at each fixed prime modulus.
default(parisizemax, 1000000000);
{
  forstep(bits=44,56,4,
    my(p=precprime(2^bits), a=p-3, found=0);
    for(b=1,8192,
      if(Mod(4*a^3+27*b^2,p)==0,next);
      my(E=ellinit([a,b],p), r=ellcard(E));
      if(!isprime(r),next);
      for(x=1,1000,
        my(v=Mod(x^3+a*x+b,p));
        if(!issquare(v),next);
        my(y=lift(sqrt(v))); y=min(y,p-y);
        print(bits," ",p," ",a," ",b," ",r," ",x," ",y);
        found=1;break;
      );
      if(found,break);
    );
    if(!found,error("fixture generation exhausted"));
  );
}
quit;
