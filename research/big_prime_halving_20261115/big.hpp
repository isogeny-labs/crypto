// Multiprecision candidate. The independent reference includes none of this file.
#pragma once
#include <gmpxx.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_mod_poly_factor.h>
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <filesystem>
#include <iostream>
#include <limits>
#include <random>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace big {
using Z=mpz_class;
using Poly=std::vector<Z>;
inline void need(bool b,const std::string&s){if(!b)throw std::runtime_error(s);}
inline Z parse(const std::string&s){
    need(!s.empty()&&(s=="0"||s[0]!='0'),"noncanonical decimal");
    for(char c:s)need(c>='0'&&c<='9',"invalid decimal");
    Z n;need(mpz_set_str(n.get_mpz_t(),s.c_str(),10)==0,"decimal parse");return n;
}
inline std::string text(const Z&n){return n.get_str();}
struct Field {
    Z p;
    explicit Field(Z prime):p(std::move(prime)){
        need(p>=5&&mpz_sizeinbase(p.get_mpz_t(),2)<=4096,"modulus admission");
        fmpz_t n;fmpz_init(n);fmpz_set_mpz(n,p.get_mpz_t());
        const bool prime_ok=fmpz_is_prime(n)==1;fmpz_clear(n);
        need(prime_ok,"proved prime required");
    }
    Z mod(const Z&n)const{Z r;mpz_mod(r.get_mpz_t(),n.get_mpz_t(),p.get_mpz_t());return r;}
    bool canonical(const Z&n)const{return n>=0&&n<p;}
    Z inv(const Z&n)const{Z r;need(mpz_invert(r.get_mpz_t(),n.get_mpz_t(),p.get_mpz_t())!=0,"inverse of zero");return r;}
    std::vector<Z> sqrt(const Z&n)const{
        need(canonical(n),"sqrt canonical input");
        fmpz_t a,m,r;fmpz_init(a);fmpz_init(m);fmpz_init(r);
        fmpz_set_mpz(a,n.get_mpz_t());fmpz_set_mpz(m,p.get_mpz_t());
        const int ok=fmpz_sqrtmod(r,a,m);Z z;fmpz_get_mpz(z.get_mpz_t(),r);
        fmpz_clear(r);fmpz_clear(m);fmpz_clear(a);
        if(!ok)return {};
        need(mod(z*z)==n,"sqrt arithmetic check");
        if(z==0)return {z};
        Z neg=p-z;return z<neg?std::vector<Z>{z,neg}:std::vector<Z>{neg,z};
    }
};
struct Point {Z x=0,y=0;bool infinity=true;};
inline bool equal(const Point&a,const Point&b){return a.infinity==b.infinity&&(a.infinity||(a.x==b.x&&a.y==b.y));}
inline bool less(const Point&a,const Point&b){return a.x<b.x||(a.x==b.x&&a.y<b.y);}
inline std::string pt(const Point&q){return q.infinity?"-":text(q.x)+":"+text(q.y);}
inline std::string zs(const std::vector<Z>&v){
    if(v.empty())return "-";
    std::string s;
    for(const Z&z:v){if(!s.empty())s+=',';s+=text(z);}return s;
}
inline std::string pts(const std::vector<Point>&v){
    if(v.empty())return "-";
    std::string s;
    for(const auto&q:v){if(!s.empty())s+=';';s+=pt(q);}return s;
}
struct Curve {
    const Field*f;Z a,b;
    Z rhs(const Z&x)const{return f->mod(x*x*x+a*x+b);}
    bool good()const{return f->canonical(a)&&f->canonical(b)&&f->mod(4*a*a*a+27*b*b)!=0;}
    bool contains(const Point&q)const{return q.infinity||(f->canonical(q.x)&&f->canonical(q.y)&&f->mod(q.y*q.y)==rhs(q.x));}
    Point add(const Point&q,const Point&r)const{
        need(contains(q)&&contains(r),"add off curve");
        if(q.infinity)return r;
        if(r.infinity)return q;
        if(q.x==r.x&&f->mod(q.y+r.y)==0)return {};
        Z slope;
        if(q.x==r.x)slope=f->mod((3*q.x*q.x+a)*f->inv(f->mod(2*q.y)));
        else slope=f->mod((r.y-q.y)*f->inv(f->mod(r.x-q.x)));
        Z x=f->mod(slope*slope-q.x-r.x),y=f->mod(slope*(q.x-x)-q.y);
        Point out{x,y,false};need(contains(out),"add result off curve");return out;
    }
    Point mul(Point q,unsigned n)const{
        Point r;while(n){if(n&1)r=add(r,q);n>>=1;if(n)q=add(q,q);}return r;
    }
};
inline void trim(Poly&v){while(!v.empty()&&v.back()==0)v.pop_back();}
inline Poly plus(const Field&f,Poly a,const Poly&b){
    a.resize(std::max(a.size(),b.size()));for(std::size_t i=0;i<b.size();++i)a[i]=f.mod(a[i]+b[i]);trim(a);return a;
}
inline Poly scale(const Field&f,Poly a,const Z&s){for(Z&v:a)v=f.mod(v*s);trim(a);return a;}
inline Poly times(const Field&f,const Poly&a,const Poly&b){
    if(a.empty()||b.empty())return {};
    Poly c(a.size()+b.size()-1);
    for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)c[i+j]=f.mod(c[i+j]+a[i]*b[j]);
    trim(c);return c;
}
inline Z eval(const Field&f,const Poly&a,const Z&x){Z r=0;for(auto i=a.rbegin();i!=a.rend();++i)r=f.mod(r*x+*i);return r;}
inline std::vector<Z> roots(const Field&f,const Poly&v){
    need(!v.empty(),"zero root polynomial");
    fmpz_t p,c;fmpz_init(p);fmpz_init(c);fmpz_set_mpz(p,f.p.get_mpz_t());
    fmpz_mod_ctx_t ctx;fmpz_mod_ctx_init(ctx,p);
    fmpz_mod_poly_t q;fmpz_mod_poly_init(q,ctx);
    for(std::size_t i=0;i<v.size();++i){fmpz_set_mpz(c,v[i].get_mpz_t());fmpz_mod_poly_set_coeff_fmpz(q,static_cast<slong>(i),c,ctx);}
    fmpz_mod_poly_factor_t fac;fmpz_mod_poly_factor_init(fac,ctx);fmpz_mod_poly_roots(fac,q,0,ctx);
    std::vector<Z> out;
    for(slong i=0;i<fac->num;++i){
        need(fmpz_mod_poly_degree(fac->poly+i,ctx)==1,"root factor degree");
        fmpz_mod_poly_get_coeff_fmpz(c,fac->poly+i,0,ctx);Z c0;fmpz_get_mpz(c0.get_mpz_t(),c);
        fmpz_mod_poly_get_coeff_fmpz(c,fac->poly+i,1,ctx);Z c1;fmpz_get_mpz(c1.get_mpz_t(),c);
        out.push_back(f.mod(-c0*f.inv(c1)));
    }
    fmpz_mod_poly_factor_clear(fac,ctx);fmpz_mod_poly_clear(q,ctx);fmpz_mod_ctx_clear(ctx);fmpz_clear(c);fmpz_clear(p);
    std::sort(out.begin(),out.end());out.erase(std::unique(out.begin(),out.end()),out.end());
    for(const Z&x:out)need(eval(f,v,x)==0,"root arithmetic check");
    return out;
}
struct Preimages {std::vector<Z> xs;std::vector<Point> halves;};
inline Preimages preimages(const Curve&e,const Point&p){
    need(e.good()&&!p.infinity&&e.contains(p),"canonical finite target on nonsingular curve required");
    const auto&f=*e.f;
    const Poly q{f.mod(e.a*e.a-4*e.b*p.x),f.mod(-8*e.b-4*e.a*p.x),f.mod(-2*e.a),f.mod(-4*p.x),1};
    Preimages out;out.xs=roots(f,q);
    for(const Z&x:out.xs)for(const Z&y:f.sqrt(e.rhs(x))){Point h{x,y,false};if(equal(e.add(h,h),p))out.halves.push_back(h);}
    std::sort(out.halves.begin(),out.halves.end(),less);need(out.halves.size()<=4,"degree four preimage cap");return out;
}
class Random {
    std::mt19937_64 engine;std::ostream&events;
public:
    Random(std::uint64_t seed,std::ostream&out):engine(seed),events(out){}
    Z bounded(const Z&size,const std::string&domain){
        need(size>0,"empty random domain");
        const Z limit=std::numeric_limits<std::uint64_t>::max();
        for(unsigned attempt=0;attempt<4096;++attempt){
            Z value=0;std::string words;bool accepted;unsigned bits=0;
            if(size<=limit){
                const auto n=static_cast<std::uint64_t>(size.get_ui());
                const auto threshold=(std::uint64_t(0)-n)%n,raw=engine();
                accepted=raw>=threshold;words=std::to_string(raw);if(accepted)value=Z(raw%n);
            }else{
                Z max=size-1;bits=static_cast<unsigned>(mpz_sizeinbase(max.get_mpz_t(),2));
                Z raw=0;
                for(unsigned i=0;i<(bits+63)/64;++i){
                    const auto w=engine();if(!words.empty())words+=',';words+=std::to_string(w);
                    Z part(w);mpz_mul_2exp(part.get_mpz_t(),part.get_mpz_t(),64*i);raw+=part;
                }
                mpz_fdiv_r_2exp(raw.get_mpz_t(),raw.get_mpz_t(),bits);accepted=raw<size;if(accepted)value=raw;
            }
            events<<"D "<<domain<<' '<<size<<' '<<bits<<' '<<words<<' '<<accepted<<' '<<value<<std::endl;
            if(accepted)return value;
        }
        throw std::runtime_error("bounded RNG cap");
    }
};
inline Point image(const Curve&e,const Curve&next,const Z&xi,const Point&p){
    if(p.infinity||p.x==xi)return {};
    const auto&f=*e.f;Z tau=f.mod(3*xi*xi+e.a),den=f.mod(p.x-xi),inv=f.inv(den);
    Point q{f.mod((p.x*p.x-xi*p.x+tau)*inv),f.mod(p.y*(den*den-tau)*inv*inv),false};
    need(next.contains(q),"isogeny image off codomain");return q;
}
struct Setup {Curve e;Point t,g;Z terminal;std::vector<Z> fibre;unsigned trials;Poly n,d;};
inline Setup setup(const Field&f,unsigned depth,std::uint64_t seed,std::ostream&out){
    need(depth>=3&&depth<=12,"unsupported tower depth");const unsigned degree=1U<<depth;
    out<<"BIG-HALVING-1 "<<f.p<<' '<<depth<<' '<<seed<<std::endl;Random rng(seed,out);
    for(unsigned trial=0;trial<4096;++trial){
        Z a=rng.bounded(f.p,"A"),x=rng.bounded(f.p,"X");Curve e{&f,a,f.mod(-x*x*x-a*x)};
        out<<"C "<<trial<<' '<<a<<' '<<x<<' '<<e.b<<' '<<e.good()<<std::endl;
        if(!e.good())continue;
        Point t{x,0,false};bool dead=false;
        for(unsigned i=0;i<depth-1;++i){
            const auto h=preimages(e,t);
            out<<"Q "<<i<<' '<<pt(t)<<' '<<h.xs.size()<<' '<<zs(h.xs)<<' '<<h.halves.size()<<' '<<pts(h.halves)<<std::endl;
            if(h.halves.empty()){dead=true;break;}
            const Z selected=rng.bounded(Z(h.halves.size()),"H"+std::to_string(i));t=h.halves.at(selected.get_ui());
            out<<"K "<<i<<' '<<selected<<' '<<pt(t)<<std::endl;
        }
        if(dead)continue;
        need(e.mul(t,degree).infinity&&!e.mul(t,degree/2).infinity,"exact subgroup order");out<<"T "<<pt(t)<<std::endl;
        std::vector<Point> subgroup;Point q;
        for(unsigned i=0;i<degree;++i){subgroup.push_back(q);q=e.add(q,t);}need(q.infinity,"subgroup cycle");
        for(unsigned slot=0;slot<64;++slot){
            Point g;bool proposed=false;
            for(unsigned j=0;j<128;++j){
                Z u=rng.bounded(2*f.p+1,"G");
                if(u==2*f.p){out<<"P "<<slot<<' '<<j<<' '<<u<<" i -"<<std::endl;proposed=true;break;}
                Z gx=u/2;const unsigned sign=mpz_tstbit(u.get_mpz_t(),0);const auto ys=f.sqrt(e.rhs(gx));
                if(ys.empty()){out<<"P "<<slot<<' '<<j<<' '<<u<<" n "<<gx<<std::endl;continue;}
                if(ys.size()==1&&sign==1){out<<"P "<<slot<<' '<<j<<' '<<u<<" z "<<gx<<std::endl;continue;}
                g={gx,ys.at(sign),false};out<<"P "<<slot<<' '<<j<<' '<<u<<" p "<<pt(g)<<std::endl;proposed=true;break;
            }
            need(proposed,"point proposal cap");const Point twice=e.add(g,g);
            bool trace=!g.infinity;for(const Point&h:subgroup)if(equal(twice,h))trace=false;
            out<<"R "<<slot<<' '<<pt(g)<<' '<<trace<<std::endl;if(!trace)continue;
            Curve current=e;Point ti=t,gi=g;Poly n{0,1},d{1};
            for(unsigned i=0;i<depth;++i){
                Point k=current.mul(ti,1U<<(depth-i-1));need(!k.infinity&&k.y==0,"degree two kernel");
                Z tau=f.mod(3*k.x*k.x+current.a);Curve next{&f,f.mod(current.a-5*tau),f.mod(current.b-7*k.x*tau)};
                need(next.good(),"singular codomain");Point nt=image(current,next,k.x,ti),ng=image(current,next,k.x,gi);
                out<<"S "<<i<<' '<<current.a<<' '<<current.b<<' '<<k.x<<' '<<tau<<' '<<next.a<<' '<<next.b<<' '
                   <<pt(ti)<<' '<<pt(gi)<<' '<<pt(nt)<<' '<<pt(ng)<<std::endl;
                Poly nd=times(f,n,d),dd=times(f,d,d);
                Poly nn=plus(f,plus(f,times(f,n,n),scale(f,nd,-k.x)),scale(f,dd,tau));
                d=plus(f,nd,scale(f,dd,-k.x));n=std::move(nn);current=next;ti=nt;gi=ng;
            }
            need(ti.infinity&&!gi.infinity&&!current.add(gi,gi).infinity,"terminal trace admission");
            need(n.size()==degree+1&&d.size()==degree&&n.back()==1&&d.back()==1,"composed degree/monicity");
            std::vector<Z> fibre;Poly product{1};
            for(const Point&h:subgroup){const Point r=e.add(g,h);need(!r.infinity,"coset pole");fibre.push_back(r.x);}
            std::sort(fibre.begin(),fibre.end());need(std::adjacent_find(fibre.begin(),fibre.end())==fibre.end(),"coset duplicate abscissa");
            for(const Z&r:fibre)product=times(f,product,Poly{f.mod(-r),1});
            need(plus(f,n,scale(f,d,-gi.x))==product,"composed fibre equality");
            out<<"N "<<zs(n)<<std::endl<<"V "<<zs(d)<<std::endl;
            out<<"F "<<e.a<<' '<<e.b<<' '<<pt(t)<<' '<<pt(g)<<' '<<gi.x<<' '<<zs(fibre)<<' '<<trial+1<<std::endl;
            need(bool(out),"transcript write");return {e,t,g,gi.x,fibre,trial+1,n,d};
        }
    }
    out<<"U curve_cap"<<std::endl;throw std::runtime_error("curve trial cap");
}
} // namespace big
