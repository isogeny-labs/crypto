#pragma once
#include <cstdint>
#include <stdexcept>

// Separate affine arithmetic and extended-Euclidean inversion. No production
// curve, field, table, row-space code or logarithms are used by this checker.
namespace independent {
using U=std::uint64_t;using W=__uint128_t;using I=__int128_t;
struct Point {U x=0,y=0;bool infinity=true;bool operator==(Point q)const{return infinity==q.infinity&&(infinity||(x==q.x&&y==q.y));}};
class Curve {
    U p,a,b;
    U add(U x,U y)const{return U((W(x)+y)%p);}
    U sub(U x,U y)const{return U((W(x)+p-y)%p);}
    U mul(U x,U y)const{return U(W(x)*y%p);}
    U inv(U x)const {
        I t=0,u=1;U n=p,m=x;while(m){U q=n/m,z=n%m;n=m;m=z;I v=t-I(q)*u;t=u;u=v;}
        if(n!=1)throw std::runtime_error("independent inverse");
        t%=I(p);if(t<0)t+=p;return U(t);
    }
public:
    Curve(U prime,U aa,U bb):p(prime),a(aa),b(bb){if(p<5||p>=(U(1)<<62))throw std::runtime_error("independent modulus range");}
    bool on(Point q)const{return q.infinity||(q.x<p&&q.y<p&&mul(q.y,q.y)==add(add(mul(mul(q.x,q.x),q.x),mul(a,q.x)),b));}
    Point neg(Point q)const{if(!q.infinity&&q.y)q.y=p-q.y;return q;}
    Point plus(Point x,Point y)const {
        if(x.infinity)return y;
        if(y.infinity)return x;
        if(x.x==y.x&&add(x.y,y.y)==0)return {};
        U slope=x.x==y.x?mul(add(mul(3,mul(x.x,x.x)),a),inv(mul(2,x.y))):mul(sub(y.y,x.y),inv(sub(y.x,x.x)));
        U xx=sub(sub(mul(slope,slope),x.x),y.x);return {xx,sub(mul(slope,sub(x.x,xx)),x.y),false};
    }
    Point times(U n,Point q)const{Point out;while(n){if(n&1)out=plus(out,q);q=plus(q,q);n>>=1;}return out;}
    Point signed_times(int n,Point q)const{return n<0?neg(times(U(-n),q)):times(U(n),q);}
};
} // namespace independent
