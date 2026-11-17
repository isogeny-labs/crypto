// Eight normalized even Weil functions, with parity-qualified path reversal.
TP ek_polynomial(const Json& j){std::vector<T24> v;for(auto& c:j.arr)v.push_back(read_t24(c));return TP(v);}
FiniteDivisor ek_divisor(const Json& j){return {ek_polynomial(j.at("U")),ek_polynomial(j.at("V"))};}
int ek_sign(int a,int b){return __builtin_popcount(unsigned(a&b))%2?-1:1;}
struct EvenKummer {
    ContactTarget& target;
    FiniteDivisor origin;
    std::vector<T24> theta;
    Values8 nulls{},scales{};
    std::array<std::array<int,2>,8> paths{};
    std::map<int,FiniteDivisor> even_cache;
    explicit EvenKummer(ContactTarget& t,const std::string& root):target(t){
        auto w=read_json(root+"/evidence/quotient-theta2-v1/result.json"),r=read_json(root+"/evidence/target-weil-v3/result.json");
        for(auto& c:w.at("theta_squares").arr)theta.push_back(read_t24(c));
        require(theta.size()==64&&theta[0]==T24(1),"even-Kummer theta vector");origin=ek_divisor(r.at("canonical_origin_divisor"));target.check(origin,3);even_cache.emplace(0,origin);
        for(int s=0;s<8;++s)paths[s]=path(s);
        for(int s=0;s<8;++s){T24 raw=at_torsion_raw(s,0);require(raw!=T24(0)&&theta[s]!=T24(0),"nonzero even origin normalization");scales[s]=theta[s]/raw;}
        for(int a=0;a<8;++a){for(int b=0;b<8;++b)nulls[a]=nulls[a]+T24(ek_sign(a,b))*theta[8*a+b];require(nulls[a]!=T24(0),"nonzero theta-null coordinate chart");}
        for(int a=0;a<8;++a){T24 sum;for(int b=0;b<8;++b)sum=sum+T24(ek_sign(a,b))*theta[b];require(nulls[0]*sum==nulls[a]*nulls[a],"theta-null signed-sum origin identity");}
    }
    bool single(int code)const{return theta_parity(code^52)==1;}
    std::array<int,2> path(int code)const{
        require(theta_parity(code)==0,"even translation path input");
        if(code==0||single(code))return {code,0};
        for(int a=1;a<64;++a)if(theta_parity(a)==0&&single(a)&&single(a^code))return {a,a^code};
        throw std::runtime_error("even-first bitangent path bound");
    }
    T24 raw_single(int code,const FiniteDivisor& e){return code==0?T24(1):target.raw(code^52,e);}
    const FiniteDivisor& even_point(int code){
        require(theta_parity(code)==0,"even torsion representative");auto hit=even_cache.find(code);if(hit!=even_cache.end())return hit->second;
        auto p=path(code);auto e=target.translate(origin,p[0]^52);if(p[1])e=target.translate(e,p[1]^52);target.check(e,3);return even_cache.emplace(code,e).first->second;
    }
    T24 at_torsion_raw(int s,int t){
        require(s>=0&&s<8&&theta_parity(t)==0&&theta_parity(s^t)==0,"regular even torsion Weil value");auto p=paths[s];const auto& e=even_point(t);
        if(p[1]==0)return raw_single(p[0],e);
        if(theta_parity(t^p[0])==0)return raw_single(p[0],e)*raw_single(p[1],even_point(t^p[0]));
        require(theta_parity(t^p[1])==0,"reverse even intermediate");T24 sign(theta_pair(p[0],p[1])?-1:1);
        return sign*raw_single(p[1],e)*raw_single(p[0],even_point(t^p[1]));
    }
    T24 raw(int s,const FiniteDivisor& e,bool reverse=false){
        auto p=paths[s];if(p[1]==0)return raw_single(p[0],e);
        int first=reverse?p[1]:p[0],second=reverse?p[0]:p[1];auto shifted=target.translate(e,first^52);
        T24 value=raw_single(first,e)*raw_single(second,shifted);if(reverse&&theta_pair(p[0],p[1]))value=-value;return value;
    }
    FiniteDivisor shift(int s,FiniteDivisor e){auto p=paths[s];if(p[0])e=target.translate(e,p[0]^52);if(p[1])e=target.translate(e,p[1]^52);return e;}
    Values8 values(const FiniteDivisor& e){Values8 out{};for(int s=0;s<8;++s)out[s]=scales[s]*raw(s,e);require(out[0]==T24(1),"even zero-characteristic function");return out;}
    Values8 coordinates(const Values8& values)const{Values8 out{};for(int i=0;i<8;++i){for(int b=0;b<8;++b)out[i]=out[i]+T24(ek_sign(i,b))*values[b];out[i]=out[i]/nulls[i];}return out;}
};
bool ek_projective_equal(const Values8& a,const Values8& b){int pivot=0;while(pivot<8&&b[pivot]==T24(0))++pivot;require(pivot<8,"nonzero projective comparison");if(a[pivot]==T24(0))return false;for(int j=0;j<8;++j)if(a[j]*b[pivot]!=b[j]*a[pivot])return false;return true;}
FiniteDivisor ek_points(const GoodPoint& a,const GoodPoint& b,const GoodPoint& c,TargetWeil& target){
    std::array<Line,3> points{a.point,b.point,c.point};TP x{T24(0),T24(1)},u{T24(1)},v;
    for(auto& p:points)u=u*TP{-p[0],T24(1)};
    for(int i=0;i<3;++i){TP term{T24(1)};T24 den(1);for(int j=0;j<3;++j)if(i!=j){term=term*TP{-points[j][0],T24(1)};den=den*(points[i][0]-points[j][0]);}require(den!=T24(0),"distinct generic x support");v=v+term.scale(points[i][1]/den);}
    FiniteDivisor e{u,v};target.check(e,3);return e;
}
