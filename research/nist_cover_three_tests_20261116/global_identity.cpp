// Generic adjugate controls in the limited identity.certificate/v1 field.
#include "identity_arithmetic.hpp"
int main(int argc,char** argv){try{
    require(argc==2,"usage: global_identity NEW-output-directory");std::string dir=argv[1];auto d=var("d"),e=var("e"),f=var("f"),g=var("g"),h=var("h"),i=var("i");auto q0=sub(mul({e,i}),mul({f,h})),q1=sub(mul({f,g}),mul({d,i})),q2=sub(mul({d,h}),mul({e,g}));
    issue(dir+"/adjugate-row1.identity.json","Generic first adjugate column annihilates second matrix row",{"d","e","f","g","h","i"},add({mul({d,q0}),mul({e,q1}),mul({f,q2})}),num(0),3);
    issue(dir+"/adjugate-row2.identity.json","Generic first adjugate column annihilates third matrix row",{"d","e","f","g","h","i"},add({mul({g,q0}),mul({h,q1}),mul({i,q2})}),num(0),3);
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
