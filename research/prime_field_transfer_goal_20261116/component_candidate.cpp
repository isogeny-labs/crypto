// Native correctness checks for a candidate target, not an ECDLP evaluator.
#include <gmpxx.h>
#include <openssl/bn.h>
#include <array>
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>

using Big = mpz_class;
using Rat = mpq_class;
using Coeff = std::array<Big, 5>;

void require(bool ok, const std::string& why) {
    if (!ok) throw std::runtime_error(why);
}
Big mod(Big a, const Big& p) { a %= p; if(a<0) a+=p; return a; }
Big inverse(Big a, Big p) {
    a = mod(a, p); Big u=1, v=0, oldp=p;
    while (p != 0) {
        Big q=a/p, na=a-q*p, nu=u-q*v;
        a=p; p=na; u=v; v=nu;
    }
    require(a==1, "inverse of nonunit"); return mod(u, oldp);
}
Big power_mod(Big a, Big n, const Big& p) {
    Big out=1; a=mod(a,p);
    while (n>0) { if ((n&1)!=0) out=mod(out*a,p); n>>=1; a=mod(a*a,p); }
    return out;
}
struct Model { Big p, r; Coeff c; };
Model construct(const Big& p, const Big& r, const Coeff& a) {
    require(p>2 && (p&1)!=0 && r>0, "odd source and positive exponent required");
    Big two=power_mod(2,r,p), inv=inverse(two,p), inv_next=inverse(mod(2*two,p),p);
    Coeff c{mod((a[0]-1)*inv_next,p),mod(a[1]*inv_next,p),
        mod(a[2]*inv_next,p),mod(a[3]*inv_next,p),mod(a[4]*inv,p)};
    if ((c[4]&1)==0) c[4]+=p;
    return {p,r,c};
}
Coeff source_projection(const Model& m) {
    Big t=power_mod(2,m.r,m.p), s=mod(2*t,m.p);
    return {mod(1+s*m.c[0],m.p),mod(s*m.c[1],m.p),mod(s*m.c[2],m.p),
        mod(s*m.c[3],m.p),mod(t*m.c[4],m.p)};
}
Coeff expand(const Model& m) {
    require(m.r<=4096, "dense expansion deliberately bounded");
    Big t=Big(1)<<m.r.get_ui(), s=2*t;
    return {1+s*m.c[0],s*m.c[1],s*m.c[2],s*m.c[3],t*m.c[4]};
}
std::array<Big,2> invariants(const Coeff& a) {
    Big b2=a[0]*a[0]+4*a[1], b4=a[0]*a[2]+2*a[3], b6=a[2]*a[2]+4*a[4];
    Big b8=a[0]*a[0]*a[4]+4*a[1]*a[4]-a[0]*a[2]*a[3]+a[1]*a[2]*a[2]-a[3]*a[3];
    return {b2*b2-24*b4,-b2*b2*b8-8*b4*b4*b4-27*b6*b6+9*b2*b4*b6};
}
unsigned valuation(Big a, unsigned prime=2) {
    if (a==0) return 1000000000U;
    if (a<0) a=-a;
    unsigned v=0; while (a%prime==0) {a/=prime; ++v;} return v;
}
int valuation(const Rat& q, unsigned prime=2) {
    if (q==Rat(0)) return 1000000000;
    return static_cast<int>(valuation(q.get_num(),prime))-static_cast<int>(valuation(q.get_den(),prime));
}
void print_coeff(const Coeff& c) {
    std::cout<<'[';
    for(unsigned i=0;i<5;++i) {if(i) std::cout<<','; std::cout<<'"'<<c[i]<<'"';}
    std::cout<<']';
}
void model_check(const Big& p, const Big& r, const Coeff& a) {
    Model m=construct(p,r,a); Coeff wanted=a;
    for (auto& z:wanted) z=mod(z,p);
    require(source_projection(m)==wanted,"source projection mismatch");
    Coeff full=expand(m); auto iv=invariants(full);
    require(valuation(iv[0])==0 && valuation(iv[1])==r,"wrong local invariants");
    require(mod(iv[1],p)!=0,"source fixture singular");
    Model wrong=m; wrong.c[0]+=1;
    require(source_projection(wrong)!=wanted,"coefficient mutation accepted");
    wrong=m; wrong.c[4]+=1;
    require(valuation(invariants(expand(wrong))[1])!=r,"parity mutation accepted");
    std::cout<<"{\"kind\":\"model\",\"p\":\""<<p<<"\",\"r\":\""<<r
        <<"\",\"compressed\":";print_coeff(m.c);std::cout<<",\"expanded\":";print_coeff(full);
    std::cout<<",\"c4\":\""<<iv[0]<<"\",\"delta\":\""<<iv[1]<<"\",\"v2_delta\":"
        <<valuation(iv[1])<<",\"negative_controls_rejected\":2}\n";
}
Big openssl_pow2(const Big& exponent,const Big& modulus) {
    BIGNUM *a=BN_new(), *e=nullptr, *m=nullptr, *z=BN_new(); BN_CTX* ctx=BN_CTX_new();
    require(a&&z&&ctx,"OpenSSL allocation failure");
    require(BN_set_word(a,2)==1 && BN_dec2bn(&e,exponent.get_str().c_str())>0 &&
        BN_dec2bn(&m,modulus.get_str().c_str())>0,"OpenSSL input failure");
    require(BN_mod_exp(z,a,e,m,ctx)==1,"OpenSSL modular exponentiation failure");
    char* out=BN_bn2dec(z);require(out!=nullptr,"OpenSSL result failure"); Big result(out);
    OPENSSL_free(out); BN_free(a);BN_free(e);BN_free(m);BN_free(z);BN_CTX_free(ctx);return result;
}
struct Point { bool inf=true; Rat x=Rat(0), y=Rat(0); };
Point finite(Rat x,Rat y) {return {false,x,y};}
bool on_curve(const Coeff& a,const Point& p) {
    return p.inf || p.y*p.y+Rat(a[0])*p.x*p.y+Rat(a[2])*p.y==
        p.x*p.x*p.x+Rat(a[1])*p.x*p.x+Rat(a[3])*p.x+Rat(a[4]);
}
Point neg(const Coeff& a,const Point& p) {
    return p.inf?p:finite(p.x,-p.y-Rat(a[0])*p.x-Rat(a[2]));
}
Point add(const Coeff& a,const Point& p,const Point& q) {
    if(p.inf) return q;
    if(q.inf) return p;
    if(p.x==q.x && p.y+q.y+Rat(a[0])*p.x+Rat(a[2])==Rat(0)) return {};
    Rat slope;
    if(p.x==q.x) slope=(3*p.x*p.x+2*Rat(a[1])*p.x+Rat(a[3])-Rat(a[0])*p.y)/
        (2*p.y+Rat(a[0])*p.x+Rat(a[2]));
    else slope=(q.y-p.y)/(q.x-p.x);
    Rat intercept=p.y-slope*p.x;
    Rat x=slope*slope+Rat(a[0])*slope-Rat(a[1])-p.x-q.x;
    Point out=finite(x,-(slope+Rat(a[0]))*x-intercept-Rat(a[2]));
    require(on_curve(a,out),"rational addition left curve"); return out;
}
Point multiply(const Coeff& a,Point p,unsigned n) {
    Point out;while(n) {if(n&1U) out=add(a,out,p);n>>=1U;if(n) p=add(a,p,p);}return out;
}
struct FPoint {bool inf=true;std::int64_t x=0,y=0;};
bool equal(const FPoint& a,const FPoint& b) {
    return a.inf==b.inf && (a.inf || (a.x==b.x&&a.y==b.y));
}
struct FiniteCurve {
    std::int64_t p;
    std::int64_t m(std::int64_t x) const {x%=p;return x<0?x+p:x;}
    FPoint add(FPoint s,FPoint t) const {
        if(s.inf) return t;
        if(t.inf) return s;
        if(s.x==t.x && m(s.y+t.y+s.x)==0) return {};
        std::int64_t num,den;
        if(s.x==t.x) {num=m(3*s.x*s.x-64-s.y);den=m(2*s.y+s.x);}
        else {num=m(t.y-s.y);den=m(t.x-s.x);}
        auto lam=m(num*inverse(Big(den),Big(p)).get_si());
        auto nu=m(s.y-lam*s.x), x=m(lam*lam+lam-s.x-t.x);
        return {false,x,m(-(lam+1)*x-nu)};
    }
    FPoint mul(FPoint s,unsigned n) const {
        FPoint out;while(n) {if(n&1U) out=add(out,s);n>>=1U;if(n) s=add(s,s);}return out;
    }
    unsigned count() const {
        unsigned total=1;
        for(std::int64_t x=0;x<p;++x) {
            for(std::int64_t y=0;y<p;++y) {
                total+=m(y*y+x*y-x*x*x+64*x-288)==0;
            }
        }
        return total;
    }
    FPoint reduce(const Point& s) const {
        if(s.inf) return {};
        if(mod(s.x.get_den(),Big(p))==0) {
            require(valuation(s.x,static_cast<unsigned>(p))<0 &&
                2*valuation(s.y,static_cast<unsigned>(p))==3*valuation(s.x,static_cast<unsigned>(p)),
                "invalid infinity reduction");return {};
        }
        auto coord=[&](const Rat& q){return mod(q.get_num()*inverse(q.get_den(),Big(p)),Big(p))
            .get_si();};return {false,coord(s.x),coord(s.y)};
    }
};
bool prime(unsigned n) {if(n<2)return false;for(unsigned d=2;d*d<=n;++d)if(n%d==0)return false;return true;}
unsigned component(const Point& q) {
    if(q.inf || valuation(q.x)<0 || valuation(q.y)<0) return 0;
    if(valuation(q.x)==0 || valuation(q.y)==0) return 0;
    int a=valuation(q.y+q.x),b=valuation(q.y);
    require(a!=b && std::min(a,b)>0 && std::min(a,b)<3,"invalid oriented component case");
    return static_cast<unsigned>(a<b?a:5-b);
}
void print_rat(const Rat& x) {std::cout<<'"'<<x.get_num()<<'/'<<x.get_den()<<'"';}
void obstruction_check() {
    Coeff a{1,0,0,-64,288};Point s=finite(4,8),t=finite(2,12);auto iv=invariants(a);
    require(on_curve(a,s)&&on_curve(a,t)&&valuation(iv[0])==0&&valuation(iv[1])==5,"bad fixed fixture");
    require(component(s)==2&&component(t)==1,"base orientation mismatch");
    unsigned examined=0;
    for(unsigned p=11;p<=251;++p) {
        if(!prime(p)||mod(iv[1],Big(p))==0)continue;
        ++examined;FiniteCurve e{p};unsigned order=e.count();if(order%5)continue;unsigned h=order/5;
        FPoint ps=e.mul(e.reduce(s),h),qt=e.mul(e.reduce(t),h);
        if(ps.inf||qt.inf||!e.mul(ps,5).inf||!e.mul(qt,5).inf)continue;
        for(unsigned n=0;n<5;++n) {
            if(!equal(e.mul(ps,n),qt)||(h+5-(2*h*n)%5)%5==0)continue;
            Point hs=multiply(a,s,h),ht=multiply(a,t,h);
            Point u=add(a,ht,neg(a,multiply(a,hs,n)));
            require(e.reduce(u).inf,"kernel witness does not reduce to identity");
            unsigned ku=component(u),kp=component(hs),kq=component(ht);
            require(ku!=0 && ku==(kq+5-(n*kp)%5)%5,"kernel component witness failed");
            std::cout<<"{\"kind\":\"kernel_obstruction\",\"p\":"<<p<<",\"order\":"<<order
                <<",\"h\":"<<h<<",\"n\":"<<n<<",\"P\":["<<ps.x<<','<<ps.y<<"],\"Q\":["
                <<qt.x<<','<<qt.y<<"],\"kappa_P\":"<<kp<<",\"kappa_Q\":"<<kq
                <<",\"kernel_component\":"<<ku<<",\"good_primes_examined\":"<<examined<<",\"U\":[";
            print_rat(u.x);std::cout<<',';print_rat(u.y);std::cout<<"]}\n";return;
        }
    }
    throw std::runtime_error("no kernel counterexample within frozen range");
}
int main() try {
    model_check(17,19,{0,0,0,2,2});model_check(257,83,{0,0,0,23,11});
    model_check(37,7,{0,-1,1,-10,-20});
    Big p("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
    Big r("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");
    Coeff a{0,0,0,0,7};Model large=construct(p,r,a);
    require(source_projection(large)==a,"large model projection failed");
    Big native=power_mod(2,r,p),external=openssl_pow2(r,p);
    require(native==external,"OpenSSL disagreement");
    std::cout<<"{\"kind\":\"compressed_256_bit_source_model_only\",\"pow2_r_mod_p\":\""<<native
        <<"\",\"openssl_agrees\":true,\"dense_expansion\":false,\"point_transfer\":false}\n";
    obstruction_check();
    std::cout<<"{\"kind\":\"status\",\"arbitrary_point_map\":\"not_constructed\",\"goal_complete\":false}\n";
} catch(const std::exception& e) {std::cerr<<e.what()<<'\n';return 1;}
