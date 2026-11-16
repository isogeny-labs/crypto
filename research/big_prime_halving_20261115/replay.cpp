// Independent BN polynomial and EC_GROUP replay. No candidate/GMP/FLINT includes.
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <algorithm>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <limits>
#include <memory>
#include <random>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace ref {
void require(bool b,const std::string&s){if(!b)throw std::runtime_error(s);}
struct N {
    BIGNUM*v;
    N():v(BN_new()){require(v!=nullptr,"BN allocation");BN_zero(v);}
    N(unsigned long n):N(){require(BN_set_word(v,n)==1,"BN word");}
    explicit N(const std::string&s):N(){
        require(!s.empty()&&(s=="0"||s[0]!='0'),"canonical BN decimal");
        for(char c:s)require(c>='0'&&c<='9',"BN decimal character");
        require(BN_dec2bn(&v,s.c_str())>0,"BN decimal parse");
    }
    N(const N&n):v(BN_dup(n.v)){require(v!=nullptr,"BN copy");}
    N&operator=(const N&n){if(this!=&n)require(BN_copy(v,n.v)!=nullptr,"BN assign");return *this;}
    ~N(){BN_free(v);}
    std::string str()const{char*s=BN_bn2dec(v);require(s!=nullptr,"BN decimal output");std::string r(s);OPENSSL_free(s);return r;}
    bool zero()const{return BN_is_zero(v);}
};
bool operator==(const N&a,const N&b){return BN_cmp(a.v,b.v)==0;}
bool operator!=(const N&a,const N&b){return !(a==b);}
bool operator<(const N&a,const N&b){return BN_cmp(a.v,b.v)<0;}
struct Field {
    N p;BN_CTX*ctx;
    explicit Field(const N&modulus):p(modulus),ctx(BN_CTX_new()){
        require(ctx&&BN_num_bits(p.v)>=3&&BN_num_bits(p.v)<=4096&&!(p<N(5)),"reference modulus admission");
        require(BN_check_prime(p.v,ctx,nullptr)==1,"OpenSSL prime admission");
    }
    ~Field(){BN_CTX_free(ctx);}
    bool canonical(const N&x)const{return !BN_is_negative(x.v)&&x<p;}
    N add(const N&a,const N&b)const{N r;require(BN_mod_add(r.v,a.v,b.v,p.v,ctx)==1,"BN add");return r;}
    N sub(const N&a,const N&b)const{N r;require(BN_mod_sub(r.v,a.v,b.v,p.v,ctx)==1,"BN subtract");return r;}
    N mul(const N&a,const N&b)const{N r;require(BN_mod_mul(r.v,a.v,b.v,p.v,ctx)==1,"BN multiply");return r;}
    N inv(const N&a)const{N r;require(BN_mod_inverse(r.v,a.v,p.v,ctx)!=nullptr,"BN inverse");return r;}
    N pow(const N&a,unsigned n)const{N r(1),b(a);while(n){if(n&1)r=mul(r,b);n>>=1;if(n)b=mul(b,b);}return r;}
    std::vector<N> square_roots(const N&a)const{
        require(canonical(a),"BN sqrt canonical");
        if(a.zero())return {N(0)};
        N exp(p),one(1),symbol;
        require(BN_sub(exp.v,exp.v,one.v)==1&&BN_rshift1(exp.v,exp.v)==1,"BN Legendre exponent");
        require(BN_mod_exp(symbol.v,a.v,exp.v,p.v,ctx)==1,"BN Legendre");
        if(symbol!=one)return {};
        N r;require(BN_mod_sqrt(r.v,a.v,p.v,ctx)!=nullptr,"BN square root");require(mul(r,r)==a,"BN sqrt check");
        N neg=sub(N(0),r);return r<neg?std::vector<N>{r,neg}:std::vector<N>{neg,r};
    }
};
using P=std::vector<N>;
void trim(P&a){while(!a.empty()&&a.back().zero())a.pop_back();}
P add(const Field&f,P a,const P&b){a.resize(std::max(a.size(),b.size()));for(std::size_t i=0;i<b.size();++i)a[i]=f.add(a[i],b[i]);trim(a);return a;}
P scale(const Field&f,P a,const N&s){for(N&x:a)x=f.mul(x,s);trim(a);return a;}
P minus(const Field&f,P a,const P&b){return add(f,a,scale(f,b,f.sub(N(0),N(1))));}
P mul(const Field&f,const P&a,const P&b){
    if(a.empty()||b.empty())return {};
    P r(a.size()+b.size()-1);
    for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)r[i+j]=f.add(r[i+j],f.mul(a[i],b[j]));
    trim(r);return r;
}
P power(const Field&f,const P&a,unsigned n){P r{N(1)},b=a;while(n){if(n&1)r=mul(f,r,b);n>>=1;if(n)b=mul(f,b,b);}return r;}
P rem(const Field&f,P a,const P&b){
    require(!b.empty(),"polynomial division by zero");const N inverse=f.inv(b.back());
    while(a.size()>=b.size()){
        const std::size_t offset=a.size()-b.size();N q=f.mul(a.back(),inverse);
        for(std::size_t j=0;j<b.size();++j)a[j+offset]=f.sub(a[j+offset],f.mul(q,b[j]));
        trim(a);
    }
    return a;
}
P monic(const Field&f,P a){return a.empty()?a:scale(f,a,f.inv(a.back()));}
P gcd(const Field&f,P a,P b){while(!b.empty()){P r=rem(f,a,b);a=b;b=r;}return monic(f,a);}
P x_to_p_mod(const Field&f,const P&q){
    P r{N(1)},x{N(0),N(1)};
    for(int i=BN_num_bits(f.p.v)-1;i>=0;--i){r=rem(f,mul(f,r,r),q);if(BN_is_bit_set(f.p.v,i))r=rem(f,mul(f,r,x),q);}return r;
}
std::string list(const P&v){if(v.empty())return "-";std::string s;for(const N&x:v){if(!s.empty())s+=',';s+=x.str();}return s;}
struct Point {N x,y;bool infinity=true;};
bool eq(const Point&a,const Point&b){return a.infinity==b.infinity&&(a.infinity||(a.x==b.x&&a.y==b.y));}
bool order(const Point&a,const Point&b){return a.x<b.x||(a.x==b.x&&a.y<b.y);}
std::string pt(const Point&a){return a.infinity?"-":a.x.str()+":"+a.y.str();}
std::string points(const std::vector<Point>&v){if(v.empty())return "-";std::string s;for(const auto&q:v){if(!s.empty())s+=';';s+=pt(q);}return s;}
using EP=std::unique_ptr<EC_POINT,decltype(&EC_POINT_free)>;
struct Curve {
    const Field*f;N a,b;std::shared_ptr<EC_GROUP>group;
    Curve(const Field&field,const N&aa,const N&bb):f(&field),a(aa),b(bb){
        require(f->canonical(a)&&f->canonical(b),"canonical curve coefficients");
        EC_GROUP*g=EC_GROUP_new_curve_GFp(f->p.v,a.v,b.v,f->ctx);
        require(g!=nullptr,"OpenSSL curve allocation");group={g,EC_GROUP_free};
        require(EC_GROUP_check_discriminant(g,f->ctx)==1,"OpenSSL singular curve");
    }
    EP fresh()const{EP p(EC_POINT_new(group.get()),EC_POINT_free);require(bool(p),"EC point allocation");return p;}
    EP encode(const Point&p)const{
        auto q=fresh();
        if(p.infinity)require(EC_POINT_set_to_infinity(group.get(),q.get())==1,"EC infinity");
        else{
            require(f->canonical(p.x)&&f->canonical(p.y),"noncanonical point");
            require(EC_POINT_set_affine_coordinates(group.get(),q.get(),p.x.v,p.y.v,f->ctx)==1
                &&EC_POINT_is_on_curve(group.get(),q.get(),f->ctx)==1,"off-curve point");
        }
        return q;
    }
    Point decode(const EC_POINT*q)const{
        if(EC_POINT_is_at_infinity(group.get(),q)==1)return {};
        Point p;p.infinity=false;require(EC_POINT_get_affine_coordinates(group.get(),q,p.x.v,p.y.v,f->ctx)==1,"EC coordinates");return p;
    }
    Point sum(const Point&p,const Point&q)const{auto a=encode(p),b=encode(q),r=fresh();require(EC_POINT_add(group.get(),r.get(),a.get(),b.get(),f->ctx)==1,"EC add");return decode(r.get());}
    Point dbl(const Point&p)const{auto a=encode(p),r=fresh();require(EC_POINT_dbl(group.get(),r.get(),a.get(),f->ctx)==1,"EC double");return decode(r.get());}
    Point multiple(const Point&p,unsigned n)const{auto a=encode(p),r=fresh();N k(n);require(EC_POINT_mul(group.get(),r.get(),nullptr,a.get(),k.v,f->ctx)==1,"EC multiply");return decode(r.get());}
    N rhs(const N&x)const{return f->add(f->add(f->pow(x,3),f->mul(a,x)),b);}
};
bool nonsingular(const Field&f,const N&a,const N&b){return !f.add(f.mul(N(4),f.pow(a,3)),f.mul(N(27),f.pow(b,2))).zero();}
std::vector<std::string> split(const std::string&s,char delimiter){std::vector<std::string>r;std::istringstream in(s);std::string t;while(std::getline(in,t,delimiter))r.push_back(t);return r;}
std::vector<std::string> tokens(const std::string&s){std::vector<std::string>v;std::istringstream in(s);std::string t;while(in>>t)v.push_back(t);return v;}
unsigned small(const std::string&s){N n(s);require(BN_num_bits(n.v)<=31,"small integer cap");return static_cast<unsigned>(BN_get_word(n.v));}
P numbers(const Field&f,const std::string&s){
    P r;if(s=="-")return r;
    for(const auto&t:split(s,',')){N n(t);require(f.canonical(n),"canonical field list");r.push_back(n);}require(list(r)==s,"canonical list syntax");return r;
}
Point point(const Field&f,const std::string&s){if(s=="-")return {};const auto t=split(s,':');require(t.size()==2,"point syntax");Point p{N(t[0]),N(t[1]),false};require(f.canonical(p.x)&&f.canonical(p.y)&&pt(p)==s,"canonical point syntax");return p;}
struct Reader {
    std::ifstream in;unsigned line=0;
    explicit Reader(const std::filesystem::path&path):in(path){require(bool(in),"missing transcript");}
    std::string next(){std::string s;require(bool(std::getline(in,s)),"truncated transcript at line "+std::to_string(line+1));++line;require(s.size()<1048576,"transcript line cap");return s;}
    void expect(const std::string&s,const std::string&domain){require(next()==s,domain+" mismatch at line "+std::to_string(line));}
    void end(){std::string s;require(!std::getline(in,s),"trailing transcript bytes");}
};
class Random {
    std::mt19937_64 stream;Reader&r;
public:
    Random(std::uint64_t seed,Reader&reader):stream(seed),r(reader){}
    N sample(const N&size,const std::string&domain){
        require(!size.zero(),"empty RNG range");
        for(unsigned attempt=0;attempt<4096;++attempt){
            N val;std::string words;bool yes;unsigned bits=0;
            if(BN_num_bits(size.v)<=64){
                const auto n=static_cast<std::uint64_t>(BN_get_word(size.v));
                const auto threshold=(std::uint64_t(0)-n)%n,raw=stream();
                yes=raw>=threshold;words=std::to_string(raw);if(yes)val=N(static_cast<unsigned long>(raw%n));
            }else{
                N top(size);require(BN_sub_word(top.v,1)==1,"RNG range bits");bits=static_cast<unsigned>(BN_num_bits(top.v));N raw;
                for(unsigned i=0;i<(bits+63)/64;++i){
                    const auto w=stream();if(!words.empty())words+=',';words+=std::to_string(w);
                    N part(static_cast<unsigned long>(w));require(BN_lshift(part.v,part.v,static_cast<int>(64*i))==1&&BN_add(raw.v,raw.v,part.v)==1,"RNG word assembly");
                }
                if(BN_num_bits(raw.v)>static_cast<int>(bits))require(BN_mask_bits(raw.v,static_cast<int>(bits))==1,"RNG top mask");
                yes=raw<size;if(yes)val=raw;
            }
            r.expect("D "+domain+" "+size.str()+" "+std::to_string(bits)+" "+words+" "+std::to_string(yes)+" "+val.str(),"RNG");
            if(yes)return val;
        }
        throw std::runtime_error("reference RNG cap");
    }
};
// Derive q from the differentiation/doubling equation, not expanded coefficients.
P quartic(const Curve&e,const Point&p){
    const Field&f=*e.f;const P cubic{e.b,e.a,N(0),N(1)},derivative{e.a,N(0),N(3)};
    return minus(f,mul(f,derivative,derivative),scale(f,mul(f,P{p.x,N(2)},cubic),N(4)));
}
std::vector<Point> verify_preimages(const Curve&e,const Point&target,const P&xs){
    const Field&f=*e.f;require(!target.infinity,"finite reference target");e.encode(target);
    require(std::is_sorted(xs.begin(),xs.end(),[](const N&a,const N&b){return a<b;})&&std::adjacent_find(xs.begin(),xs.end())==xs.end(),"sorted distinct claimed roots");
    const P q=quartic(e,target),frobenius=minus(f,x_to_p_mod(f,q),P{N(0),N(1)});
    P product{N(1)};for(const N&x:xs)product=mul(f,product,P{f.sub(N(0),x),N(1)});
    require(product==gcd(f,q,frobenius),"root completeness");
    std::vector<Point> halves;
    for(const N&x:xs)for(const N&y:f.square_roots(e.rhs(x))){Point h{x,y,false};if(eq(e.dbl(h),target))halves.push_back(h);}
    std::sort(halves.begin(),halves.end(),order);require(halves.size()<=4,"reference degree cap");return halves;
}
std::vector<Point> read_halves(Reader&r,const Curve&e,const Point&t,unsigned stage){
    const auto fields=tokens(r.next());require(fields.size()==7&&fields[0]=="Q"&&fields[1]==std::to_string(stage)&&fields[2]==pt(t),"preimage event binding");
    P xs=numbers(*e.f,fields[4]);require(xs.size()==small(fields[3]),"preimage root count");
    const auto halves=verify_preimages(e,t,xs);
    require(halves.size()==small(fields[5])&&points(halves)==fields[6],"complete preimage lifts");return halves;
}
void curve_identity(const Field&f,const N&a,const N&b,const N&xi,const N&tau,const N&ap,const N&bp){
    const P den{f.sub(N(0),xi),N(1)},num{tau,f.sub(N(0),xi),N(1)};
    const P cubic{b,a,N(0),N(1)},ydiff=minus(f,power(f,den,2),P{tau});
    const P left=mul(f,cubic,power(f,ydiff,2));
    const P right=add(f,add(f,mul(f,power(f,num,3),den),scale(f,mul(f,num,power(f,den,3)),ap)),scale(f,power(f,den,4),bp));
    require(left==right,"global curve identity");
}
Point map_point(const Curve&e,const Curve&next,const N&xi,const N&tau,const Point&p){
    e.encode(p);if(p.infinity||p.x==xi)return {};
    const Field&f=*e.f;const N den=f.sub(p.x,xi),inverse=f.inv(den);
    // x + tau/(x-xi), equivalent to the shared numerator identity.
    Point q{f.add(p.x,f.mul(tau,inverse)),f.mul(p.y,f.sub(N(1),f.mul(tau,f.mul(inverse,inverse)))),false};
    next.encode(q);return q;
}
void receipt(const std::filesystem::path&path,const N&p,const N&a,const N&b,const Point&t,const Point&g,const N&terminal,const P&roots,unsigned trials){
    std::ifstream in(path);require(bool(in),"missing setup receipt");std::ostringstream expected;
    expected<<"p\ta\tb\ttx\tty\tgx\tgy\tterminal\tfibre\tcurve_trials\n"<<p.str()<<'\t'<<a.str()<<'\t'<<b.str()<<'\t'<<t.x.str()<<'\t'<<t.y.str()<<'\t'<<g.x.str()<<'\t'<<g.y.str()<<'\t'<<terminal.str()<<'\t'<<list(roots)<<'\t'<<trials<<'\n';
    const std::string actual{std::istreambuf_iterator<char>(in),{}};require(actual==expected.str(),"receipt binding");
}
void setup(const std::filesystem::path&root){
    Reader r(root/"events.txt");const auto h=tokens(r.next());
    require(h.size()==4&&h[0]=="BIG-HALVING-1"&&h[3]=="1234","setup header/seed");
    Field f{N(h[1])};const unsigned depth=small(h[2]);require(depth>=3&&depth<=12,"reference depth cap");
    const unsigned degree=1U<<depth;Random rng(1234,r);
    for(unsigned trial=0;trial<4096;++trial){
        const N a=rng.sample(f.p,"A"),x=rng.sample(f.p,"X");const N b=f.sub(N(0),f.add(f.pow(x,3),f.mul(a,x)));
        const bool good=nonsingular(f,a,b);
        r.expect("C "+std::to_string(trial)+" "+a.str()+" "+x.str()+" "+b.str()+" "+std::to_string(good),"model branch");
        if(!good)continue;
        Curve e(f,a,b);Point t{x,N(0),false};bool dead=false;
        for(unsigned i=0;i<depth-1;++i){
            const auto halves=read_halves(r,e,t,i);if(halves.empty()){dead=true;break;}
            const N chosen=rng.sample(N(halves.size()),"H"+std::to_string(i));t=halves.at(BN_get_word(chosen.v));
            r.expect("K "+std::to_string(i)+" "+chosen.str()+" "+pt(t),"halving choice");
        }
        if(dead)continue;
        require(e.multiple(t,degree).infinity&&!e.multiple(t,degree/2).infinity,"reference exact order");r.expect("T "+pt(t),"subgroup generator");
        std::vector<Point> subgroup;Point hp;
        for(unsigned j=0;j<degree;++j){subgroup.push_back(hp);hp=e.sum(hp,t);}require(hp.infinity,"reference subgroup cycle");
        for(unsigned slot=0;slot<64;++slot){
            Point g;bool proposed=false;N domain;require(BN_lshift1(domain.v,f.p.v)==1&&BN_add_word(domain.v,1)==1,"point sample domain");
            for(unsigned j=0;j<128;++j){
                const N u=rng.sample(domain,"G");N inf(domain);require(BN_sub_word(inf.v,1)==1,"infinity code");
                const std::string prefix="P "+std::to_string(slot)+" "+std::to_string(j)+" "+u.str();
                if(u==inf){r.expect(prefix+" i -","point infinity");proposed=true;break;}
                N gx;require(BN_rshift1(gx.v,u.v)==1,"point x code");const unsigned sign=BN_is_odd(u.v)?1:0;const auto ys=f.square_roots(e.rhs(gx));
                if(ys.empty()){r.expect(prefix+" n "+gx.str(),"nonsquare proposal");continue;}
                if(ys.size()==1&&sign==1){r.expect(prefix+" z "+gx.str(),"duplicate zero sign");continue;}
                g={gx,ys.at(sign),false};e.encode(g);r.expect(prefix+" p "+pt(g),"accepted point proposal");proposed=true;break;
            }
            require(proposed,"reference point proposal cap");const Point two=e.dbl(g);bool trace=!g.infinity;
            for(const Point&q:subgroup)if(eq(q,two))trace=false;
            r.expect("R "+std::to_string(slot)+" "+pt(g)+" "+std::to_string(trace),"trace admission");if(!trace)continue;
            Curve current=e;Point ti=t,gi=g;P n{N(0),N(1)},d{N(1)};
            for(unsigned i=0;i<depth;++i){
                const Point k=current.multiple(ti,1U<<(depth-i-1));require(!k.infinity&&k.y.zero(),"reference kernel");
                const auto row=tokens(r.next());require(row.size()==12&&row[0]=="S"&&row[1]==std::to_string(i),"tower row syntax");
                const N tau=f.add(f.mul(N(3),f.pow(k.x,2)),current.a),ap(row[6]),bp(row[7]);
                require(f.canonical(ap)&&f.canonical(bp),"canonical codomain");
                // Check the advertised codomain identity before coefficient/point checks.
                curve_identity(f,current.a,current.b,k.x,tau,ap,bp);
                require(row[2]==current.a.str()&&row[3]==current.b.str()&&row[4]==k.x.str()&&row[5]==tau.str()
                    &&row[8]==pt(ti)&&row[9]==pt(gi),"tower input binding");
                require(ap==f.sub(current.a,f.mul(N(5),tau))&&bp==f.sub(current.b,f.mul(N(7),f.mul(k.x,tau))),"codomain coefficient binding");
                Curve next(f,ap,bp);const Point nt=map_point(current,next,k.x,tau,ti),ng=map_point(current,next,k.x,tau,gi);
                require(row[10]==pt(nt)&&row[11]==pt(ng),"tower point images");
                const P nd=mul(f,n,d),dd=mul(f,d,d);
                P nn=add(f,minus(f,mul(f,n,n),scale(f,nd,k.x)),scale(f,dd,tau));
                d=minus(f,nd,scale(f,dd,k.x));n=nn;current=next;ti=nt;gi=ng;
            }
            require(ti.infinity&&!gi.infinity&&!current.dbl(gi).infinity,"reference terminal trace");
            require(n.size()==degree+1&&d.size()==degree&&n.back()==N(1)&&d.back()==N(1),"reference composition degrees");
            P roots;for(const Point&q:subgroup){const Point coset=e.sum(g,q);require(!coset.infinity,"reference coset infinity");roots.push_back(coset.x);}
            std::sort(roots.begin(),roots.end(),[](const N&a,const N&b){return a<b;});require(std::adjacent_find(roots.begin(),roots.end())==roots.end(),"reference coset duplicates");
            P product{N(1)};for(const N&xroot:roots)product=mul(f,product,P{f.sub(N(0),xroot),N(1)});
            require(minus(f,n,scale(f,d,gi.x))==product,"reference fibre factorization");
            r.expect("N "+list(n),"composed numerator binding");r.expect("V "+list(d),"composed denominator binding");
            r.expect("F "+a.str()+" "+b.str()+" "+pt(t)+" "+pt(g)+" "+gi.x.str()+" "+list(roots)+" "+std::to_string(trial+1),"final setup binding");r.end();
            receipt(root/"setup.tsv",f.p,a,b,t,g,gi.x,roots,trial+1);
            std::cout<<"PASS OpenSSL full setup replay bits="<<BN_num_bits(f.p.v)<<" t="<<depth<<" trials="<<trial+1<<'\n';return;
        }
    }
    throw std::runtime_error("reference curve cap without completed setup");
}
void census(const std::filesystem::path&path){
    Reader r(path);r.expect("BIG-PREIMAGES-1","census header");unsigned cases=0;
    for(unsigned p:{5,7,11,13,17,19}){
        Field f{N(p)};
        for(unsigned a=0;a<p;++a)for(unsigned b=0;b<p;++b){
            if(!nonsingular(f,N(a),N(b)))continue;
            Curve e(f,N(a),N(b));std::vector<Point> all;
            for(unsigned x=0;x<p;++x)for(unsigned y=0;y<p;++y)if(f.mul(N(y),N(y))==e.rhs(N(x)))all.push_back({N(x),N(y),false});
            for(const Point&target:all){
                const auto row=tokens(r.next());require(row.size()==8&&row[0]==std::to_string(p)&&row[1]==std::to_string(a)&&row[2]==std::to_string(b)&&row[3]==pt(target),"census input binding");
                const P xs=numbers(f,row[5]);require(xs.size()==small(row[4]),"census root count");const auto halves=verify_preimages(e,target,xs);
                std::vector<Point> exhaustive;for(const Point&q:all)if(eq(e.dbl(q),target))exhaustive.push_back(q);
                require(points(halves)==points(exhaustive)&&row[6]==std::to_string(halves.size())&&row[7]==points(halves),"exhaustive preimage census");++cases;
            }
        }
    }
    r.end();require(cases==14754,"complete census count");std::cout<<"PASS OpenSSL exhaustive and polynomial census cases="<<cases<<'\n';
}
void legacy(const std::filesystem::path&fresh,const std::filesystem::path&old){
    const auto read=[](const std::filesystem::path&p){std::ifstream in(p);require(bool(in),"missing legacy receipt");return std::string{std::istreambuf_iterator<char>(in),{}};};
    require(read(fresh/"setup.tsv")==read(old/"setup.tsv"),"legacy semantic receipt mismatch");std::cout<<"PASS preserved legacy semantic receipt\n";
}
} // namespace ref
int main(int argc,char**argv){
    try{
        if(argc==3&&std::string(argv[1])=="--setup"){ref::setup(argv[2]);return 0;}
        if(argc==3&&std::string(argv[1])=="--census"){ref::census(argv[2]);return 0;}
        if(argc==4&&std::string(argv[1])=="--legacy"){ref::legacy(argv[2],argv[3]);return 0;}
        throw std::runtime_error("usage: replay --setup ROOT | --census FILE | --legacy NEW_ROOT OLD_ROOT");
    }catch(const std::exception&e){std::cerr<<"REJECT: "<<e.what()<<'\n';return 2;}
}
