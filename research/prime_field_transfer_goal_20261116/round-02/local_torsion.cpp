// Classical prime-to-p torsion lifting: an input-only local component.
// No source logarithm, subgroup table, global root selector, or target decoder.
#include <gmpxx.h>
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/evp.h>
#include <algorithm>
#include <array>
#include <iomanip>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using Z = mpz_class;
using Coeff = std::array<Z, 5>;
void check(bool b, const char* why) { if (!b) throw std::runtime_error(why); }
Z mod(Z a, const Z& m) { a %= m; if (a < 0) a += m; return a; }
Z inv(const Z& a, const Z& m) {
    Z out, v = mod(a, m);
    check(mpz_invert(out.get_mpz_t(), v.get_mpz_t(), m.get_mpz_t()) != 0,
          "nonunit denominator");
    return out;
}
Z power(const Z& a, unsigned n) {
    Z out; mpz_pow_ui(out.get_mpz_t(), a.get_mpz_t(), n); return out;
}
Z powmod(const Z& a, const Z& n, const Z& m) {
    Z out; mpz_powm(out.get_mpz_t(), a.get_mpz_t(), n.get_mpz_t(), m.get_mpz_t());
    return out;
}
std::string digest(const std::string& text) {
    unsigned char bytes[32]; size_t len = 0;
    check(EVP_Q_digest(nullptr, "SHA256", nullptr, text.data(), text.size(), bytes, &len)
              == 1 && len == 32, "SHA256 failure");
    std::ostringstream out; out << std::hex << std::setfill('0');
    for (unsigned char b : bytes) out << std::setw(2) << unsigned(b);
    return out.str();
}
struct Point {
    bool infinity = true;
    Z x = 0, y = 0;
};
bool equal(const Point& a, const Point& b) {
    return a.infinity == b.infinity && (a.infinity || (a.x == b.x && a.y == b.y));
}
std::string encoded(const Point& q) {
    return q.infinity ? "O\n" : q.x.get_str() + "," + q.y.get_str() + "\n";
}
struct Model {
    Z p, r;
    Coeff source, compact;
    Model(Z pp, Z rr, Coeff a) : p(pp), r(rr), source(a) {
        check(p > 3 && r > 3 && p != r, "unsupported characteristic/order");
        Z t = powmod(2, r, p), it = inv(t, p), is = inv(mod(2*t, p), p);
        compact = {mod((a[0]-1)*is,p), mod(a[1]*is,p), mod(a[2]*is,p),
                   mod(a[3]*is,p), mod(a[4]*it,p)};
        if (mpz_even_p(compact[4].get_mpz_t())) compact[4] += p;
    }
    Coeff at(const Z& modulus) const {
        Z t = powmod(2, r, modulus), s = mod(2*t, modulus);
        return {mod(1+s*compact[0],modulus), mod(s*compact[1],modulus),
                mod(s*compact[2],modulus), mod(s*compact[3],modulus),
                mod(t*compact[4],modulus)};
    }
};
Z equation(const Coeff& a, const Point& q, const Z& m) {
    return q.infinity ? Z(0) : mod(q.y*q.y+a[0]*q.x*q.y+a[2]*q.y-q.x*q.x*q.x
                                  -a[1]*q.x*q.x-a[3]*q.x-a[4], m);
}
// Normalized division polynomials f_even=psi_even/psi_2, f_odd=psi_odd.
// Dual numbers propagate the derivative without polynomial expansion.
struct Dual { Z value, derivative; };
struct Division {
    Z modulus;
    Coeff a;
    std::map<Z, Dual> memo;
    Dual x, D, D2;
    Dual constant(const Z& z) const { return {mod(z,modulus),0}; }
    Dual add(const Dual& u, const Dual& v) const {
        return {mod(u.value+v.value,modulus),mod(u.derivative+v.derivative,modulus)};
    }
    Dual sub(const Dual& u, const Dual& v) const {
        return {mod(u.value-v.value,modulus),mod(u.derivative-v.derivative,modulus)};
    }
    Dual mul(const Dual& u, const Dual& v) const {
        return {mod(u.value*v.value,modulus),
                mod(u.derivative*v.value+u.value*v.derivative,modulus)};
    }
    Dual square(const Dual& u) const { return mul(u,u); }
    Dual cube(const Dual& u) const { return mul(square(u),u); }
    Dual poly(const std::vector<Z>& coefficients) const {
        Dual out=constant(0);
        for (auto it=coefficients.rbegin(); it!=coefficients.rend(); ++it)
            out=add(mul(out,x),constant(*it));
        return out;
    }
    Division(Coeff aa, const Z& xx, Z mm) : modulus(mm), a(aa), x{mod(xx,mm),1} {
        Z b2=a[0]*a[0]+4*a[1], b4=a[0]*a[2]+2*a[3];
        Z b6=a[2]*a[2]+4*a[4];
        Z b8=a[0]*a[0]*a[4]+4*a[1]*a[4]-a[0]*a[2]*a[3]
             +a[1]*a[2]*a[2]-a[3]*a[3];
        D=poly({b6,2*b4,b2,4}); D2=square(D);
        memo[0]=constant(0); memo[1]=constant(1); memo[2]=constant(1);
        memo[3]=poly({b8,3*b6,3*b4,b2,3});
        memo[4]=poly({b4*b8-b6*b6,b2*b8-b4*b6,10*b8,10*b6,5*b4,b2,2});
    }
    Dual f(const Z& n) {
        auto it=memo.find(n); if (it!=memo.end()) return it->second;
        check(n>=0,"negative division index");
        Z h=n/2; Dual out;
        if (mpz_odd_p(n.get_mpz_t())) {
            Dual left=mul(f(h+2),cube(f(h)));
            Dual right=mul(f(h-1),cube(f(h+1)));
            out=mpz_even_p(h.get_mpz_t()) ? sub(mul(D2,left),right)
                                         : sub(left,mul(D2,right));
        } else {
            out=mul(f(h),sub(mul(f(h+2),square(f(h-1))),
                             mul(f(h-2),square(f(h+1)))));
        }
        memo.emplace(n,out); return out;
    }
};
struct Lift { Point q; unsigned rounds=0; size_t nodes=0; };
// This is the candidate component interface. It is never passed k or P.
Lift lift(const Model& model, const Point& input, unsigned precision) {
    check(precision>0,"zero precision");
    if (input.infinity) return {};
    check(input.x>=0 && input.x<model.p && input.y>=0 && input.y<model.p,
          "noncanonical source encoding");
    check(equation(model.at(model.p),input,model.p)==0,"off-curve source input");
    Lift out{input,0,0}; unsigned current=1;
    for (;;) {
        unsigned next=std::min(2*current,precision);
        Z modulus=power(model.p,next); Coeff a=model.at(modulus);
        Division div(a,out.q.x,modulus); Dual z=div.f(model.r);
        out.nodes=std::max(out.nodes,div.memo.size());
        check(mod(z.derivative,model.p)!=0,"nonsimple division root");
        if (current==1) check(mod(z.value,model.p)==0,"input is not r-torsion");
        out.q.x=mod(out.q.x-z.value*inv(z.derivative,modulus),modulus);
        Z derivative=mod(2*out.q.y+a[0]*out.q.x+a[2],modulus);
        out.q.y=mod(out.q.y-equation(a,out.q,modulus)*inv(derivative,modulus),modulus);
        Division verify(a,out.q.x,modulus);
        check(verify.f(model.r).value==0 && equation(a,out.q,modulus)==0,
              "Newton lift failed");
        check(mod(out.q.x,model.p)==input.x && mod(out.q.y,model.p)==input.y,
              "source encoding changed");
        ++out.rounds;
        if (next==precision) break;
        current=next;
    }
    return out;
}
// Independent group-law path: short Weierstrass Jacobian arithmetic.
struct Jac { Z x=1,y=1,z=0; };
struct Group {
    Z modulus, A, B, shift;
    Coeff a;
    Group(Coeff aa,Z mm) : modulus(mm),a(aa) {
        Z b2=a[0]*a[0]+4*a[1], b4=a[0]*a[2]+2*a[3], b6=a[2]*a[2]+4*a[4];
        shift=mod(b2*inv(12,mm),mm);
        A=mod((24*b4-b2*b2)*inv(48,mm),mm);
        B=mod((b2*b2*b2-36*b2*b4+216*b6)*inv(864,mm),mm);
    }
    Jac from(const Point& p) const {
        return p.infinity ? Jac{} : Jac{mod(p.x+shift,modulus),
            mod((2*p.y+a[0]*p.x+a[2])*inv(2,modulus),modulus),1};
    }
    Point to(const Jac& p) const {
        if (p.z==0) return {};
        Z iz=inv(p.z,modulus), x=mod(p.x*iz*iz-shift,modulus);
        Z y=mod(p.y*iz*iz*iz-(a[0]*x+a[2])*inv(2,modulus),modulus);
        return {false,x,y};
    }
    bool valid(const Jac& p) const {
        return mod(p.y*p.y-p.x*p.x*p.x-A*p.x*power(p.z,4)
                   -B*power(p.z,6),modulus)==0 &&
               (p.z!=0 || p.x!=0);
    }
    Jac twice(const Jac& p) const {
        if (p.z==0) return p;
        Z s=mod(4*p.x*p.y*p.y,modulus);
        Z m=mod(3*p.x*p.x+A*power(p.z,4),modulus);
        Z x=mod(m*m-2*s,modulus);
        return {x,mod(m*(s-x)-8*power(p.y,4),modulus),mod(2*p.y*p.z,modulus)};
    }
    Jac add(const Jac& p,const Jac& q) const {
        if (p.z==0) return q;
        if (q.z==0) return p;
        Z u1=mod(p.x*q.z*q.z,modulus),u2=mod(q.x*p.z*p.z,modulus);
        Z s1=mod(p.y*q.z*q.z*q.z,modulus),s2=mod(q.y*p.z*p.z*p.z,modulus);
        if (u1==u2 && s1==s2) return twice(p);
        Z h=mod(u2-u1,modulus),r=mod(s2-s1,modulus);
        Z x=mod(r*r-h*h*h-2*u1*h*h,modulus);
        return {x,mod(r*(u1*h*h-x)-s1*h*h*h,modulus),mod(h*p.z*q.z,modulus)};
    }
    Jac times(Jac p,Z n) const {
        Jac out;
        while (n>0) {
            if (mpz_odd_p(n.get_mpz_t())) out=add(out,p);
            n>>=1; if (n>0) p=twice(p);
        }
        check(valid(out),"invalid Jacobian result"); return out;
    }
};
BIGNUM* bn(const Z& z) {
    BIGNUM* out=nullptr;
    check(BN_hex2bn(&out,z.get_str(16).c_str())>0,"BIGNUM input"); return out;
}
Z integer(const BIGNUM* b) {
    char* hex=BN_bn2hex(b); check(hex!=nullptr,"BIGNUM output");
    Z z(hex,16); OPENSSL_free(hex); return z;
}
struct Source {
    BN_CTX* ctx=BN_CTX_new();
    EC_GROUP* group=nullptr;
    Source(const Model& model) {
        check(model.source[0]==0 && model.source[1]==0 && model.source[2]==0,
              "source checker requires short model");
        BIGNUM *p=bn(model.p),*a=bn(model.source[3]),*b=bn(model.source[4]);
        group=EC_GROUP_new_curve_GFp(p,a,b,ctx); BN_free(p);BN_free(a);BN_free(b);
        check(group!=nullptr && EC_GROUP_check_discriminant(group,ctx)==1,
              "OpenSSL curve failure");
    }
    ~Source() { EC_GROUP_free(group); BN_CTX_free(ctx); }
    EC_POINT* from(const Point& q) {
        EC_POINT* out=EC_POINT_new(group);check(out!=nullptr,"point allocation");
        if (q.infinity) check(EC_POINT_set_to_infinity(group,out)==1,"set infinity");
        else {
            BIGNUM *x=bn(q.x),*y=bn(q.y);
            int ok=EC_POINT_set_affine_coordinates(group,out,x,y,ctx);
            BN_free(x);BN_free(y);check(ok==1,"OpenSSL point membership");
        }
        return out;
    }
    Point to(const EC_POINT* q) {
        if (EC_POINT_is_at_infinity(group,q)) return {};
        BIGNUM *x=BN_new(),*y=BN_new();
        check(EC_POINT_get_affine_coordinates(group,q,x,y,ctx)==1,"point read");
        Point out{false,integer(x),integer(y)};BN_free(x);BN_free(y);return out;
    }
    Point times(const Point& q,const Z& scalar) {
        EC_POINT *input=from(q),*out=EC_POINT_new(group);BIGNUM* n=bn(scalar);
        check(EC_POINT_mul(group,out,nullptr,input,n,ctx)==1,"OpenSSL multiply");
        Point result=to(out);EC_POINT_free(input);EC_POINT_free(out);BN_free(n);return result;
    }
    Point add(const Point& p,const Point& q) {
        EC_POINT *pp=from(p),*qq=from(q),*out=EC_POINT_new(group);
        check(EC_POINT_add(group,out,pp,qq,ctx)==1,"OpenSSL add");
        Point result=to(out);EC_POINT_free(pp);EC_POINT_free(qq);EC_POINT_free(out);return result;
    }
};
Point public_point(const Model& model,unsigned index) {
    Z x(digest("prime-field-transfer/round-02/public-point/"+std::to_string(index)),16);
    x=mod(x,model.p);
    for (;;) {
        Z rhs=mod(x*x*x+7,model.p),y=powmod(rhs,Z((model.p+1)/4),model.p);
        if (mod(y*y,model.p)==rhs && y!=0) return {false,x,std::min(y,Z(model.p-y))};
        x=mod(x+1,model.p);
    }
}
void fixture(const char* name,const Model& model,const Point& generator,bool small) {
    Source source(model);check(source.times(generator,model.r).infinity,"generator order");
    check(model.at(model.p)==model.source,"compact model projection");
    std::vector<Point> inputs;
    if (small) for (unsigned k=1;k<model.r.get_ui();++k)
        inputs.push_back(source.times(generator,Z(k)));
    else for (unsigned i=0;i<4;++i) {
        Point q=public_point(model,i);check(source.times(q,model.r).infinity,"public point order");
        inputs.push_back(q);
        std::cout<<"{\"kind\":\"public_input\",\"curve\":\""<<name<<"\",\"index\":"<<i
                 <<",\"x\":\""<<q.x<<"\",\"y\":\""<<q.y<<"\"}\n";
    }
    for (unsigned precision : {1U,2U,4U,8U,16U,32U,64U}) {
        Z modulus=power(model.p,precision);Coeff a=model.at(modulus);Group group(a,modulus);
        Lift gp=lift(model,generator,precision);
        check(lift(model,{},precision).q.infinity,"identity lift");
        size_t nodes=0;unsigned rounds=0,relations=0,controls=0,recurrence_checks=0;
        std::string transcript;
        for (size_t index=0;index<inputs.size();++index) {
            const Point& input=inputs[index];Lift out=lift(model,input,precision);
            Jac q=group.from(out.q);
            check(group.valid(q) && group.times(q,model.r).z==0,"independent torsion check");
            if (small) {
                Point expect=group.to(group.times(group.from(gp.q),Z(index+1)));
                check(equal(out.q,expect),"lift does not preserve auditor scalar");++relations;
            }
            Division div(a,out.q.x,modulus);
            for (unsigned n : {1U,2U,3U,4U}) {
                Z fn=div.f(Z(n)).value, product=div.f(Z(n-1)).value*div.f(Z(n+1)).value;
                Z den=mod(fn*fn,modulus);
                if (n%2==0) den=mod(den*div.D.value,modulus);
                else product=mod(product*div.D.value,modulus);
                Z x=mod(out.q.x-product*inv(den,modulus),modulus);
                check(x==group.to(group.times(q,Z(n))).x,"division recurrence/group mismatch");
                ++recurrence_checks;
            }
            Point negated{false,out.q.x,mod(-out.q.y-a[0]*out.q.x-a[2],modulus)};
            check(mod(negated.y,model.p)!=input.y,"wrong ordinate control accepted");++controls;
            if (precision>=2) {
                Division wrong(a,mod(out.q.x+model.p,modulus),modulus);
                check(wrong.f(model.r).value!=0,"perturbed x control accepted");++controls;
            }
            if (!small && precision==64) {
                Point negsource{false,input.x,mod(-input.y,model.p)};
                check(equal(lift(model,negsource,precision).q,negated),"negation consistency");
                Point sum=source.add(input,generator);
                check(equal(lift(model,sum,precision).q,
                            group.to(group.add(q,group.from(gp.q)))),"addition consistency");
                relations+=2;
            }
            nodes=std::max(nodes,out.nodes);rounds=std::max(rounds,out.rounds);
            transcript+=encoded(out.q);
        }
        Point malformed=inputs.front();malformed.y=mod(malformed.y+1,model.p);
        check(equation(model.source,malformed,model.p)!=0,"malformed fixture is valid");
        bool rejected=false;try { (void)lift(model,malformed,precision); }
        catch (const std::runtime_error&) { rejected=true; }
        check(rejected,"malformed input accepted");++controls;
        std::cout<<"{\"kind\":\"local_lift\",\"curve\":\""<<name<<"\",\"order_bits\":"
                 <<mpz_sizeinbase(model.r.get_mpz_t(),2)<<",\"precision_digits\":"<<precision
                 <<",\"inputs\":"<<inputs.size()<<",\"torsion_checks\":"<<inputs.size()
                 <<",\"scalar_or_additive_relations\":"<<relations<<",\"controls_rejected\":"
                 <<controls<<",\"recurrence_checks\":"<<recurrence_checks
                 <<",\"max_recurrence_nodes\":"<<nodes<<",\"newton_rounds\":"<<rounds
                 <<",\"coordinate_sha256\":\""<<digest(transcript)<<"\"}\n";
    }
}
int main() {
    try {
        fixture("p17-r19",Model(17,19,{0,0,0,2,2}),{false,5,1},true);
        fixture("p257-r83",Model(257,83,{0,0,0,23,11}),{false,7,1},true);
        Model secp(Z("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16),
                   Z("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",16),
                   {0,0,0,0,7});
        Point generator{false,
            Z("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16),
            Z("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",16)};
        fixture("secp256k1",secp,generator,false);
        return 0;
    } catch (const std::exception& e) { std::cerr<<e.what()<<'\n';return 1; }
}
