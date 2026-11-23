// Bounded local development checker and selected-chart system exporter.
// No discrete logarithm solver, benchmark, or solving-degree estimator.
#include <algorithm>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <functional>
#include <iostream>
#include <memory>
#include <numeric>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace nt {
using Poly = std::vector<int>; // low-to-high coefficients
static constexpr int p = 17;
int mod(long long x) { x %= p; return static_cast<int>(x < 0 ? x + p : x); }
int power(int x, unsigned n) { int z = 1; while (n) { if (n & 1U) z = mod(z*x); x=mod(x*x); n>>=1U; } return z; }
int inv(int x) { if (!mod(x)) throw std::invalid_argument("inverse of zero"); return power(mod(x),p-2); }
void trim(Poly &a) { while (a.size()>1 && !a.back()) a.pop_back(); if (a.empty()) a.push_back(0); }
bool zero(const Poly &a) { return a.size()==1 && a[0]==0; }
int at(const Poly &a, std::size_t i) { return i<a.size()?a[i]:0; }
Poly plus(Poly a,const Poly &b,int sign=1) { a.resize(std::max(a.size(),b.size()),0); for (std::size_t i=0;i<b.size();++i) a[i]=mod(a[i]+sign*b[i]); trim(a);return a; }
Poly scale(Poly a,int c) { for (int &x:a)x=mod(x*c);trim(a);return a; }
Poly times(const Poly &a,const Poly &b) { Poly c(a.size()+b.size()-1,0); for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)c[i+j]=mod(c[i+j]+a[i]*b[j]);trim(c);return c; }
Poly shift(Poly a) { a.insert(a.begin(),0);trim(a);return a; }
std::pair<Poly,Poly> divide(Poly a,Poly b) {
    trim(a);trim(b);if(zero(b))throw std::invalid_argument("zero polynomial divisor");
    Poly q(a.size()>=b.size()?a.size()-b.size()+1:1,0);
    while(!zero(a)&&a.size()>=b.size()) {std::size_t k=a.size()-b.size();int c=mod(a.back()*inv(b.back()));q[k]=c;for(std::size_t j=0;j<b.size();++j)a[k+j]=mod(a[k+j]-c*b[j]);trim(a);}
    trim(q);return {q,a};
}
int eval(const Poly &a,int x) {int z=0;for(auto it=a.rbegin();it!=a.rend();++it)z=mod(z*x+*it);return z;}
Poly derivative(const Poly &a) {Poly b(std::max<std::size_t>(1,a.size()-1),0);for(std::size_t i=1;i<a.size();++i)b[i-1]=mod(static_cast<long long>(i)*a[i]);trim(b);return b;}
struct Bezout {Poly gcd,s,t;};
Bezout xgcd(Poly a,Poly b) {
    Poly s0{1},s1{0},t0{0},t1{1};
    while(!zero(b)) {auto qr=divide(a,b);Poly ns=plus(s0,times(qr.first,s1),-1),nt=plus(t0,times(qr.first,t1),-1);a=b;b=qr.second;s0=s1;s1=ns;t0=t1;t1=nt;}
    int c=inv(a.back());return {scale(a,c),scale(s0,c),scale(t0,c)};
}
Poly roots_poly(const std::vector<int> &roots) {Poly u{1};for(int x:roots)u=times(u,{mod(-x),1});return u;}
Poly graeffe(const Poly &u) {
    Poly e((u.size()+1)/2,0),o(u.size()/2,0);
    for(std::size_t i=0;i<u.size();++i)(i%2?o[i/2]:e[i/2])=u[i];
    if(o.empty())o={0};
    trim(e);trim(o);
    // (-1)^degree * (E(T)^2 - T O(T)^2), valid for either parity.
    return scale(plus(times(e,e),shift(times(o,o)),-1),(u.size()-1)%2?-1:1);
}
struct DualPoly {Poly real,epsilon;}; // coefficients in F17[e]/(e^2)
DualPoly dual_plus(DualPoly a,const DualPoly&b,int sign=1){return {plus(a.real,b.real,sign),plus(a.epsilon,b.epsilon,sign)};}
DualPoly dual_times(const DualPoly&a,const DualPoly&b){return {times(a.real,b.real),plus(times(a.real,b.epsilon),times(a.epsilon,b.real))};}
DualPoly dual_shift(const DualPoly&a){return {shift(a.real),shift(a.epsilon)};}
DualPoly dual_graeffe(const DualPoly &u){
    std::size_t count=std::max(u.real.size(),u.epsilon.size());Poly er((count+1)/2,0),ee((count+1)/2,0),orr(std::max<std::size_t>(1,count/2),0),oe(orr.size(),0);
    for(std::size_t i=0;i<count;++i){(i%2?orr[i/2]:er[i/2])=at(u.real,i);(i%2?oe[i/2]:ee[i/2])=at(u.epsilon,i);}
    DualPoly out=dual_plus(dual_times({er,ee},{er,ee}),dual_shift(dual_times({orr,oe},{orr,oe})),-1);
    int sign=(count-1)%2?-1:1;return {scale(out.real,sign),scale(out.epsilon,sign)};
}
struct Point {int x=0,y=0;bool infinity=true;};
bool same(Point q,Point r) {return q.infinity==r.infinity && (q.infinity||(q.x==r.x&&q.y==r.y));}
bool on_curve(Point q) {return q.infinity||mod(q.y*q.y)==mod(q.x*q.x*q.x+2*q.x+2);}
Point neg(Point q) {if(!q.infinity)q.y=mod(-q.y);return q;}
Point add(Point q,Point r) {
    if(!on_curve(q)||!on_curve(r))throw std::invalid_argument("off-curve input");
    if(q.infinity)return r;
    if(r.infinity)return q;
    if(q.x==r.x&&mod(q.y+r.y)==0)return {};
    int slope=same(q,r)?mod((3*q.x*q.x+2)*inv(2*q.y)):mod((r.y-q.y)*inv(r.x-q.x));
    int x=mod(slope*slope-q.x-r.x);return {x,mod(slope*(q.x-x)-q.y),false};
}
Point sum(const std::vector<Point> &v) {Point r;for(Point q:v)r=add(r,q);return r;}
// A separate quadratic-extension arithmetic regression: z^2=3, a nonsquare in F17.
struct F2 {int a=0,b=0;};
F2 f2(int a,int b=0){return {mod(a),mod(b)};}
bool eq2(F2 x,F2 y){return x.a==y.a&&x.b==y.b;}
F2 add2(F2 x,F2 y){return f2(x.a+y.a,x.b+y.b);}
F2 neg2(F2 x){return f2(-x.a,-x.b);}
F2 sub2(F2 x,F2 y){return add2(x,neg2(y));}
F2 mul2(F2 x,F2 y){return f2(x.a*y.a+3*x.b*y.b,x.a*y.b+x.b*y.a);}
F2 inv2(F2 x){int d=inv(mod(x.a*x.a-3*x.b*x.b));return f2(x.a*d,-x.b*d);}
struct Point2 {F2 x,y;bool infinity=true;};
Point2 lift(Point q){return {f2(q.x),f2(q.y),q.infinity};}
bool on_curve2(Point2 q){return q.infinity||eq2(mul2(q.y,q.y),add2(add2(mul2(mul2(q.x,q.x),q.x),mul2(f2(2),q.x)),f2(2)));}
Point2 add_point2(Point2 q,Point2 r){
    if(!on_curve2(q)||!on_curve2(r))throw std::invalid_argument("off-curve extension input");
    if(q.infinity)return r;
    if(r.infinity)return q;
    if(eq2(q.x,r.x)&&eq2(add2(q.y,r.y),f2(0)))return {};
    bool equal=eq2(q.x,r.x)&&eq2(q.y,r.y);
    F2 slope=equal?mul2(add2(mul2(f2(3),mul2(q.x,q.x)),f2(2)),inv2(mul2(f2(2),q.y))):mul2(sub2(r.y,q.y),inv2(sub2(r.x,q.x)));
    F2 x=sub2(sub2(mul2(slope,slope),q.x),r.x);return {x,sub2(mul2(slope,sub2(q.x,x)),q.y),false};
}
using Poly2=std::vector<F2>;
Poly2 mulpoly2(const Poly2&a,const Poly2&b){Poly2 c(a.size()+b.size()-1);for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)c[i+j]=add2(c[i+j],mul2(a[i],b[j]));return c;}
Poly2 addpoly2(Poly2 a,const Poly2&b,int sign=1){a.resize(std::max(a.size(),b.size()));for(std::size_t i=0;i<b.size();++i)a[i]=add2(a[i],mul2(f2(sign),b[i]));return a;}
F2 power2(F2 x,unsigned n){F2 y=f2(1);while(n){if(n&1U)y=mul2(y,x);x=mul2(x,x);n>>=1U;}return y;}
F2 evalpoly2(const Poly2&a,F2 x){F2 y;for(auto i=a.rbegin();i!=a.rend();++i)y=add2(mul2(y,x),*i);return y;}
void require_rational(const std::vector<F2>&coeffs){for(F2 x:coeffs)if(x.b)throw std::invalid_argument("coefficient outside F17");}
std::vector<F2> solve_linear2(std::vector<std::vector<F2>> a,std::size_t n){
    std::size_t row=0;std::vector<int>where(n,-1);
    for(std::size_t col=0;col<n&&row<a.size();++col){std::size_t pivot=row;while(pivot<a.size()&&eq2(a[pivot][col],f2(0)))++pivot;if(pivot==a.size())continue;std::swap(a[pivot],a[row]);F2 c=inv2(a[row][col]);for(std::size_t j=col;j<=n;++j)a[row][j]=mul2(a[row][j],c);for(std::size_t i=0;i<a.size();++i)if(i!=row&&!eq2(a[i][col],f2(0))){c=a[i][col];for(std::size_t j=col;j<=n;++j)a[i][j]=sub2(a[i][j],mul2(c,a[row][j]));}where[col]=static_cast<int>(row++);}
    for(const auto&r:a){bool empty=true;for(std::size_t j=0;j<n;++j)empty=empty&&eq2(r[j],f2(0));if(empty&&!eq2(r[n],f2(0)))throw std::runtime_error("inconsistent extension interpolation");}
    std::vector<F2>x(n);for(std::size_t col=0;col<n;++col){if(where[col]<0)throw std::runtime_error("nonunique extension interpolation");x[col]=a[static_cast<std::size_t>(where[col])][n];}return x;
}
bool prime(int n) {if(n<2)return false;for(int d=2;d*d<=n;++d)if(n%d==0)return false;return true;}
std::vector<Point> points() {std::vector<Point> out;for(int x=0;x<p;++x)for(int y=0;y<p;++y)if(on_curve({x,y,false}))out.push_back({x,y,false});return out;}
std::vector<int> solve_linear(std::vector<std::vector<int>> a,std::size_t n) {
    std::size_t row=0;std::vector<int> where(n,-1);
    for(std::size_t col=0;col<n&&row<a.size();++col) {
        std::size_t pivot=row;while(pivot<a.size()&&!a[pivot][col])++pivot;if(pivot==a.size())continue;
        std::swap(a[pivot],a[row]);int c=inv(a[row][col]);for(std::size_t j=col;j<=n;++j)a[row][j]=mod(a[row][j]*c);
        for(std::size_t i=0;i<a.size();++i)if(i!=row&&a[i][col]) {c=a[i][col];for(std::size_t j=col;j<=n;++j)a[i][j]=mod(a[i][j]-c*a[row][j]);}
        where[col]=static_cast<int>(row++);
    }
    for(const auto &r:a) {bool empty=true;for(std::size_t j=0;j<n;++j)empty=empty&&!r[j];if(empty&&r[n])throw std::runtime_error("inconsistent interpolation");}
    std::vector<int>x(n,0);for(std::size_t col=0;col<n;++col) {if(where[col]<0)throw std::runtime_error("nonunique interpolation");x[col]=a[static_cast<std::size_t>(where[col])][n];}return x;
}
struct Fixture {int m=0,t=4,c=1;std::vector<Point> summands;Point target;Poly a,c_poly,u;};
Fixture fixture(int m) {
    if(m<3||m%2==0)throw std::invalid_argument("arity must be odd and at least three");
    // Deterministic bounded fixture construction, not a distribution/scaling study.
    std::vector<Point> base;for(Point q:points())if(q.x&&q.y<=p/2)base.push_back(q);
    std::vector<Point> chosen;Fixture found;bool done=false;
    std::function<void(std::size_t)> visit=[&](std::size_t pos) {
        if(done)return;
        if(chosen.size()==static_cast<std::size_t>(m)) {
            for(unsigned mask=0;mask<(1U<<m)&&!done;++mask) {
                std::vector<Point> v=chosen;for(int i=0;i<m;++i)if(mask&(1U<<i))v[static_cast<std::size_t>(i)]=neg(v[static_cast<std::size_t>(i)]);
                Point r=sum(v);if(r.infinity)continue;bool overlap=false;for(Point q:v)overlap=overlap||q.x==r.x;if(overlap)continue;
                const int h=(m+1)/2;std::vector<Point> zeros=v;zeros.push_back(neg(r));std::vector<std::vector<int>> mat;
                for(Point q:zeros) {std::vector<int> row(static_cast<std::size_t>(m+1),0);for(int j=0;j<h;++j)row[static_cast<std::size_t>(j)]=power(q.x,static_cast<unsigned>(j));for(int j=0;j<h-1;++j)row[static_cast<std::size_t>(h+j)]=mod(q.y*power(q.x,static_cast<unsigned>(j)));row[static_cast<std::size_t>(m)]=mod(-power(q.x,static_cast<unsigned>(h)));mat.push_back(row);}
                try {
                    auto z=solve_linear(mat,static_cast<std::size_t>(m));Poly aa(z.begin(),z.begin()+h);aa.push_back(1);Poly cc(z.begin()+h,z.end());trim(cc);
                    std::vector<int> xs;for(Point q:v)xs.push_back(q.x);Poly u=roots_poly(xs);
                    if(xgcd(u,cc).gcd!=Poly{1})continue;
                    found={m,4,1,v,r,aa,cc,u};done=true;
                }catch(const std::runtime_error&){continue;}
            }
            return;
        }
        for(std::size_t i=pos;i<base.size();++i) {chosen.push_back(base[i]);visit(i+1);chosen.pop_back();if(done)return;}
    };
    visit(0);if(!done)throw std::runtime_error("no fixture in bounded tiny corpus");return found;
}

// Small expression tree for generating coefficient systems and evaluating witnesses.
struct Node {char op='k';int value=0;std::shared_ptr<Node> left,right;};
struct Expr {
    std::shared_ptr<Node> n;
    explicit Expr(int x=0):n(std::make_shared<Node>()) {n->value=mod(x);}
    static Expr variable(int i) {Expr e;e.n->op='v';e.n->value=i;return e;}
    bool constant(int x)const{return n->op=='k'&&n->value==mod(x);}
    int evaluate(const std::vector<int>&v)const {
        if(n->op=='k')return n->value;
        if(n->op=='v')return v.at(static_cast<std::size_t>(n->value));
        Expr a,b;a.n=n->left;b.n=n->right;return n->op=='+'?mod(a.evaluate(v)+b.evaluate(v)):mod(a.evaluate(v)*b.evaluate(v));
    }
    int degree()const {if(n->op=='k')return 0;if(n->op=='v')return 1;Expr a,b;a.n=n->left;b.n=n->right;return n->op=='+'?std::max(a.degree(),b.degree()):a.degree()+b.degree();}
    std::string str(const std::vector<std::string>&v)const {
        if(n->op=='k')return std::to_string(n->value);
        if(n->op=='v')return v.at(static_cast<std::size_t>(n->value));
        Expr a,b;a.n=n->left;b.n=n->right;return "("+a.str(v)+n->op+b.str(v)+")";
    }
};
Expr operator+(Expr a,Expr b) {if(a.constant(0))return b;if(b.constant(0))return a;if(a.n->op=='k'&&b.n->op=='k')return Expr(a.n->value+b.n->value);Expr e;e.n->op='+';e.n->left=a.n;e.n->right=b.n;return e;}
Expr operator*(Expr a,Expr b) {if(a.constant(0)||b.constant(0))return Expr(0);if(a.constant(1))return b;if(b.constant(1))return a;if(a.n->op=='k'&&b.n->op=='k')return Expr(a.n->value*b.n->value);Expr e;e.n->op='*';e.n->left=a.n;e.n->right=b.n;return e;}
Expr operator-(Expr a,Expr b) {return a+Expr(-1)*b;}
using XPoly=std::vector<Expr>;
Expr xat(const XPoly&a,std::size_t i){return i<a.size()?a[i]:Expr(0);}
XPoly xplus(XPoly a,const XPoly&b,int sign=1){a.resize(std::max(a.size(),b.size()));for(std::size_t i=0;i<b.size();++i)a[i]=a[i]+Expr(sign)*b[i];return a;}
XPoly xtimes(const XPoly&a,const XPoly&b){XPoly c(a.size()+b.size()-1);for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)c[i+j]=c[i+j]+a[i]*b[j];return c;}
XPoly xconstant(const Poly&a){XPoly b;for(int x:a)b.emplace_back(x);return b;}
XPoly xgraeffe(const XPoly&u){XPoly e((u.size()+1)/2),o(u.size()/2);for(std::size_t i=0;i<u.size();++i)(i%2?o[i/2]:e[i/2])=u[i];if(o.empty())o.emplace_back(0);XPoly oo=xtimes(o,o);oo.insert(oo.begin(),Expr(0));XPoly out=xplus(xtimes(e,e),oo,-1);if((u.size()-1)%2)for(Expr &z:out)z=Expr(-1)*z;return out;}
struct System {
    std::vector<std::string> names;std::vector<int>witness;std::vector<Expr> eqs;std::vector<std::string> labels;
    Expr variable(const std::string&name,int value){names.push_back(name);witness.push_back(mod(value));return Expr::variable(static_cast<int>(names.size()-1));}
    XPoly coefficients(const std::string&prefix,std::size_t count,const Poly&value,bool monic=false){XPoly z;for(std::size_t i=0;i<count;++i)z.push_back(variable(prefix+std::to_string(i),at(value,i)));if(monic)z.emplace_back(1);return z;}
    void equation(const std::string&label,Expr e){eqs.push_back(e);labels.push_back(label);}
    void equality(const std::string&label,const XPoly&a,const XPoly&b){for(std::size_t i=0;i<std::max(a.size(),b.size());++i){Expr e=xat(a,i)-xat(b,i);if(!e.constant(0))equation(label+"_"+std::to_string(i),e);}}
    int max_degree()const{int d=0;for(const Expr&e:eqs)d=std::max(d,e.degree());return d;}
    bool valid(const std::vector<int>&v)const{for(const Expr&e:eqs)if(e.evaluate(v))return false;return true;}
};
System build_system(const Fixture &f,const std::string&membership) {
    if(f.t<1||f.t>4||((p-1)%(1U<<f.t))||!f.c)throw std::invalid_argument("invalid split nonzero tower");
    if(power(f.c,(p-1)/(1U<<f.t))!=1)throw std::invalid_argument("terminal is not a split coset");
    if(f.m<3||f.m%2==0||f.target.infinity||!on_curve(f.target))throw std::invalid_argument("invalid target/arity");
    System s;int m=f.m,h=(m+1)/2;
    XPoly a=s.coefficients("a",static_cast<std::size_t>(h),f.a,true);
    XPoly c=s.coefficients("c",static_cast<std::size_t>(h-1),f.c_poly);
    XPoly u=s.coefficients("u0_",static_cast<std::size_t>(m),f.u,true);
    XPoly curve=xconstant({2,2,0,1});
    s.equality("norm",xplus(xtimes(a,a),xtimes(curve,xtimes(c,c)),-1),xtimes(xconstant({mod(-f.target.x),1}),u));
    Expr av(0),cv(0);for(std::size_t i=0;i<a.size();++i)av=av+a[i]*Expr(power(f.target.x,static_cast<unsigned>(i)));for(std::size_t i=0;i<c.size();++i)cv=cv+c[i]*Expr(power(f.target.x,static_cast<unsigned>(i)));s.equation("minus_target_zero",av-Expr(f.target.y)*cv);
    if(membership=="graeffe") {
        Poly numeric=f.u;XPoly previous=u;
        for(int j=1;j<=f.t;++j){numeric=graeffe(numeric);XPoly next=j==f.t?xconstant(roots_poly(std::vector<int>(static_cast<std::size_t>(m),f.c))):s.coefficients("u"+std::to_string(j)+"_",static_cast<std::size_t>(m),numeric,true);s.equality("graeffe_"+std::to_string(j),xgraeffe(previous),next);previous=next;}
    } else if(membership=="remainder") {
        Poly numeric{0,1};XPoly previous=xconstant(numeric);
        for(int j=1;j<=f.t;++j){auto qr=divide(times(numeric,numeric),f.u);XPoly q=s.coefficients("q"+std::to_string(j)+"_",static_cast<std::size_t>(m-1),qr.first);XPoly next=j==f.t?xconstant({f.c}):s.coefficients("r"+std::to_string(j)+"_",static_cast<std::size_t>(m),qr.second);s.equality("remainder_"+std::to_string(j),xtimes(previous,previous),xplus(xtimes(q,u),next));numeric=qr.second;previous=next;}
    } else throw std::invalid_argument("unknown membership encoding");
    // Selected-chart constraints are explicit, not omitted from exports.
    auto uc=xgcd(f.u,f.c_poly);if(uc.gcd!=Poly{1})throw std::invalid_argument("C shares a root with U");
    XPoly cs=s.coefficients("bc_s",static_cast<std::size_t>(h-1),uc.s),ct=s.coefficients("bc_t",static_cast<std::size_t>(m),uc.t);
    s.equality("coprime_C",xplus(xtimes(u,cs),xtimes(c,ct)),xconstant({1}));
    auto ud=xgcd(f.u,derivative(f.u));if(ud.gcd!=Poly{1})throw std::invalid_argument("U is not squarefree");
    XPoly du(static_cast<std::size_t>(m));for(int i=1;i<=m;++i)du[static_cast<std::size_t>(i-1)]=Expr(i)*u[static_cast<std::size_t>(i)];
    XPoly ds=s.coefficients("bd_s",static_cast<std::size_t>(m-1),ud.s),dt=s.coefficients("bd_t",static_cast<std::size_t>(m),ud.t);
    s.equality("squarefree_U",xplus(xtimes(u,ds),xtimes(du,dt)),xconstant({1}));
    int ur=eval(f.u,f.target.x);if(!ur)throw std::invalid_argument("target abscissa overlaps U");Expr ur_expr(0);for(std::size_t i=0;i<u.size();++i)ur_expr=ur_expr+u[i]*Expr(power(f.target.x,static_cast<unsigned>(i)));s.equation("target_separate",s.variable("target_inverse",inv(ur))*ur_expr-Expr(1));
    return s;
}
std::string poly_json(const Poly&a){std::ostringstream z;z<<'[';for(std::size_t i=0;i<a.size();++i){if(i)z<<',';z<<a[i];}return z.str()+']';}
void export_system(const std::filesystem::path &dir,const Fixture&f,const System&s,const std::string&arm){
    std::filesystem::create_directories(dir);std::string stem="m"+std::to_string(f.m)+"_t"+std::to_string(f.t)+"_"+arm;
    std::ofstream ms(dir/(stem+".ms"));if(!ms)throw std::runtime_error("cannot open export");for(std::size_t i=0;i<s.names.size();++i){if(i)ms<<',';ms<<s.names[i];}ms<<'\n'<<p<<'\n';for(std::size_t i=0;i<s.eqs.size();++i)ms<<s.eqs[i].str(s.names)<<(i+1<s.eqs.size()?",\n":"\n");
    std::ofstream tab(dir/(stem+".assignment.tsv"));tab<<"variable\tvalue\n";for(std::size_t i=0;i<s.names.size();++i)tab<<s.names[i]<<'\t'<<s.witness[i]<<'\n';
    std::ofstream js(dir/(stem+".witness.json"));js<<"{\n  \"schema\": \"norm-tower-development-witness/v1\",\n  \"kind\": \"known-answer selected-chart encoding check, not solver output\",\n  \"p\": "<<p<<", \"a\": 2, \"b\": 2, \"curve_order\": 19,\n  \"m\": "<<f.m<<", \"t\": "<<f.t<<", \"terminal_c\": "<<f.c<<",\n  \"target\": ["<<f.target.x<<','<<f.target.y<<"],\n  \"summands\": [";for(std::size_t i=0;i<f.summands.size();++i){if(i)js<<',';js<<'['<<f.summands[i].x<<','<<f.summands[i].y<<']';}js<<"],\n  \"A\": "<<poly_json(f.a)<<", \"C\": "<<poly_json(f.c_poly)<<", \"U0\": "<<poly_json(f.u)<<",\n  \"equation_count\": "<<s.eqs.size()<<", \"variable_count\": "<<s.names.size()<<", \"max_syntactic_input_degree\": "<<s.max_degree()<<",\n  \"assignment\": {";for(std::size_t i=0;i<s.names.size();++i){if(i)js<<',';js<<'\n'<<"    \""<<s.names[i]<<"\": "<<s.witness[i];}js<<"\n  }\n}\n";
}
struct Tests {
    unsigned passed=0;
    void check(bool ok,const std::string&name){if(!ok)throw std::runtime_error("FAILED: "+name);++passed;}
    template<class F>void rejects(F f,const std::string&name){try{f();}catch(const std::invalid_argument&){++passed;return;}throw std::runtime_error("FAILED (did not reject): "+name);}
};
void selftest(const std::filesystem::path&dir){
    Tests t;auto pts=points();t.check(pts.size()+1==19&&prime(19)&&19!=p,"prime nonanomalous curve order");t.check(mod(4*2*2*2+27*2*2)!=0,"nonsingular curve");
    // Each affine point's inverse and order in this fixed unit-test fixture.
    for(Point q:pts){t.check(add(q,neg(q)).infinity,"inverse group law");Point z;for(int i=0;i<19;++i)z=add(z,q);t.check(z.infinity,"subgroup order");}
    t.rejects([]{inv(0);},"zero inverse");t.rejects([]{add({1,1,false},{0,6,false});},"off-curve point");t.rejects([]{divide({1},{0});},"zero divisor");
    for(int m:{3,5,7}) {
        Fixture f=fixture(m);std::vector<int>roots;for(Point q:f.summands)roots.push_back(q.x);
        t.check(same(sum(f.summands),f.target),"independent group sum");
        t.check(plus(times(f.a,f.a),times({2,2,0,1},times(f.c_poly,f.c_poly)),-1)==times({mod(-f.target.x),1},f.u),"norm polynomial identity");
        t.check(eval(f.a,f.target.x)==mod(f.target.y*eval(f.c_poly,f.target.x)),"minus target condition");
        Poly g=f.u;std::vector<int>squared=roots;
        for(int j=0;j<f.t;++j){g=graeffe(g);for(int&x:squared)x=mod(x*x);t.check(g==roots_poly(squared),"Graeffe versus independent root product");}
        t.check(g==roots_poly(std::vector<int>(static_cast<std::size_t>(m),f.c)),"membership terminal");
        for(Point q:f.summands){t.check(power(q.x,1U<<f.t)==f.c,"per-root membership");t.check(mod(-eval(f.a,q.x)*inv(eval(f.c_poly,q.x)))==q.y,"sign recovered without sign enumeration");}
        for(const std::string arm:{"graeffe","remainder"}){
            System s=build_system(f,arm);t.check(s.max_degree()<=2,"export syntactic degree at most two");t.check(s.valid(s.witness),"every exported equation holds at fixture witness");auto bad=s.witness;bad[0]=mod(bad[0]+1);t.check(!s.valid(bad),"corrupt norm coefficient rejected");export_system(dir,f,s,arm);
            std::cout<<"PASS fixture m="<<m<<" t="<<f.t<<" arm="<<arm<<" variables="<<s.names.size()<<" equations="<<s.eqs.size()<<" max_input_degree="<<s.max_degree()<<" target=("<<f.target.x<<','<<f.target.y<<")\n";
        }
        Fixture malformed=f;malformed.t=5;t.rejects([&]{build_system(malformed,"remainder");},"nonsplit d does not divide p-1");malformed=f;malformed.c=0;t.rejects([&]{build_system(malformed,"remainder");},"zero terminal coset");malformed=f;malformed.c=2;t.rejects([&]{build_system(malformed,"remainder");},"nonsplit nonzero terminal coset");malformed=f;malformed.target={1,1,false};t.rejects([&]{build_system(malformed,"remainder");},"offcurve target");malformed=f;malformed.target={};t.rejects([&]{build_system(malformed,"remainder");},"target infinity outside chart");malformed=f;malformed.u=roots_poly(std::vector<int>(static_cast<std::size_t>(m),f.summands[0].x));t.rejects([&]{build_system(malformed,"remainder");},"repeated-root chart rejected");malformed=f;malformed.c_poly={0};t.rejects([&]{build_system(malformed,"remainder");},"zero C fails coprime chart");
    }
    // Multiplicity distinction, over dual numbers F17[e]/(e^2).
    // U=X^2-1+e X. For even degree, Graeffe(U)=(T-1)^2-e^2 T,
    // so the e-linear perturbation is invisible to the terminal equations.
    // But X^2-1 mod U = -e X, a nonzero remainder.
    Poly base{16,0,1};t.check(graeffe(base)==Poly({1,15,1}),"squarefree pair terminal");t.check(xgcd(base,derivative(base)).gcd==Poly{1},"pair U is squarefree");
    DualPoly perturbed{{16,0,1},{0,1}},terminal{roots_poly({1,1}),{0}};
    DualPoly transformed=dual_graeffe(perturbed);
    t.check(transformed.real==terminal.real&&zero(transformed.epsilon),"Graeffe terminal admits exact dual perturbation");
    DualPoly direct_remainder=dual_plus({{16,0,1},{0}},perturbed,-1);
    t.check(zero(direct_remainder.real)&&direct_remainder.epsilon==Poly({0,16}),"divisibility rejects exact dual perturbation");
    // Root-product identity also holds with repeated roots, but squarefree chart rejects them.
    t.check(graeffe(roots_poly({1,1,4}))==roots_poly({1,1,16}),"root squaring retains multiplicities");
    std::cout<<"PASS scheme distinction: squarefree U=X^2-1, d=2, Graeffe admits tangent epsilon*X; divisibility rejects it\n";
    // Odd arity counterpart: roots 1+e, -1+e and 4, all modulo e^2.
    Poly odd_base=roots_poly({1,16,4}),odd_perturb{0,8,15};
    DualPoly odd_transformed=dual_graeffe(dual_graeffe({odd_base,odd_perturb}));
    t.check(odd_transformed.real==roots_poly({1,1,1})&&zero(odd_transformed.epsilon),"odd arity Graeffe also admits dual tangent");
    t.check(xgcd(odd_base,derivative(odd_base)).gcd==Poly{1},"odd arity U0 is squarefree");
    auto odd_division=divide({16,0,0,0,1},odd_base);t.check(zero(odd_division.second),"odd base divides X^4-1");
    Poly odd_tangent_remainder=scale(divide(times(odd_division.first,odd_perturb),odd_base).second,-1);
    t.check(!zero(odd_tangent_remainder),"odd arity divisibility rejects dual tangent");
    std::cout<<"PASS odd scheme distinction: m=3,d=4, roots 1+epsilon,-1+epsilon,4 pass Graeffe terminal but fail divisibility\n";
    t.check(power(3,(p-1)/2)==p-1,"extension parameter is nonsquare");
    Point2 anti{f2(1),f2(0,8),false},anti2{f2(12),f2(0,16),false},minus_anti3{f2(15),f2(0,12),false};
    t.check(on_curve2(anti)&&on_curve2(anti2)&&on_curve2(minus_anti3),"anti-Frobenius extension points lie on curve");
    Point2 doubled=add_point2(anti,anti);t.check(eq2(doubled.x,anti2.x)&&eq2(doubled.y,anti2.y),"extension doubling");
    t.check(add_point2(add_point2(anti,anti2),minus_anti3).infinity,"anti-Frobenius triple sums to zero");
    Point2 mixed=add_point2(add_point2(add_point2(add_point2(anti,anti2),minus_anti3),lift({5,1,false})),lift({6,3,false}));
    t.check(!mixed.infinity&&eq2(mixed.x,f2(10))&&eq2(mixed.y,f2(6)),"extension residual tuple sums to rational R");
    std::vector<int> extension_roots{1,12,15,5,6};for(int x:extension_roots)t.check(power(x,16)==1&&x!=10,"extension tuple has allowed rational distinct abscissas");
    t.check(anti.y.b!=0&&anti2.y.b!=0&&minus_anti3.y.b!=0,"rational abscissas do not imply rational points");
    std::vector<Point2> extension_zeros{anti,anti2,minus_anti3,lift({5,1,false}),lift({6,3,false}),lift({10,11,false})};
    std::vector<std::vector<F2>> extension_mat;
    for(Point2 q:extension_zeros){std::vector<F2>r;for(unsigned j=0;j<3;++j)r.push_back(power2(q.x,j));for(unsigned j=0;j<2;++j)r.push_back(mul2(q.y,power2(q.x,j)));r.push_back(neg2(power2(q.x,3)));extension_mat.push_back(r);}
    auto extension_coeffs=solve_linear2(extension_mat,5);Poly2 extension_a(extension_coeffs.begin(),extension_coeffs.begin()+3),extension_c(extension_coeffs.begin()+3,extension_coeffs.end());extension_a.push_back(f2(1));
    Poly2 extension_norm=addpoly2(mulpoly2(extension_a,extension_a),mulpoly2({f2(2),f2(2),f2(0),f2(1)},mulpoly2(extension_c,extension_c)),-1);
    Poly expected_norm=times({mod(-10),1},roots_poly(extension_roots));bool identity=extension_norm.size()==expected_norm.size(),nonrational=false;
    for(std::size_t i=0;i<extension_norm.size()&&identity;++i)identity=eq2(extension_norm[i],f2(expected_norm[i]));
    for(F2 z:extension_coeffs)nonrational=nonrational||z.b!=0;
    t.check(identity&&nonrational,"geometric norm witness has nonrational coefficients despite rational roots and target");
    for(int x:extension_roots)t.check(!eq2(evalpoly2(extension_c,f2(x)),f2(0)),"geometric witness stays on coprime chart");
    t.rejects([&]{require_rational(extension_coeffs);},"geometric norm witness rejected by F17 coefficient-domain check");
    bool violates_field_equation=false;for(F2 x:extension_coeffs)violates_field_equation=violates_field_equation||!eq2(power2(x,p),x);
    t.check(violates_field_equation,"geometric witness violates a coefficient field equation");
    std::cout<<"PASS rationality distinction: F17^2 tuple with x={1,12,15,5,6} sums to R=(10,6), but three y coordinates are nonrational\n";
    std::cout<<"PASS checks="<<t.passed<<" failures=0; local development correctness only; solver_runs=0; timings=not_collected\n";
}
} // namespace nt
int main(int argc,char**argv){
    try {if(argc!=3||std::string(argv[1])!="--selftest-export"){std::cerr<<"usage: norm_tower --selftest-export OUTPUT_DIR\n";return 2;}nt::selftest(argv[2]);return 0;}
    catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}
}
