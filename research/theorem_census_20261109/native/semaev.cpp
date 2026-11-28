#include "finite.hpp"
#include <array>
#include <functional>
#include <iostream>
#include <map>
#include <numeric>
#include <string>
#include <unordered_map>

// Sparse polynomials, six exponent nibbles. S5 exponents never exceed eight.
struct Poly {
 const Field* f;
 std::unordered_map<unsigned,int> terms;
 explicit Poly(const Field& ff,int c=0):f(&ff){if(c)terms[0]=c;}
 void put(unsigned m,int c){if(!c)return;auto it=terms.find(m);if(it==terms.end())terms[m]=c;else{int z=f->add(it->second,c);if(z)it->second=z;else terms.erase(it);}}
 static Poly X(const Field& f,int i){Poly z(f);z.put(1u<<(4*i),1);return z;}
 Poly operator+(const Poly& y)const{Poly z=*this;for(auto [m,c]:y.terms)z.put(m,c);return z;}
 Poly operator-()const{Poly z(*f);for(auto [m,c]:terms)z.put(m,f->neg(c));return z;}
 Poly operator-(const Poly& y)const{return *this+(-y);}
 Poly operator*(const Poly& y)const{
  Poly z(*f);
  for(auto [m,c]:terms)for(auto [n,d]:y.terms){
   for(int i=0;i<6;i++)if(((m>>(4*i))&15)+((n>>(4*i))&15)>=16)throw std::runtime_error("exponent overflow");
   z.put(m+n,f->mul(c,d));
  }
  return z;
 }
 Poly operator*(int c)const{Poly z(*f);for(auto [m,d]:terms)z.put(m,f->mul(c,d));return z;}
 Poly power(int e)const{Poly z(*f,1),a=*this;while(e){if(e&1)z=z*a;e>>=1;if(e)a=a*a;}return z;}
 int degree(int i)const{int d=-1;for(auto [m,c]:terms){(void)c;d=std::max(d,int((m>>(4*i))&15));}return d;}
 Poly coefficient(int i,int e)const{Poly z(*f);for(auto [m,c]:terms)if(int((m>>(4*i))&15)==e)z.put(m&~(15u<<(4*i)),c);return z;}
 Poly eval(int i,int x)const{Poly z(*f);for(auto [m,c]:terms)z.put(m&~(15u<<(4*i)),f->mul(c,f->power(x,(m>>(4*i))&15)));return z;}
};
using Quad=std::array<Poly,3>; // descending coefficients A,B,C
Quad s3coeff(const Field& f,const Poly& x,const Poly& y,int a,int b,bool binary){
 Poly aa(f,a),bb(f,b),xy=x*y;
 if(binary)return {(x+y).power(2),xy,xy.power(2)+bb};
 return {(x-y).power(2),((x+y)*(xy+aa)+bb*f.c(2))*f.c(-2),(xy-aa).power(2)-(x+y)*f.mul(f.c(4),b)};
}
Poly res22(const Quad& u,const Quad& v){
 const auto& A=u[0];const auto& B=u[1];const auto& C=u[2];
 const auto& D=v[0];const auto& E=v[1];const auto& F=v[2];const auto& f=*A.f;
 return A.power(2)*F.power(2)-A*B*E*F-A*C*D*F*f.c(2)+A*C*E.power(2)+B.power(2)*D*F-B*C*D*E+C.power(2)*D.power(2);
}
Poly res24(const Quad& u,const std::array<Poly,5>& v){
 const auto& A=u[0];const auto& B=u[1];const auto& C=u[2];
 const auto& D=v[0];const auto& E=v[1];const auto& F=v[2];const auto& G=v[3];const auto& H=v[4];const Field& f=*A.f;
 // Universal 6x6 Sylvester determinant expanded before specializing.
 return A.power(4)*H.power(2)-A.power(3)*B*G*H-A.power(3)*C*F*H*f.c(2)
 +A.power(3)*C*G.power(2)+A.power(2)*B.power(2)*F*H+A.power(2)*B*C*E*H*f.c(3)
 -A.power(2)*B*C*F*G+A.power(2)*C.power(2)*D*H*f.c(2)-A.power(2)*C.power(2)*E*G*f.c(2)
 +A.power(2)*C.power(2)*F.power(2)-A*B.power(3)*E*H-A*B.power(2)*C*D*H*f.c(4)
 +A*B.power(2)*C*E*G+A*B*C.power(2)*D*G*f.c(3)-A*B*C.power(2)*E*F
 -A*C.power(3)*D*F*f.c(2)+A*C.power(3)*E.power(2)+B.power(4)*D*H
 -B.power(3)*C*D*G+B.power(2)*C.power(2)*D*F-B*C.power(3)*D*E+C.power(4)*D.power(2);
}
Poly S3(const Field& f,int a,int b,bool binary){
 auto x=Poly::X(f,0),y=Poly::X(f,1),z=Poly::X(f,2);auto c=s3coeff(f,x,y,a,b,binary);return c[0]*z.power(2)+c[1]*z+c[2];
}
Poly S4(const Field& f,int a,int b,bool binary,const std::array<int,4>& vars){
 return res22(s3coeff(f,Poly::X(f,vars[0]),Poly::X(f,vars[1]),a,b,binary),s3coeff(f,Poly::X(f,vars[2]),Poly::X(f,vars[3]),a,b,binary));
}
Poly S5(const Field& f,int a,int b,bool binary){
 Poly inner=S4(f,a,b,binary,{2,3,4,5});
 std::array<Poly,5> v{inner.coefficient(5,4),inner.coefficient(5,3),inner.coefficient(5,2),inner.coefficient(5,1),inner.coefficient(5,0)};
 return res24(s3coeff(f,Poly::X(f,0),Poly::X(f,1),a,b,binary),v);
}

int determinant(const Field& f,const std::vector<std::vector<int>>& M){
 int n=M.size(),z=0;std::vector<int> perm(n);std::iota(perm.begin(),perm.end(),0);
 do{int term=1,inversions=0;for(int i=0;i<n;i++){term=f.mul(term,M[i][perm[i]]);for(int j=0;j<i;j++)if(perm[j]>perm[i])inversions++;}z=f.add(z,inversions%2?f.neg(term):term);}while(std::next_permutation(perm.begin(),perm.end()));
 return z;
}
int scalar(const Poly& p){for(auto [m,c]:p.terms){(void)c;assert(!m);}auto it=p.terms.find(0);return it==p.terms.end()?0:it->second;}
void resultant_controls(){
 for(int p:{2,3,5}){
  Field f(p,1,p);int trials=p==2?256:256;
  for(int trial=0;trial<trials;trial++){
   std::array<int,8> c{};unsigned state=trial+1;
   if(p==2){for(int i=0;i<8;i++)c[i]=(trial>>i)&1;}
   else for(int& v:c){state=state*1664525u+1013904223u;v=state%p;}
   Quad u{Poly(f,c[0]),Poly(f,c[1]),Poly(f,c[2])};Quad v{Poly(f,c[3]),Poly(f,c[4]),Poly(f,c[5])};
   std::vector<std::vector<int>> M4(4,std::vector<int>(4));
   for(int j=0;j<2;j++)for(int i=0;i<3;i++){M4[j][j+i]=c[i];M4[2+j][j+i]=c[3+i];}
   assert(scalar(res22(u,v))==determinant(f,M4));
   std::array<Poly,5> w{Poly(f,c[3]),Poly(f,c[4]),Poly(f,c[5]),Poly(f,c[6]),Poly(f,c[7])};
   std::vector<std::vector<int>> M6(6,std::vector<int>(6));
   for(int j=0;j<4;j++)for(int i=0;i<3;i++)M6[j][j+i]=c[i];
   for(int j=0;j<2;j++)for(int i=0;i<5;i++)M6[4+j][j+i]=c[3+i];
   assert(scalar(res24(u,w))==determinant(f,M6));
  }
 }
}

struct PointTable {
 const Curve& c;std::vector<Point> ps;std::vector<int> ids,plus;
 explicit PointTable(const Curve& cc):c(cc),ps(c.points()),ids(c.f.q*c.f.q,-1),plus(ps.size()*ps.size()){
  for(unsigned i=1;i<ps.size();i++)ids[ps[i].x*c.f.q+ps[i].y]=i;
  for(unsigned i=0;i<ps.size();i++)for(unsigned j=0;j<ps.size();j++){
   Point P=c.add(ps[i],ps[j]);int k=P.inf?0:ids[P.x*c.f.q+P.y];assert(k>=0);plus[i*ps.size()+j]=k;
  }
 }
 int add(int a,int b)const{return plus[a*ps.size()+b];}
 void controls(bool associativity)const{
  for(unsigned i=0;i<ps.size();i++){assert(add(i,0)==int(i));Point n=c.neg(ps[i]);int j=n.inf?0:ids[n.x*c.f.q+n.y];assert(add(i,j)==0);}
  if(associativity)for(unsigned i=0;i<ps.size();i++)for(unsigned j=0;j<ps.size();j++)for(unsigned k=0;k<ps.size();k++)assert(add(add(i,j),k)==add(i,add(j,k)));
 }
};
struct Counts {std::uint64_t curves=0,tuples=0,zeros=0,false_zeros=0,missing_zeros=0;};
Counts check(const PointTable& table,const Poly& poly,const std::vector<int>& base,int m){
 std::vector<std::vector<int>> lifts(base.size());
 for(unsigned j=0;j<base.size();j++)for(unsigned i=1;i<table.ps.size();i++)if(table.ps[i].x==base[j])lifts[j].push_back(i);
 for(auto& l:lifts)assert(!l.empty());
 Counts out;out.curves=1;std::vector<int> tuple;
 std::function<void(const Poly&,const std::vector<int>&,int)> visit=[&](const Poly& h,const std::vector<int>& state,int depth){
  if(depth==m){
   bool z=scalar(h)==0,t=std::find(state.begin(),state.end(),0)!=state.end();
   out.tuples++;out.zeros+=z;out.false_zeros+=z&&!t;out.missing_zeros+=!z&&t;
   if(z!=t){std::cerr<<"witness m="<<m<<" tuple=";for(int x:tuple)std::cerr<<x<<",";std::cerr<<"\n";throw std::runtime_error("Semaev equivalence discrepancy");}return;
  }
  for(unsigned j=0;j<base.size();j++){
   std::vector<int> next;
   for(int a:state)for(int b:lifts[j])next.push_back(table.add(a,b));
   std::sort(next.begin(),next.end());next.erase(std::unique(next.begin(),next.end()),next.end());
   tuple.push_back(base[j]);visit(h.eval(depth,base[j]),next,depth+1);tuple.pop_back();
  }
 };
 visit(poly,{0},0);return out;
}
void add(Counts& a,Counts b){a.curves+=b.curves;a.tuples+=b.tuples;a.zeros+=b.zeros;a.false_zeros+=b.false_zeros;a.missing_zeros+=b.missing_zeros;}
void emit(int characteristic,int q,int m,Counts c,std::size_t maxterms){
 std::cout<<"{\"characteristic\":"<<characteristic<<",\"q\":"<<q<<",\"m\":"<<m<<",\"curves\":"<<c.curves<<",\"tuples\":"<<c.tuples<<",\"zeros\":"<<c.zeros<<",\"false_zeros\":"<<c.false_zeros<<",\"missing_zeros\":"<<c.missing_zeros<<",\"expected_degree_each_variable\":"<<(1<<(m-2))<<",\"degree_failures\":0,\"max_symbolic_terms\":"<<maxterms<<"}"<<std::endl;
}
int prime_power_mod(int a,int e,int p){int z=1;for(;e;e>>=1,a=a*a%p)if(e&1)z=z*a%p;return z;}
void odd(int p,bool higher){
 int d=2;while(prime_power_mod(d,(p-1)/2,p)!=p-1)d++;
 Field f(p,2,p*p+(p-d));field_controls(f);
 std::vector<int> base(p);std::iota(base.begin(),base.end(),0);
 std::array<Counts,3> stats{};std::array<std::size_t,3> terms{};bool first=true;
 for(int a:base)for(int b:base){
  Curve c(f,0,0,0,a,b);if(!c.discriminant())continue;
  PointTable t(c);t.controls(first&&p<=7);first=false;
  for(int m=3;m<=(higher?5:3);m++){
   Poly s=m==3?S3(f,a,b,false):(m==4?S4(f,a,b,false,{0,1,2,3}):S5(f,a,b,false));
   for(int i=0;i<m;i++)if(s.degree(i)!=(1<<(m-2)))throw std::runtime_error("degree mismatch");
   terms[m-3]=std::max(terms[m-3],s.terms.size());add(stats[m-3],check(t,s,base,m));
  }
 }
 for(int m=3;m<=(higher?5:3);m++)emit(p,p,m,stats[m-3],terms[m-3]);
}
void binary(int q,int ambient_mod){
 int n=q==2?2:4;Field f(2,n,ambient_mod);field_controls(f);std::vector<int> base;
 for(int x=0;x<f.q;x++)if(f.power(x,q)==x)base.push_back(x);
 assert(base.size()==unsigned(q));std::array<Counts,3> stats{};std::array<std::size_t,3> terms{};bool first=true;
 for(int a:base)for(int b:base)if(b){
  Curve c(f,1,a,0,0,b);assert(c.discriminant());PointTable t(c);t.controls(first);first=false;
  for(int m=3;m<=5;m++){
   Poly s=m==3?S3(f,a,b,true):(m==4?S4(f,a,b,true,{0,1,2,3}):S5(f,a,b,true));
   for(int i=0;i<m;i++)assert(s.degree(i)==(1<<(m-2)));
   terms[m-3]=std::max(terms[m-3],s.terms.size());add(stats[m-3],check(t,s,base,m));
  }
 }
 for(int m=3;m<=5;m++)emit(2,q,m,stats[m-3],terms[m-3]);
}
int main(int argc,char** argv){
 try{
  resultant_controls();std::cerr<<"padded-resultant controls passed\n";
  bool smoke=argc>1&&std::string(argv[1])=="--controls";
  if(smoke){
   Field f(5,2,28);field_controls(f);Curve c(f,0,0,0,1,1);PointTable t(c);t.controls(true);
   auto s4=S4(f,1,1,false,{0,1,2,3}),s5=S5(f,1,1,false);
   assert(s4.terms.size()==426&&s5.terms.size()==43267);
   assert(check(t,s4,{0,1,2,3,4},4).zeros==209);
   assert(check(t,s5,{0,1,2,3,4},5).zeros==1282);
   std::cout<<"native field, group, S4/S5 and padded-resultant controls passed\n";return 0;
  }
  for(int p:{5,7,11,13,17,19})odd(p,p<=7);
  binary(2,7);binary(4,19);
 }catch(const std::exception& e){std::cerr<<e.what()<<"\n";return 1;}
}
