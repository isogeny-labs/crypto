// Fp24 as the exact cubic tower over the unchanged archived Fp8.
P geometry_tower_modulus;
struct T24 {
    P a;
    T24(long x=0):a{F(x)}{}
    T24(const F& x):a{x}{}
    explicit T24(const P& x):a(x%geometry_tower_modulus){}
    T24 operator+(const T24& b)const{return T24(a+b.a);}
    T24 operator-()const{return T24(-a);}
    T24 operator-(const T24& b)const{return *this+-b;}
    T24 operator*(const T24& b)const{return T24((a*b.a)%geometry_tower_modulus);}
    T24 inv()const{auto[g,s,t]=xgcd(a,geometry_tower_modulus);require(g.degree()==0,"tower inverse");return T24(s);}
    T24 operator/(const T24& b)const{return *this*b.inv();}
    bool operator==(const T24& b)const{return a==b.a;}
    bool operator!=(const T24& b)const{return !(*this==b);}
    T24 power(mpz_class n)const{return T24(powmod(a,n,geometry_tower_modulus));}
    std::string key()const{return a.key();}
};
template<class T> Poly<T> embed_polynomial(const P& a){
    std::vector<T> out;for(auto& c:a.c)out.emplace_back(c);return Poly<T>(out);
}
template<class T> struct JGeometry {
    inline static Poly<T> f;
    Poly<T> u{T(1)},v;
    JGeometry()=default;
    JGeometry(const Poly<T>&a,const Poly<T>&b):u(a),v(b){}
    explicit JGeometry(const G& g):u(embed_polynomial<T>(g.u)),v(embed_polynomial<T>(g.v)){}
    bool operator==(const JGeometry& b)const{return u==b.u&&v==b.v;}
    bool operator!=(const JGeometry& b)const{return !(*this==b);}
    void check()const{
        require(u.degree()>=0&&u.degree()<=3&&u.c.back()==T(1)&&v.degree()<u.degree(),"geometry Mumford shape");
        require(((v*v-f)%u).degree()<0,"geometry Mumford equation");
    }
    JGeometry neg()const{return {u,(-v)%u};}
    JGeometry add(const JGeometry& b)const{
        auto[d1,e1,e2]=xgcd(u,b.u);auto[d,c1,c2]=xgcd(d1,v+b.v);
        auto uu=(u*b.u).exact(d*d);
        auto vv=(c1*e1*u*b.v+c1*e2*b.u*v+c2*(v*b.v+f)).exact(d)%uu;
        while(uu.degree()>3){auto n=(f-vv*vv).exact(uu).monic();vv=(-vv)%n;uu=n;}
        JGeometry out{uu.monic(),vv};out.check();return out;
    }
    JGeometry frob()const{auto a=u.c,b=v.c;for(auto& c:a)c=c.power(prime);for(auto& c:b)c=c.power(prime);return {Poly<T>(a),Poly<T>(b)};}
    std::string key()const{return u.key()+"|"+v.key();}
};
using GT24=JGeometry<T24>;
void emit_t24(const T24& x){std::cout<<'[';for(int i=0;i<3;++i){if(i)std::cout<<',';emit_f(x.a.at(i));}std::cout<<']';}
void emit_t24_polynomial(const Poly<T24>& p){std::cout<<'[';for(int i=0;i<=p.degree();++i){if(i)std::cout<<',';emit_t24(p.at(i));}std::cout<<']';}

int parity2(int a,int b){return ((__builtin_popcount(unsigned(a))*__builtin_popcount(unsigned(b)))+__builtin_popcount(unsigned(a&b)))%2;}
int canonical_subset(int mask){return __builtin_popcount(unsigned(mask))>3?(mask^127):mask;}
struct Level2Input {
    std::vector<T24> branch;
    std::array<int,6> symplectic{};
    std::vector<int> masks;
    std::vector<GT24> classes;
    int delta=0;
    F sqrt_minus_one;
    Level2Input(){
        F source_b=F(Poly<Z>{Z(mpz_class("2455155546008943817740293915197451784769108058161191238065"))});
        geometry_tower_modulus=P{source_b,F(-3),F(0),F(1)};
        mpz_class q;mpz_pow_ui(q.get_mpz_t(),prime.get_mpz_t(),8);
        P x{F(0),F(1)},a=powmod(x,q,geometry_tower_modulus);
        auto[g,s,t]=xgcd(geometry_tower_modulus,a-x);require(g.degree()==0,"irreducible cubic tower gcd");
        a=powmod(a,q,geometry_tower_modulus);a=powmod(a,q,geometry_tower_modulus);
        require(a==x,"irreducible cubic tower closure");
        bool found=false;
        for(int shift=0;shift<16&&!found;++shift){
            F candidate(Poly<Z>{Z(shift),Z(1)});sqrt_minus_one=candidate.power(mpz_class((q-1)/4));
            found=sqrt_minus_one*sqrt_minus_one==F(-1);
        }
        require(found,"square root of minus one input bound");
        mpz_class p3;mpz_pow_ui(p3.get_mpz_t(),prime.get_mpz_t(),3);
        T24 e(P{F(0),F(1)}),first=e;
        branch.push_back(T24(F(-1)/(F(2)*cc)));
        for(int i=0;i<3;++i){
            require(e*e*e-T24(3)*e+T24(source_b)==T24(0),"elliptic cubic branch root");
            T24 value=(e+T24(7))/(e-T24(1)),symbol=value.power(mpz_class((p3-1)/2));
            require(symbol==T24(1)||symbol==T24(-1),"Fp3 quadratic character");
            T24 root=(symbol==T24(1)?value:-value).power(mpz_class((p3+1)/4));
            if(symbol==T24(-1))root=root*T24(sqrt_minus_one);
            require(root*root==value,"cover branch square root");
            for(auto sign:{T24(1),T24(-1)})branch.push_back(T24(1)/(T24(cc)*(sign*root-T24(1))));
            e=e.power(prime);
        }
        require(e==first,"elliptic cubic Frobenius cycle");
        auto f=embed_polynomial<T24>(curve);GT24::f=f;
        Poly<T24> product{T24(1)};std::set<std::string> distinct;
        for(auto& r:branch){require(f.eval(r)==T24(0),"normalized cover branch equation");distinct.insert(r.key());product=product*Poly<T24>{-r,T24(1)};}
        require(distinct.size()==7&&product==f,"all seven finite branch roots");
        std::vector<int> remaining{1,2,4,8,16,32};std::vector<int> aa,bb;
        while(!remaining.empty()){
            int a0=remaining.front(),j=1;while(j<int(remaining.size())&&!parity2(a0,remaining[j]))++j;
            require(j<int(remaining.size()),"branch symplectic partner");int b0=remaining[j];aa.push_back(a0);bb.push_back(b0);
            std::vector<int> next;
            for(int i=1;i<int(remaining.size());++i)if(i!=j){int v=remaining[i];next.push_back(v^(parity2(v,b0)?a0:0)^(parity2(v,a0)?b0:0));}
            remaining=next;
        }
        for(int i=0;i<3;++i){symplectic[i]=aa[i];symplectic[i+3]=bb[i];}
        for(int i=0;i<6;++i)for(int j=0;j<6;++j)require(parity2(symplectic[i],symplectic[j])==((i+3==j||j+3==i)?1:0),"branch symplectic form");
        for(int code=0;code<64;++code){
            int mask=0;for(int i=0;i<6;++i)if((code>>i)&1)mask^=symplectic[i];mask=canonical_subset(mask);masks.push_back(mask);
            Poly<T24> u{T24(1)};for(int i=0;i<7;++i)if((mask>>i)&1)u=u*Poly<T24>{-branch[i],T24(1)};
            GT24 point{u,{}};point.check();require(point.add(point)==GT24{},"source order2 branch class");classes.push_back(point);
        }
        std::set<std::string> keys;for(auto& p:classes)keys.insert(p.key());require(keys.size()==64,"all source geometric2-torsion");
        auto qbase=[](int mask){int n=__builtin_popcount(unsigned(canonical_subset(mask)));return (n==1||n==2)?1:0;};
        for(int i=0;i<3;++i){if(qbase(symplectic[i]))delta^=1<<(i+3);if(qbase(symplectic[i+3]))delta^=1<<i;}
        require(qbase(masks[delta])==0,"even characteristic shift");
        for(int code=0;code<64;++code)require(qbase(masks[code^delta])==(__builtin_popcount(unsigned((code&7)&(code>>3)))%2),"theta characteristic parity");
    }
};
