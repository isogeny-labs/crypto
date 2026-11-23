// Exact divisor-map development qualifier; no scalar solver or timing claims.
#include "generic_bridge_arithmetic.hpp"
struct MCycle {CFPoly u,x,y;int coordinate=0,dimension=0;std::vector<int> rejected;};
CFPoly cm_inverse(const CFPoly& a,const CFPoly& u){auto[g,s,t]=xgcd(a,u);require(g.degree()==0,"complete-map polynomial unit");return s%u;}
GBP cm_power(GBP a,int n,const GBP& u){GBP r{GBSource(1)};while(n){if(n&1)r=(r*a)%u;n>>=1;if(n)a=(a*a)%u;}return r;}
GBP cm_evaluate(const CFPoly& p,const GBP& x,const GBP& u){GBP r;for(int i=p.degree();i>=0;--i)r=(r*x+GBP{GBSource(CFPoly{p.at(i)})})%u;return r;}
CFPoly cm_curve(const EBPoly& f,const CFPoly& u,const CFPoly& x,const CFPoly& y){CFPoly r;for(auto& [e,c]:f.terms){auto term=pc_power_mod(x,e[0],u)*pc_power_mod(y,e[1],u);r=(r+term.scale(c))%u;}return r;}
struct MAlgebra {
    int d,n;GBP u,x,y;Vector traces;
    MAlgebra(int degree,GBP uu,GBP xx,GBP yy):d(degree),n(3*degree),u(std::move(uu)),x(std::move(xx)),y(std::move(yy)){
        require(u.degree()==3&&u.at(3)==GBSource(1),"complete-map monic cubic algebra");
        for(int j=0;j<3;++j)for(int i=0;i<d;++i){std::vector<Z> s(size_t(i+1),Z(0));s[size_t(i)]=Z(1);std::vector<GBSource> t(size_t(j+1));t[size_t(j)]=GBSource(CFPoly(s));GBP q(t);Z trace;for(int k=0;k<3;++k)for(int l=0;l<d;++l){std::vector<Z> ss(size_t(l+1),Z(0));ss[size_t(l)]=Z(1);std::vector<GBSource> tt(size_t(k+1));tt[size_t(k)]=GBSource(CFPoly(ss));auto product=(q*GBP(tt))%u;trace=trace+product.at(k).a.at(l);}traces.push_back(trace);}
        require(trace(GBP{GBSource(1)})==Z(n),"complete-map algebra dimension trace");
    }
    Z trace(const GBP& q)const{auto v=gb_flat(q,d);Z s;require(v.size()==traces.size(),"complete-map trace shape");for(size_t i=0;i<v.size();++i)s=s+v[i]*traces[i];return s;}
    MCycle cycle(const EBPoly& f)const{
        MCycle out;out.dimension=n;int bound=n*(n-1)/2+n;
        for(int l=0;l<=bound;++l){
            GBP z=(x+y.scale(GBSource(l)))%u,power{GBSource(1)};std::vector<Z> tr{Z(n)},ty;
            for(int k=0;k<=n;++k){if(k<n)ty.push_back(trace((y*power)%u));if(k>0)tr.push_back(trace(power));power=(power*z)%u;}
            std::vector<Z> c(size_t(n+1));c[size_t(n)]=Z(1);for(int k=1;k<=n;++k){Z sum;for(int j=1;j<=k;++j)sum=sum+c[size_t(n-k+j)]*tr[size_t(j)];c[size_t(n-k)]=-sum/Z(k);}CFPoly support(c);require(cm_evaluate(support,z,u).degree()<0,"complete-map characteristic identity");
            std::vector<Z> b(size_t(n),Z(0));for(int k=0;k<n;++k)for(int j=0;j<=k;++j)b[size_t(n-1-k)]=b[size_t(n-1-k)]+c[size_t(n-j)]*ty[size_t(k-j)];CFPoly numerator(b),derivative=cf_derivative(support),g=cf_gcd(support,derivative),rad=support.exact(g).monic();
            require((numerator%g).degree()<0,"trace numerator repeated-factor divisibility");CFPoly yy=(numerator.exact(g)*cm_inverse(derivative.exact(g),rad))%rad;
            auto mismatch=(y-cm_evaluate(yy,z,u))%u;bool nilpotent=cm_power(mismatch,n,u).degree()<0;CFPoly zz{Z(0),Z(1)},xx=(zz-yy.scale(Z(l)))%rad;
            if(!nilpotent||cm_curve(f,rad,xx,yy).degree()>=0){out.rejected.push_back(l);continue;}
            EBPoly transverse=f.partial(1)-f.partial(0).scale(Z(l));if(cf_gcd(cm_curve(transverse,rad,xx,yy),rad).degree()>0){out.rejected.push_back(l);continue;}
            yy=yy%support;for(int precision=1;precision<n;precision*=2){xx=(zz-yy.scale(Z(l)))%support;auto error=cm_curve(f,support,xx,yy),dy=cm_curve(transverse,support,xx,yy);yy=(yy-error*cm_inverse(dy,support))%support;}
            xx=(zz-yy.scale(Z(l)))%support;require(cm_curve(f,support,xx,yy).degree()<0,"complete-map full cycle quartic equation");require((xx+yy.scale(Z(l))-zz)%support==CFPoly{},"complete-map chart reconstruction");out.u=support;out.x=xx;out.y=yy;out.coordinate=l;return out;
        }
        throw std::runtime_error("complete-map separating coordinate bound exhausted");
    }
};
struct MForward {
    CFPoly original_u,original_v,e,v;MCycle cycle;int shift=-1;bool zero=false;std::vector<int> rejected_moves;
};
struct MMap {
    const Json& global;EBPoly f;CFPoly h,den{Z(1)};
    MMap(const Json& gg,EBPoly ff):global(gg),f(std::move(ff)),h(rational(curve)){
        for(auto key:{"U","V"})for(auto& row:global.at(key).arr){auto c=pc_read_poly(row.at("c"));den=(den*c).exact(cf_gcd(den,c)).monic();}
        require(prime>mpz_class(4*den.degree()+200),"complete-map finite schedule below characteristic");
    }
    MForward forward(const G& input,bool force_move=false)const{
        input.check();MForward out;out.original_u=rational(input.u);out.original_v=rational(input.v);int d=out.original_u.degree();if(d==0){out.zero=true;return out;}out.e=out.original_u;out.v=out.original_v;
        if(force_move||cf_gcd(out.e,den).degree()>0){bool found=false;for(int a=0;a<=4*den.degree();++a){auto g=out.original_v+out.original_u.scale(Z(a));auto e=(h-g*g).exact(out.original_u).monic();require(e.degree()==7-d,"complete-map moving degree");if(cf_gcd(e,den).degree()>0){out.rejected_moves.push_back(a);continue;}out.e=e;out.v=(-g)%e;out.shift=a;require(((out.v*out.v-h)%out.e).degree()<0,"complete-map moved source equation");require((h-g*g)==out.original_u*out.e,"complete-map exact principal moving identity");found=true;break;}require(found,"complete-map finite moving bound");}
        GBSource::modulus=out.e;GBSource sx(CFPoly{Z(0),Z(1)}),sy(out.v);std::vector<GBSource> uc,vc;for(auto& row:global.at("U").arr)uc.push_back(gb_field(row,sx,sy));for(auto& row:global.at("V").arr)vc.push_back(gb_field(row,sx,sy));GBP u(uc),x{GBSource(0),GBSource(1)},y(vc);MAlgebra algebra(out.e.degree(),u,x,y);out.cycle=algebra.cycle(f);return out;
    }
};
void cm_emit_cycle(const MCycle& c){std::cout<<"{\"dimension\":"<<c.dimension<<",\"coordinate_lambda\":"<<c.coordinate<<",\"support\":";cf_emit_poly(c.u);std::cout<<",\"x\":";cf_emit_poly(c.x);std::cout<<",\"y\":";cf_emit_poly(c.y);std::cout<<",\"rejected_coordinates\":[";for(size_t i=0;i<c.rejected.size();++i)std::cout<<(i?",":"")<<c.rejected[i];std::cout<<"]}";}
void cm_emit_forward(const MForward& v){std::cout<<"{\"zero\":"<<(v.zero?"true":"false")<<",\"move_parameter\":"<<v.shift<<",\"original_U\":";cf_emit_poly(v.original_u);std::cout<<",\"original_V\":";cf_emit_poly(v.original_v);std::cout<<",\"moved_U\":";cf_emit_poly(v.e);std::cout<<",\"moved_V\":";cf_emit_poly(v.v);std::cout<<",\"reference_multiplicity\":"<<(v.zero?0:v.e.degree())<<",\"rejected_moves\":[";for(size_t i=0;i<v.rejected_moves.size();++i)std::cout<<(i?",":"")<<v.rejected_moves[i];std::cout<<"],\"cycle\":";cm_emit_cycle(v.cycle);std::cout<<'}';}
G cm_source(const Json& row){std::vector<F> u,v;for(auto& a:row.at("source_U").arr)u.push_back(F(CFPoly{Z(mpz_class(a.scalar))}));for(auto& a:row.at("source_V").arr)v.push_back(F(CFPoly{Z(mpz_class(a.scalar))}));G out{P(u),P(v)};out.check();return out;}
int main(int argc,char** argv){try{
    require(argc==4,"usage: complete_maps kernel3-study global.json generic-bridge.json");std::string root=argv[1];initialize_cover(read_json(root+"/inputs/p192-d7-prym.json"));Level2Input input;auto global=read_json(argv[2]),bridge=read_json(argv[3]),model=read_json(root+"/evidence/p192-d7-kernel3-plane-quartic.json"),points=read_json(root+"/evidence/destination-relations-v2/result.json");EBPoly f;for(auto& row:model.at("terms").arr){auto e=row.at("exponents");f.add({e.arr[0].integer(),e.arr[1].integer()},Z(mpz_class(row.at("coefficient").scalar)));}MMap map(global,f);std::vector<G> sources{G{},G{P{F(1)/(F(2)*cc),F(1)},P{}}};for(auto& row:bridge.at("images").arr)sources.push_back(cm_source(row));std::vector<MForward> direct,moved;
    for(size_t i=0;i<sources.size();++i){direct.push_back(map.forward(sources[i]));moved.push_back(map.forward(sources[i],true));std::cerr<<"forward fixture "<<i<<" direct dimension="<<direct.back().cycle.dimension<<" moved="<<moved.back().cycle.dimension<<'\n';if(i>=2&&direct.back().shift<0&&direct.back().cycle.coordinate==0){auto& old=bridge.at("images").arr[i-2];require(direct.back().cycle.u==pc_read_poly(old.at("target_U"))&&direct.back().cycle.y==pc_read_poly(old.at("target_V")),"complete-map pinned regular image binding");}}
    auto& fiber=points.at("split_fibers").arr.at(0).at("point_indices").arr;require(fiber.size()==4,"vertical control fiber");std::array<Z,3> ys;Z xr;for(int i=0;i<3;++i){auto& p=points.at("points").arr.at(size_t(fiber[size_t(i)].integer()));xr=Z(mpz_class(p.at("x").scalar));ys[size_t(i)]=Z(mpz_class(p.at("y").scalar));}GBSource::modulus=CFPoly{Z(0),Z(1)};GBP u{GBSource(1)};for(auto yy:ys)u=u*GBP{GBSource(CFPoly{-yy}),GBSource(1)};MAlgebra collision(1,u,GBP{GBSource(CFPoly{xr})},GBP{GBSource(0),GBSource(1)});auto separated=collision.cycle(f);require(separated.coordinate>0&&separated.rejected.front()==0,"old x collision repaired by separating coordinate");MAlgebra repeated(1,GBP{GBSource(0),GBSource(0),GBSource(0),GBSource(1)},GBP{GBSource(CFPoly{xr})},GBP{GBSource(CFPoly{ys[0]})});auto triple=repeated.cycle(f);require(cf_gcd(triple.u,cf_derivative(triple.u)).degree()==2,"length3 repeated cycle retained");
    rejects([&]{auto bad=triple.y+CFPoly{Z(1)};require(cm_curve(f,triple.u,triple.x,bad).degree()<0,"changed complete-map target y");},"changed complete-map target y");rejects([&]{G bad=sources[2];bad.v=bad.v+P{F(1)};bad.check();},"changed source support");rejects([&]{auto& d=moved[2];auto g=d.original_v+d.original_u.scale(Z(d.shift));require(map.h-g*g==d.original_u*(d.e+CFPoly{Z(1)}),"changed moving identity");},"changed moving identity");
    std::cout<<"{\"scope\":\"P192-kernel3-exceptional-forward-signed-divisor-map\",\"standing\":\"producer-unverified\",\"source_denominator_lcm\":";cf_emit_poly(map.den);std::cout<<",\"fixtures\":[";for(size_t i=0;i<sources.size();++i){if(i)std::cout<<',';std::cout<<"{\"index\":"<<i<<",\"direct\":";cm_emit_forward(direct[i]);std::cout<<",\"forced_moving\":";cm_emit_forward(moved[i]);std::cout<<'}';}std::cout<<"],\"collision_cycle\":";cm_emit_cycle(separated);std::cout<<",\"repeated_cycle\":";cm_emit_cycle(triple);std::cout<<",\"negative_controls\":3,\"generic_transpose_executed\":false}\n";
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
