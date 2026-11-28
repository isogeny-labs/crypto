#include "projective_base.hpp"
#include <fstream>
#include <set>

using FM=std::vector<std::vector<F>>;
template<class T> void fs_reject(T action){bool rejected=false;try{action();}catch(const std::exception&){rejected=true;}require(rejected,"fibre control accepted corruption");}
FM fs_zero(size_t r,size_t c){return FM(r,std::vector<F>(c));}
FM fs_eye(size_t n){auto a=fs_zero(n,n);for(size_t i=0;i<n;++i)a[i][i]=F(1);return a;}
FM fs_mul(const FM&a,const FM&b){require(!a.empty()&&!b.empty()&&a[0].size()==b.size(),"matrix product shape");auto c=fs_zero(a.size(),b[0].size());for(size_t i=0;i<a.size();++i)for(size_t k=0;k<b.size();++k)if(a[i][k]!=F(0))for(size_t j=0;j<b[0].size();++j)if(b[k][j]!=F(0))c[i][j]=c[i][j]+a[i][k]*b[k][j];return c;}
FM fs_scale(FM a,const F&b){for(auto&r:a)for(auto&v:r)v=v*b;return a;}
FM fs_plus(FM a,const FM&b){require(a.size()==b.size(),"matrix sum shape");for(size_t i=0;i<a.size();++i){require(a[i].size()==b[i].size(),"matrix sum width");for(size_t j=0;j<a[i].size();++j)a[i][j]=a[i][j]+b[i][j];}return a;}
FM fs_transpose(const FM&a){require(!a.empty(),"transpose nonempty");auto r=fs_zero(a[0].size(),a.size());for(size_t i=0;i<a.size();++i)for(size_t j=0;j<a[0].size();++j)r[j][i]=a[i][j];return r;}
struct FRref{FM a;std::vector<size_t> pivots;};
FRref fs_rref(FM a){FRref out;size_t r=0;if(a.empty())return out;for(size_t j=0;j<a[0].size()&&r<a.size();++j){size_t k=r;while(k<a.size()&&a[k][j]==F(0))++k;if(k==a.size())continue;std::swap(a[k],a[r]);F inv=a[r][j].inverse();for(auto&v:a[r])v=v*inv;for(size_t i=0;i<a.size();++i)if(i!=r&&a[i][j]!=F(0)){F c=a[i][j];for(size_t h=j;h<a[i].size();++h)a[i][h]=a[i][h]-c*a[r][h];}out.pivots.push_back(j);++r;}a.resize(r);out.a=std::move(a);return out;}
FM fs_inverse(const FM&a){size_t n=a.size();require(n&&a[0].size()==n,"square inverse");auto b=fs_zero(n,2*n);for(size_t i=0;i<n;++i){for(size_t j=0;j<n;++j)b[i][j]=a[i][j];b[i][i+n]=F(1);}auto r=fs_rref(b);require(r.pivots.size()==n,"invertible matrix rank");for(size_t i=0;i<n;++i)require(r.pivots[i]==i,"invertible left block");auto c=fs_zero(n,n);for(size_t i=0;i<n;++i)for(size_t j=0;j<n;++j)c[i][j]=r.a[i][j+n];return c;}
FM fs_power(FM a,int n){auto b=fs_eye(a.size());while(n){if(n&1)b=fs_mul(b,a);n>>=1;if(n)a=fs_mul(a,a);}return b;}
std::vector<Exponent> fs_monomials(int d){std::vector<Exponent> out;for(int i=0;i<=d;++i)for(int j=0;i+j<=d;++j)out.push_back({i,j});return out;}
struct Quotient{
 int degree=0;std::vector<Exponent> monomials;std::map<Exponent,size_t> index;FRref relations;std::vector<size_t> free;
 Quotient(const BP&c,const BP&g,int d):degree(d),monomials(fs_monomials(d)){
  for(size_t i=0;i<monomials.size();++i)index.emplace(monomials[i],i);
  FM rows;for(const BP*p:{&c,&g})for(auto e:fs_monomials(d-p->degree())){std::vector<F> row(monomials.size());for(const auto&[a,b]:p->t)row[index.at({e[0]+a[0],e[1]+a[1]})]=b;rows.push_back(std::move(row));}
  relations=fs_rref(std::move(rows));std::set<size_t> piv(relations.pivots.begin(),relations.pivots.end());for(size_t i=0;i<monomials.size();++i)if(!piv.count(i))free.push_back(i);
 }
 std::vector<F> normal(Exponent e)const{size_t col=index.at(e);std::vector<F> v(free.size());auto p=std::find(relations.pivots.begin(),relations.pivots.end(),col);if(p==relations.pivots.end()){auto q=std::find(free.begin(),free.end(),col);require(q!=free.end(),"quotient coordinate");v[size_t(q-free.begin())]=F(1);}else{size_t row=size_t(p-relations.pivots.begin());for(size_t i=0;i<free.size();++i)v[i]=-relations.a[row][free[i]];}return v;}
};
FM fs_multiplication(const Quotient&a,const Quotient&b,int axis){auto m=fs_zero(b.free.size(),a.free.size());for(size_t j=0;j<a.free.size();++j){auto e=a.monomials[a.free[j]];if(axis<2)++e[size_t(axis)];auto v=b.normal(e);for(size_t i=0;i<v.size();++i)m[i][j]=v[i];}return m;}
FM fs_restrict(const FM&a,const FM&u,const std::vector<size_t>&rows,const FM&left){auto rhs=fs_mul(a,u);auto small=fs_zero(rows.size(),u[0].size());for(size_t i=0;i<rows.size();++i)small[i]=rhs[rows[i]];auto m=fs_mul(left,small);require(fs_mul(u,m)==rhs,"complete finite invariant subspace");return m;}
struct FAlgebra{FM x;size_t projective=0,affine=0;int chart=-1;};
FAlgebra fs_algebra(const BP&c,const BP&g){
 require(!g.t.empty()&&!g.reduced(c).t.empty(),"nonzero curve restriction");require(c.degree()==4&&g.degree()<=5&&g.degree()>0,"bounded complete intersection degrees");
 Quotient a(c,g,9),b(c,g,10);size_t n=size_t(4*g.degree());require(a.free.size()==n&&b.free.size()==n&&n<=20,"complete intersection Hilbert dimensions");
 auto xx=fs_multiplication(a,b,0),zz=fs_multiplication(a,b,2);FM inv;int chart=-1;
 for(int t=0;t<=20;++t){auto l=fs_plus(zz,fs_scale(xx,F(t)));if(fs_rref(l).pivots.size()==n){inv=fs_inverse(l);chart=t;break;}}
 require(chart>=0,"fixed unit linear-form schedule");auto x=fs_mul(inv,xx),z=fs_mul(inv,zz);require(fs_mul(x,z)==fs_mul(z,x),"quotient operator commutation");
 auto stable=fs_power(z,20);auto cols=fs_rref(stable).pivots;FAlgebra out;out.projective=n;out.affine=cols.size();out.chart=chart;if(cols.empty())return out;
 auto u=fs_zero(n,cols.size());for(size_t i=0;i<n;++i)for(size_t j=0;j<cols.size();++j)u[i][j]=stable[i][cols[j]];
 auto rows=fs_rref(fs_transpose(u)).pivots;require(rows.size()==cols.size(),"finite basis rank");auto square=fs_zero(rows.size(),rows.size());for(size_t i=0;i<rows.size();++i)square[i]=u[rows[i]];auto left=fs_inverse(square);
 auto xr=fs_restrict(x,u,rows,left),zr=fs_restrict(z,u,rows,left);out.x=fs_mul(fs_inverse(zr),xr);return out;
}
Poly fs_charpoly(const FM&a){if(a.empty())return Poly{F(1)};auto b=fs_eye(a.size());std::vector<F> c(a.size()+1);c.back()=F(1);for(size_t k=1;k<=a.size();++k){b=fs_mul(a,b);F tr;for(size_t i=0;i<a.size();++i)tr=tr+b[i][i];F v=-tr/F(long(k));c[a.size()-k]=v;for(size_t i=0;i<a.size();++i)b[i][i]=b[i][i]+v;}for(const auto&r:b)for(const auto&v:r)require(v==F(0),"Cayley-Hamilton remainder");return Poly(std::move(c));}
bool fs_annihilates(const FM&a,const Poly&p){auto r=fs_zero(a.size(),a.size());for(int i=p.degree();i>=0;--i){r=fs_mul(r,a);for(size_t j=0;j<a.size();++j)r[j][j]=r[j][j]+p.at(i);}for(const auto&row:r)for(const auto&v:row)if(v!=F(0))return false;return true;}
std::vector<F> fs_roots(const Poly&f,bool controls=false){if(f.degree()<=0)return {};Poly x{F(0),F(1)};auto linear=gcd(f,wide_power(x,prime(),f)-x);std::vector<Poly> factors;if(linear.degree()>0)equal_factors(linear,1,factors);std::vector<F> roots;for(const auto&a:factors){require(a.degree()==1,"rational factor degree");roots.push_back(-a.at(0)/a.at(1));}verify_factor_product(factors,linear);if(controls&&!factors.empty()){auto dropped=factors;dropped.erase(dropped.begin());fs_reject([&]{verify_factor_product(dropped,linear);});}return roots;}
F fs_power(F a,int n){F r(1);while(n--)r=r*a;return r;}
F fs_eval(const BP&a,const F&x,const F&y){F r;for(const auto&[e,c]:a.t)r=r+c*fs_power(x,e[0])*fs_power(y,e[1]);return r;}
Poly fs_y(const BP&c,const F&x){std::vector<F> v(5);for(const auto&[e,b]:c.t)v[size_t(e[1])]=v[size_t(e[1])]+b*fs_power(x,e[0]);return Poly(std::move(v));}
struct FImage{Point value;bool local=false;};
FImage fs_image(const BP&c,const std::array<BP,4>&m,const F&x,const F&y){
 require(fs_eval(c,x,y)==F(0),"fibre point incidence");F a=fs_eval(m[0],x,y),b=fs_eval(m[1],x,y),v=fs_eval(m[2],x,y),d=fs_eval(m[3],x,y);Point p;bool local=false;
 if(b!=F(0)&&d!=F(0)){p=Point(K(a/b),K(v/d));valid(p);}else{local=true;precision=45;std::array<HP,4> h;for(size_t i=0;i<4;++i)h[i]=homogenize(m[i]);auto jet=lift(homogenize(c),{R{K(x)},R{K(y)},R{K(1)}},2);p=endpoint(ratio(h[0],h[1],jet),ratio(h[2],h[3],jet));}
 require(p.zero||(p.x.p.degree()<=0&&p.y.p.degree()<=0),"rational elliptic image");return {p,local};
}
void fs_emit_fibre(const BP&c,const std::array<BP,4>&m,const Json&baseline,int target,bool controls){
 const bool zero=target<0;BP g=zero?m[1]:m[0]-m[1].scale(coefficient(baseline.at("projected_base").arr.at(size_t(target)).at("x")));
 auto a=fs_algebra(c,g);auto cp=fs_charpoly(a.x);if(controls&&!a.x.empty()){require(fs_annihilates(a.x,cp),"full characteristic identity control");auto bad=cp+Poly{F(1)};require(!fs_annihilates(a.x,bad),"changed characteristic constant rejected");}
 auto xs=fs_roots(cp,controls);std::vector<std::tuple<F,F,Point,bool>> kept;std::set<std::string> keys;size_t candidates=0,common=0;
 for(const auto&x:xs)for(const auto&y:fs_roots(fs_y(c,x))){if(fs_eval(g,x,y)!=F(0))continue;++candidates;auto mapped=fs_image(c,m,x,y);bool match=zero?mapped.value.zero:!mapped.value.zero&&mapped.value.x.p.at(0)==coefficient(baseline.at("projected_base").arr.at(size_t(target)).at("x"));if(!match){++common;continue;}auto key=decimal(x)+":"+decimal(y);require(keys.insert(key).second,"unique retained fibre point");kept.emplace_back(x,y,mapped.value,mapped.local);}
 size_t expected=0;const auto&old=baseline.at("pool").at("points").arr;const auto&images=baseline.at("images").arr;require(old.size()==226&&images.size()==226,"original old-pool ordering");
 for(size_t i=0;i<old.size();++i){bool is_zero=images[i].at("point").at("infinity").scalar=="true";bool match=zero?is_zero:!is_zero&&images[i].at("projected_index").integer()==target;if(!match)continue;++expected;auto key=old[i].at("x").scalar+":"+old[i].at("y").scalar;require(keys.count(key)==1,"all known original fibre points recovered");auto actual=fs_image(c,m,coefficient(old[i].at("x")),coefficient(old[i].at("y")));require(actual.value==point(images[i].at("point")),"exact original image binding");}
 require(expected>0,"original target has known witness");if(controls){auto dropped=keys;dropped.clear();fs_reject([&]{require(!dropped.empty(),"known original fibre witness omitted");});}
 std::cout<<"{\"target\":"<<target<<",\"projective_dimension\":"<<a.projective<<",\"affine_dimension\":"<<a.affine<<",\"unit_chart\":"<<a.chart<<",\"intersection_rational_candidates\":"<<candidates<<",\"base_points_filtered\":"<<common<<",\"known_points_recovered\":"<<expected<<",\"characteristic\":";emit(cp);std::cout<<",\"points\":[";
 for(size_t i=0;i<kept.size();++i){if(i)std::cout<<',';const auto&[x,y,p,local]=kept[i];std::cout<<"{\"x\":\""<<decimal(x)<<"\",\"y\":\""<<decimal(y)<<"\",\"image\":";emit(p);std::cout<<",\"local_chart\":"<<(local?"true":"false")<<'}';}std::cout<<"]}";std::cerr<<"completed fibre "<<target<<" rational_points="<<kept.size()<<'\n';
}
int main(int argc,char**argv){try{
 require(argc==6,"usage: fibres quartic backmap baseline START COUNT");int first=std::stoi(argv[4]),count=std::stoi(argv[5]);require(first>=0&&count>0&&count<=16&&first+count<=225,"fixed fibre batch bounds");auto model=read_json(argv[1]),back=read_json(argv[2]),baseline=read_json(argv[3]);require(baseline.at("projected_base").arr.size()==225&&baseline.at("rank_mod_r").integer()==6,"frozen original base/rank");auto c=read_bp(model.at("terms"),true);require(c.t.count({0,4})&&c.t.at({0,4})!=F(0),"unit Y-four chart");std::array<BP,4> map;size_t j=0;for(const auto*key:{"X_numerator","X_denominator","Y_numerator","Y_denominator"})map[j++]=read_bp(back.at(key));require(global_identity(c,map),"actual global elliptic map identity");
 std::vector<F> inf(5);for(const auto&[e,b]:c.t)if(e[0]+e[1]==4)inf[size_t(e[1])]=b;Poly infinity(std::move(inf)),x{F(0),F(1)};require(infinity.degree()==4&&gcd(infinity,wide_power(x,prime(),infinity)-x).degree()==0,"no rational source infinity omitted");
 std::cout<<"{\"scope\":\"fixed-old-base-rational-fibre-lift\",\"old_base_width\":225,\"start\":"<<first<<",\"count\":"<<count<<",\"no_rational_infinity\":true,\"fibres\":[";bool comma=false;if(first==0){fs_emit_fibre(c,map,baseline,-1,true);comma=true;}for(int i=first;i<first+count;++i){if(comma)std::cout<<',';fs_emit_fibre(c,map,baseline,i,i==0);comma=true;}std::cout<<"],\"standing\":\"producer-unverified\"}\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
