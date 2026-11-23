#pragma once
// Frozen round03 helper prefix, reused verbatim without its fixture runner.
// Candidate setup and actual target solver. There is NO source->target mapper.
// Reuse the frozen native arithmetic; the core file remains byte-identical.
#define main round02_aggregate_main
#include "../round-02/local_torsion.cpp"
#undef main
#include <set>

using Factors=std::map<Z,unsigned>;
struct PrimeCert { Z factor; std::map<Z,Z> witnesses; };
Z gcd_z(const Z& a,const Z& b) {
    Z g;mpz_gcd(g.get_mpz_t(),a.get_mpz_t(),b.get_mpz_t());return g;
}
Factors factor_small(unsigned d) {
    Factors out;
    for(unsigned q=2;static_cast<uint64_t>(q)*q<=d;++q)
        while(d%q==0) {++out[Z(q)];d/=q;}
    if(d>1)++out[Z(d)];
    return out;
}
bool pocklington(const Z& n,const Factors& factors,PrimeCert& cert) {
    cert.factor=1;cert.witnesses.clear();
    for(const auto& [q,e]:factors)cert.factor*=power(q,e);
    if(n<=2 || (n-1)%cert.factor!=0 || cert.factor*cert.factor<=n)return false;
    for(const auto& [q,e]:factors) {
        (void)e;bool found=false;
        for(unsigned a=2;a<=256;++a) {
            if(powmod(Z(a),n-1,n)==1 &&
               gcd_z(powmod(Z(a),(n-1)/q,n)-1,n)==1) {
                cert.witnesses[q]=Z(a);found=true;break;
            }
        }
        if(!found)return false;
    }
    return true;
}
void print_factors(const Factors& f,const PrimeCert* cert=nullptr) {
    std::cout<<'[';bool comma=false;
    for(const auto& [q,e]:f) {
        if(comma)std::cout<<',';
        comma=true;
        std::cout<<"{\"prime\":\""<<q<<"\",\"exponent\":"<<e;
        if(cert)std::cout<<",\"witness\":\""<<cert->witnesses.at(q)<<'"';
        std::cout<<'}';
    }
    std::cout<<']';
}
bool square_root(const Z& input,const Z& p,Z& out) {
    Z a=mod(input,p);
    if(a==0) {out=0;return true;}
    if(powmod(a,(p-1)/2,p)!=1)return false;
    if(p%4==3) {out=powmod(a,(p+1)/4,p);return true;}
    Z q=p-1;unsigned s=0;
    while(mpz_even_p(q.get_mpz_t())) {q>>=1;++s;}
    Z z=2;while(powmod(z,(p-1)/2,p)!=p-1)++z;
    Z c=powmod(z,q,p),x=powmod(a,(q+1)/2,p),t=powmod(a,q,p);unsigned m=s;
    while(t!=1) {
        Z tt=t;unsigned i=0;
        while(tt!=1 && i<m) {tt=mod(tt*tt,p);++i;}
        check(i<m,"Tonelli-Shanks failed");
        Z exponent=Z(1)<<(m-i-1),b=powmod(c,exponent,p);
        x=mod(x*b,p);t=mod(t*b*b,p);c=mod(b*b,p);m=i;
    }
    out=x;check(mod(out*out,p)==a,"wrong square root");return true;
}
Point hash_point(const Z& q,const Z& B,const std::string& label) {
    Z x=mod(Z(digest(label),16),q),y;
    for(unsigned tries=0;tries<4096;++tries,x=mod(x+1,q))
        if(square_root(x*x*x+B,q,y) && y!=0)
            return {false,x,std::min(y,Z(q-y))};
    throw std::runtime_error("bounded hash-point admission exhausted");
}
void primitive(const Group& g,const Jac& p) {
    check(g.valid(p),"certificate point left curve");
    if(p.z==0)check(gcd_z(p.x,g.modulus)==1,"nonprimitive infinity");
    else check(gcd_z(p.z,g.modulus)==1,"nonunit intermediate Z");
}
Jac strong_times(const Group& g,Jac p,Z n) {
    Jac out;primitive(g,p);
    while(n>0) {
        if(mpz_odd_p(n.get_mpz_t())) {out=g.add(out,p);primitive(g,out);}
        n>>=1;
        if(n>0) {p=g.twice(p);primitive(g,p);}
    }
    return out;
}
Point times(const Group& g,const Point& p,const Z& n) {
    return g.to(strong_times(g,g.from(p),n));
}
Point external_times(const Z& q,const Z& a,const Z& b,const Point& p,const Z& n) {
    BN_CTX* ctx=BN_CTX_new();BIGNUM *qq=bn(q),*aa=bn(a),*bb=bn(b),*nn=bn(n);
    EC_GROUP* group=EC_GROUP_new_curve_GFp(qq,aa,bb,ctx);
    check(group!=nullptr,"OpenSSL curve allocation");
    EC_POINT *in=EC_POINT_new(group),*out=EC_POINT_new(group);
    if(p.infinity)check(EC_POINT_set_to_infinity(group,in)==1,"OpenSSL infinity");
    else {
        BIGNUM *x=bn(p.x),*y=bn(p.y);
        int ok=EC_POINT_set_affine_coordinates(group,in,x,y,ctx);
        BN_free(x);BN_free(y);check(ok==1,"OpenSSL input point");
    }
    check(EC_POINT_mul(group,out,nullptr,in,nn,ctx)==1,"OpenSSL multiply");Point result;
    if(!EC_POINT_is_at_infinity(group,out)) {
        BIGNUM *x=BN_new(),*y=BN_new();
        check(EC_POINT_get_affine_coordinates(group,out,x,y,ctx)==1,"OpenSSL output");
        result={false,integer(x),integer(y)};BN_free(x);BN_free(y);
    }
    EC_POINT_free(in);EC_POINT_free(out);EC_GROUP_free(group);BN_CTX_free(ctx);
    BN_free(qq);BN_free(aa);BN_free(bb);BN_free(nn);return result;
}
Z count_small(const Z& B,unsigned q,bool direct=false) {
    unsigned count=1;unsigned b=mod(B,Z(q)).get_ui();
    if(direct) {
        for(unsigned x=0;x<q;++x)for(unsigned y=0;y<q;++y)
            if((static_cast<uint64_t>(y)*y)%q==
               (static_cast<uint64_t>(x)*x%q*x+b)%q)++count;
    } else {
        std::vector<unsigned> roots(q,0);
        for(unsigned y=0;y<q;++y)++roots[static_cast<uint64_t>(y)*y%q];
        for(unsigned x=0;x<q;++x)count+=roots[(static_cast<uint64_t>(x)*x%q*x+b)%q];
    }
    return Z(count);
}
Z sixth_class_generator(const Z& q) {
    for(unsigned g=2;g<=10000;++g)
        if(powmod(Z(g),(q-1)/2,q)==q-1 && powmod(Z(g),(q-1)/3,q)!=1)return Z(g);
    throw std::runtime_error("bounded sextic generator search exhausted");
}
struct CurveChoice { Z B;Point generator;unsigned twist; };
CurveChoice choose_curve(const Z& q,const Z& r,unsigned cofactor,const std::string& label) {
    Z generator=sixth_class_generator(q),B=1;
    for(unsigned twist=0;twist<6;++twist,B=mod(B*generator,q)) {
        Group group({0,0,0,0,B},q);
        for(unsigned attempt=0;attempt<16;++attempt) {
            Point T=hash_point(q,B,label+"/twist/"+std::to_string(twist)+
                               "/generator/"+std::to_string(attempt));
            Point P=times(group,T,Z(cofactor));if(P.infinity)continue;
            if(strong_times(group,group.from(P),r).z==0)return {B,P,twist};
        }
    }
    throw std::runtime_error("no admitted CM twist");
}
Jac signed_times(const Group& group,const Point& q,const Z& n) {
    Jac out=strong_times(group,group.from(q),n<0?Z(-n):n);
    if(n<0)out.y=mod(-out.y,group.modulus);
    return out;
}
Point cm(const Group& g,const Point& p,const Z& a,const Z& b,const Z& zeta) {
    if(p.infinity)return {};
    Point w{false,mod(zeta*p.x,g.modulus),p.y};
    check(equation(g.a,w,g.modulus)==0,"CM automorphism left curve");
    Jac out=g.add(signed_times(g,p,a),signed_times(g,w,b));primitive(g,out);
    return g.to(out);
}
Z lift_zeta(const Z& z,const Z& q) {
    Z modulus=q*q;return mod(z-(z*z+z+1)*inv(2*z+1,modulus),modulus);
}
Point lift_ordinate(const Point& q,const Coeff& a,const Z& modulus) {
    if(q.infinity)return q;
    Point out=q;
    out.y=mod(out.y-equation(a,out,modulus)*inv(2*out.y,modulus),modulus);
    check(equation(a,out,modulus)==0,"ordinate lift failed");return out;
}
bool small_prime(unsigned q) {
    if(q<2)return false;
    for(unsigned d=2;d*d<=q;++d)if(q%d==0)return false;
    return true;
}
Z multiplicative_order(const Z& a,const Z& r,const Factors& f) {
    check(mod(a,r)!=0,"zero Frobenius eigenvalue");Z n=r-1;
    for(const auto& [q,e]:f) {
        (void)e;while(n%q==0 && powmod(a,n/q,r)==1)n/=q;
    }
    check(powmod(a,n,r)==1,"order certificate failed");return n;
}
struct Aux {bool found=false;unsigned variant=0,ell=0;Z B,t,a,v,zeta,eigen;};
Aux common_field(const Z& B0,const Z& p,const Z& r,const Z& z0,const Factors& factors) {
    for(unsigned variant=0;variant<=5;++variant) {
        Z B=B0+Z(variant)*p*r;
        for(unsigned ell=7;ell<=257;++ell) {
            if(ell%3!=1 || !small_prime(ell) || Z(ell)==r || B%ell==0)continue;
            Z t=Z(ell+1)-count_small(B,ell),n=(4*Z(ell)-t*t)/3,v;
            if(n<0)continue;
            mpz_sqrt(v.get_mpz_t(),n.get_mpz_t());
            check(3*v*v==4*Z(ell)-t*t,"CM trace norm does not fit");
            Z zeta=powmod(sixth_class_generator(Z(ell)),Z((ell-1)/3),Z(ell));
            Z a=(t+v)/2;
            if(mod(a+v*zeta,Z(ell))!=0) {v=-v;a=(t+v)/2;}
            check(2*a-v==t && mod(a+v*zeta,Z(ell))==0 && a*a-a*v+v*v==ell,
                  "CM Frobenius orientation failed");
            Z eigen=mod(a+v*z0,r);
            if(multiplicative_order(eigen,r,factors)!=r-1)continue;
            check(count_small(B,ell)==count_small(B,ell,true),"independent small count");
            Group local({0,0,0,0,mod(B,Z(ell))},Z(ell));
            Point test=hash_point(Z(ell),B,"round03/aux/"+std::to_string(ell));
            check(equal(cm(local,test,a,v,zeta),test),"auxiliary Frobenius not identity on rational point");
            return {true,variant,ell,B,t,a,v,zeta,eigen};
        }
    }
    return {false,0,0,B0,0,0,0,0,0};
}
Z smart_lambda(const Point& p,const Z& r,const Z& B,unsigned tau) {
    if(p.infinity)return 0;
    Z modulus=r*r;Coeff a{0,0,0,Z(tau)*r,mod(B,modulus)};Group g(a,modulus);
    // The final formal-kernel point has nonunit Z. The elliptic primality
    // certificate's unit-Z restriction applies to its field chain, not here.
    Point lifted=lift_ordinate(p,a,modulus);Jac product=g.times(g.from(lifted),r);
    check(g.valid(product),"Smart output left curve");
    check(product.z%r==0 && gcd_z(product.y,modulus)==1,"Smart output not in formal kernel");
    Z t=mod(-product.x*product.z*inv(product.y,modulus),modulus);
    check(t%r==0,"nonintegral formal quotient");return t/r;
}
void emit_point(const char* side,const char* kind,unsigned index,const Point& p) {
    std::cout<<"{\"kind\":\""<<kind<<"\",\"side\":\""<<side<<"\",\"index\":"<<index
             <<",\"x\":\""<<p.x<<"\",\"y\":\""<<p.y<<"\"}\n";
}
