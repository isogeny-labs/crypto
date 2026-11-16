for(i=1,2,t=if(i==1,2,24);r=if(i==1,71,47);P=X^10-t*X^5+3125;d=t^2-12500;print("trace=",t," prime=",r," discriminant_formula=",poldisc(P)==5^30*d^5," residual=",d," remainder=",d%r," factors=",factor(abs(d))))
quit;
