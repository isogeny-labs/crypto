// Independent OpenSSL reference; includes no candidate implementation.
#define main preserved_norm_replay_cli_main
#include "../shared_norm_algebraic_20261115/replay.cpp"
#undef main
struct AuxiliaryGroup {
    int modulus;
    BN_CTX*ctx=BN_CTX_new();EC_GROUP*group=nullptr;
    AuxiliaryGroup(int a,int b,int prime=19):modulus(prime){
        auto p=BN_new(),aa=BN_new(),bb=BN_new();BN_set_word(p,prime);BN_set_word(aa,a);BN_set_word(bb,b);
        group=EC_GROUP_new_curve_GFp(p,aa,bb,ctx);BN_free(p);BN_free(aa);BN_free(bb);
        if(!group)throw std::runtime_error("OpenSSL auxiliary curve");
    }
    ~AuxiliaryGroup(){EC_GROUP_free(group);BN_CTX_free(ctx);}
    ECPoint fresh(){return ECPoint(EC_POINT_new(group),EC_POINT_free);}
    ECPoint point(Point P){
        if(P.first<0||P.first>=modulus||P.second<0||P.second>=modulus)throw std::runtime_error("noncanonical auxiliary point");
        auto out=fresh();auto x=BN_new(),y=BN_new();BN_set_word(x,P.first);BN_set_word(y,P.second);
        const int ok=EC_POINT_set_affine_coordinates(group,out.get(),x,y,ctx);BN_free(x);BN_free(y);
        if(ok!=1||EC_POINT_is_on_curve(group,out.get(),ctx)!=1)throw std::runtime_error("off-curve F19 point");
        return out;
    }
    Point coordinates(const EC_POINT*P){
        auto x=BN_new(),y=BN_new();if(!EC_POINT_get_affine_coordinates(group,P,x,y,ctx))throw std::runtime_error("OpenSSL coordinates");
        const Point out{static_cast<int>(BN_get_word(x)),static_cast<int>(BN_get_word(y))};BN_free(x);BN_free(y);return out;
    }
    std::vector<Point> all(){std::vector<Point> out;
        for(int x=0;x<modulus;++x)for(int y=0;y<modulus;++y){try{point({x,y});out.emplace_back(x,y);}catch(const std::runtime_error&){}}
        return out;
    }
    bool sum_matches(const std::vector<Point>&points,Point R){
        auto sum=fresh();EC_POINT_set_to_infinity(group,sum.get());
        for(const auto&P:points){const auto point_value=point(P);
            if(!EC_POINT_add(group,sum.get(),sum.get(),point_value.get(),ctx))throw std::runtime_error("OpenSSL addition");}
        const auto target=point(R);return EC_POINT_cmp(group,sum.get(),target.get(),ctx)==0;
    }
    void sum_check(const std::vector<Point>&points,Point R){
        if(!sum_matches(points,R))throw std::runtime_error("OpenSSL auxiliary sum mismatch");
    }
    bool equal(const EC_POINT*A,const EC_POINT*B){return EC_POINT_cmp(group,A,B,ctx)==0;}
    ECPoint mixed(Point Q,int a,int b){
        if(a<0||a>=13||b<0||b>=13)throw std::runtime_error("noncanonical order13 label");
        const auto P=point({4,3}),target=point(Q);auto A=fresh(),B=fresh(),out=fresh();
        auto aa=BN_new(),bb=BN_new();BN_set_word(aa,a);BN_set_word(bb,b);
        const int ok=EC_POINT_mul(group,A.get(),nullptr,P.get(),aa,ctx)&&EC_POINT_mul(group,B.get(),nullptr,target.get(),bb,ctx);
        BN_free(aa);BN_free(bb);
        if(!ok||!EC_POINT_add(group,out.get(),A.get(),B.get(),ctx))throw std::runtime_error("OpenSSL order13 mixed query");
        return out;
    }
    std::set<int> fibre(Point subgroup={16,11},Point representative={12,5}){
        const auto T=point(subgroup),G=point(representative);std::set<int> roots;
        for(int i=0;i<8;++i){auto H=fresh(),P=fresh();auto n=BN_new();BN_set_word(n,i);
            const int ok=EC_POINT_mul(group,H.get(),nullptr,T.get(),n,ctx);BN_free(n);
            if(!ok||!EC_POINT_add(group,P.get(),H.get(),G.get(),ctx)||EC_POINT_is_at_infinity(group,P.get())==1)
                throw std::runtime_error("OpenSSL auxiliary coset");
            if(!roots.insert(coordinates(P.get()).first).second)throw std::runtime_error("OpenSSL coset duplicate");
        }return roots;
    }
};
#ifndef AUXILIARY_REPLAY_NO_MAIN
int main(int argc,char**argv){
    try{
        if(argc==4&&std::string(argv[1])=="--cold"){
            AuxiliaryGroup aux(2,2),main_group(0,2);const auto fibre=aux.fibre();std::vector<Point> base;
            for(const auto&P:main_group.all())if(P.second<=9&&fibre.count(P.first))base.push_back(P);
            if(base.size()!=3)throw std::runtime_error("OpenSSL auxiliary cold base");
            cold_replay_rows(argv[2],argv[3],main_group,base,19,13,std::set<Point>{{18,1}},[&](int x){return fibre.count(x)!=0;});
            return 0;
        }
        if(argc!=3)throw std::runtime_error("usage: replay ROOT SUMMARY.tsv");
        AuxiliaryGroup aux(2,2),main_group(0,2);const auto fibre=aux.fibre();const auto points=main_group.all();
        if(aux.all().size()!=23||points.size()!=12||fibre.size()!=8)throw std::runtime_error("OpenSSL auxiliary/main census");
        const auto rows=table(argv[2],"rx\try\treference\tmodels\tquotient\tstatus");
        if(rows.size()!=12)throw std::runtime_error("OpenSSL target census incomplete");
        std::set<Point> seen;std::size_t completed=0,unknown=0,models=0;
        for(std::size_t id=0;id<rows.size();++id){const auto&row=rows[id];if(row.size()!=6)throw std::runtime_error("bad auxiliary summary");
            const Point R{number(row[0]),number(row[1])};main_group.point(R);
            if(!seen.insert(R).second)throw std::runtime_error("duplicate auxiliary target");
            std::vector<Point> base;for(const auto&P:points)if(P.first!=R.first&&fibre.count(P.first))base.push_back(P);
            std::set<std::string> expected;
            for(std::size_t i=0;i<base.size();++i)for(std::size_t j=i+1;j<base.size();++j)for(std::size_t k=j+1;k<base.size();++k){
                if(base[i].first==base[j].first||base[i].first==base[k].first||base[j].first==base[k].first)continue;
                if(main_group.sum_matches({base[i],base[j],base[k]},R))expected.insert(encode({base[i],base[j],base[k]}));
            }
            if(expected.size()!=static_cast<unsigned>(number(row[2])))throw std::runtime_error("OpenSSL reference set/count mismatch");
            if(row[5].rfind("unknown",0)==0){++unknown;continue;}
            if(row[5]!="match")throw std::runtime_error("unexpected auxiliary status");
            const auto path=std::filesystem::path(argv[1])/("call-"+std::to_string(id+1))/"models.tsv";
            std::ifstream in(path);if(!in)throw std::runtime_error("missing auxiliary models");std::set<std::string> actual;std::string line;
            while(std::getline(in,line)){const auto parsed=model_points(line);
                if(!main_group.sum_matches(parsed,R))throw std::runtime_error("OpenSSL auxiliary point sum");
                if(!actual.insert(line).second)throw std::runtime_error("duplicate auxiliary model");}
            if(actual!=expected||actual.size()!=static_cast<unsigned>(number(row[3])))throw std::runtime_error("OpenSSL complete auxiliary model mismatch");
            ++completed;models+=actual.size();
        }
        if(seen!=std::set<Point>(points.begin(),points.end()))throw std::runtime_error("OpenSSL missing auxiliary targets");
        std::cout<<"PASS OpenSSL auxiliary targets="<<rows.size()<<" completed="<<completed<<" unknown="<<unknown<<" models="<<models<<'\n';return 0;
    }catch(const std::exception&e){std::cerr<<"REJECT: "<<e.what()<<'\n';return 2;}
}
#endif
