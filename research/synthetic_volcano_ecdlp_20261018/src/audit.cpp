// Independent consistency audit of the invented CSV records, not EC correctness.
#include <algorithm>
#include <array>
#include <cmath>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using namespace std;
struct CSV {
    ifstream f; map<string,int> h; vector<string> row;
    static vector<string> split(const string& s) {vector<string> a;stringstream x(s);string v;while(getline(x,v,','))a.push_back(v);return a;}
    CSV(const filesystem::path& p):f(p){string s;if(!getline(f,s))throw runtime_error("cannot read "+p.string());auto a=split(s);for(int i=0;i<int(a.size());i++)h[a[i]]=i;}
    bool next(){string s;if(!getline(f,s))return false;row=split(s);if(row.size()!=h.size())throw runtime_error("CSV width");return true;}
    string s(const string& k){return row.at(h.at(k));}
    double d(const string& k){return stod(s(k));}
};
void check(bool yes,const string& why){if(!yes)throw runtime_error(why);}
bool close(double x,double y){return abs(x-y)<=1e-9*max({1.0,abs(x),abs(y)});}
string key(CSV& f,const vector<string>& columns){string k;for(auto c:columns)k+=f.s(c)+"|";return k;}
struct Group {array<vector<double>,6> v;set<int> targets;};
void check_stats(CSV& f,vector<double> a) {
    sort(a.begin(),a.end());long double sum=0;for(auto x:a)sum+=x;double mean=sum/a.size();
    double median=static_cast<double>((static_cast<long double>(a[(a.size()-1)/2])+a[a.size()/2])/2);long double sq=0;for(auto x:a){long double delta=static_cast<long double>(x)-mean;sq+=delta*delta;}
    double sd=sqrt(double(sq/(a.size()-1)));
    check(int(f.d("n"))==int(a.size()),"summary n");
    check(close(f.d("min"),a.front())&&close(f.d("max"),a.back()),"summary extrema");
    check(close(f.d("mean"),mean)&&close(f.d("median"),median)&&close(f.d("sd"),sd),"summary statistics");
}
int main(int argc,char** argv){try {
    check(argc==2,"usage: audit RUN_DIRECTORY");filesystem::path dir=argv[1];
    map<string,Group> curves;array<double,3> weights{0,.1,1};uint64_t raw=0,summary=0,strategies=0;
    CSV f(dir/"trials.csv");
    while(f.next()) {
        check(f.s("status")=="SYNTHETIC","missing synthetic label");
        auto& group=curves[key(f,{"family","field_log2","scenario","vertex","method"})];
        int t=f.d("target");check(t>=0&&t<10&&group.targets.insert(t).second,"ten unique targets");
        double g=0,n=0;for(auto phase:{"curve","fb","relations","la","online","verify"}) {
            double x=f.d(string(phase)+"_g"),y=f.d(string(phase)+"_native");
            check(isfinite(x)&&isfinite(y)&&x>=0&&y>=0,"cost finite and nonnegative");g+=x;n+=y;
        }
        check(f.d("relation_attempts")>=f.d("assumed_full_rank_rows"),"attempts at least assumed successes");
        check(f.d("assumed_success_p")>=0&&f.d("assumed_success_p")<=1,"valid probability");
        check(!(f.s("family")=="koblitz_origin"&&f.d("level")>0)||f.d("orbit_discount")==1,"no Koblitz discount below surface");
        for(int w=0;w<3;w++) {
            group.v[2*w].push_back(g+weights[w]*n);
            double tail=f.d("online_g")+f.d("verify_g")+weights[w]*(f.d("online_native")+f.d("verify_native"));
            group.v[2*w+1].push_back((f.d("batch_shared_g")+weights[w]*f.d("batch_shared_native"))/10+tail);
        }raw++;
    }
    CSV c(dir/"curve_summary.csv");
    while(c.next()) {
        auto k=key(c,{"family","field_log2","scenario","vertex","method"});check(curves.count(k),"summary has raw records");
        int w=c.d("native_weight")==0?0:c.d("native_weight")==.1?1:2;
        check_stats(c,curves.at(k).v[2*w+(c.s("accounting")=="batch10_per_target")]);summary++;
    }
    check(summary==curves.size()*6,"all six accounting bands summarized");
    check(raw==curves.size()*10,"all curves have ten raw targets");
    map<string,vector<double>> strategy_groups;
    CSV st(dir/"strategy_trials.csv");
    while(st.next())strategy_groups[key(st,{"family","field_log2","scenario","method","strategy","native_weight","accounting"})].push_back(st.d("modeled_work"));
    CSV ss(dir/"strategy_summary.csv");
    while(ss.next()) {
        auto k=key(ss,{"family","field_log2","scenario","method","strategy","native_weight","accounting"});
        check(strategy_groups.count(k),"strategy has holdout records");check_stats(ss,strategy_groups.at(k));strategies++;
    }
    check(strategies==strategy_groups.size(),"all strategies summarized");
    CSV ps(dir/"pilot_selection.csv");int selections=0;
    while(ps.next()){check(ps.d("test_targets_used_for_selection")==0,"holdout leakage");check(ps.d("pilot_targets")==3,"three pilots");selections++;}
    cout<<"CSV audit passed: "<<raw<<" raw rows, "<<summary<<" curve summaries, "<<strategies<<" strategy summaries, "<<selections<<" pilot selections. No cryptographic correctness claim.\n";
    return 0;
}catch(const exception& e){cerr<<"audit failed: "<<e.what()<<'\n';return 1;}}
