// Evaluated identity.certificate/v1 records; this producer is not an independent verifier.
#include "json.hpp"
#include <gmpxx.h>
#include <openssl/sha.h>
#include <algorithm>
#include <array>
#include <cstdint>
#include <iostream>
struct Expr {std::string op,arg;long constant=0;std::vector<Expr> children;};
Expr var(std::string s){return {"var",s,0,{}};}Expr num(long x){return {"const","",x,{}};}
Expr add(std::initializer_list<Expr>a){return {"add","",0,a};}Expr mul(std::initializer_list<Expr>a){return {"mul","",0,a};}
Expr neg(Expr a){return {"neg","",0,{a}};}Expr sub(Expr a,Expr b){return add({a,neg(b)});}Expr power(Expr a,int n){Expr r=num(1);for(int i=0;i<n;++i)r=mul({r,a});return r;}
std::string expression(const Expr&e){std::string s="{\"args\":";if(e.op=="var")s+='"'+e.arg+'"';else if(e.op=="const")s+=std::to_string(e.constant);else if(e.op=="neg")s+=expression(e.children[0]);else{s+='[';for(size_t i=0;i<e.children.size();++i)s+=(i?",":"")+expression(e.children[i]);s+=']';}return s+",\"op\":\""+e.op+"\"}";}
int degree(const Expr&e){if(e.op=="var")return 1;if(e.op=="const")return 0;if(e.op=="neg")return degree(e.children[0]);int n=0;for(auto&c:e.children)n=e.op=="mul"?n+degree(c):std::max(n,degree(c));return n;}
std::array<unsigned char,32> hash(const std::string&s){std::array<unsigned char,32> h{};SHA256(reinterpret_cast<const unsigned char*>(s.data()),s.size(),h.data());return h;}
std::string hex(const std::array<unsigned char,32>&h){const char* d="0123456789abcdef";std::string s;for(auto c:h){s+=d[c>>4];s+=d[c&15];}return s;}
uint64_t draw(uint64_t&state){state+=0x9E3779B97F4A7C15ULL;uint64_t z=state;z=(z^(z>>30))*0xBF58476D1CE4E5B9ULL;z=(z^(z>>27))*0x94D049BB133111EBULL;return (z^(z>>31))%2305843009213693951ULL;}
mpz_class evaluate(const Expr&e,const std::map<std::string,mpz_class>&env){mpz_class q("2305843009213693951"),r;if(e.op=="var")return env.at(e.arg);if(e.op=="const")r=e.constant;else if(e.op=="neg")r=-evaluate(e.children[0],env);else{r=e.op=="mul"?1:0;for(auto&c:e.children){auto x=evaluate(c,env);if(e.op=="mul")r*=x;else r+=x;r%=q;}}r%=q;if(r<0)r+=q;return r;}
void issue(const std::string&path,const std::string&name,const std::vector<std::string>&vars,const Expr&lhs,const Expr&rhs,int bound){require(std::max(degree(lhs),degree(rhs))<=bound,"identity degree bound");std::string vs="[";for(size_t i=0;i<vars.size();++i){if(i)vs+=',';vs+='"';vs+=vars[i];vs+='"';}vs+=']';
    std::string statement="{\"degree_bound\":"+std::to_string(bound)+",\"lhs\":"+expression(lhs)+",\"name\":\""+name+"\",\"rhs\":"+expression(rhs)+",\"source\":\"research/nist_cover_three_tests_20261116/README.md\",\"variables\":"+vs+"}";
    std::string id="IDC1h"+hex(hash(statement)).substr(0,16),input=id;uint64_t seed=20261116;for(int i=0;i<8;++i)input+=char(seed>>(8*i));input+="cert";auto digest=hash(input);uint64_t state=0;for(int i=0;i<8;++i)state|=uint64_t(digest[i])<<(8*i);std::string values;
    for(int i=0;i<32;++i){std::map<std::string,mpz_class> env;for(auto&v:vars)env.emplace(v,mpz_class(static_cast<unsigned long>(draw(state))));auto l=evaluate(lhs,env),r=evaluate(rhs,env);require(l==r,"false identity");require(l!=evaluate(add({rhs,num(1)}),env),"corrupt identity accepted");uint64_t x=l.get_ui();for(int k=0;k<8;++k)values+=char(x>>(8*k));}
    std::ofstream out(path);require(bool(out),"identity output");out<<"{\"schema\":\"identity.certificate/v1\",\"id\":\""<<id<<"\",\"statement\":"<<statement<<",\"modulus\":2305843009213693951,\"points\":32,\"seed\":"<<seed<<",\"value_digest\":\""<<hex(hash(values))<<"\",\"independence\":\"Producer-issued evaluated record, not independently verified or formally proved\",\"proof\":null}\n";std::cout<<id<<" "<<path<<'\n';}
int main(int argc,char**argv){try{require(argc==2,"usage: issue_identity NEW-output-directory");std::string dir=argv[1];auto t=var("t"),d=var("d"),b=var("b"),a=add({power(t,2),d}),z=sub(power(t,2),num(1));issue(dir+"/cover-equation.identity.json","Cleared-denominator cover equation",{"t","d","b"},mul({z,add({power(a,3),neg(mul({num(3),a,power(z,2)})),mul({b,power(z,3)})})}),add({mul({power(a,3),z}),neg(mul({num(3),a,power(z,3)})),mul({b,power(z,4)})}),9);
    auto x=var("x"),c=var("c"),X=var("X"),xm=sub(x,num(1));issue(dir+"/pullback-fiber.identity.json","Quadratic pullback fiber after rational branch normalization",{"x","c","X","d"},sub(mul({xm,power(add({mul({c,X}),num(1)}),2)}),mul({add({x,d}),power(mul({c,X}),2)})),add({neg(mul({power(c,2),add({d,num(1)}),power(X,2)})),mul({num(2),c,xm,X}),xm}),5);
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
