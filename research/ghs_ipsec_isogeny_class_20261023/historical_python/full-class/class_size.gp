\\ Counts class-group/curve classes; does NOT enumerate curves or test GHS.
\\ quadclassunit and qfbhclassno at this size are conditional on GRH.
default(parisizemax, 1024000000);
default(parisize, 128000000);
default(realprecision, 60);
Q=2^155;
N=45671926166590716193865565914344635196769237316;
t=Q+1-N;
D=t^2-4*Q;
print("VERSION=", version());
print("Q=", Q);
print("N=", N);
print("TRACE=", t);
print("DISCRIMINANT=", D);
print("FACTOR_DISCRIMINANT=", factor(-D));
print("IS_FUNDAMENTAL=", isfundamental(D));
gettime();
G=quadclassunit(D);
print("QUADCLASSUNIT=", G);
print("CLASSNUMBER_GRH=", G.no);
print("CLASSNUMBER_LOG2=", log(G.no)/log(2));
print("CLASSGROUP_MILLISECONDS=", gettime());
print("HURWITZ_GRH=", qfbhclassno(-D));
print("HURWITZ_MILLISECONDS=", gettime());
quit;
