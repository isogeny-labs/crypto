#include <gmpxx.h>
#include <algorithm>
#include <cmath>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>

using Z=mpz_class;
using Q=mpq_class;
void need(bool b,const std::string& s) { if(!b)throw std::runtime_error(s); }
std::string read(const std::string& p) {
    std::ifstream f(p);need(bool(f),"cannot open "+p);
    return {std::istreambuf_iterator<char>(f),std::istreambuf_iterator<char>()};
}
std::vector<std::string> split(const std::string&s,char c) {
    std::vector<std::string> r;size_t start=0;
    while(true) {size_t end=s.find(c,start);r.push_back(s.substr(start,end-start));
        if(end==std::string::npos)break;
        start=end+1;}
    return r;
}
std::map<int,int> hist(const std::string&s,int q) {
    std::map<int,int> h;if(s.empty())return h;
    for(auto&v:split(s,';')) {
        auto p=split(v,':');need(p.size()==2,"bad histogram");
        int w=std::stoi(p[0]),n=std::stoi(p[1]);
        need(w>0&&w<q&&n>0&&h.emplace(w,n).second,"bad histogram weight/count");
    }return h;
}
// Independent recompute: grouped generating functions, using binomial
// coefficients for repeated weights, rather than the instrument's coin loop.
Z recompute(const std::map<int,int>&h,int q) {
    std::vector<Z> d(q);d[0]=1;
    for(auto [w,n]:h) {
        std::vector<Z> next(q);
        for(int k=0;k*w<q;++k) {
            Z multiplicity;mpz_bin_uiui(multiplicity.get_mpz_t(),n+k-1,k);
            for(int i=0;i+k*w<q;++i)next[i+k*w]+=d[i]*multiplicity;
        }
        d=next;
    }
    Z total=0;for(const Z&v:d)total+=v;return total;
}
using Key=std::tuple<int,int,int,int,int,std::string>;
using Group=std::tuple<int,int,std::string>;
struct Stats {
    int fewer=0,equal=0,more=0;
    Q minimum=0,maximum=0;
    void add(const Z&a,const Z&b) {
        Q r(a,b);r.canonicalize();
        if(!fewer&&!equal&&!more)minimum=maximum=r;
        minimum=std::min(minimum,r);maximum=std::max(maximum,r);
        if(b<a)++fewer;else if(b==a)++equal;else ++more;
    }
    int n() const {return fewer+equal+more;}
};
int main(int argc,char**argv) {
    try {
        if(argc==2&&std::string(argv[1])=="--unit") {
            need(recompute({{2,1},{3,1}},6)==5,"independent unit1");
            need(recompute({{2,2}},6)==6,"independent unit2");
            need(recompute({},24)==1,"empty product unit");
            std::cout<<"grouped_recompute_units=PASS\n";return 0;
        }
        need(argc==4,"use PANEL REPLAY OUTPUT_DIR");
        std::string a=read(argv[1]);need(a==read(argv[2]),"replay differs");
        std::set<Key> expected;
        for(int l:{4,8,16,32})for(int t=-l+1;t<l;++t)if(t) {
            int i=__builtin_ctz(unsigned(std::abs(t)));
            for(int z:{-1,1,3})for(int q:{8,12,16,24})for(std::string m:{"newton","smith"})
                expected.emplace(l,t,i,z,q,m);
        }
        need(expected.size()==2688,"bad prospective census size");
        std::istringstream in(a);std::string line;std::getline(in,line);
        need(line=="L,t,level,zeta,q,method,original_tuples,candidate_tuples,original_weights,candidate_weights","header");
        std::map<Group,Stats> groups;std::map<std::string,Stats> methods;
        int rows=0,recomputations=0;
        std::filesystem::create_directories(argv[3]);
        std::ofstream normalized(std::string(argv[3])+"/rows.jsonl");
        while(std::getline(in,line)) {
            auto c=split(line,',');need(c.size()==10,"wrong column count");
            int l=std::stoi(c[0]),t=std::stoi(c[1]),i=std::stoi(c[2]),z=std::stoi(c[3]),q=std::stoi(c[4]);
            std::string method=c[5];Key key(l,t,i,z,q,method);
            need(expected.erase(key)==1,"duplicate or unexpected input cell");
            Z original(c[6]),candidate(c[7]);need(original>0&&candidate>0,"nonpositive tuple count");
            need(recompute(hist(c[8],q),q)==original,"original tuple recompute mismatch");
            need(recompute(hist(c[9],q),q)==candidate,"candidate tuple recompute mismatch");
            recomputations+=2;++rows;groups[{l,i,method}].add(original,candidate);methods[method].add(original,candidate);
            // Adapter rows preserve the raw exact primary counts. This is
            // descriptive export formatting, not an additional experiment.
            std::string id="L"+c[0]+"-t"+c[1]+"-z"+c[3]+"-q"+c[4]+"-"+method;
            normalized<<"{\"arm\":\"original\",\"instance\":\""<<id<<"\",\"seed\":0,\"replicate\":0,\"status\":\"ok\",\"tuples\":"<<original<<"}\n";
            normalized<<"{\"arm\":\"square\",\"instance\":\""<<id<<"\",\"seed\":0,\"replicate\":0,\"status\":\"ok\",\"tuples\":"<<candidate<<"}\n";
        }
        need(expected.empty()&&rows==2688,"missing input cells");
        std::ofstream csv(std::string(argv[3])+"/per-level.csv");
        csv<<"L,level,method,cells,fewer,equal,more,min_original_over_candidate,max_original_over_candidate\n";
        for(auto&[key,s]:groups) {
            auto [l,i,m]=key;
            csv<<l<<','<<i<<','<<m<<','<<s.n()<<','<<s.fewer<<','<<s.equal<<','<<s.more<<','<<s.minimum<<','<<s.maximum<<'\n';
        }
        std::ofstream summary(std::string(argv[3])+"/summary.json");
        summary<<"{\n  \"rows\": "<<rows<<",\n  \"input_cells\": 1344,\n  \"tuple_recomputations\": "<<recomputations<<",\n  \"replay_byte_identical\": true,\n  \"methods\": {\n";
        bool first=true;for(auto&[m,s]:methods) {
            if(!first)summary<<",\n";
            first=false;
            summary<<"    \""<<m<<"\": {\"fewer\": "<<s.fewer<<", \"equal\": "<<s.equal<<", \"more\": "<<s.more
                   <<", \"minimum_original_over_candidate\": \""<<s.minimum<<"\", \"maximum_original_over_candidate\": \""<<s.maximum<<"\"}";
        }summary<<"\n  }\n}\n";
        std::ofstream svg(std::string(argv[3])+"/per-level.svg");
        int height=110+int(groups.size())*25;
        svg<<"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\""<<height<<"\" viewBox=\"0 0 1000 "<<height<<"\">"
           <<"<rect width=\"100%\" height=\"100%\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"13\">"
           <<"<text x=\"20\" y=\"25\" font-size=\"18\">Exact finite-cell comparisons: candidate tuple counts</text>"
           <<"<text x=\"20\" y=\"48\">Green: fewer; gray: equal; red: more. Bar width shows each group's finite census.</text>";
        int y=80;for(auto&[key,s]:groups) {
            auto[l,i,m]=key;svg<<"<text x=\"20\" y=\""<<y+14<<"\">L="<<l<<", i="<<i<<", "<<m<<"</text>";
            double x=230;for(auto [n,color]:std::vector<std::pair<int,std::string>>{{s.fewer,"#20854e"},{s.equal,"#999999"},{s.more,"#bd3232"}}) {
                double w=500.0*n/s.n();if(n)svg<<"<rect x=\""<<x<<"\" y=\""<<y<<"\" width=\""<<w<<"\" height=\"18\" fill=\""<<color<<"\"/>";x+=w;
            }
            svg<<"<text x=\"745\" y=\""<<y+14<<"\">"<<s.fewer<<" / "<<s.equal<<" / "<<s.more<<" (n="<<s.n()<<")</text>";y+=25;
        }svg<<"<text x=\"20\" y=\""<<height-15<<"\">Component tuple census only; degree, recovery and basis costs are separate.</text></g></svg>\n";
        std::cout<<"rows="<<rows<<" independent_tuple_recomputations="<<recomputations<<" replay=IDENTICAL\n";
        for(auto&[m,s]:methods)std::cout<<m<<" fewer="<<s.fewer<<" equal="<<s.equal<<" more="<<s.more<<'\n';
        std::cout<<"PASS\n";
    }catch(const std::exception&e){std::cerr<<"FAIL "<<e.what()<<'\n';return 1;}
}
