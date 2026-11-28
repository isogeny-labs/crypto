// Independent development audit of the supplied paper's native pipeline.
// No timings are collected. Field/group enumeration occurs only in this file.
// Expected values use scalar arithmetic, line/cubic intersection, polynomial
// products and Lagrange interpolation; they never call production arithmetic.
// Sources: supplied 2610.32508, Algorithms 1--6, Remark 5.4, sections 6.1/8.1.
// KB unavailable in this runtime; root receipt records missing MCP/wrapper127.
// Test assertions are producer evidence, not a signed PCC verification.
#include "paper.hpp"
#include "pullback.hpp"
#include <flint/flint.h>
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <random>
#include <regex>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>

namespace audit {
using paper_ec::Word;
using paper_ec::Coefficients;
using paper_ec::Point;
using paper_ec::Curve;
using paper_ec::Component;
using paper_ec::Divisor;
using paper_ec::Datum;
using paper_ec::GroupBasis;
using paper_ec::PrimePower;
using paper_ec::TranslationMethod;
using Poly = Coefficients;
using Wide = __uint128_t;
void need(bool ok, const std::string& why) { if (!ok) throw std::runtime_error(why); }
Word add(Word a, Word b, Word p) { return (Wide(a)+b)%p; }
Word sub(Word a, Word b, Word p) { return (Wide(a)+p-b%p)%p; }
Word mul(Word a, Word b, Word p) { return Wide(a)*b%p; }
Word power(Word a, Word n, Word p) { Word z=1; for (;n;n>>=1,a=mul(a,a,p)) if(n&1)z=mul(z,a,p);return z; }
Word inverse(Word a, Word p) { need(a%p!=0,"oracle division by zero");return power(a,p-2,p); }
Poly clean(Poly a, Word p) { for(auto& c:a)c%=p;while(!a.empty()&&!a.back())a.pop_back();return a; }
Word eval(const Poly& a, Word x, Word p) { Word r=0;for(auto i=a.rbegin();i!=a.rend();++i)r=add(mul(r,x,p),*i,p);return r; }
Poly product(const Poly& a,const Poly& b,Word p) {
    if(a.empty()||b.empty())return {};
    Poly z(a.size()+b.size()-1);for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)z[i+j]=add(z[i+j],mul(a[i],b[j],p),p);
    return clean(z,p);
}
Poly divide(Poly a,Poly b,Word p) {
    a=clean(a,p);b=clean(b,p);need(!b.empty(),"oracle polynomial division by zero");
    Poly z(a.size()>=b.size()?a.size()-b.size()+1:0);Word inv=inverse(b.back(),p);
    while(a.size()>=b.size()) {std::size_t k=a.size()-b.size();Word c=mul(a.back(),inv,p);z[k]=c;for(std::size_t j=0;j<b.size();++j)a[k+j]=sub(a[k+j],mul(c,b[j],p),p);a=clean(a,p);}
    need(a.empty(),"oracle polynomial division is not exact");return clean(z,p);
}
Poly vanish(const std::vector<Word>& xs,Word p) {Poly z{1};for(Word x:xs)z=product(z,{sub(0,x,p),1},p);return z;}
Poly interpolate(const std::vector<Word>& xs,const std::vector<Word>& ys,Word p) {
    need(xs.size()==ys.size(),"oracle interpolation size");need(std::set<Word>(xs.begin(),xs.end()).size()==xs.size(),"oracle repeated x");
    Poly z(xs.size());for(std::size_t i=0;i<xs.size();++i) {std::vector<Word> others=xs;others.erase(others.begin()+static_cast<std::ptrdiff_t>(i));Poly b=vanish(others,p);Word s=mul(ys[i],inverse(eval(b,xs[i],p),p),p);for(std::size_t j=0;j<b.size();++j)z[j]=add(z[j],mul(s,b[j],p),p);}return clean(z,p);
}
Point affine(Word x,Word y) { return {x,y,false}; }
Point neg(Point a,Word p) {if(!a.infinity)a.y=sub(0,a.y,p);return a;}
Word cubic(const Curve& c,Word x) {return add(add(mul(mul(x,x,c.p),x,c.p),mul(c.a,x,c.p),c.p),c.b,c.p);}
bool on(const Curve& c,Point a) {return a.infinity||mul(a.y,a.y,c.p)==cubic(c,a.x);}
Point sum(const Curve& c,Point a,Point b) {
    need(on(c,a)&&on(c,b),"oracle input point off curve");if(a.infinity)return b;if(b.infinity)return a;
    Word p=c.p;if(a.x==b.x&&add(a.y,b.y,p)==0)return {};
    Word s=a.x==b.x?mul(add(mul(3,mul(a.x,a.x,p),p),c.a,p),inverse(mul(2,a.y,p),p),p):mul(sub(b.y,a.y,p),inverse(sub(b.x,a.x,p),p),p);
    Word v=sub(a.y,mul(s,a.x,p),p);
    // Cubic curve minus the square of the line; divide out both intersections.
    Poly q{sub(c.b,mul(v,v,p),p),sub(c.a,mul(2,mul(s,v,p),p),p),sub(0,mul(s,s,p),p),1};
    q=divide(divide(q,{sub(0,a.x,p),1},p),{sub(0,b.x,p),1},p);need(q.size()==2,"oracle third intersection");
    Word x=mul(sub(0,q[0],p),inverse(q[1],p),p);Point r=affine(x,sub(0,add(mul(s,x,p),v,p),p));need(on(c,r),"oracle result off curve");return r;
}
Point times(const Curve& c,Word k,Point p) {Point r;for(;k;k>>=1,p=sum(c,p,p))if(k&1)r=sum(c,r,p);return r;}
std::vector<Point> points(const Curve& c) {std::vector<Point> z{{}};for(Word x=0;x<c.p;++x)for(Word y=0;y<c.p;++y)if(on(c,affine(x,y)))z.push_back(affine(x,y));return z;}
Word order(const Curve& c,Point p) {Point r;for(Word n=1;n<=2*c.p+3;++n){r=sum(c,r,p);if(r.infinity)return n;}throw std::runtime_error("oracle order limit");}
std::vector<PrimePower> factors(Word n) {std::vector<PrimePower> z;for(Word q=2;q<=n/q;++q)if(n%q==0){unsigned e=0;do{++e;n/=q;}while(n%q==0);z.push_back({q,e});}if(n>1)z.push_back({n,1});return z;}
GroupBasis basis(const Curve& c) {
    auto ps=points(c);Word n=ps.size(),m=0;Point first;
    for(Point p:ps){Word k=order(c,p);if(k>m){m=k;first=p;}}
    Word other=n/m;
    for(Point q:ps)if(order(c,q)==other){std::set<Point> generated;for(Word i=0;i<m;++i)for(Word j=0;j<other;++j)generated.insert(sum(c,times(c,i,first),times(c,j,q)));if(generated.size()==n)return {first,q,m,other};}
    throw std::runtime_error("oracle cannot construct direct-product basis");
}
Datum datum(Curve c) {
    Word beta=2;while(power(beta,(c.p-1)/2,c.p)!=c.p-1)++beta;Word inv=inverse(beta,c.p);
    Curve twist{c.p,mul(c.a,mul(inv,inv,c.p),c.p),mul(c.b,mul(mul(inv,inv,c.p),inv,c.p),c.p)};
    Word n=points(c).size(),nt=points(twist).size();need(n+nt==2*(c.p+1),"oracle twist order sum");
    return {beta,c,twist,n,nt,factors(n),factors(nt),basis(c),basis(twist)};
}
Component encode(const std::vector<Point>& ps,Word p) {std::vector<Word> xs,ys;for(Point a:ps){need(!a.infinity,"encoding infinity");xs.push_back(a.x);ys.push_back(a.y);}return {vanish(xs,p),interpolate(xs,ys,p)};}
std::map<Point,unsigned> decode(const Divisor& d,const Curve& c) {
    std::map<Point,unsigned> z;if(d.infinity)++z[{}];
    for(const auto& component:d.components){Poly u=clean(component.u,c.p),v=clean(component.v,c.p);need(!u.empty()&&u.back()==1,"output u not monic");need(v.size()<u.size(),"output ordinate degree");std::size_t found=0;for(Word x=0;x<c.p;++x)if(eval(u,x,c.p)==0){++found;Point a=affine(x,eval(v,x,c.p));need(on(c,a),"output off curve");++z[a];}need(found+1==u.size(),"output u not split/squarefree");}return z;
}
std::vector<Word> scalar_roots(const Poly& f,Word p) {std::vector<Word> r;for(Word x=0;x<p;++x)if(eval(f,x,p)==0)r.push_back(x);return r;}
void check_roots(const Poly& f,std::vector<Word> got,Word p) {
    for(Word x:got){need(x<p&&eval(f,x,p)==0,"root candidate invalid");}
    std::sort(got.begin(),got.end());need(std::adjacent_find(got.begin(),got.end())==got.end(),"duplicate root");need(got==scalar_roots(f,p),"root set differs from scalar truth");need(vanish(got,p)==clean(f,p),"root product does not reconstruct input");
}
Word scalar_psi(const Curve& c,Word n,Point p,std::map<Word,Word>& memo) {
    auto it=memo.find(n);if(it!=memo.end())return it->second;Word q=c.p,x=p.x,y=p.y,a=c.a,b=c.b;
    Word z=0;
    if(n==0)z=0;else if(n==1)z=1;else if(n==2)z=mul(2,y,q);
    else if(n==3)z=sub(add(add(mul(3,power(x,4,q),q),mul(6*a%q,power(x,2,q),q),q),mul(12*b%q,x,q),q),mul(a,a,q),q);
    else if(n==4){Word v=power(x,6,q);v=add(v,mul(5*a%q,power(x,4,q),q),q);v=add(v,mul(20*b%q,power(x,3,q),q),q);v=sub(v,mul(5*mul(a,a,q)%q,power(x,2,q),q),q);v=sub(v,mul(4*mul(a,b,q)%q,x,q),q);v=sub(v,8*mul(b,b,q)%q,q);v=sub(v,power(a,3,q),q);z=mul(mul(4,y,q),v,q);}
    else {Word m=n/2;auto f=[&](Word k){return scalar_psi(c,k,p,memo);};if(n&1)z=sub(mul(f(m+2),power(f(m),3,q),q),mul(f(m-1),power(f(m+1),3,q),q),q);else z=mul(mul(f(m),sub(mul(f(m+2),power(f(m-1),2,q),q),mul(f(m-2),power(f(m+1),2,q),q),q),q),inverse(mul(2,y,q),q),q);}
    memo[n]=z;return z;
}
struct Row {std::string name,status,message;};
struct Runner {
    std::vector<Row> rows;paper_ec::Stats online,setup;Word group_cases=0,psi_point_cases=0,translation_cases=0,root_cases=0,pullback_root_cases=0;
    void check(const std::string& name,const std::function<void()>& f){try{f();rows.push_back({name,"passed",""});}catch(const std::exception& e){rows.push_back({name,"failed",e.what()});std::cerr<<name<<": "<<e.what()<<'\n';}}
    void rejects(const std::string& name,const std::function<void()>& f){check(name,[&]{bool caught=false;try{f();}catch(const std::invalid_argument&){caught=true;}need(caught,"expected invalid_argument was not thrown");});}
};
void group_checks(Runner& r,const Curve& c) {
    auto ps=points(c);for(Point a:ps){need(paper_ec::on_curve(c,a),"production on_curve rejects oracle point");need(paper_ec::point_neg(c,a)==neg(a,c.p),"negation mismatch");for(Point b:ps){need(paper_ec::point_add(c,a,b)==sum(c,a,b),"group addition differs from cubic intersection");++r.group_cases;}for(Word n:{Word(0),Word(1),Word(2),Word(3),Word(17),Word(128),Word(1025)}){need(paper_ec::point_mul(c,n,a)==times(c,n,a),"group multiplication differs from oracle");++r.group_cases;}}
}
void psi_checks(Runner& r,const Curve& c) {
    std::map<Word,Point> by_x;for(Point a:points(c))if(!a.infinity)by_x.emplace(a.x,a);std::vector<Point> ps;for(auto entry:by_x)ps.push_back(entry.second);Component divisor=encode(ps,c.p);
    for(Word n=1;n<=64;++n){Poly got=paper_ec::division_polynomial_mod(c,n,{0,1},divisor.v,divisor.u,&r.online);for(Point a:ps){Word z=eval(got,a.x,c.p);need((z==0)==times(c,n,a).infinity,"division-polynomial torsion zero set differs from scalar multiplication");if(a.y){std::map<Word,Word> memo;need(z==scalar_psi(c,n,a,memo),"division polynomial value differs from raw scalar recurrence");}++r.psi_point_cases;}}
    // Generic x need not be the quotient variable. Include a zero divisor 2y
    // at rational 2-torsion points and a nonconstant substituted x map.
    std::vector<Word> xs,ys;for(std::size_t i=0;i<ps.size();++i){xs.push_back(i);ys.push_back(ps[i].x);}Poly modulus=vanish(xs,c.p),xp=interpolate(xs,ys,c.p);ys.clear();for(Point a:ps)ys.push_back(a.y);Poly yp=interpolate(xs,ys,c.p);
    for(Word n:{Word(0),Word(2),Word(3),Word(4),Word(8),Word(18),Word(37),Word(96),Word(1025),c.p,2*c.p}){Poly got=paper_ec::division_polynomial_mod(c,n,xp,yp,modulus,&r.online);for(std::size_t i=0;i<ps.size();++i){need((eval(got,i,c.p)==0)==times(c,n,ps[i]).infinity,"substituted division polynomial zero set differs");++r.psi_point_cases;}}
}
void translation_check(Runner& r,const Curve& c,const Divisor& d,Point q,Word beta) {
    std::map<Point,unsigned> expected;for(auto entry:decode(d,c))expected[sum(c,entry.first,q)]+=entry.second;
    for(auto method:{TranslationMethod::Trace,TranslationMethod::Dense}){auto out=paper_ec::translate(c,d,q,method,beta,&r.online);need(decode(out,c)==expected,"translated divisor differs from independent group truth");++r.translation_cases;}
}
void translation_checks(Runner& r,const Datum& d) {
    const Curve& c=d.curve;auto all=points(c);std::vector<Point> finite(all.begin()+1,all.end());std::map<Word,Point> by_x;for(Point p:finite)by_x.emplace(p.x,p);std::vector<Point> simple;for(auto e:by_x)simple.push_back(e.second);
    std::vector<Point> qs{{},finite.front(),d.basis.first,d.basis.second,finite.back()};
    for(Point q:qs){translation_check(r,c,{},q,d.beta);translation_check(r,c,{{},true},q,d.beta);for(Point p:finite)translation_check(r,c,{{encode({p},c.p)},false},q,d.beta);for(std::size_t k:{std::size_t(2),std::size_t(5),simple.size()})if(k<=simple.size())translation_check(r,c,{{encode({simple.begin(),simple.begin()+static_cast<std::ptrdiff_t>(k)},c.p)},true},q,d.beta);}
    // Manufacture opposite output pairs, then encode their distinct-x preimages.
    Point q=finite.back();std::set<Word> used;std::vector<Point> selected;
    for(Point out:finite){if(!out.y)continue;Point a=sum(c,out,neg(q,c.p)),b=sum(c,neg(out,c.p),neg(q,c.p));if(a.infinity||b.infinity||a.x==b.x||used.count(a.x)||used.count(b.x))continue;selected.push_back(a);selected.push_back(b);used.insert(a.x);used.insert(b.x);if(selected.size()>=8)break;}
    if(!selected.empty()){translation_check(r,c,{{encode(selected,c.p)},false},q,d.beta);Divisor exceptional{{encode(selected,c.p)},true};if(std::find(selected.begin(),selected.end(),q)==selected.end())exceptional.components.push_back(encode({q},c.p));translation_check(r,c,exceptional,q,d.beta);translation_check(r,c,{{encode(selected,c.p)},false},q,0);}
    // Multiple components can have opposite points at a common x. They remain
    // separate simple components; support multiplicity is checked as a multiset.
    auto non_torsion=std::find_if(finite.begin(),finite.end(),[](Point p){return p.y!=0;});if(non_torsion!=finite.end())translation_check(r,c,{{encode({*non_torsion},c.p),encode({neg(*non_torsion,c.p)},c.p)},true},q,d.beta);
}
void embedding_check(Runner& r,const Datum& d,const std::vector<Word>& roots) {
    Word p=d.curve.p;Poly f=vanish(roots,p);auto e=paper_ec::finite_field_embedding(d.curve,f,d.beta,&r.online);need(product(e.u0,e.u1,p)==f,"embedding factors do not reconstruct input");need(e.v0.size()<e.u0.size()&&e.v1.size()<e.u1.size(),"embedding ordinate not reduced");for(Word x:roots){Word h=cubic(d.curve,x);need(h!=0,"audit passed torsion root to embedding");bool square=power(h,(p-1)/2,p)==1;need((eval(e.u0,x,p)==0)==square,"square/nonsquare branch misclassified");need((eval(e.u1,x,p)==0)!=square,"twist branch misclassified");Word y=eval(square?e.v0:e.v1,x,p);need(mul(square?1:d.beta,mul(y,y,p),p)==h,"embedding lift identity failed");if(!square){Word inv=inverse(d.beta,p);need(on(d.twist,affine(mul(x,inv,p),mul(y,inv,p))),"twist coordinate scaling incorrect");}}
}
void root_checks(Runner& r,const Datum& d,std::mt19937_64& rng) {
    paper_ec::PreparedDatum prepared(d,&r.setup);Word p=d.curve.p;std::vector<Word> square,nonsquare,torsion,nonzero,full;for(Word x=0;x<p;++x){full.push_back(x);Word h=cubic(d.curve,x);if(h==0)torsion.push_back(x);else{nonzero.push_back(x);(power(h,(p-1)/2,p)==1?square:nonsquare).push_back(x);}}
    for(auto roots:{std::vector<Word>{},square,nonsquare,nonzero})embedding_check(r,d,roots);
    std::vector<std::vector<Word>> sets{{},{0},torsion,square,nonsquare,nonzero,full};
    if(!torsion.empty())sets.push_back({torsion.front()});
    for(Word k:{Word(2),Word(4),Word(8),Word(16),Word(32)})if(k<=p)for(unsigned j=0;j<3;++j){auto xs=full;std::shuffle(xs.begin(),xs.end(),rng);xs.resize(k);sets.push_back(xs);}
    for(const auto& xs:sets){Poly f=vanish(xs,p);for(auto method:{TranslationMethod::Trace,TranslationMethod::Dense}){auto got=paper_ec::root_find(prepared,f,method,&r.online);check_roots(f,got,p);++r.root_cases;}paper_ec::Stats pullback_counts;auto got=paper_ec::root_find_pullback(prepared,f,&pullback_counts);check_roots(f,got,p);need(pullback_counts.counts["translation_calls"]==0&&pullback_counts.counts["trace_reconstruction_calls"]==0&&pullback_counts.counts["dense_reconstruction_calls"]==0,"pullback unexpectedly called reconstruction backend");r.online.merge(pullback_counts);++r.pullback_root_cases;}
    r.rejects("root-invalid-nonmonic-p"+std::to_string(p),[&]{paper_ec::root_find(prepared,{2,2},TranslationMethod::Trace,&r.online);});
    r.rejects("root-invalid-repeated-p"+std::to_string(p),[&]{paper_ec::root_find(prepared,{0,0,1},TranslationMethod::Trace,&r.online);});
    r.rejects("root-invalid-zero-polynomial-p"+std::to_string(p),[&]{paper_ec::root_find(prepared,{},TranslationMethod::Trace,&r.online);});
    Poly nonsplit;for(Word c=1;c<p;++c)if(power(c,(p-1)/2,p)==p-1){nonsplit={sub(0,c,p),0,1};break;}
    r.rejects("root-invalid-nonsplit-p"+std::to_string(p),[&]{paper_ec::root_find(prepared,nonsplit,TranslationMethod::Trace,&r.online);});
    r.rejects("pullback-invalid-nonmonic-p"+std::to_string(p),[&]{paper_ec::root_find_pullback(prepared,{2,2},&r.online);});
    r.rejects("pullback-invalid-repeated-p"+std::to_string(p),[&]{paper_ec::root_find_pullback(prepared,{0,0,1},&r.online);});
    r.rejects("pullback-invalid-zero-polynomial-p"+std::to_string(p),[&]{paper_ec::root_find_pullback(prepared,{},&r.online);});
    r.rejects("pullback-invalid-nonsplit-p"+std::to_string(p),[&]{paper_ec::root_find_pullback(prepared,nonsplit,&r.online);});
}
std::string read(const std::filesystem::path& path) {std::ifstream in(path);need(bool(in),"cannot open source "+path.string());std::ostringstream out;out<<in.rdbuf();return out.str();}
bool forbidden(const std::string& text) {
    return std::regex_search(text,std::regex(R"(\b(nmod_poly_roots\w*|nmod_poly_factor\w*|fq_nmod_poly_roots\w*|fq_nmod_poly_factor\w*)\s*\()"));
}
bool field_scan(const std::string& text) {
    // Conservative lexical guard for explicit for/while scans to p. It is
    // tested by canaries but is not a proof about arbitrary dynamic programs.
    return std::regex_search(text,std::regex(R"((for|while)\s*\([^\)]*(<|!=)\s*([A-Za-z_][A-Za-z_0-9]*\.)?p\b)"));
}
std::string quote(const std::string& s) {std::string z="\"";for(char c:s){if(c=='\\'||c=='\"'){z+='\\';z+=c;}else if(c=='\n')z+="\\n";else if(c=='\r')z+="\\r";else if(c=='\t')z+="\\t";else z+=c;}return z+'\"';}
void output(const Runner& r,std::ostream& out) {
    std::size_t failures=std::count_if(r.rows.begin(),r.rows.end(),[](const Row& row){return row.status!="passed";});
    out<<"{\n\"schema\":\"paper-native-development-audit/v1\",\n\"pcc_standing\":\"unverified\",\n\"native_backend\":"<<quote(std::string("FLINT ")+FLINT_VERSION)<<",\n\"compiler\":"<<quote(__VERSION__)<<",\n\"status\":"<<quote(failures?"failed":"passed")<<",\n\"timings_collected\":false,\n\"operation_count_scope\":\"Aggregate from all audit calls including invalid inputs, explicit psi tests and both literal backends; not one solve or a matched cost comparison\",\n\"group_scalar_checks\":"<<r.group_cases<<",\n\"division_polynomial_point_checks\":"<<r.psi_point_cases<<",\n\"complete_translation_checks\":"<<r.translation_cases<<",\n\"complete_root_finding_checks\":"<<r.root_cases<<",\n\"complete_pullback_root_finding_checks\":"<<r.pullback_root_cases<<",\n\"failures\":"<<failures<<",\n\"rows\":[";
    bool first=true;for(auto row:r.rows){if(!first)out<<',';first=false;out<<"\n{\"name\":"<<quote(row.name)<<",\"status\":"<<quote(row.status)<<",\"message\":"<<quote(row.message)<<'}';}out<<"\n],\n\"online_operation_counts\":{";first=true;for(auto entry:r.online.counts){if(!first)out<<',';first=false;out<<quote(entry.first)<<':'<<entry.second;}out<<"},\n\"setup_operation_counts\":{";first=true;for(auto entry:r.setup.counts){if(!first)out<<',';first=false;out<<quote(entry.first)<<':'<<entry.second;}out<<"},\n\"limitations\":[\"Small synthetic fields only; not a formal proof or signed PCC verdict\",\"No performance timing or comparison is collected\",\"Static lexical guards and source review do not prove absence of every possible hidden dynamic algorithm\",\"Datum construction enumeration occurs only in the audit; production receives only datum and polynomial coefficients\"]\n}\n";
}
} // namespace audit

int main(int argc,char** argv) {
    using namespace audit;Runner r;std::string source_dir=argc>1?argv[1]:"research/trace_incidence_20261116/native";std::mt19937_64 rng(2026111604);
    std::vector<Datum> data;
    r.check("construct-independent-data",[&]{data.push_back(paper_ec::paper_f83_datum());for(Curve c:std::vector<Curve>{{5,3,0},{7,0,1},{17,2,3},{97,2,3}})data.push_back(datum(c));});
    r.check("literal-F83-datum",[&]{need(!data.empty(),"missing datum");const auto& d=data.front();need(d.beta==2&&d.curve.p==83&&d.curve.a==31&&d.curve.b==21&&d.twist.a==70&&d.twist.b==13&&d.order==72&&d.twist_order==96,"paper datum constants differ");need(d.basis.first==affine(32,0)&&d.basis.second==affine(59,71)&&d.twist_basis.first==affine(16,0)&&d.twist_basis.second==affine(6,20),"paper literal basis differs");paper_ec::validate_datum(d,&r.setup);});
    for(const auto& d:data){std::string tag="p"+std::to_string(d.curve.p);r.check("datum-"+tag,[&]{paper_ec::validate_datum(d,&r.setup);need(points(d.curve).size()==d.order&&points(d.twist).size()==d.twist_order,"datum order differs from independent count");});r.check("group-"+tag,[&]{group_checks(r,d.curve);group_checks(r,d.twist);});r.check("division-polynomials-"+tag,[&]{psi_checks(r,d.curve);psi_checks(r,d.twist);});r.check("translation-"+tag,[&]{translation_checks(r,d);Datum swapped=d;swapped.curve=d.twist;swapped.order=d.twist_order;swapped.basis=d.twist_basis;translation_checks(r,swapped);});r.check("root-finding-"+tag,[&]{root_checks(r,d,rng);});}
    if(!data.empty()){
        const Datum original=data.front();auto invalid=[&](const std::string& name,const std::function<void(Datum&)>& change){Datum d=original;change(d);r.rejects("datum-invalid-"+name,[&]{paper_ec::PreparedDatum rejected(d,&r.setup);});};
        invalid("square-beta",[](Datum& d){d.beta=1;});invalid("zero-beta",[](Datum& d){d.beta=0;});invalid("twist-equation",[](Datum& d){++d.twist.a;});invalid("order",[](Datum& d){++d.order;});invalid("factorization",[](Datum& d){++d.factors.front().exponent;});invalid("composite-factor",[](Datum& d){d.factors={{4,1},{18,1}};});invalid("wrong-point-order",[](Datum& d){++d.basis.second_order;});invalid("off-curve-generator",[](Datum& d){++d.basis.second.y;});invalid("dependent-basis",[](Datum& d){d.basis.first=times(d.curve,18,d.basis.second);});
        r.check("paper-F83-polynomial",[&]{paper_ec::PreparedDatum prepared(original,&r.setup);Poly f{26,12,7,48,4,58,78,41,32,28,1};for(auto method:{TranslationMethod::Trace,TranslationMethod::Dense}){check_roots(f,paper_ec::root_find(prepared,f,method,&r.online),83);++r.root_cases;}check_roots(f,paper_ec::root_find_pullback(prepared,f,&r.online),83);++r.pullback_root_cases;});
    }
    r.check("erased-root-canary",[&]{Poly f=vanish({0,2,7},83);bool caught=false;try{check_roots(f,{0,2},83);}catch(const std::exception&){caught=true;}need(caught,"erased root escaped checker");});
    r.check("changed-root-canary",[&]{Poly f=vanish({0,2,7},83);bool caught=false;try{check_roots(f,{0,2,8},83);}catch(const std::exception&){caught=true;}need(caught,"changed root escaped checker");});
    r.check("translation-output-mutation-canaries",[&]{Curve c{83,31,21};Point p=affine(59,71);Divisor signed_point{{encode({p},83)},false};auto expected=decode(signed_point,c);Divisor wrong_sign=signed_point;wrong_sign.components.front().v={sub(0,p.y,83)};need(decode(wrong_sign,c)!=expected,"wrong ordinate sign escaped multiset checker");Divisor collision{{encode({p},83),encode({neg(p,83)},83)},false};expected=decode(collision,c);collision.components.pop_back();need(decode(collision,c)!=expected,"missing opposite branch escaped multiset checker");Divisor infinity{{},true};expected=decode(infinity,c);infinity.infinity=false;need(decode(infinity,c)!=expected,"missing infinity escaped multiset checker");Divisor duplicate=signed_point;duplicate.components.push_back(duplicate.components.front());need(decode(duplicate,c)!=decode(signed_point,c),"duplicate component escaped multiset checker");});
    r.check("Algorithm4-subtraction-sign",[&]{Curve c{83,31,21};Point source=affine(59,71);Point representative;for(Point p:points(c))if(!p.infinity&&p.y&&p.x!=source.x){representative=p;break;}need(!representative.infinity,"no subtraction-sign fixture");Word slope=mul(add(source.y,representative.y,83),inverse(sub(source.x,representative.x,83),83),83);Word x=sub(sub(mul(slope,slope,83),source.x,83),representative.x,83);Word plus=add(representative.y,mul(slope,sub(representative.x,x,83),83),83),minus=sub(mul(slope,sub(representative.x,x,83),83),representative.y,83);need(affine(x,plus)==sum(c,source,neg(representative,83)),"paper +yP formula differs from line/cubic subtraction");need(plus!=minus,"minus-sign mutation was not discriminated");});
    r.check("production-source-no-root-factor-scan",[&]{for(const std::string name:{"translation.cpp","paper.cpp","pullback.cpp"}){std::string text=read(std::filesystem::path(source_dir)/name);need(!forbidden(text),name+" contains forbidden root/factor call");need(!field_scan(text),name+" contains explicit online field-scan pattern; inspect whether preprocessing-only");}});
    r.check("hidden-call-canaries",[&]{need(forbidden("nmod_poly_roots(out,f,1);"),"root call canary missed");need(forbidden("nmod_poly_factor(fac,poly);"),"factor call canary missed");need(field_scan("for (Word x=0; x<c.p; ++x) f(x);"),"field scan canary missed");need(!field_scan("for(Word i=0;i<ell;++i) torsion(i);"),"torsion representatives mistaken for field scan");});
    output(r,std::cout);if(argc>2){need(!std::filesystem::exists(argv[2]),"refusing to replace existing audit output");std::ofstream file(argv[2]);need(bool(file),"cannot open audit output");output(r,file);}return std::any_of(r.rows.begin(),r.rows.end(),[](const Row& row){return row.status!="passed";})?1:0;
}
