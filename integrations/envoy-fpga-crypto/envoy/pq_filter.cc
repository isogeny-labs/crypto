#include <cmath>
#include <map>
#include <set>

#include "envoy/common/exception.h"
#include "envoy/event/dispatcher.h"
#include "envoy/http/filter.h"
#include "envoy/http/header_map.h"
#include "envoy/registry/registry.h"
#include "envoy/server/filter_config.h"

#include "source/common/protobuf/protobuf.h"
#include "source/extensions/fpga_crypto/config.pb.h"

#include "fpga/mailbox.h"
#include "fpga/pq.h"

namespace Envoy::Extensions::PqCrypto {
namespace {
using Struct = Protobuf::Struct;
using Value = Protobuf::Value;
const Value *field(const Struct &s, const std::string &key) {
  const auto it = s.fields().find(key);
  return it == s.fields().end() ? nullptr : &it->second;
}
void allowed(const Struct &s, const std::set<std::string> &keys) {
  for (const auto &p : s.fields())
    if (!keys.count(p.first))
      throw EnvoyException("unknown PQ field: " + p.first);
}
std::string text(const Struct &s, const std::string &key, const std::string &fallback = "") {
  const auto *v = field(s, key);
  if (!v)
    return fallback;
  if (v->kind_case() != Value::kStringValue || v->string_value().empty())
    throw EnvoyException("invalid PQ string: " + key);
  return v->string_value();
}
uint32_t number(const Struct &s, const std::string &key, uint32_t fallback, uint32_t low,
                uint32_t high) {
  const auto *v = field(s, key);
  if (!v)
    return fallback;
  if (v->kind_case() != Value::kNumberValue || !std::isfinite(v->number_value()) ||
      v->number_value() < low || v->number_value() > high ||
      std::floor(v->number_value()) != v->number_value())
    throw EnvoyException("invalid PQ number: " + key);
  return static_cast<uint32_t>(v->number_value());
}
bool boolean(const Struct &s, const std::string &key) {
  const auto *v = field(s, key);
  if (!v)
    return false;
  if (v->kind_case() != Value::kBoolValue)
    throw EnvoyException("invalid PQ boolean: " + key);
  return v->bool_value();
}
const std::map<std::string, fpga::Operation> operations{
    {"keygen", fpga::Operation::PqKeygen},
    {"sign", fpga::Operation::PqSign},
    {"verify", fpga::Operation::PqVerify},
    {"encapsulate", fpga::Operation::PqEncapsulate},
    {"decapsulate", fpga::Operation::PqDecapsulate},
    {"public-key", fpga::Operation::PqPublicKey},
    {"erase", fpga::Operation::PqErase}};
struct Config {
  struct Route {
    fpga::Operation operation;
    std::string algorithm;
    uint32_t slot;
  };
  std::shared_ptr<fpga::Engine> engine;
  std::string backend, prefix;
  std::chrono::milliseconds timeout;
  std::map<std::string, Route> routes;
  bool experimental;
  explicit Config(const Struct &s) {
    allowed(s, {"backend", "plugin_library", "allow_experimental", "max_keys", "queue_capacity",
                "timeout_ms", "path_prefix", "routes", "slot_id", "sdk_library", "lock_directory"});
    backend = text(s, "backend", "aws-fpga");
    prefix = text(s, "path_prefix", "/_pq/");
    if (prefix.front() != '/' || prefix.back() != '/' ||
        prefix.find_first_of("?#") != std::string::npos)
      throw EnvoyException("PQ path_prefix must start and end with /");
    timeout = std::chrono::milliseconds(number(s, "timeout_ms", 5000, 1, 60000));
    experimental = boolean(s, "allow_experimental");
    const auto capacity = number(s, "queue_capacity", 128, 1, 65536);
    if (backend == "software-liboqs") {
      for (const auto *key : {"slot_id", "sdk_library", "lock_directory"})
        if (field(s, key))
          throw EnvoyException("AWS fields are invalid for the software PQ backend");
      fpga::OqsOptions o;
      o.library = text(s, "plugin_library");
      o.allow_experimental = experimental;
      o.max_keys = number(s, "max_keys", 32, 1, 1024);
      o.queue_capacity = capacity;
      engine = fpga::oqsEngine(o);
    } else if (backend == "aws-fpga") {
      if (field(s, "plugin_library") || field(s, "max_keys"))
        throw EnvoyException("software PQ fields are invalid for AWS");
      fpga::AwsOptions o;
      o.slot = number(s, "slot_id", 0, 0, 7);
      o.queue_capacity = capacity;
      o.library = text(s, "sdk_library", o.library);
      o.lock_directory = text(s, "lock_directory", o.lock_directory);
      engine = fpga::awsEngine(o);
    } else
      throw EnvoyException("PQ backend must be aws-fpga or software-liboqs");
    fpga::Request query;
    query.operation = fpga::Operation::PqCatalog;
    auto result = engine->execute(query, timeout);
    auto catalog = result ? fpga::decodeCatalog(result.data) : std::nullopt;
    if (!catalog)
      throw EnvoyException("selected backend does not provide a valid PQ catalog");
    routes.emplace(prefix + "algorithms", Route{fpga::Operation::PqCatalog, "", 0});
    const auto *routes_value = field(s, "routes");
    if (!routes_value)
      return;
    if (routes_value->kind_case() != Value::kStructValue)
      throw EnvoyException("PQ routes must be an object");
    std::map<uint32_t, std::string> bindings;
    for (const auto &p : routes_value->struct_value().fields()) {
      if (p.first.empty() || p.first == "algorithms" || p.first.front() == '/' ||
          p.first.back() == '/' || p.first.find_first_of("?#%") != std::string::npos ||
          p.first.find("..") != std::string::npos || p.second.kind_case() != Value::kStructValue)
        throw EnvoyException("invalid PQ route name");
      const auto &route = p.second.struct_value();
      allowed(route, {"operation", "algorithm", "key_slot"});
      const auto op = operations.find(text(route, "operation"));
      if (op == operations.end())
        throw EnvoyException("unknown PQ operation");
      const auto name = text(route, "algorithm");
      const fpga::PqAlgorithm *algorithm = nullptr;
      for (const auto &a : *catalog)
        if (a.name == name)
          algorithm = &a;
      if (!algorithm ||
          (!experimental && (!algorithm->standardized || !fpga::pqDefaultAllowed(name))))
        throw EnvoyException("PQ algorithm is unavailable or disallowed: " + name);
      const bool sig =
          op->second == fpga::Operation::PqSign || op->second == fpga::Operation::PqVerify;
      const bool kem = op->second == fpga::Operation::PqEncapsulate ||
                       op->second == fpga::Operation::PqDecapsulate;
      if ((sig && algorithm->kind != fpga::PqKind::Signature) ||
          (kem && algorithm->kind != fpga::PqKind::Kem))
        throw EnvoyException("PQ operation has the wrong algorithm kind");
      const bool external =
          op->second == fpga::Operation::PqVerify || op->second == fpga::Operation::PqEncapsulate;
      const auto slot = number(route, "key_slot", 0, external ? 0 : 1, external ? 0 : UINT32_MAX);
      if (!external && !slot)
        throw EnvoyException("PQ operation requires a key_slot");
      if (slot) {
        auto binding = bindings.emplace(slot, name);
        if (!binding.second && binding.first->second != name)
          throw EnvoyException("PQ key_slot binds more than one algorithm");
      }
      routes.emplace(prefix + p.first, Route{op->second, name, slot});
    }
  }
};
class Filter final : public Http::StreamDecoderFilter {
public:
  explicit Filter(std::shared_ptr<const Config> config) : config_(std::move(config)) {}
  void setDecoderFilterCallbacks(Http::StreamDecoderFilterCallbacks &callbacks) override {
    callbacks_ = &callbacks;
  }
  void onDestroy() override {
    ticket_.cancel();
    timer_.reset();
  }
  Http::FilterHeadersStatus decodeHeaders(Http::RequestHeaderMap &h, bool end_stream) override {
    const std::string path(h.getPathValue());
    if (path.compare(0, config_->prefix.size(), config_->prefix))
      return Http::FilterHeadersStatus::Continue;
    active_ = true;
    if (h.getMethodValue() != "POST") {
      fail(Http::Code::MethodNotAllowed, "POST required");
      return Http::FilterHeadersStatus::StopIteration;
    }
    const auto route = config_->routes.find(path);
    if (route == config_->routes.end()) {
      fail(Http::Code::NotFound, "unknown PQ operation");
      return Http::FilterHeadersStatus::StopIteration;
    }
    request_.operation = route->second.operation;
    request_.algorithm = route->second.algorithm;
    request_.key_slot = route->second.slot;
    auto context = h.get(Http::LowerCaseString("x-pq-context"));
    if (!context.empty()) {
      if (context.size() != 1 || context[0]->value().size() > 510) {
        fail(Http::Code::BadRequest, "invalid PQ context");
        return Http::FilterHeadersStatus::StopIteration;
      }
      auto decoded = fpga::unhex(std::string(context[0]->value().getStringView()));
      if (!decoded || decoded->size() > 255) {
        fail(Http::Code::BadRequest, "invalid PQ context");
        return Http::FilterHeadersStatus::StopIteration;
      }
      request_.aad = std::move(*decoded);
    }
    if (end_stream)
      start();
    return Http::FilterHeadersStatus::StopIteration;
  }
  Http::FilterDataStatus decodeData(Buffer::Instance &data, bool end_stream) override {
    if (!active_)
      return Http::FilterDataStatus::Continue;
    if (!submitted_ && !replied_) {
      const size_t limit =
          request_.operation == fpga::Operation::PqVerify ? fpga::MaxPqWire : fpga::MaxPqData;
      if (data.length() > limit - request_.data.size()) {
        data.drain(data.length());
        fail(Http::Code::PayloadTooLarge, "PQ payload too large");
        return Http::FilterDataStatus::StopIterationNoBuffer;
      }
      const auto old = request_.data.size();
      request_.data.resize(old + data.length());
      if (data.length())
        data.copyOut(0, data.length(), request_.data.data() + old);
    }
    data.drain(data.length());
    if (end_stream && !replied_)
      start();
    return Http::FilterDataStatus::StopIterationNoBuffer;
  }
  Http::FilterTrailersStatus decodeTrailers(Http::RequestTrailerMap &) override {
    if (!active_)
      return Http::FilterTrailersStatus::Continue;
    if (!submitted_ && !replied_)
      start();
    return Http::FilterTrailersStatus::StopIteration;
  }

private:
  void fail(Http::Code code, const std::string &message) {
    replied_ = true;
    callbacks_->sendLocalReply(code, message, nullptr, absl::nullopt, "pq_crypto_error");
  }
  void start() {
    if (submitted_ || replied_)
      return;
    if (request_.operation == fpga::Operation::PqVerify) {
      auto parts = fpga::unpackParts(request_.data, 3);
      if (!parts) {
        fail(Http::Code::BadRequest, "verify body requires framed public key, signature, message");
        return;
      }
      request_.auxiliary = fpga::packParts({(*parts)[0], (*parts)[1]});
      fpga::wipe(request_.data);
      request_.data = std::move((*parts)[2]);
    }
    if (!fpga::valid(request_)) {
      fail(Http::Code::BadRequest, "invalid PQ request");
      return;
    }
    catalog_ = request_.operation == fpga::Operation::PqCatalog;
    submitted_ = true;
    deadline_ = fpga::Clock::now() + config_->timeout;
    ticket_ = config_->engine->submit(std::move(request_), config_->timeout);
    timer_ = callbacks_->dispatcher().createTimer([this] { poll(); });
    timer_->enableTimer(std::chrono::milliseconds(1));
  }
  void poll() {
    auto result = ticket_.take();
    if (!result) {
      if (fpga::Clock::now() >= deadline_) {
        ticket_.cancel();
        fail(Http::Code::GatewayTimeout, "deadline_exceeded");
        return;
      }
      timer_->enableTimer(std::chrono::milliseconds(1));
      return;
    }
    if (!*result) {
      const auto status =
          result->code == fpga::Code::Invalid || result->code == fpga::Code::AuthFailed
              ? Http::Code::BadRequest
          : result->code == fpga::Code::Timeout ? Http::Code::GatewayTimeout
                                                : Http::Code::ServiceUnavailable;
      fail(status, fpga::codeName(result->code));
      return;
    }
    std::string body;
    struct BodyWiper {
      std::string &body;
      ~BodyWiper() {
        volatile char *p = body.data();
        for (size_t i = 0; i < body.size(); ++i)
          p[i] = 0;
      }
    } body_wiper{body};
    if (catalog_) {
      auto algorithms = fpga::decodeCatalog(result->data);
      if (!algorithms) {
        fail(Http::Code::ServiceUnavailable, "invalid PQ catalog");
        return;
      }
      std::vector<fpga::PqAlgorithm> visible;
      for (const auto &a : *algorithms)
        if ((a.standardized && fpga::pqDefaultAllowed(a.name)) || config_->experimental)
          visible.push_back(a);
      body = fpga::catalogJson(visible, config_->backend);
    } else
      body.assign(result->data.begin(), result->data.end());
    fpga::wipe(result->data);
    replied_ = true;
    const auto backend = config_->backend;
    const bool json = catalog_;
    callbacks_->sendLocalReply(
        Http::Code::OK, body,
        [backend, json](Http::ResponseHeaderMap &h) {
          h.setContentType(json ? "application/json" : "application/octet-stream");
          h.setCopy(Http::LowerCaseString("cache-control"), "no-store");
          h.setCopy(Http::LowerCaseString("x-crypto-backend"), backend);
        },
        absl::nullopt, "pq_crypto_success");
  }
  std::shared_ptr<const Config> config_;
  Http::StreamDecoderFilterCallbacks *callbacks_{nullptr};
  fpga::Request request_;
  fpga::Ticket ticket_;
  Event::TimerPtr timer_;
  fpga::Clock::time_point deadline_;
  bool active_{false}, submitted_{false}, replied_{false}, catalog_{false};
};
class Factory final : public Server::Configuration::NamedHttpFilterConfigFactory {
public:
  std::string name() const override { return "envoy.filters.http.pq_crypto"; }
  ProtobufTypes::MessagePtr createEmptyConfigProto() override {
    return std::make_unique<envoy::extensions::fpga_crypto::v3::PqHttp>();
  }
  ProtobufTypes::MessagePtr createEmptyRouteConfigProto() override { return nullptr; }
  absl::StatusOr<Http::FilterFactoryCb>
  createFilterFactoryFromProto(const Protobuf::Message &s, const std::string &,
                               Server::Configuration::FactoryContext &) override {
    return create(s);
  }
  absl::StatusOr<Http::FilterFactoryCb>
  createHttpFilterFactoryFromProto(const Protobuf::Message &s, const std::string &,
                                   Server::Configuration::ServerFactoryContext &) override {
    return create(s);
  }

private:
  static Http::FilterFactoryCb create(const Protobuf::Message &s) {
    const auto *typed =
        Protobuf::DynamicCastMessage<envoy::extensions::fpga_crypto::v3::PqHttp>(&s);
    if (!typed || !typed->has_value())
      throw EnvoyException("PQ filter requires PqHttp.value");
    std::shared_ptr<Config> config;
    try {
      config = std::make_shared<Config>(typed->value());
    } catch (const std::exception &e) {
      throw EnvoyException(e.what());
    }
    return [config](Http::FilterChainFactoryCallbacks &c) {
      c.addStreamDecoderFilter(std::make_shared<Filter>(config));
    };
  }
};
} // namespace
REGISTER_FACTORY(Factory, Server::Configuration::NamedHttpFilterConfigFactory);
} // namespace Envoy::Extensions::PqCrypto
