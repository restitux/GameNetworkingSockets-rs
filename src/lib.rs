use autocxx::include_cpp;

include_cpp! {
    #include "steamnetworkingsockets.h"
    #include "isteamnetworkingsockets.h"
    safety!(unsafe)
    generate!("GameNetworkingSockets_Init")
    generate!("GameNetworkingSockets_Kill")
    generate!("ISteamNetworkingSockets")
    generate!("SteamNetworkingSockets")
}
