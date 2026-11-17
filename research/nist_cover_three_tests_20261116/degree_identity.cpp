// Generic polynomial controls only; no finite-field inequality is asserted.
#include "identity_arithmetic.hpp"
int main(int argc,char** argv){try{
    require(argc==2,"usage: degree_identity NEW-output-directory");std::string dir=argv[1];auto A=var("A"),B=var("B"),C=var("C"),D=var("D"),h=var("h");
    issue(dir+"/quadratic-norm.identity.json","Generic quadratic-algebra norm multiplicativity",{"A","B","C","D","h"},sub(power(add({mul({A,C}),mul({h,B,D})}),2),mul({h,power(add({mul({A,D}),mul({B,C})}),2)})),mul({sub(power(A,2),mul({h,power(B,2)})),sub(power(C,2),mul({h,power(D,2)}))}),6);
    auto x=var("x"),y=var("y"),z=var("z");issue(dir+"/cubic-equality.identity.json","Generic cubic equality identity used with positive real roots",{"x","y","z"},mul({num(2),sub(add({power(x,3),power(y,3),power(z,3)}),mul({num(3),x,y,z}))}),mul({add({x,y,z}),add({power(sub(x,y),2),power(sub(y,z),2),power(sub(z,x),2)})}),3);
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
