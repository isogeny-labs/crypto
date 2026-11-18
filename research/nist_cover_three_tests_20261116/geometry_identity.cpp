// Structural Riemann-form certificate over the identity/v1 field only.
#include "identity_arithmetic.hpp"
int main(int argc,char** argv){try{
    require(argc==2,"usage: geometry_identity NEW-output-directory");
    std::vector<std::string> vars{"x","y","z","a1","a2","a3","b1","b2","b3","c1","c2","c3"};
    auto linear=[](const std::string& prefix){return add({mul({var(prefix+"1"),var("x")}),mul({var(prefix+"2"),var("y")}),mul({var(prefix+"3"),var("z")})});};
    auto X=mul({var("x"),linear("a")}),Y=mul({var("y"),linear("b")}),Z=mul({var("z"),linear("c")});
    issue(std::string(argv[1])+"/riemann-form.identity.json","Riemann quartic structural expansion",vars,
        sub(power(sub(add({X,Y}),Z),2),mul({num(4),X,Y})),
        sub(add({power(X,2),power(Y,2),power(Z,2)}),mul({num(2),add({mul({X,Y}),mul({X,Z}),mul({Y,Z})})})),8);
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
