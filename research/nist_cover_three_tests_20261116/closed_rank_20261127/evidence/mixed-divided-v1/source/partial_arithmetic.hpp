#include "sections_arithmetic.hpp"
#include <openssl/evp.h>
#include <iomanip>

using PL=std::array<F,3>;
struct PLine {PL coefficient;std::map<int,int> support;};
struct PProof {PL line;Poly restriction;std::map<int,int> support;SCRow old_row;int residual_column=-1,residual_sign=0;F residual_x,residual_y;Point residual_image;};
PL pnormalize(PL line){require(line[0]!=F(0)||line[1]!=F(0),"nonconstant projective line");F inv=(line[0]!=F(0)?line[0]:line[1]).inverse();for(auto&v:line)v=v*inv;return line;}
std::string pkey(const PL&line){return decimal(line[0])+":"+decimal(line[1])+":"+decimal(line[2]);}
PL pthrough(const SPoint&a,const SPoint&b){F dx=b.x-a.x,dy=b.y-a.y;return pnormalize({-dy,dx,dy*a.x-dx*a.y});}
struct PHash {
 std::unique_ptr<EVP_MD_CTX,decltype(&EVP_MD_CTX_free)> value{EVP_MD_CTX_new(),EVP_MD_CTX_free};
 PHash(){require(bool(value)&&EVP_DigestInit_ex(value.get(),EVP_sha256(),nullptr)==1,"pair-line hash init");}
 void add(size_t i,size_t j,const std::string&key){std::array<unsigned char,12> frame{};for(size_t b=0;b<4;++b){frame[b]=static_cast<unsigned char>((i>>(8*b))&255U);frame[4+b]=static_cast<unsigned char>((j>>(8*b))&255U);frame[8+b]=static_cast<unsigned char>((key.size()>>(8*b))&255U);}require(EVP_DigestUpdate(value.get(),frame.data(),frame.size())==1&&EVP_DigestUpdate(value.get(),key.data(),key.size())==1,"pair-line hash update");}
 std::string finish(){std::array<unsigned char,32> bytes{};unsigned int n=0;require(EVP_DigestFinal_ex(value.get(),bytes.data(),&n)==1&&n==32,"pair-line hash final");std::ostringstream out;for(auto b:bytes)out<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b);return out.str();}
};
void pcoverage(size_t pairs,size_t n){require(2*pairs==n*(n-1),"complete unordered pair coverage");}
struct PRestriction {Poly full,factor,residual;F x,y,dx,dy;int degree=0;};
PRestriction prestrict(const BP&c,const PLine&line,const std::vector<SPoint>&points){
 require(!line.support.empty(),"nonempty old support");const auto&origin=points.at(size_t(line.support.begin()->first));F dx=line.coefficient[1],dy=-line.coefficient[0];
 auto q=ss_restrict(c,origin.x,origin.y,dx,dy);require(q.degree()==4&&q.at(0)==F(0),"complete line restriction at old support");Poly factor{F(1)};int degree=0;
 for(const auto&[i,m]:line.support){const auto&p=points.at(size_t(i));require(m>=1&&m<=4&&line.coefficient[0]*p.x+line.coefficient[1]*p.y+line.coefficient[2]==F(0),"exact line membership and multiplicity bound");F t=dx!=F(0)?(p.x-origin.x)/dx:(p.y-origin.y)/dy;require(origin.x+dx*t==p.x&&origin.y+dy*t==p.y,"exact line parameter");for(int j=0;j<m;++j)factor=factor*Poly{-t,F(1)};degree+=m;}
 require(degree<=4,"old line Bezout bound");auto residual=q.exact(factor);require(residual.degree()==4-degree&&residual*factor==q,"complete old-factor division");return {q,factor,residual,origin.x,origin.y,dx,dy,degree};
}
std::array<F,2> presult(const PRestriction&r){require(r.degree==3&&r.residual.degree()==1,"one rational residual slot");F t=-r.residual.at(0)/r.residual.at(1);return {r.x+r.dx*t,r.y+r.dy*t};}
