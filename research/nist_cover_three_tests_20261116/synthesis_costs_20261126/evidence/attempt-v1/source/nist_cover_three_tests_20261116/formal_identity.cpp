// Limited-field structural control; does not certify the P192 correspondence.
#include "identity_arithmetic.hpp"
int main(int argc,char** argv){try{
    require(argc==2,"usage: formal_identity NEW-output-directory");std::string dir=argv[1];auto x=var("x"),y=var("y"),z=var("z"),p=add({x,y,z});auto p2=add({power(x,2),power(y,2),power(z,2)}),p3=add({power(x,3),power(y,3),power(z,3)});
    issue(dir+"/newton-cubic.identity.json","Generic third elementary symmetric polynomial from three traces",{"x","y","z"},add({power(p,3),neg(mul({num(3),p,p2})),mul({num(2),p3})}),mul({num(6),x,y,z}),3);
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
