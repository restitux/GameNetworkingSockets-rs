#ifndef __GNS_H_
#define __GNS_H_

#include "steamnetworkingsockets.h"
#include "isteamnetworkingsockets.h"
#include "steamnetworkingtypes.h"

#include "gamenetworkingsockets-sys/src/lib.rs.h"

typedef void c_void;
typedef long long int c_longlong;
typedef int c_uint;

HSteamListenSocket HSteamListenSocket_Invalid = k_HSteamNetPollGroup_Invalid;
HSteamNetPollGroup HSteamNetPollGroup_Invalid = k_HSteamNetPollGroup_Invalid;


InitReturn GameNetworkingSockets_Init_rs() {
    SteamNetworkingErrMsg errMsg = "";
    bool ret = GameNetworkingSockets_Init(nullptr, errMsg);
    return (InitReturn) {
      ret,
      rust::String(errMsg),
    };
}

#endif // __GNS_H_
