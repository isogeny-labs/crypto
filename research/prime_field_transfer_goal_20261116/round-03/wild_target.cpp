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
void run(unsigned m) {
    unsigned limit=m==1?1:1000001,winning_d=0;Z u,r,p;Factors uf,rf,full;PrimeCert uc,rc;
    for(unsigned d=1;d<=limit;d+=2) {
        u=(Z(d)<<m)+1;r=3*u*(u-1)+1;p=4*r+3;
        if(mpz_probab_prime_p(u.get_mpz_t(),25)==0 ||
           mpz_probab_prime_p(r.get_mpz_t(),25)==0 ||
           mpz_probab_prime_p(p.get_mpz_t(),25)==0)continue;
        uf=factor_small(d);uf[2]+=m;rf=uf;++rf[3];
        if(!pocklington(u,uf,uc) || !pocklington(r,rf,rc))continue;
        full=rf;++full[u];winning_d=d;break;
    }
    if(winning_d==0) {
        std::cout<<"{\"kind\":\"empty_bin\",\"m\":"<<m<<",\"d_limit\":"<<limit<<"}\n";return;
    }
    Z b=2*u-1;
    CurveChoice source=choose_curve(p,r,4,"prime-field-transfer/round-03/m"+std::to_string(m)+"/source");
    Z fourth;mpz_root(fourth.get_mpz_t(),p.get_mpz_t(),4);
    if(power(fourth,4)<p)++fourth;
    check(r>power(fourth+1,2),"elliptic primality bound not met");
    check(gcd_z(Z(-432)*source.B*source.B,p)==1,"singular primality witness");
    Group sg({0,0,0,0,source.B},p);
    check(strong_times(sg,sg.from(source.generator),r).z==0,"elliptic prime certificate");
    Z radius;Z fourp=4*p;mpz_sqrt(radius.get_mpz_t(),fourp.get_mpz_t());
    Z low=(p+1-radius+r-1)/r,high=(p+1+radius)/r;
    if(low!=4 || high!=4) {
        check(p<=10000 && count_small(source.B,p.get_ui())==4*r,"source order not certified");
    }
    CurveChoice target=choose_curve(r,r,1,"prime-field-transfer/round-03/m"+std::to_string(m)+"/target");
    check(external_times(p,0,source.B,source.generator,r).infinity &&
          external_times(r,0,target.B,target.generator,r).infinity,"OpenSSL orders");
    Z B0=source.B+p*mod((target.B-source.B)*inv(p,r),r);
    Z zs=mod((3-2*u)*inv(2*b,p),p),zt=mod(-u*inv(b,r),r),z0=mod((1-u)*inv(b,r),r);
    check(mod(zs*zs+zs+1,p)==0 && zs!=1 && mod(zt*zt+zt+1,r)==0 && zt!=1 &&
          mod(z0*z0+z0+1,r)==0,"bad CM roots");
    Aux aux=common_field(B0,p,r,z0,full);Z B=aux.B;
    check(mod(B,p)==source.B && mod(B,r)==target.B,"common model projection");
    check(equal(cm(sg,source.generator,u,b,zs),source.generator),"source fixed subgroup");
    Group tg({0,0,0,0,target.B},r);
    check(equal(cm(tg,target.generator,u,b,zt),target.generator),"target fixed subgroup");
    check(cm(sg,source.generator,u,b,mod(zs*zs,p)).infinity &&
          cm(tg,target.generator,u,b,mod(zt*zt,r)).infinity,"conjugate orientation control");
    std::cout<<"{\"kind\":\"fixture\",\"m\":"<<m<<",\"d\":"<<winning_d
             <<",\"u\":\""<<u<<"\",\"r\":\""<<r<<"\",\"p\":\""<<p
             <<"\",\"r_bits\":"<<mpz_sizeinbase(r.get_mpz_t(),2)
             <<",\"p_bits\":"<<mpz_sizeinbase(p.get_mpz_t(),2)<<",\"B\":\""<<B
             <<"\",\"B_bits\":"<<mpz_sizeinbase(B.get_mpz_t(),2)
             <<",\"Bp\":\""<<source.B<<"\",\"Br\":\""<<target.B
             <<"\",\"zeta_p\":\""<<zs<<"\",\"zeta_r\":\""<<zt
             <<"\",\"etale_eigenvalue\":\""<<z0<<"\",\"source_embedding_degree\":\""
             <<multiplicative_order(mod(p,r),r,full)<<"\",\"source_to_target_evaluator\":false}\n";
    std::cout<<"{\"kind\":\"prime_certificates\",\"m\":"<<m<<",\"u_factor\":\""<<uc.factor
             <<"\",\"u_witnesses\":";print_factors(uf,&uc);
    std::cout<<",\"r_factor\":\""<<rc.factor<<"\",\"r_witnesses\":";print_factors(rf,&rc);
    std::cout<<",\"r_minus_one_factors\":";print_factors(full);
    std::cout<<",\"source_order_point_x\":\""<<source.generator.x
             <<"\",\"source_order_point_y\":\""<<source.generator.y
             <<"\",\"ecpp_integer_bound\":\""<<power(fourth+1,2)<<"\"}\n";
    std::cout<<"{\"kind\":\"common_kernel_field\",\"m\":"<<m<<",\"certificate_found\":"
             <<(aux.found?"true":"false")<<",\"variant\":"<<aux.variant<<",\"ell\":"<<aux.ell
             <<",\"trace\":\""<<aux.t<<"\",\"a\":\""<<aux.a<<"\",\"v\":\""<<aux.v
             <<"\",\"zeta_ell\":\""<<aux.zeta<<"\",\"eigenvalue\":\""<<aux.eigen<<"\"}\n";
    emit_point("source","generator",0,source.generator);emit_point("target","generator",0,target.generator);
    unsigned source_checks=0,target_checks=0,canonical_zero=0,projection_checks=0;
    unsigned tau=1;Z lambda_gen;
    for(;tau<=4;++tau) {lambda_gen=smart_lambda(target.generator,r,B,tau);if(lambda_gen!=0)break;}
    check(tau<=4,"noncanonical target solver is degenerate");
    auto check_source=[&](const Point& Q) {
        check(equal(cm(sg,Q,u,b,zs),Q) && equal(cm(sg,Q,0,1,zs),times(sg,Q,z0)),
              "source CM eigenpoint mismatch");
        Z modulus=p*p;Coeff a{0,0,0,0,mod(B,modulus)};Group lifted(a,modulus);
        Point raw=lift_ordinate(Q,a,modulus),projected=cm(lifted,raw,3-2*u,-2*b,lift_zeta(zs,p));
        check(strong_times(lifted,lifted.from(projected),r).z==0 &&
              mod(projected.x,p)==Q.x && mod(projected.y,p)==Q.y,"source contraction lift");
        ++source_checks;++projection_checks;
    };
    auto check_target=[&](const Point& S,const Z* known=nullptr) {
        check(equal(cm(tg,S,u,b,zt),S),"target CM eigenpoint mismatch");
        Z zero=smart_lambda(S,r,B,0);check(zero==0,"canonical-lift control not zero");++canonical_zero;
        Z k=mod(smart_lambda(S,r,B,tau)*inv(lambda_gen,r),r);
        check(equal(external_times(r,0,target.B,target.generator,k),S),"target scalar failed replay");
        if(known)check(k==*known,"target auditor scalar mismatch");
        check(!equal(external_times(r,0,target.B,target.generator,mod(k+1,r)),S),"perturbed scalar accepted");
        Z modulus=r*r;Coeff a{0,0,0,0,mod(B,modulus)};Group lifted(a,modulus);
        Point raw=lift_ordinate(S,a,modulus),projected=cm(lifted,raw,u,b,lift_zeta(zt,r));
        check(strong_times(lifted,lifted.from(projected),r).z==0 &&
              mod(projected.x,r)==S.x && mod(projected.y,r)==S.y,"target contraction lift");
        ++target_checks;++projection_checks;return k;
    };
    for(unsigned index=0;index<4;++index) {
        Point Q;
        for(unsigned attempt=0;attempt<16 && Q.infinity;++attempt)
            Q=times(sg,hash_point(p,source.B,"prime-field-transfer/round-03/m"+
                      std::to_string(m)+"/source/twist/"+std::to_string(source.twist)+
                      "/public/"+std::to_string(index)+"/attempt/"+std::to_string(attempt)),4);
        check(!Q.infinity,"public source admission exhausted");
        Point S=hash_point(r,target.B,"prime-field-transfer/round-03/m"+
                          std::to_string(m)+"/target/twist/"+std::to_string(target.twist)+
                          "/public/"+std::to_string(index));
        check_source(Q);Z k=check_target(S);
        emit_point("source","public_input_unmapped",index,Q);
        emit_point("target","independent_solver_input",index,S);
        std::cout<<"{\"kind\":\"target_only_solve\",\"m\":"<<m<<",\"index\":"<<index
                 <<",\"scalar\":\""<<k<<"\",\"is_source_image\":false}\n";
    }
    if(r==19)for(unsigned k=1;k<19;++k) {
        Point Q=times(sg,source.generator,Z(k)),S=times(tg,target.generator,Z(k));
        check_source(Q);Z known(k);check_target(S,&known);
    }
    // Target homomorphism control on independently generated public points.
    Point A=hash_point(r,target.B,"round03/target/addition/A"),C=hash_point(r,target.B,"round03/target/addition/C");
    Point sum=tg.to(tg.add(tg.from(A),tg.from(C)));
    check(mod(smart_lambda(A,r,B,tau)+smart_lambda(C,r,B,tau),r)==
          smart_lambda(sum,r,B,tau),"target logarithm addition law");
    std::cout<<"{\"kind\":\"checks\",\"m\":"<<m<<",\"source_inputs\":"<<source_checks
             <<",\"target_inputs\":"<<target_checks<<",\"canonical_zero_controls\":"<<canonical_zero
             <<",\"local_projection_checks\":"<<projection_checks<<",\"smart_lift_tau\":"<<tau
             <<",\"lambda_generator\":\""<<lambda_gen
             <<"\",\"source_to_target_outputs\":0,\"target_addition_check\":true}\n";
}
int main(int argc,char** argv) {
    std::cout.setf(std::ios::unitbuf);
    try {
        check(argc==2,"choose fixed m1,12,24,48,96,112");unsigned m=std::stoul(argv[1]);
        check(std::set<unsigned>{1,12,24,48,96,112}.count(m)==1,"undeclared bin");
        run(m);return 0;
    } catch(const std::exception& e) {std::cerr<<e.what()<<'\n';return 1;}
}
