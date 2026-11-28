#pragma once
// Separate typed copy for the frozen recorded-JSON input contract.
// Numeric tokens remain exact decimal strings; unsupported escapes reject.
#include <cctype>
#include <fstream>
#include <map>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
inline void require(bool b, const std::string& s) { if (!b) throw std::runtime_error(s); }
enum class JsonKind { null_value,boolean,number,string,array,object };
struct Json {
    JsonKind kind=JsonKind::null_value;
    std::string scalar;
    std::vector<Json> arr;
    std::map<std::string,Json> obj;
    const Json& at(const std::string& s) const { return obj.at(s); }
    int integer() const { size_t n=0; int v=std::stoi(scalar,&n); require(n==scalar.size(),"invalid integer"); return v; }
};
struct Parser {
    std::string s; size_t p=0;
    void ws() { while(p<s.size() && std::isspace(static_cast<unsigned char>(s[p]))) ++p; }
    char take() { require(p<s.size(),"truncated JSON"); return s[p++]; }
    std::string str() {
        require(take()=='"',"JSON string expected"); std::string r;
        while(true) { char c=take(); if(c=='"') return r;
            if(c=='\\') { c=take(); if(c=='n') c='\n'; else if(c=='t') c='\t'; else if(c=='r') c='\r';
                else require(c=='"'||c=='\\'||c=='/',"unsupported JSON escape"); }
            r+=c;
        }
    }
    Json value() {
        ws(); require(p<s.size(),"empty JSON"); Json j; char c=s[p];
        if(c=='{') { j.kind=JsonKind::object; ++p; ws(); require(p<s.size(),"truncated object"); if(s[p]=='}') {++p;return j;}
            while(true) {ws();auto k=str();ws();require(take()==':',"JSON colon");
                require(j.obj.emplace(k,value()).second,"duplicate JSON key");ws();c=take();if(c=='}')return j;require(c==',',"JSON comma");}
        }
        if(c=='[') {j.kind=JsonKind::array;++p;ws();require(p<s.size(),"truncated array");if(s[p]==']'){++p;return j;}
            while(true){j.arr.push_back(value());ws();c=take();if(c==']')return j;require(c==',',"JSON comma");}}
        if(c=='"'){j.kind=JsonKind::string;j.scalar=str();return j;}
        size_t start=p;while(p<s.size()&&s[p]!=','&&s[p]!=']'&&s[p]!='}'&&!std::isspace(static_cast<unsigned char>(s[p])))++p;
        require(p>start,"JSON token");j.scalar=s.substr(start,p-start);
        if(j.scalar=="true"||j.scalar=="false")j.kind=JsonKind::boolean;
        else if(j.scalar=="null")j.kind=JsonKind::null_value;
        else {static const std::regex number(R"(-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?)");
              require(std::regex_match(j.scalar,number),"JSON token grammar");j.kind=JsonKind::number;}
        return j;
    }
};
inline Json read_json(const std::string& path) {
    std::ifstream f(path);require(bool(f),"cannot read "+path);std::stringstream ss;ss<<f.rdbuf();Parser p{ss.str()};
    auto j=p.value();p.ws();require(p.p==p.s.size(),"trailing JSON");return j;
}
