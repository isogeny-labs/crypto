// Generic cleared involution identity; identity/v1 field only.
#include "identity_arithmetic.hpp"
int main(int argc,char** argv){try{
    require(argc==2,"usage: involution_identity NEW-output-directory");auto x=var("x"),c=var("c"),cx=mul({c,x}),d=add({num(1),mul({num(2),cx})});
    std::vector<Expr> lhs,rhs;std::vector<std::string> vars{"x","c"};
    for(int k=0;k<=4;++k){auto name="a"+std::to_string(2*k);vars.push_back(name);auto a=var(name);lhs.push_back(mul({a,power(neg(cx),8-2*k),power(sub(d,cx),2*k)}));rhs.push_back(mul({a,power(cx,8-2*k),power(add({num(1),cx}),2*k)}));}
    issue(std::string(argv[1])+"/cover-involution.identity.json","Generic cleared even-cover involution",vars,Expr{"add","",0,lhs},Expr{"add","",0,rhs},17);
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
