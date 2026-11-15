// Independent OpenSSL replay. No candidate implementation is included.
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

using Point=std::pair<int,int>;
std::vector<std::string> split(const std::string&s,char delimiter) {
    std::vector<std::string> out;std::stringstream input(s);std::string part;
    while(std::getline(input,part,delimiter))out.push_back(part);
    return out;
}
int number(const std::string&s) {
    if(s.empty()||s.find_first_not_of("0123456789")!=std::string::npos)
        throw std::runtime_error("nonnegative decimal expected");
    std::size_t used=0;int out=std::stoi(s,&used);
    if(used!=s.size())throw std::runtime_error("trailing integer text");
    return out;
}
int mod(int x){x%=19;return x<0?x+19:x;}
std::vector<int> vector_read(const std::string&s) {
    std::vector<int> out;for(const auto&part:split(s,','))out.push_back(number(part));return out;
}
std::string encode(std::vector<Point>points) {
    std::sort(points.begin(),points.end());std::ostringstream out;
    for(std::size_t i=0;i<points.size();++i){if(i)out<<';';out<<points[i].first<<':'<<points[i].second;}
    return out.str();
}
using ECPoint=std::unique_ptr<EC_POINT,decltype(&EC_POINT_free)>;
struct Group {
    BN_CTX*ctx=BN_CTX_new();
    EC_GROUP*group=nullptr;
    Group() {
        auto p=BN_new(),a=BN_new(),b=BN_new(),r=BN_new(),h=BN_new();
        BN_set_word(p,17);BN_set_word(a,2);BN_set_word(b,2);BN_set_word(r,19);BN_one(h);
        group=EC_GROUP_new_curve_GFp(p,a,b,ctx);
        if(!group)throw std::runtime_error("OpenSSL curve construction");
        auto P=point({5,1});
        if(!EC_GROUP_set_generator(group,P.get(),r,h)||EC_GROUP_check(group,ctx)!=1)
            throw std::runtime_error("OpenSSL group admission");
        BN_free(p);BN_free(a);BN_free(b);BN_free(r);BN_free(h);
    }
    ~Group(){EC_GROUP_free(group);BN_CTX_free(ctx);}
    ECPoint fresh(){return ECPoint(EC_POINT_new(group),EC_POINT_free);}
    ECPoint point(Point xy) {
        if(xy.first<0||xy.first>=17||xy.second<0||xy.second>=17)
            throw std::runtime_error("noncanonical point");
        auto out=fresh();auto x=BN_new(),y=BN_new();BN_set_word(x,xy.first);BN_set_word(y,xy.second);
        const int ok=EC_POINT_set_affine_coordinates(group,out.get(),x,y,ctx);
        BN_free(x);BN_free(y);
        if(ok!=1||EC_POINT_is_on_curve(group,out.get(),ctx)!=1)throw std::runtime_error("off-curve point");
        return out;
    }
    Point coordinates(const EC_POINT*p) {
        auto x=BN_new(),y=BN_new();
        if(!EC_POINT_get_affine_coordinates(group,p,x,y,ctx))throw std::runtime_error("affine coordinates");
        Point out{static_cast<int>(BN_get_word(x)),static_cast<int>(BN_get_word(y))};
        BN_free(x);BN_free(y);return out;
    }
    bool equal(const EC_POINT*a,const EC_POINT*b){return EC_POINT_cmp(group,a,b,ctx)==0;}
    ECPoint mixed(Point Q,int a,int b) {
        if(a<0||a>=19||b<0||b>=19)throw std::runtime_error("noncanonical scalar label");
        auto out=fresh(),target=point(Q);auto aa=BN_new(),bb=BN_new();BN_set_word(aa,a);BN_set_word(bb,b);
        const int ok=EC_POINT_mul(group,out.get(),aa,target.get(),bb,ctx);BN_free(aa);BN_free(bb);
        if(!ok)throw std::runtime_error("OpenSSL mixed query");
        return out;
    }
    bool membership(int x,int t,int c) {
        auto base=BN_new(),exponent=BN_new(),p=BN_new(),out=BN_new();
        BN_set_word(base,x);BN_set_word(exponent,1U<<t);BN_set_word(p,17);
        if(!BN_mod_exp(out,base,exponent,p,ctx))throw std::runtime_error("OpenSSL membership");
        const bool ok=BN_get_word(out)==static_cast<unsigned>(c);
        BN_free(base);BN_free(exponent);BN_free(p);BN_free(out);return ok;
    }
    std::vector<Point> all() {
        std::vector<Point> out;
        for(int x=0;x<17;++x)for(int y=0;y<17;++y) {
            try{auto P=point({x,y});out.emplace_back(x,y);}catch(const std::runtime_error&){}
        }
        if(out.size()!=18)throw std::runtime_error("OpenSSL affine point count");
        return out;
    }
    bool sum_matches(const std::vector<Point>&points,Point R) {
        auto sum=fresh();EC_POINT_set_to_infinity(group,sum.get());
        for(Point xy:points){auto P=point(xy);if(!EC_POINT_add(group,sum.get(),sum.get(),P.get(),ctx))throw std::runtime_error("OpenSSL addition");}
        const auto target=point(R);
        return equal(sum.get(),target.get());
    }
    void sum_check(const std::vector<Point>&points,Point R) {
        if(!sum_matches(points,R))throw std::runtime_error("OpenSSL point-sum mismatch");
    }
};
std::vector<std::vector<std::string>> table(const std::filesystem::path&file,const std::string&header) {
    std::ifstream input(file);std::string line;
    if(!std::getline(input,line)||line!=header)throw std::runtime_error("table header mismatch: "+file.string());
    std::vector<std::vector<std::string>> out;
    while(std::getline(input,line)){if(line.empty())throw std::runtime_error("blank table row");out.push_back(split(line,'\t'));}
    return out;
}
std::vector<Point> model_points(const std::string&model) {
    std::vector<Point> out;
    for(const auto&part:split(model,';')) {
        const auto xy=split(part,':');if(xy.size()!=2)throw std::runtime_error("bad point model");
        out.emplace_back(number(xy[0]),number(xy[1]));
    }
    if(out.size()!=3||encode(out)!=model)throw std::runtime_error("noncanonical triple");
    std::set<int> xs;for(Point p:out)xs.insert(p.first);
    if(xs.size()!=3)throw std::runtime_error("repeated model abscissa");
    return out;
}
void model_replay(const std::filesystem::path&root,const std::filesystem::path&summary) {
    Group group;const auto all=group.all();
    const auto rows=table(summary,"t\tc\trx\try\tmodels\tquotient_dimension\tstatus");
    if(rows.size()!=90)throw std::runtime_error("model-control census incomplete");
    std::size_t models=0;
    std::set<std::string> census;
    for(std::size_t id=0;id<rows.size();++id) {
        const auto&row=rows[id];if(row.size()!=7||row[6]!="match")throw std::runtime_error("bad model summary");
        const int t=number(row[0]),c=number(row[1]);const Point R{number(row[2]),number(row[3])};
        if(!census.insert(row[0]+":"+row[1]+":"+row[2]+":"+row[3]).second)
            throw std::runtime_error("duplicate model-control case");
        std::vector<Point> base;
        for(Point p:all)if(p.first!=R.first&&group.membership(p.first,t,c))base.push_back(p);
        std::set<std::string> expected;
        for(std::size_t i=0;i<base.size();++i)for(std::size_t j=i+1;j<base.size();++j)for(std::size_t k=j+1;k<base.size();++k) {
            std::vector<Point> points{base[i],base[j],base[k]};
            if(points[0].first==points[1].first||points[0].first==points[2].first||points[1].first==points[2].first)continue;
            if(group.sum_matches(points,R))expected.insert(encode(points));
        }
        std::ifstream input(root/("call-"+std::to_string(id+1))/"models.tsv");
        std::string line;std::set<std::string> actual;bool summary_seen=false;
        while(std::getline(input,line)) {
            const auto fields=split(line,'\t');
            if(fields.size()==2&&fields[0]=="model") {
                auto points=model_points(fields[1]);
                for(Point p:points)if(p.first==R.first||!group.membership(p.first,t,c))throw std::runtime_error("model chart/membership mismatch");
                group.sum_check(points,R);
                if(!actual.insert(fields[1]).second)throw std::runtime_error("duplicate model");
            }else if(fields.size()==5&&fields[0]=="summary")summary_seen=true;
            else throw std::runtime_error("invalid model file");
        }
        if(!summary_seen||actual!=expected||actual.size()!=static_cast<unsigned>(number(row[4])))
            throw std::runtime_error("independent complete model mismatch");
        models+=actual.size();
    }
    for(const auto&domain:std::vector<Point>{{1,1},{2,1},{3,1},{3,16},{4,1}})
        for(Point R:all)if(!census.count(std::to_string(domain.first)+":"+std::to_string(domain.second)+":"+std::to_string(R.first)+":"+std::to_string(R.second)))
            throw std::runtime_error("missing frozen model-control case");
    std::cout<<"PASS OpenSSL complete model cases="<<rows.size()<<" models="<<models<<'\n';
}
template<class GroupType,class Membership>
void cold_replay_rows(const std::filesystem::path&root,const std::filesystem::path&summary,
    GroupType&group,const std::vector<Point>&base,int field_modulus,int scalar_order,
    const std::set<Point>&wanted,Membership member) {
    auto reduce=[scalar_order](int x){x%=scalar_order;return x<0?x+scalar_order:x;};
    const auto targets=table(summary,"target_x\ttarget_y\trecovered_scalar\tattempts\tbackend_calls\tmisses\trank");
    if(targets.size()!=wanted.size())throw std::runtime_error("cold target census incomplete");
    std::size_t relation_total=0,kernel_total=0;
    std::set<Point> census;
    for(const auto&row:targets) {
        if(row.size()!=7)throw std::runtime_error("invalid cold summary");
        const Point Q{number(row[0]),number(row[1])};
        if(!wanted.count(Q))throw std::runtime_error("cold target binding mismatch");
        if(!census.insert(Q).second)throw std::runtime_error("duplicate cold target");
        const auto directory=root/("target-x"+row[0]+"-y"+row[1]);
        const auto attempts=table(directory/"cold-attempts.tsv","attempt\tbatch\ta\tb\trx\try\tstatus\tbackend_call");
        const auto relations=table(directory/"cold-relations.tsv","attempt\tbatch\ta\tb\trx\try\trow\tmodel");
        const auto kernels=table(directory/"cold-kernels.tsv","batch\trank\tw\talpha\tbeta\tstatus");
        if(attempts.size()!=static_cast<unsigned>(number(row[3])))throw std::runtime_error("attempt ledger incomplete");
        std::map<int,std::vector<std::string>> accepted;
        int calls=0,misses=0;
        for(std::size_t id=0;id<attempts.size();++id) {
            const auto&a=attempts[id];if(a.size()!=8||number(a[0])!=static_cast<int>(id+1))throw std::runtime_error("attempt sequence");
            const auto R=group.mixed(Q,number(a[2]),number(a[3]));
            if(a[6]=="identity") {
                if(a[4]!="-"||a[5]!="-"||a[7]!="-"||EC_POINT_is_at_infinity(group.group,R.get())!=1)
                    throw std::runtime_error("identity-query mismatch");
                ++misses;continue;
            }
            if(++calls!=number(a[7])||group.coordinates(R.get())!=Point{number(a[4]),number(a[5])})
                throw std::runtime_error("mixed query/call mismatch");
            if(a[6]=="accepted")accepted.emplace(number(a[0]),a);
            else if(a[6]=="no_relation")++misses;
            else throw std::runtime_error("unknown attempt status");
        }
        if(calls!=number(row[4])||misses!=number(row[5])||accepted.size()!=relations.size())
            throw std::runtime_error("cold counter partition mismatch");
        std::map<int,std::vector<std::vector<std::string>>> batches;
        std::set<int> seen_relations;
        for(const auto&r:relations) {
            if(r.size()!=8)throw std::runtime_error("bad relation row");
            const auto a=accepted.find(number(r[0]));
            if(!seen_relations.insert(number(r[0])).second)throw std::runtime_error("duplicate relation attempt");
            if(a==accepted.end()||!std::equal(r.begin(),r.begin()+6,a->second.begin()))throw std::runtime_error("relation labels mismatch");
            const Point R{number(r[4]),number(r[5])};auto points=model_points(r[7]);
            std::vector<int> coefficients(base.size());
            for(Point p:points) {
                if(p.first==R.first||!member(p.first))throw std::runtime_error("cold model chart");
                const auto b=std::find_if(base.begin(),base.end(),[&](Point v){return v.first==p.first;});
                if(b==base.end()||coefficients[b-base.begin()]!=0)throw std::runtime_error("cold base binding");
                if(p.second==b->second)coefficients[b-base.begin()]=1;
                else if(p.second==field_modulus-b->second)coefficients[b-base.begin()]=scalar_order-1;
                else throw std::runtime_error("cold sign binding");
            }
            if(coefficients!=vector_read(r[6]))throw std::runtime_error("signed row mismatch");
            group.sum_check(points,R);batches[number(r[1])].push_back(r);++relation_total;
        }
        if(kernels.size()!=batches.size())throw std::runtime_error("kernel batch census");
        for(const auto&k:kernels) {
            if(k.size()!=6)throw std::runtime_error("bad kernel row");
            const auto batch=batches.find(number(k[0]));if(batch==batches.end())throw std::runtime_error("missing kernel batch");
            const auto w=vector_read(k[2]);
            if(w.size()!=batch->second.size()||w.size()!=base.size()+1||std::all_of(w.begin(),w.end(),[](int v){return v==0;}))
                throw std::runtime_error("invalid kernel dimension/zero vector");
            std::vector<int> combined(base.size());int alpha=0,beta=0;
            for(std::size_t i=0;i<w.size();++i) {
                if(w[i]>=scalar_order)throw std::runtime_error("noncanonical kernel coefficient");
                const auto&r=batch->second[i];const auto c=vector_read(r[6]);
                if(c.size()!=base.size())throw std::runtime_error("kernel row width");
                for(std::size_t j=0;j<base.size();++j)combined[j]=reduce(combined[j]+w[i]*c[j]);
                alpha=reduce(alpha+w[i]*number(r[2]));beta=reduce(beta+w[i]*number(r[3]));
            }
            if(combined!=std::vector<int>(base.size())||alpha!=number(k[3])||beta!=number(k[4]))throw std::runtime_error("kernel arithmetic mismatch");
            if(beta==0) { if(alpha!=0||k[5]!="retry")throw std::runtime_error("bad zero-denominator retry"); }
            else if(k[5]!="return"||reduce(alpha+number(row[2])*beta)!=0)throw std::runtime_error("scalar-label recovery mismatch");
            ++kernel_total;
        }
        const auto actual=group.mixed(Q,number(row[2]),0),target=group.point(Q);
        if(!group.equal(actual.get(),target.get()))throw std::runtime_error("OpenSSL final scalar mismatch");
    }
    if(census!=wanted)throw std::runtime_error("missing cold targets");
    std::cout<<"PASS OpenSSL cold targets="<<targets.size()<<" relations="<<relation_total<<" kernels="<<kernel_total<<'\n';
}
void cold_replay(const std::filesystem::path&root,const std::filesystem::path&summary,bool one=false) {
    Group group;const auto all=group.all();std::vector<Point> base;
    for(Point p:all)if(p.second<=8&&group.membership(p.first,3,1))base.push_back(p);
    if(base.size()!=3)throw std::runtime_error("OpenSSL base dimension");
    const auto wanted=one?std::set<Point>{{0,6}}:std::set<Point>(all.begin(),all.end());
    cold_replay_rows(root,summary,group,base,17,19,wanted,[&](int x){return group.membership(x,3,1);});
}
int main(int argc,char**argv) {
    try {
        if(argc!=4)throw std::runtime_error("usage: replay --models|--cold|--cold-one ROOT SUMMARY.tsv");
        if(std::string(argv[1])=="--models")model_replay(argv[2],argv[3]);
        else if(std::string(argv[1])=="--cold")cold_replay(argv[2],argv[3]);
        else if(std::string(argv[1])=="--cold-one")cold_replay(argv[2],argv[3],true);
        else throw std::runtime_error("unknown replay command");
        return 0;
    }catch(const std::exception&e){std::cerr<<"REJECT: "<<e.what()<<'\n';return 2;}
}
