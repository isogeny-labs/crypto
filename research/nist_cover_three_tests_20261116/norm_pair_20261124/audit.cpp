#include "signed_words_arithmetic.hpp"

namespace tangent_pairs {
using namespace audit;
using PairSet=std::set<Pair>;
using Entry=std::pair<std::string,Pair>;
struct Query { int index; std::string category; Point trace; PairSet witnesses; };
Point symbol(int label,const std::vector<Point>& base) {
    if(!label) return {};
    const auto& point=base.at(size_t((label-1)/2));
    return label%2?point:negative(point);
}
void ssl_zero(const Row& row,const std::vector<Point>& base,audit::SSL& ssl) {
    auto total=ssl.make({});
    for(size_t i=0;i<row.size();++i) {
        require(row[i]>=-8&&row[i]<=8,"bounded signed norm coefficient");
        long count=row[i].get_si();
        auto term=ssl.make(count<0?negative(base[i]):base[i]);
        if(count<0) count=-count;
        while(count--) require(EC_POINT_add(ssl.group.get(),total.get(),total.get(),term.get(),ssl.context.get())==1,"independent row sum");
    }
    require(EC_POINT_is_at_infinity(ssl.group.get(),total.get())==1,"OpenSSL norm row zero");
}
int run(int argc,char** argv) {
    require(argc==5,"usage: audit BASELINE.json TANGENTS.json PRIOR-SIGNED.json OUTPUT.json");
    const auto data=read_json(argv[1]),traces=read_json(argv[2]),prior=read_json(argv[3]);
    std::vector<Point> base,images;
    std::vector<Pair> projection;
    for(const auto& q:data.at("projected_base").arr) {
        auto point=read_point(q);
        require(!point.zero&&on_curve(point)&&point.y<=p-point.y,"canonical old base");
        base.push_back(point);
    }
    require(base.size()==225,"exact old base size");
    for(const auto& item:data.at("images").arr) {
        const auto point=read_point(item.at("point"));
        const int j=item.at("projected_index").integer(),s=item.at("sign").integer();
        require(on_curve(point),"recorded image curve equation");
        if(j<0) require(j==-1&&s==0&&point.zero,"identity image binding");
        else {
            require(size_t(j)<base.size()&&(s==1||s==-1),"image projection range");
            require(point==(s==1?base[size_t(j)]:negative(base[size_t(j)])),"exact old projection binding");
        }
        images.push_back(point);projection.push_back({j,s});
    }
    require(images.size()==226&&traces.at("tangents").arr.size()==226,"complete frozen query domain");
    Row canonical(base.size());
    Point kappa;
    audit::SSL ssl;
    auto sk=ssl.make({});
    for(const auto& index:data.at("pool").at("split_fibers").arr.at(0).at("point_indices").arr) {
        const size_t i=size_t(index.integer());
        kappa=plus(kappa,images.at(i));
        if(projection[i][0]>=0) canonical.at(size_t(projection[i][0]))+=projection[i][1];
        auto point=ssl.make(images.at(i));
        require(EC_POINT_add(ssl.group.get(),sk.get(),sk.get(),point.get(),ssl.context.get())==1,"reference SSL sum");
    }
    require(key(kappa)==ssl.encode(sk.get()),"independent canonical trace");
    const auto first=signed_audit::enumerate(base,true),second=signed_audit::enumerate(base,false);
    const std::string digest=fingerprint(first);
    require(first.size()==101926&&digest==fingerprint(second)&&digest==prior.at("pair_fingerprint_sha256").scalar,
            "full stream matches GMP and frozen prior fingerprint");
    std::map<std::string,std::vector<Pair>> grouped;
    std::vector<Entry> sorted;
    for(const auto& row:first) grouped[row.sum].push_back(row.indices);
    for(const auto& row:second) sorted.emplace_back(row.sum,row.indices);
    std::sort(sorted.begin(),sorted.end());
    auto lookup=[&](const Point& trace) {
        const std::string encoded=key(trace);
        PairSet a,b;
        const auto hit=grouped.find(encoded);
        if(hit!=grouped.end()) a.insert(hit->second.begin(),hit->second.end());
        auto cursor=std::lower_bound(sorted.begin(),sorted.end(),encoded,
            [](const Entry& entry,const std::string& value){return entry.first<value;});
        while(cursor!=sorted.end()&&cursor->first==encoded) {b.insert(cursor->second);++cursor;}
        require(a==b,"independent complete pair query joins");
        return a;
    };
    const std::vector<std::pair<Point,Pair>> known{
        {plus(base[0],base[1]),{1,3}}, {plus(base[0],base[0]),{1,1}},
        {base[0],{0,1}}, {{},{0,0}}};
    for(const auto& control:known) require(lookup(control.first).count(control.second)==1,"known-answer pair query");
    int controls=4;
    const Point changed_query=plus(known[0].first,base[0]);
    rejects([&]{require(plus(symbol(1,base),symbol(3,base))==changed_query,"changed query witness");});++controls;
    auto missing=first;missing.pop_back();
    rejects([&]{require(missing.size()==first.size()&&fingerprint(missing)==digest,"missing fingerprint item");});++controls;
    Matrix baseline;
    for(const auto& sparse:data.at("rows").arr) {
        Row row(base.size());
        for(const auto& item:sparse.arr) row.at(size_t(item.arr[0].integer()))+=Big(item.arr[1].scalar);
        require(row_sum(row,base).zero,"baseline GMP sum");ssl_zero(row,base,ssl);baseline.push_back(row);
    }
    require(baseline.size()==6&&matrix_rank(baseline)==6,"fixed baseline rank");
    auto mutation=baseline[0];auto nonzero=std::find_if(mutation.begin(),mutation.end(),[](const Big& value){return value!=0;});
    require(nonzero!=mutation.end(),"nonzero baseline control");++*nonzero;
    rejects([&]{require(row_sum(mutation,base).zero,"changed coefficient");});++controls;
    auto row_for=[&](size_t index,Pair pair) {
        Row row=canonical;for(auto& value:row) value=-value;
        const auto p0=projection.at(index);
        if(p0[0]>=0) row.at(size_t(p0[0]))+=2*p0[1];
        for(int label:pair) if(label) row.at(size_t((label-1)/2))+=label%2?1:-1;
        require(row_sum(row,base).zero,"candidate GMP elliptic sum");ssl_zero(row,base,ssl);
        return row;
    };
    std::vector<Query> queries;
    std::set<Row> distinct;
    size_t witness_count=0,split_count=0,split_witnesses=0;
    for(size_t i=0;i<images.size();++i) {
        const auto& record=traces.at("tangents").arr[i];
        require(record.at("point_index").integer()==int(i),"complete ordered tangent records");
        const auto trace=plus(kappa,negative(plus(images[i],images[i])));
        auto image=ssl.make(images[i]),twice=ssl.make({}),actual=ssl.make({});
        require(EC_POINT_dbl(ssl.group.get(),twice.get(),image.get(),ssl.context.get())==1&&
            EC_POINT_invert(ssl.group.get(),twice.get(),ssl.context.get())==1&&
            EC_POINT_add(ssl.group.get(),actual.get(),sk.get(),twice.get(),ssl.context.get())==1,
            "independent residual query trace");
        require(key(trace)==ssl.encode(actual.get())&&trace==read_point(record.at("trace")),"both traces match frozen query");
        const auto matches=lookup(trace);
        const std::string category=record.at("category").scalar;
        witness_count+=matches.size();
        require(witness_count<=4096,"pair witness cap; incomplete if exceeded");
        if(category=="split-distinct-rational") {++split_count;split_witnesses+=matches.size();}
        for(const auto& pair:matches) distinct.insert(row_for(i,pair));
        require(distinct.size()<=256,"target row cap; incomplete if exceeded");
        queries.push_back({int(i),category,trace,matches});
    }
    require(split_count==102,"frozen split subset binding");
    std::vector<Row> rows(distinct.begin(),distinct.end());
    std::map<Row,size_t> row_index;
    Matrix all=baseline;
    size_t zero_rows=0;
    for(size_t i=0;i<rows.size();++i) {
        row_index.emplace(rows[i],i);all.push_back(rows[i]);
        if(std::all_of(rows[i].begin(),rows[i].end(),[](const Big& value){return value==0;})) ++zero_rows;
    }
    std::ofstream out(argv[4]);require(bool(out),"fresh norm-pair result");
    out << "{\"standing\":\"producer-unverified\",\"scope\":\"frozen-tangent-trace-two-signed-old-symbols\",\"alphabet_size\":451,\"pair_count\":" << first.size()
        << ",\"pair_fingerprint_sha256\":\"" << digest << "\",\"independent_streams_match\":true,\"independent_query_joins_match\":true,\"queries\":[";
    for(size_t i=0;i<queries.size();++i) {
        if(i) out << ',';
        const auto& q=queries[i];
        out << "{\"point_index\":" << q.index << ",\"category\":\"" << q.category << "\",\"trace\":";
        signed_audit::emit_point(out,q.trace);
        out << ",\"pair_witnesses\":[";
        bool comma=false;
        for(const auto& pair:q.witnesses) {
            if(comma) {out << ',';}comma=true;
            out << "{\"labels\":[" << pair[0] << ',' << pair[1] << "],\"row_index\":" << row_index.at(row_for(i,pair)) << '}';
        }
        out << "]}";
    }
    out << "],\"query_count\":" << queries.size() << ",\"split_query_count\":" << split_count << ",\"pair_witness_count\":" << witness_count
        << ",\"split_pair_witness_count\":" << split_witnesses << ",\"unique_row_count\":" << rows.size() << ",\"zero_row_count\":" << zero_rows
        << ",\"baseline_rank\":6,\"all_norm_rank\":" << matrix_rank(all) << ",\"controls_passed\":" << controls << ",\"signed_rows\":[";
    for(size_t i=0;i<rows.size();++i) {if(i) out << ',';emit_row(out,rows[i]);}
    out << "],\"halftrace_alias_rows\":[],\"new_curve_points\":0}\n";
    require(bool(out),"complete norm-pair result");
    return 0;
}
}
int main(int argc,char** argv) {
    try { return tangent_pairs::run(argc,argv); }
    catch(const std::exception& error) { std::cerr << "REJECT " << error.what() << '\n';return 1; }
}
