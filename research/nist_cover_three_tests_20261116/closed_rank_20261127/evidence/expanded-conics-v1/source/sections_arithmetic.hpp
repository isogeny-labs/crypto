#include "fibre_arithmetic.hpp"
#include "rank_roots.hpp"

struct SPoint{F x,y;Point image;int index=-1,sign=0;};
struct Section{int origin=0;std::array<F,3> line;Poly restriction;std::array<int,4> support;SCRow row;};
std::string sp_key(const F&x,const F&y){return decimal(x)+":"+decimal(y);}
Poly ss_restrict(const BP&c,const F&x,const F&y,const F&dx,const F&dy){
 std::array<Poly,5> xx,yy;xx[0]=Poly{F(1)};yy[0]=Poly{F(1)};for(size_t i=1;i<5;++i){xx[i]=xx[i-1]*Poly{x,dx};yy[i]=yy[i-1]*Poly{y,dy};}Poly out;for(const auto&[e,a]:c.t)out=out+(xx[size_t(e[0])]*yy[size_t(e[1])]).scale(a);return out;
}
F ss_eval(const Poly&p,const F&t){F r;for(int i=p.degree();i>=0;--i)r=r*t+p.at(i);return r;}
std::vector<F> ss_quadratic(const Poly&p){
 require(p.degree()==2,"complete residual quadratic");F disc=p.at(1)*p.at(1)-F(4)*p.at(0)*p.at(2),root;BN e(BN_dup(prime()),BN_free);require(bool(e)&&BN_add_word(e.get(),1)==1&&BN_rshift(e.get(),e.get(),2)==1,"fixed square-root exponent");require(BN_mod_exp(root.n.get(),disc.n.get(),e.get(),prime(),context())==1,"quadratic square root");if(root*root!=disc)return {};F inv=(F(2)*p.at(2)).inverse();std::vector<F> r{(-p.at(1)+root)*inv,(-p.at(1)-root)*inv};require((Poly{-r[0],F(1)}*Poly{-r[1],F(1)}).scale(p.at(2))==p,"complete residual factor product");return r;
}
void ss_private(const Json&baseline,const Json&tangents){
 const auto&j=tangents.at("matrix").arr;require(j.size()==108&&tangents.at("original_base_width").integer()==225,"original tangent dimensions");auto old=sc_read_matrix(tangents.at("matrix"));require(old[0].size()==429,"original tangent width");std::vector<int> owners(108);for(size_t col=225;col<429;++col){int owner=-1;for(size_t i=0;i<108;++i)if(sc_reduce(old[i][col])!=0){require(owner<0,"original auxiliary must remain private");owner=int(i);}require(owner>=6,"original auxiliary owner");++owners[size_t(owner)];}for(size_t i=6;i<108;++i)require(owners[i]==2,"all original tangent rows have private witnesses");
 SCMat expected(6,SCRow(225));require(baseline.at("rows").arr.size()==6,"six original sparse rows");for(size_t i=0;i<6;++i){for(const auto&t:baseline.at("rows").arr[i].arr)expected[i].at(size_t(t.arr.at(0).integer()))+=SCBig(t.arr.at(1).scalar);for(size_t k=0;k<225;++k)require(sc_reduce(expected[i][k])==sc_reduce(old[i][k]),"original baseline coefficient binding");for(size_t k=225;k<429;++k)require(sc_reduce(old[i][k])==0,"original baseline has no auxiliary");}require(sc_rank(expected)==6,"recomputed original rank");
}
