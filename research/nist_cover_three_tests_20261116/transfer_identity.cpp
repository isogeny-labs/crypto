// Structural algebra supporting resultants and auxiliary-divisor cancellation.
#include "identity_arithmetic.hpp"
int main(int argc,char** argv){try{
    require(argc==2,"usage: transfer_identity NEW-output-directory");std::string dir=argv[1];auto a=var("a"),b=var("b"),c=var("c"),d=var("d"),e=var("e"),f=var("f"),g=var("g"),h=var("h"),i=var("i");
    auto p=sub(mul({a,e}),mul({d,b})),q=sub(mul({a,f}),mul({d,c})),r=sub(mul({a,h}),mul({g,b})),s=sub(mul({a,i}),mul({g,c}));auto det=add({mul({a,sub(mul({e,i}),mul({f,h}))}),neg(mul({b,sub(mul({d,i}),mul({f,g}))})),mul({c,sub(mul({d,h}),mul({e,g}))})});
    issue(dir+"/bareiss-step.identity.json","Generic three-by-three fraction-free determinant step",{"a","b","c","d","e","f","g","h","i"},sub(mul({p,s}),mul({q,r})),mul({a,det}),4);
    auto A=var("A"),B=var("B"),R=var("R"),D=var("D");issue(dir+"/auxiliary-cancellation.identity.json","Generic linear auxiliary-divisor cancellation",{"A","B","R","D"},sub(sub(add({A,R}),D),sub(add({B,R}),D)),sub(A,B),1);
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
