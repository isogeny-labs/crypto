#include "signed_words_arithmetic.hpp"
#include <fcntl.h>
#include <unistd.h>

namespace two_row {
using namespace audit;
using Match=std::vector<Pair>;
using Entry=std::pair<std::string,Pair>;
struct Table {
    std::map<std::string,Match> grouped;
    std::vector<Entry> sorted;
    Table(const std::vector<Item>& a,const std::vector<Item>& b0) {
        require(a.size()==b0.size()&&fingerprint(a)==fingerprint(b0),"complete backend table agreement");
        for(const auto& item:a) grouped[item.sum].push_back(item.indices);
        for(const auto& item:b0) sorted.emplace_back(item.sum,item.indices);
        std::sort(sorted.begin(),sorted.end());
    }
    Match lookup(const std::string& value) const {
        Match a,b0;
        const auto hit=grouped.find(value);
        if(hit!=grouped.end()) a=hit->second;
        auto cursor=std::lower_bound(sorted.begin(),sorted.end(),value,
            [](const Entry& entry,const std::string& v){return entry.first<v;});
        while(cursor!=sorted.end()&&cursor->first==value) {b0.push_back(cursor->second);++cursor;}
        std::sort(a.begin(),a.end());
        require(a==b0,"complete grouped/sorted witness agreement");
        return a;
    }
};
struct Witness {int i,j,sign;std::string family;Match labels;};
Point scaled(Point q,int n) {
    if(n<0) {q=negative(q);n=-n;}
    Point result;
    while(n--) result=plus(result,q);
    return result;
}
std::string ssl_query(audit::SSL& ssl,const Point& a,const Point& b0,int coefficient,int sign) {
    auto x=ssl.make(a),y=ssl.make(b0),result=ssl.make({});
    if(coefficient==2) require(EC_POINT_dbl(ssl.group.get(),x.get(),x.get(),ssl.context.get())==1,"SSL query double");
    else require(coefficient==1,"bounded query coefficient");
    if(sign<0) require(EC_POINT_invert(ssl.group.get(),y.get(),ssl.context.get())==1,"SSL signed query");
    require(EC_POINT_add(ssl.group.get(),result.get(),x.get(),y.get(),ssl.context.get())==1,"SSL query sum");
    return ssl.encode(result.get());
}
void fresh_write(const std::string& path,const std::string& value) {
    const int fd=open(path.c_str(),O_WRONLY|O_CREAT|O_EXCL,0444);
    require(fd>=0,"create-new output");
    size_t offset=0;
    while(offset<value.size()) {
        const auto n=write(fd,value.data()+offset,value.size()-offset);
        if(n<=0) {close(fd);throw std::runtime_error("write output");}
        offset+=size_t(n);
    }
    require(close(fd)==0,"close output");
}
int run(int argc,char** argv) {
    require(argc==6,"usage: audit BASELINE TANGENTS PRIOR-SIGNED OUTPUT IDENTITY");
    require(access(argv[4],F_OK)!=0&&access(argv[5],F_OK)!=0,"fresh output paths");
    require(std::string(argv[4])!=argv[5],"distinct output paths");
    const auto data=read_json(argv[1]),tangents=read_json(argv[2]),prior=read_json(argv[3]);
    std::vector<Point> base,images,symbols(1);
    audit::SSL ssl;
    for(const auto& item:data.at("projected_base").arr) {
        const auto q=read_point(item);
        require(!q.zero&&on_curve(q)&&q.y<=p-q.y,"canonical old symbol");
        ssl.make(q);base.push_back(q);symbols.push_back(q);symbols.push_back(negative(q));
    }
    require(base.size()==225&&symbols.size()==451,"frozen alphabet dimensions");
    for(const auto& item:data.at("images").arr) {
        const auto q=read_point(item.at("point"));
        const int j=item.at("projected_index").integer(),s=item.at("sign").integer();
        require(on_curve(q),"recorded curve equation");ssl.make(q);
        if(j<0) require(j==-1&&s==0&&q.zero,"identity binding");
        else require(size_t(j)<base.size()&&(s==1||s==-1)&&q==scaled(base[size_t(j)],s),"signed image binding");
        images.push_back(q);
    }
    require(images.size()==226&&tangents.at("tangents").arr.size()==226,"complete tangent domain");
    Point kappa;
    auto sk=ssl.make({});
    for(const auto& item:data.at("pool").at("split_fibers").arr.at(0).at("point_indices").arr) {
        const auto& q=images.at(size_t(item.integer()));
        kappa=plus(kappa,q);auto sq=ssl.make(q);
        require(EC_POINT_add(ssl.group.get(),sk.get(),sk.get(),sq.get(),ssl.context.get())==1,"SSL reference");
    }
    require(key(kappa)==ssl.encode(sk.get()),"reference arithmetic agreement");
    const auto a=signed_audit::enumerate(base,false),b0=signed_audit::enumerate(base,true);
    require(a.size()==101926&&fingerprint(a)==prior.at("pair_fingerprint_sha256").scalar,"frozen pair fingerprint");
    const Table pairs(a,b0);
    std::vector<Item> doubles_gmp,doubles_ssl;
    for(size_t i=0;i<symbols.size();++i) {
        doubles_gmp.push_back({{int(i),-1},key(scaled(symbols[i],2)),""});
        doubles_ssl.push_back({{int(i),-1},ssl_query(ssl,symbols[i],symbols[i],1,1),""});
    }
    const Table doubles(doubles_gmp,doubles_ssl);
    int controls=0;
    auto known=[&](const Table& table,const Point& q,Pair labels) {
        const auto matches=table.lookup(key(q));
        require(std::find(matches.begin(),matches.end(),labels)!=matches.end(),"known-answer retrieval");++controls;
    };
    known(pairs,plus(symbols[1],symbols[3]),{1,3});
    known(pairs,{}, {0,0});
    known(pairs,plus(symbols[2],symbols[3]),{2,3});
    known(doubles,scaled(symbols[1],2),{1,-1});
    known(doubles,scaled(symbols[2],2),{2,-1});
    known(doubles,{}, {0,-1});
    // An A/B cancellation requires the coefficient two; omitting it fails.
    const auto x=base[2],q=base[0];
    const auto ta=plus(x,q),tb=scaled(x,2),unequal=plus(scaled(ta,2),negative(tb));
    require(unequal==scaled(q,2)&&key(unequal)==ssl_query(ssl,ta,tb,2,-1),"A/B repeated-image cancellation");
    known(doubles,unequal,{1,-1});
    rejects([&]{require(plus(ta,negative(tb))==scaled(q,2),"omitted coefficient control");});++controls;
    rejects([&]{require(plus(symbols[1],symbols[3])==plus(plus(symbols[1],symbols[3]),base[0]),"changed query witness");});++controls;
    auto missing=a;missing.pop_back();
    rejects([&]{require(fingerprint(missing)==fingerprint(b0),"missing pair stream record");});++controls;
    rejects([&]{require(on_curve({false,0,0}),"off-curve point");});++controls;
    std::vector<int> indices;
    std::vector<Point> traces;
    size_t premise_matches=0;
    for(size_t i=0;i<images.size();++i) {
        const auto& record=tangents.at("tangents").arr[i];
        require(record.at("point_index").integer()==int(i),"ordered complete tangent labels");
        const auto t=plus(kappa,negative(scaled(images[i],2)));
        require(t==read_point(record.at("trace"))&&key(negative(t))==ssl_query(ssl,images[i],kappa,2,-1),"rederived trace agreement");
        if(record.at("category").scalar=="split-distinct-rational") {
            indices.push_back(int(i));traces.push_back(t);premise_matches+=pairs.lookup(key(t)).size();
        }
    }
    require(indices.size()==102,"exact split subset");
    std::vector<Witness> witnesses;
    std::vector<Item> aa_gmp,aa_ssl,ab_gmp,ab_ssl;
    size_t aa_hits=0,ab_hits=0;
    auto query=[&](size_t i,size_t j,int sign,bool mixed) {
        const int coefficient=mixed?2:1;
        const auto t=plus(scaled(traces[i],coefficient),scaled(traces[j],sign));
        const auto gkey=key(t),skey=ssl_query(ssl,traces[i],traces[j],coefficient,sign);
        require(gkey==skey,"every necessary-identity backend agreement");
        const auto& table=mixed?doubles:pairs;
        const auto left=table.lookup(gkey),right=table.lookup(skey);
        require(left==right,"every necessary-identity witness agreement");
        const std::string family=mixed?"AB":"AA";
        const auto tag=family+(sign==1?"+":"-");
        (mixed?ab_gmp:aa_gmp).push_back({{indices[i],indices[j]},gkey,tag});
        (mixed?ab_ssl:aa_ssl).push_back({{indices[i],indices[j]},skey,tag});
        if(!left.empty()) {
            witnesses.push_back({indices[i],indices[j],sign,family,left});
            (mixed?ab_hits:aa_hits)+=left.size();
        }
        require(witnesses.size()<=4096&&aa_hits+ab_hits<=16384,"witness cap; result incomplete if exceeded");
    };
    for(size_t i=0;i<indices.size();++i) for(size_t j=i+1;j<indices.size();++j)
        for(int sign:{1,-1}) query(i,j,sign,false);
    for(size_t i=0;i<indices.size();++i) for(size_t j=0;j<indices.size();++j) if(i!=j)
        for(int sign:{1,-1}) query(i,j,sign,true);
    require(aa_gmp.size()==10302&&ab_gmp.size()==20604,"complete fixed query counts");
    require(fingerprint(aa_gmp)==fingerprint(aa_ssl)&&fingerprint(ab_gmp)==fingerprint(ab_ssl),"complete query streams match");
    std::ostringstream out;
    out<<"{\"standing\":\"producer-unverified\",\"scope\":\"necessary-norm-tests-for-two-split-tangent-rows\",\"split_indices\":[";
    for(size_t i=0;i<indices.size();++i) out<<(i?",":"")<<indices[i];
    out<<"],\"split_count\":"<<indices.size()<<",\"alphabet_size\":"<<symbols.size()<<",\"pair_count\":"<<a.size()
       <<",\"pair_fingerprint_sha256\":\""<<fingerprint(a)<<"\",\"doubles_fingerprint_sha256\":\""<<fingerprint(doubles_gmp)
       <<"\",\"aa_query_count\":"<<aa_gmp.size()<<",\"ab_query_count\":"<<ab_gmp.size()
       <<",\"aa_query_fingerprint_sha256\":\""<<fingerprint(aa_gmp)<<"\",\"ab_query_fingerprint_sha256\":\""<<fingerprint(ab_gmp)
       <<"\",\"aa_witness_count\":"<<aa_hits<<",\"ab_witness_count\":"<<ab_hits<<",\"single_row_premise_witnesses\":"<<premise_matches
       <<",\"two_row_exclusion_derived\":"<<(premise_matches==0&&witnesses.empty()?"true":"false")
       <<",\"backend_streams_match\":true,\"grouped_sorted_witnesses_match\":true,\"controls_passed\":"<<controls<<",\"witnesses\":[";
    for(size_t i=0;i<witnesses.size();++i) {
        if(i) out<<',';
        const auto& w=witnesses[i];
        out<<"{\"i\":"<<w.i<<",\"j\":"<<w.j<<",\"sign\":"<<w.sign<<",\"family\":\""<<w.family<<"\",\"old_labels\":[";
        for(size_t k=0;k<w.labels.size();++k) out<<(k?",":"")<<'['<<w.labels[k][0]<<','<<w.labels[k][1]<<']';
        out<<"]}";
    }
    out<<"],\"new_curve_points\":0,\"residual_roots_extracted\":0,\"production_collector_calls\":0}\n";
    fresh_write(argv[4],out.str());
    const auto a0=var("a"),b1=var("b"),x0=var("X"),q0=var("q"),r0=var("r");
    issue(argv[5],"Generic unequal-coefficient two-row cancellation",{"a","b","X","q","r"},
          sub(mul({b1,add({mul({a0,x0}),q0})}),mul({a0,add({mul({b1,x0}),r0})})),
          sub(mul({b1,q0}),mul({a0,r0})),3);
    return 0;
}
}
int main(int argc,char** argv) {
    try {return two_row::run(argc,argv);}
    catch(const std::exception& error) {std::cerr<<"REJECT "<<error.what()<<'\n';return 1;}
}
