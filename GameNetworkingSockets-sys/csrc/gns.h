#ifndef __GNS_H_
#define __GNS_H_

#include <memory>
#include <vector>

#include "steamnetworkingsockets.h"
#include "isteamnetworkingsockets.h"
#include "steamnetworkingtypes.h"

#include "gamenetworkingsockets-sys/src/lib.rs.h"

typedef void c_void;
typedef long long int c_longlong;
typedef int c_uint;
typedef char c_char;

HSteamListenSocket HSteamListenSocket_Invalid = k_HSteamNetPollGroup_Invalid;
HSteamNetConnection HSteamNetConnection_Invalid = k_HSteamNetConnection_Invalid;
HSteamNetPollGroup HSteamNetPollGroup_Invalid = k_HSteamNetPollGroup_Invalid;

InitReturn GameNetworkingSockets_Init_rs() {
    SteamNetworkingErrMsg errMsg = "";
    bool ret = GameNetworkingSockets_Init(nullptr, errMsg);
    return (InitReturn) {
      ret,
      rust::String(errMsg),
    };
}

std::unique_ptr<SteamNetworkingIPAddr> new_SteamNetworkingIPAddr() {
  return std::make_unique<SteamNetworkingIPAddr>();
}

std::unique_ptr<SteamNetworkingConfigValue_t> new_SteamNetworkingConfigValue_t() {
  return std::make_unique<SteamNetworkingConfigValue_t>();
}

std::unique_ptr<std::vector<SteamNetworkingConfigValue_t>> new_SteamNetworkingConfigValue_t_Vector() {
  return std::make_unique<std::vector<SteamNetworkingConfigValue_t>>();
}

void SteamNetworkingConfigValue_t_Vector_push(const std::unique_ptr<std::vector<SteamNetworkingConfigValue_t>> &vec, std::unique_ptr<SteamNetworkingConfigValue_t> val) {
  vec->push_back(*val);
}



HSteamListenSocket CreateListenSocketIP_Vector(ISteamNetworkingSockets *sns, const SteamNetworkingIPAddr &localAddress, const std::unique_ptr<std::vector<SteamNetworkingConfigValue_t>> &Options) {
  return sns->CreateListenSocketIP(localAddress, Options->size(), Options->data());
}

HSteamNetConnection ConnectByIPAddress_Vector(ISteamNetworkingSockets *sns, const SteamNetworkingIPAddr &localAddress, const std::unique_ptr<std::vector<SteamNetworkingConfigValue_t>> &Options) {
  return sns->ConnectByIPAddress(localAddress, Options->size(), Options->data());
}

#endif // __GNS_H_
