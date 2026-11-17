// Build-only header is the unchanged point_replay.cpp with its CLI main removed.
#include "point_arithmetic.hpp"
// Separate genus3 balanced-map test; no fresh isogeny quotient is implemented here.
struct G {
    P u{F(1)},v;
    bool operator==(const G&b)const{return u==b.u&&v==b.v;}
    void check()const{require(u.degree()>=0&&u.degree()<=3&&u.c.back()==F(1)&&v.degree()<u.degree(),"genus3 Mumford shape");require(((v*v-curve)%u).degree()<0,"genus3 curve equation");}
    G neg()const{return {u,(-v)%u};}
    G add(const G&b)const{auto[d1,e1,e2]=xgcd(u,b.u);auto[d,c1,c2]=xgcd(d1,v+b.v);P uu=(u*b.u).exact(d*d);P vv=(c1*e1*u*b.v+c1*e2*b.u*v+c2*(v*b.v+curve)).exact(d)%uu;
        while(uu.degree()>3){P n=(curve-vv*vv).exact(uu).monic();vv=(-vv)%n;uu=n;}G out{uu.monic(),vv};out.check();return out;}
    G times(mpz_class n)const{G r,a=*this;while(n>0){if(mpz_odd_p(n.get_mpz_t()))r=r.add(a);n>>=1;if(n>0)a=a.add(a);}return r;}
};
EC times(EC a,mpz_class n){EC r;while(n>0){if(mpz_odd_p(n.get_mpz_t()))r=r.add(a);n>>=1;if(n>0)a=a.add(a);}return r;}
Z scalar(const F&a){require(a.a.degree()<=0,"non-base-field coefficient");return a.a.at(0);}
Poly<Z> rational(const P&a){std::vector<Z> out;for(auto&c:a.c)out.push_back(scalar(c));return Poly<Z>(out);}
void factors(Poly<Z> f,std::vector<Poly<Z>>&out){f=f.monic();if(f.degree()<1)return;if(f.degree()==1){out.push_back(f);return;}
    std::vector<Z> dc;for(int i=1;i<=f.degree();++i)dc.push_back(f.at(i)*Z(i));auto[g,s,t]=xgcd(f,Poly<Z>(dc));
    if(g.degree()>0){factors(g,out);factors(f.exact(g),out);return;}
    Poly<Z>x{Z(0),Z(1)};auto[h,a,b]=xgcd(f,powmod(x,prime,f)-x);
    if(h.degree()==0){require(f.degree()<=3,"factor degree limit");out.push_back(f);return;}
    if(h.degree()<f.degree()){factors(h,out);factors(f.exact(h),out);return;}
    for(int shift=0;shift<1024;++shift){auto q=powmod(x+Poly<Z>{Z(shift)},mpz_class((prime-1)/2),f)-Poly<Z>{Z(1)};auto[k,aa,bb]=xgcd(f,q);if(k.degree()>0&&k.degree()<f.degree()){factors(k,out);factors(f.exact(k),out);return;}}
    throw std::runtime_error("linear factor splitting limit");
}
// Lazily initialize field constants only after main has installed prime/modulus.
F& cover_c(){static F value;return value;}
F& cover_d(){static F value;return value;}
#define cc (cover_c())
#define dd (cover_d())
G pull(const EC&p){if(p.zero)return {};F xm=p.x-F(1),ds=dd+F(1);P u{-xm/(cc*cc*ds),-F(2)*xm/(cc*ds),F(1)};P linear{F(1),F(2)*cc};P v=(linear*linear).scale(p.y/(cc*cc*cc*cc))%u;
    G fiber{u,v};fiber.check();G rminus{P{F(1)/(F(2)*cc),F(1)},P{}};rminus.check();return fiber.add(rminus.neg());}
EC norm(const G&g){g.check();auto u=rational(g.u),v=rational(g.v);auto saved=modulus;Z c=scalar(cc),d=scalar(dd);std::vector<Poly<Z>> fs;factors(u,fs);EC total;
    // Keep the accumulated answer as base-field scalars across field switches.
    Z tx,ty;bool tz=true;int degree=0;
    for(auto factor:fs){degree+=factor.degree();modulus=factor;F X(Poly<Z>{Z(0),Z(1)});std::vector<F> vc;for(auto z:v.c)vc.emplace_back(Poly<Z>{z});F Y=P(vc).eval(X),C(Poly<Z>{c}),D0(Poly<Z>{d});EC image;
        if(X==F(0)){image=EC(F(1),C*C*C*C*Y);}else{F t=F(1)+F(1)/(C*X),den=t*t-F(1);if(den!=F(0))image=EC((t*t+D0)/den,Y/(X*X*X*X*den*den));}
        EC trace,conjugate=image;for(int i=0;i<factor.degree();++i){trace=trace.add(conjugate);if(!conjugate.zero)conjugate=EC(conjugate.x.power(prime),conjugate.y.power(prime));}
        if(!trace.zero){Z xx=scalar(trace.x),yy=scalar(trace.y);modulus=saved;EC previous=tz?EC{}:EC(F(Poly<Z>{tx}),F(Poly<Z>{ty}));EC sum=previous.add(EC(F(Poly<Z>{xx}),F(Poly<Z>{yy})));tz=sum.zero;if(!tz){tx=scalar(sum.x);ty=scalar(sum.y);}}
    }
    modulus=saved;require(degree==g.u.degree(),"norm factor degrees");return tz?EC{}:EC(F(Poly<Z>{tx}),F(Poly<Z>{ty}));
}
int main(int argc,char**argv){try{require(argc==2,"usage: cover_replay p192-d7-prym.json");auto j=read_json(argv[1]);prime=mpz_class(j.at("source_model").at("p").scalar);modulus=base_poly(j.at("finite_field_modulus_coefficients"));F b(Poly<Z>{Z(mpz_class(j.at("source_model").at("b").scalar))});dd=F(7);P t{F(0),F(1)},one{F(1)},t2=t*t,tm=t2-one,td=t2+P{dd};P h=tm*(td*td*td-(td*tm*tm).scale(F(3))+(tm*tm*tm).scale(b));
    require(h.degree()==8&&h.eval(F(1))==F(0)&&h.eval(F(-1))==F(0),"original genus3 roots");P dh;std::vector<F> derivative;for(int i=1;i<=h.degree();++i)derivative.push_back(h.at(i)*F(i));dh=P(derivative);cc=dh.eval(F(1));require(cc==F(1024),"branch derivative");auto[hg,hs,ht]=xgcd(h,dh);require(hg.degree()==0,"original genus3 smoothness");
    std::vector<F> q(8);for(int i=0;i<=8;++i){long choose=1;F cpower(1);for(int k=1;k<=i;++k){choose=choose*(i-k+1)/k;cpower=cpower*cc;q[8-k]=q[8-k]+h.at(i)*F(choose)/cpower;}}curve=P(q);require(curve.degree()==7&&curve.c.back()==F(1),"monic odd cover");
    EC base(F(Poly<Z>{Z(mpz_class("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012",16))}),F(Poly<Z>{Z(mpz_class("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811",16))}));require(base.y*base.y==base.x*base.x*base.x-F(3)*base.x+b,"NIST generator");
    mpz_class r("6277101735386680763835789423176059013767194773182842284081");require(mpz_probab_prime_p(r.get_mpz_t(),40)>0&&times(base,r).zero,"source order screen");G pulled=pull(base);require(!(pulled==G{})&&pulled.times(r)==G{},"image order");require(pull(EC{})==G{}&&norm(G{}).zero,"zero maps");
    std::vector<EC> points{EC{}};for(int i=1;i<=32;++i)points.push_back(points.back().add(base));for(int i=0;i<=32;++i){G p=pull(points[i]);require(norm(p)==times(points[i],2),"norm pull composition");require(p==pulled.times(i),"pull scalar compatibility");require(pull(points[i].neg())==p.neg(),"pull negation");}
    // Deterministic cross-pairs, including zero and cancellation, no timing comparison.
    for(int i=0;i<=12;++i)for(int k=0;k<=12;++k){G a=pull(points[i]),b0=pull(points[k]);G sum=a.add(b0);require(sum==pull(points[i].add(points[k])),"pull additivity");require(norm(sum)==norm(a).add(norm(b0)),"norm additivity");}
    int ramified=0;mpz_class rhs=scalar(b-F(2)).x,legendre;mpz_powm(legendre.get_mpz_t(),rhs.get_mpz_t(),mpz_class((prime-1)/2).get_mpz_t(),prime.get_mpz_t());if(legendre==1){require(mpz_fdiv_ui(prime.get_mpz_t(),4)==3,"sqrt model");mpz_class y;mpz_powm(y.get_mpz_t(),rhs.get_mpz_t(),mpz_class((prime+1)/4).get_mpz_t(),prime.get_mpz_t());EC p(F(1),F(Poly<Z>{Z(y)}));for(auto a:{p,p.neg()}){require(norm(pull(a))==times(a,2),"ramified x1 norm");++ramified;}}
    G branch{P{F(1)/(F(2)*cc),F(1)},P{}};require(branch.add(branch)==G{}&&norm(branch).zero,"second branch norm");
    int extension_ramified=0;
    if(ramified==0){mpz_class negative=prime-rhs,square,extension_order;mpz_powm(square.get_mpz_t(),negative.get_mpz_t(),mpz_class((prime+1)/4).get_mpz_t(),prime.get_mpz_t());mpz_pow_ui(extension_order.get_mpz_t(),prime.get_mpz_t(),8);F root_i;bool found=false;
        for(int shift=0;shift<16&&!found;++shift){F a(Poly<Z>{Z(shift),Z(1)});root_i=a.power(mpz_class((extension_order-1)/4));found=root_i*root_i==F(-1);}require(found,"quadratic root of minus1");F y=root_i*F(Poly<Z>{Z(square)});require(y*y==b-F(2),"ramified extension sqrt");EC p(F(1),y);
        for(auto a:{p,p.neg()}){G support{P{F(0),F(1)},P{a.y/(cc*cc*cc*cc)}};support.check();require(pull(a)==support.add(support).add(branch.neg()),"extension ramified pullback fiber");require(pull(a.add(a))==pull(a).add(pull(a)),"extension ramified additivity");++extension_ramified;}}
    std::vector<G> generic;std::vector<EC> generic_images;
    for(int value=1;value<=128&&generic.size()<8;++value){F X(value),yy=curve.eval(X);Z zz=scalar(yy);mpz_class test;mpz_powm(test.get_mpz_t(),zz.x.get_mpz_t(),mpz_class((prime-1)/2).get_mpz_t(),prime.get_mpz_t());if(test!=1)continue;mpz_class yr;mpz_powm(yr.get_mpz_t(),zz.x.get_mpz_t(),mpz_class((prime+1)/4).get_mpz_t(),prime.get_mpz_t());F Y(Poly<Z>{Z(yr)});G point{P{-X,F(1)},P{Y}};point.check();F tt=F(1)+F(1)/(cc*X),den=tt*tt-F(1);require(den!=F(0),"generic support excludes branch");EC image((tt*tt+dd)/den,Y/(X*X*X*X*den*den));require(norm(point)==image,"degree1 point norm");generic.push_back(point);generic_images.push_back(image);}
    require(generic.size()==8,"generic fixture count");for(int i=0;i<8;++i)for(int k=0;k<8;++k){G sum=generic[i].add(generic[k]);require(norm(sum)==generic_images[i].add(generic_images[k]),"generic degree2 norm additivity");G triple=sum.add(generic[(i+k+1)%8]);require(norm(triple)==generic_images[i].add(generic_images[k]).add(generic_images[(i+k+1)%8]),"generic degree3 norm additivity");}
    std::cout<<"PASS P192 d7 original cover: smooth degree8; monic degree7 via X=1/(1024(t-1)),Y=X^4*v; source and image order-r screens; 33 pull/norm/scalar cases; 169 addition pairs; "<<ramified<<" rational and "<<extension_ramified<<" quadratic-extension ramified x1 pullbacks; both branch/infinity classes; 8 extra curve-point norms and 64 degree2 plus 64 degree3 norm-additivity fixtures\n";
    std::cout<<"Scope: native producer correctness checks only; no quotient phi, dual phi^dagger, projected relation solver, or performance experiment\n";
}catch(const std::exception&e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
