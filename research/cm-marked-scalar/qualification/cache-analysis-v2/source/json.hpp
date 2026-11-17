#pragma once
#include <cctype>
#include <fstream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <cstdint>
#include <limits>
// Preserve integer tokens exactly: curve coefficients exceed binary64 precision.
struct Json {
  enum Kind { Object, Array, String, Number, Boolean, Null } kind=Null;
  std::string value; std::map<std::string,Json> object; std::vector<Json> array;
  const Json& at(const std::string& k) const { return object.at(k); }
  const Json& at(size_t i) const { return array.at(i); }
  std::string str() const { if(kind!=String&&kind!=Number) throw std::runtime_error("expected scalar token"); return value; }
  int integer() const {auto token=str();size_t used=0;int n=std::stoi(token,&used);if(used!=token.size())throw std::runtime_error("expected integer");return n;}
  uint64_t unsigned_integer() const {if(kind!=Number||value.empty())throw std::runtime_error("expected unsigned JSON integer");uint64_t n=0;for(char c:value){if(c<'0'||c>'9'||n>(std::numeric_limits<uint64_t>::max()-unsigned(c-'0'))/10)throw std::runtime_error("unsigned integer syntax or overflow");n=n*10+unsigned(c-'0');}return n;}
};
class JsonParser {
  std::string s; size_t i=0;
  void space(){while(i<s.size()&&std::isspace(static_cast<unsigned char>(s[i])))++i;}
  char take(){if(i==s.size())throw std::runtime_error("truncated JSON");return s[i++];}
  std::string string(){
    if(take()!='"')throw std::runtime_error("JSON string");
    std::string v;
    for(;;){char c=take();if(c=='"')break;if(static_cast<unsigned char>(c)<32)throw std::runtime_error("control in JSON");
      if(c=='\\'){c=take();switch(c){case '"':case '\\':case '/':break;case 'n':c='\n';break;case 'r':c='\r';break;case 't':c='\t';break;case 'b':c='\b';break;case 'f':c='\f';break;
      case 'u':{unsigned u=0;for(int j=0;j<4;++j){char h=take();unsigned d=h>='0'&&h<='9'?h-'0':h>='a'&&h<='f'?h-'a'+10:h>='A'&&h<='F'?h-'A'+10:99;if(d>15)throw std::runtime_error("unicode escape");u=u*16+d;}if(u<128)c=static_cast<char>(u);else throw std::runtime_error("non-ASCII unicode escape unsupported in this fixture reader");break;}
      default:throw std::runtime_error("JSON escape");}}v+=c;}
    return v;
  }
  Json parse(unsigned depth){
    if(depth>128)throw std::runtime_error("JSON depth");
    space();if(i==s.size())throw std::runtime_error("empty JSON");Json j;char c=s[i];
    if(c=='{'||c=='['){bool obj=c=='{';++i;j.kind=obj?Json::Object:Json::Array;space();if(i<s.size()&&s[i]==(obj?'}':']')){++i;return j;}
      for(;;){space();if(obj){std::string k=string();space();if(take()!=':')throw std::runtime_error("JSON colon");if(!j.object.emplace(k,parse(depth+1)).second)throw std::runtime_error("duplicate JSON key");}else j.array.push_back(parse(depth+1));space();char end=take();if(end==(obj?'}':']'))break;if(end!=',')throw std::runtime_error("JSON comma");}return j;}
    if(c=='"'){j.kind=Json::String;j.value=string();return j;}
    size_t start=i;while(i<s.size()&&s[i]!=','&&s[i]!=']'&&s[i]!='}'&&!std::isspace(static_cast<unsigned char>(s[i])))++i;j.value=s.substr(start,i-start);
    if(j.value.empty())throw std::runtime_error("empty JSON token");
    if(j.value=="null")j.kind=Json::Null;else if(j.value=="true"||j.value=="false")j.kind=Json::Boolean;else{j.kind=Json::Number;size_t k=j.value[0]=='-'?1:0;auto digits=[&](){size_t start=k;while(k<j.value.size()&&std::isdigit(static_cast<unsigned char>(j.value[k])))++k;if(k==start)throw std::runtime_error("JSON number digits");};if(k==j.value.size())throw std::runtime_error("JSON number");if(j.value[k]=='0')++k;else digits();if(k<j.value.size()&&j.value[k]=='.'){++k;digits();}if(k<j.value.size()&&(j.value[k]=='e'||j.value[k]=='E')){++k;if(k<j.value.size()&&(j.value[k]=='+'||j.value[k]=='-'))++k;digits();}if(k!=j.value.size())throw std::runtime_error("JSON number syntax");}return j;
  }
public:
  explicit JsonParser(std::string text):s(std::move(text)){}
  Json read(){Json j=parse(0);space();if(i!=s.size())throw std::runtime_error("trailing JSON");return j;}
};
inline Json read_json(const std::string& path){std::ifstream f(path);if(!f)throw std::runtime_error("open "+path);std::ostringstream b;b<<f.rdbuf();return JsonParser(b.str()).read();}
inline std::string json_quote(const std::string&s){std::ostringstream o;o<<'"';static const char*h="0123456789abcdef";for(unsigned char c:s){if(c=='"'||c=='\\')o<<'\\'<<char(c);else if(c<32)o<<"\\u00"<<h[c>>4]<<h[c&15];else o<<char(c);}o<<'"';return o.str();}
inline std::string canonical_json(const Json&j){if(j.kind==Json::Object){std::string s="{";bool comma=false;for(auto&[k,v]:j.object){if(comma)s+=",";comma=true;s+=json_quote(k)+":"+canonical_json(v);}return s+"}";}if(j.kind==Json::Array){std::string s="[";bool comma=false;for(auto&v:j.array){if(comma)s+=",";comma=true;s+=canonical_json(v);}return s+"]";}if(j.kind==Json::String)return json_quote(j.value);if(j.kind==Json::Null)return "null";return j.value;}
