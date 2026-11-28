static Samples load(const std::string& path,const std::string& panel) {
 std::ifstream in(path);if(!in)throw std::runtime_error("missing rows");
 Samples out;std::set<std::string> seen;std::map<std::string,std::string> outputs;
 unsigned expected_points=panel=="calibration"?4:8;std::string line;
 const std::set<std::string> methods={"window","window-aa","maps","fused","openssl-public","openssl-public-aa","openssl-single"};
 while(std::getline(in,line)){
  auto r=JsonParser(line).read();
  auto point=r.at("point").unsigned_integer(),round=r.at("round").unsigned_integer(),window=r.at("window").unsigned_integer();
  auto curve=r.at("curve").str(),method=r.at("arm").str();
  if(point>=expected_points||round>6||window<2||window>7||r.at("inner").unsigned_integer()!=2)throw std::runtime_error("row outside frozen design");
  if((curve!="P-384"&&curve!="P-521")||!methods.count(method))throw std::runtime_error("unknown curve or method");
  std::string key=curve+":"+method+":"+std::to_string(window)+":"+std::to_string(point)+":"+std::to_string(round);
  if(!seen.insert(key).second)throw std::runtime_error("duplicate row");
  if(r.at("warmup").kind!=Json::Boolean||r.at("correct").kind!=Json::Boolean||r.at("warmup").value!=(round==0?"true":"false")||r.at("correct").value!="true"||r.at("panel").str()!=panel)throw std::runtime_error("invalid panel, output or warmup");
  auto total=r.at("total_ns").unsigned_integer();uint64_t sum=0;
  for(const auto* name:{"setup_ns","maps_ns","group_ns","verify_ns","bookkeeping_ns"}){
   auto phase=r.at(name).unsigned_integer();if(phase>std::numeric_limits<uint64_t>::max()-sum)throw std::runtime_error("phase overflow");sum+=phase;
  }
  if(!total||total!=sum)throw std::runtime_error("unpriced or invalid total");
  std::string output_key=curve+":"+std::to_string(point),output=r.at("output").str();
  if(outputs.count(output_key)&&outputs.at(output_key)!=output)throw std::runtime_error("unequal point outputs");
  outputs[output_key]=output;
  if(round)out[curve][method+":"+std::to_string(window)][int(point)].push_back(double(total)/2);
 }
 if(seen.size()!=2*6*7*7*expected_points)throw std::runtime_error("missing design rows");
 return out;
}
