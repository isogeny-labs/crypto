#include "norm_support.hpp"
#include <openssl/ec.h>
#include <openssl/obj_mac.h>
#include <memory>
#include <set>

namespace audit {
using Big=mpz_class;
using Pair=std::array<int,2>;
using Quad=std::array<int,4>;
using Line=std::array<Big,3>;
const Big p("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFF",16);
const Big b("64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1",16);
Big mod(Big x){x%=p;if(x<0)x+=p;return x;}
Big inv(Big x){Big y; x=mod(x);require(mpz_invert(y.get_mpz_t(),x.get_mpz_t(),p.get_mpz_t())!=0,"Fp inverse");return y;}
Big power(Big x,int n){Big y=1;while(n--){y=mod(y*x);}return y;}
struct Point {bool zero=true;Big x=0,y=0;};
bool operator==(const Point& x,const Point& y){return x.zero==y.zero&&(x.zero||(x.x==y.x&&x.y==y.y));}
Point negative(Point x){if(!x.zero)x.y=mod(-x.y);return x;}
bool on_curve(const Point& x){return x.zero||(x.x>=0&&x.x<p&&x.y>=0&&x.y<p&&mod(x.y*x.y)==mod(x.x*x.x*x.x-3*x.x+b));}
Point plus(const Point& x,const Point& y){
    if(x.zero){return y;}
    if(y.zero){return x;}
    Big slope;
    if(x.x==y.x){if(mod(x.y+y.y)==0)return {};require(x.y==y.y,"equal-x elliptic inputs");slope=mod((3*x.x*x.x-3)*inv(2*x.y));}
    else slope=mod((y.y-x.y)*inv(y.x-x.x));
    Big xx=mod(slope*slope-x.x-y.x);return {false,xx,mod(slope*(x.x-xx)-x.y)};
}
Point read_point(const Json& j){if(j.at("infinity").scalar=="true")return {};require(j.at("infinity").scalar=="false","point infinity flag");return {false,Big(j.at("x").scalar),Big(j.at("y").scalar)};}
std::string key(const Point& q){
    if(q.zero)return std::string(1,'\0');
    std::array<unsigned char,24> bytes{};size_t count=0;mpz_export(bytes.data(),&count,1,1,1,0,q.x.get_mpz_t());require(count<=24,"point width");
    std::string out(25,'\0');out[0]=char(mpz_odd_p(q.y.get_mpz_t())?3:2);
    for(size_t i=0;i<count;++i){out[25-count+i]=char(bytes[i]);}
    return out;
}
struct DeletePoint{void operator()(EC_POINT* q)const{EC_POINT_free(q);}};
struct DeleteBN{void operator()(BIGNUM* q)const{BN_free(q);}};
using SSLPoint=std::unique_ptr<EC_POINT,DeletePoint>;
using BN=std::unique_ptr<BIGNUM,DeleteBN>;
BN bn(const Big& x){BIGNUM* q=nullptr;require(BN_dec2bn(&q,x.get_str().c_str())!=0,"BN conversion");return BN(q);}
struct SSL {
    std::unique_ptr<EC_GROUP,decltype(&EC_GROUP_free)> group{EC_GROUP_new_by_curve_name(NID_X9_62_prime192v1),EC_GROUP_free};
    std::unique_ptr<BN_CTX,decltype(&BN_CTX_free)> context{BN_CTX_new(),BN_CTX_free};
    SSL(){require(bool(group)&&bool(context),"OpenSSL setup");BN pp(BN_new()),aa(BN_new()),bb(BN_new()),nn(BN_new()),hh(BN_new());
        require(EC_GROUP_get_curve(group.get(),pp.get(),aa.get(),bb.get(),context.get())==1&&EC_GROUP_get_order(group.get(),nn.get(),context.get())==1&&EC_GROUP_get_cofactor(group.get(),hh.get(),context.get())==1,"named group parameters");
        auto check=[&](const BN& got,const Big& expected){auto value=bn(expected);require(BN_cmp(got.get(),value.get())==0,"exact P192 parameters");};
        check(pp,p);check(aa,mod(-3));check(bb,b);check(nn,subgroup_order);require(BN_is_one(hh.get()),"P192 cofactor1");}
    SSLPoint make(const Point& q)const{SSLPoint out(EC_POINT_new(group.get()));require(bool(out),"EC point allocation");
        if(q.zero)require(EC_POINT_set_to_infinity(group.get(),out.get())==1,"identity setup");
        else{auto x=bn(q.x),y=bn(q.y);require(EC_POINT_set_affine_coordinates(group.get(),out.get(),x.get(),y.get(),context.get())==1,"SSL affine point");}
        require(EC_POINT_is_on_curve(group.get(),out.get(),context.get())==1,"SSL curve equation");return out;}
    std::string encode(const EC_POINT* q)const{size_t n=EC_POINT_point2oct(group.get(),q,POINT_CONVERSION_COMPRESSED,nullptr,0,context.get());require(n==1||n==25,"compressed point length");std::string out(n,'\0');require(EC_POINT_point2oct(group.get(),q,POINT_CONVERSION_COMPRESSED,reinterpret_cast<unsigned char*>(out.data()),n,context.get())==n,"compressed point encoding");return out;}
};
}
