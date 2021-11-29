#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod ffi {
    pub trait ToCppString {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString>;
    }
    impl ToCppString for &str {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(&self)
        }
    }
    impl ToCppString for &String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for cxx::UniquePtr<cxx::CxxString> {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            self
        }
    }
    unsafe impl cxx::ExternType for bindgen::root::EResult {
        type Id = cxx::type_id!("EResult");
        type Kind = cxx::kind::Trivial;
    }
    unsafe impl cxx::ExternType for bindgen::root::ESteamNetworkingAvailability {
        type Id = cxx::type_id!("ESteamNetworkingAvailability");
        type Kind = cxx::kind::Trivial;
    }
    mod bindgen {
        pub(super) mod root {
            pub use cxxbridge::ISteamNetworkingSockets;
            pub use cxxbridge::SteamNetworkingConfigValue_t;
            pub use cxxbridge::SteamNetworkingIPAddr;
            #[doc = " Handle used to identify a \"listen socket\".  Unlike traditional"]
            #[doc = " Berkeley sockets, a listen socket and a connection are two"]
            #[doc = " different abstractions."]
            pub type HSteamListenSocket = root::uint32;
            #[doc = " Handle used to identify a connection to a remote host."]
            pub type HSteamNetConnection = root::uint32;
            pub use cxxbridge::SteamNetworkingIdentity;
            #[repr(u32)]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum EResult {
                k_EResultOK = 1,
                k_EResultFail = 2,
                k_EResultNoConnection = 3,
                k_EResultInvalidPassword = 5,
                k_EResultLoggedInElsewhere = 6,
                k_EResultInvalidProtocolVer = 7,
                k_EResultInvalidParam = 8,
                k_EResultFileNotFound = 9,
                k_EResultBusy = 10,
                k_EResultInvalidState = 11,
                k_EResultInvalidName = 12,
                k_EResultInvalidEmail = 13,
                k_EResultDuplicateName = 14,
                k_EResultAccessDenied = 15,
                k_EResultTimeout = 16,
                k_EResultBanned = 17,
                k_EResultAccountNotFound = 18,
                k_EResultInvalidSteamID = 19,
                k_EResultServiceUnavailable = 20,
                k_EResultNotLoggedOn = 21,
                k_EResultPending = 22,
                k_EResultEncryptionFailure = 23,
                k_EResultInsufficientPrivilege = 24,
                k_EResultLimitExceeded = 25,
                k_EResultRevoked = 26,
                k_EResultExpired = 27,
                k_EResultAlreadyRedeemed = 28,
                k_EResultDuplicateRequest = 29,
                k_EResultAlreadyOwned = 30,
                k_EResultIPNotFound = 31,
                k_EResultPersistFailed = 32,
                k_EResultLockingFailed = 33,
                k_EResultLogonSessionReplaced = 34,
                k_EResultConnectFailed = 35,
                k_EResultHandshakeFailed = 36,
                k_EResultIOFailure = 37,
                k_EResultRemoteDisconnect = 38,
                k_EResultShoppingCartNotFound = 39,
                k_EResultBlocked = 40,
                k_EResultIgnored = 41,
                k_EResultNoMatch = 42,
                k_EResultAccountDisabled = 43,
                k_EResultServiceReadOnly = 44,
                k_EResultAccountNotFeatured = 45,
                k_EResultAdministratorOK = 46,
                k_EResultContentVersion = 47,
                k_EResultTryAnotherCM = 48,
                k_EResultPasswordRequiredToKickSession = 49,
                k_EResultAlreadyLoggedInElsewhere = 50,
                k_EResultSuspended = 51,
                k_EResultCancelled = 52,
                k_EResultDataCorruption = 53,
                k_EResultDiskFull = 54,
                k_EResultRemoteCallFailed = 55,
                k_EResultPasswordUnset = 56,
                k_EResultExternalAccountUnlinked = 57,
                k_EResultPSNTicketInvalid = 58,
                k_EResultExternalAccountAlreadyLinked = 59,
                k_EResultRemoteFileConflict = 60,
                k_EResultIllegalPassword = 61,
                k_EResultSameAsPreviousValue = 62,
                k_EResultAccountLogonDenied = 63,
                k_EResultCannotUseOldPassword = 64,
                k_EResultInvalidLoginAuthCode = 65,
                k_EResultAccountLogonDeniedNoMail = 66,
                k_EResultHardwareNotCapableOfIPT = 67,
                k_EResultIPTInitError = 68,
                k_EResultParentalControlRestricted = 69,
                k_EResultFacebookQueryError = 70,
                k_EResultExpiredLoginAuthCode = 71,
                k_EResultIPLoginRestrictionFailed = 72,
                k_EResultAccountLockedDown = 73,
                k_EResultAccountLogonDeniedVerifiedEmailRequired = 74,
                k_EResultNoMatchingURL = 75,
                k_EResultBadResponse = 76,
                k_EResultRequirePasswordReEntry = 77,
                k_EResultValueOutOfRange = 78,
                k_EResultUnexpectedError = 79,
                k_EResultDisabled = 80,
                k_EResultInvalidCEGSubmission = 81,
                k_EResultRestrictedDevice = 82,
                k_EResultRegionLocked = 83,
                k_EResultRateLimitExceeded = 84,
                k_EResultAccountLoginDeniedNeedTwoFactor = 85,
                k_EResultItemDeleted = 86,
                k_EResultAccountLoginDeniedThrottle = 87,
                k_EResultTwoFactorCodeMismatch = 88,
                k_EResultTwoFactorActivationCodeMismatch = 89,
                k_EResultAccountAssociatedToMultiplePartners = 90,
                k_EResultNotModified = 91,
                k_EResultNoMobileDevice = 92,
                k_EResultTimeNotSynced = 93,
                k_EResultSmsCodeFailed = 94,
                k_EResultAccountLimitExceeded = 95,
                k_EResultAccountActivityLimitExceeded = 96,
                k_EResultPhoneActivityLimitExceeded = 97,
                k_EResultRefundToWallet = 98,
                k_EResultEmailSendFailure = 99,
                k_EResultNotSettled = 100,
                k_EResultNeedCaptcha = 101,
                k_EResultGSLTDenied = 102,
                k_EResultGSOwnerDenied = 103,
                k_EResultInvalidItemType = 104,
                k_EResultIPBanned = 105,
                k_EResultGSLTExpired = 106,
                k_EResultInsufficientFunds = 107,
                k_EResultTooManyPending = 108,
                k_EResultNoSiteLicensesFound = 109,
                k_EResultWGNetworkSendExceeded = 110,
            }
            pub type int64 = autocxx::c_longlong;
            pub type uint32 = autocxx::c_uint;
            pub use cxxbridge::SteamNetConnectionInfo_t;
            pub use cxxbridge::SteamNetworkingQuickConnectionStatus;
            #[repr(i32)]
            #[doc = " Describe the status of a particular network resource"]
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
            pub enum ESteamNetworkingAvailability {
                k_ESteamNetworkingAvailability_CannotTry = -102,
                k_ESteamNetworkingAvailability_Failed = -101,
                k_ESteamNetworkingAvailability_Previously = -100,
                k_ESteamNetworkingAvailability_Retrying = -10,
                k_ESteamNetworkingAvailability_NeverTried = 1,
                k_ESteamNetworkingAvailability_Waiting = 2,
                k_ESteamNetworkingAvailability_Attempting = 3,
                k_ESteamNetworkingAvailability_Current = 100,
                k_ESteamNetworkingAvailability_Unknown = 0,
                k_ESteamNetworkingAvailability__Force32bit = 2147483647,
            }
            pub use cxxbridge::SteamNetAuthenticationStatus_t;
            #[doc = " Handle used to identify a poll group, used to query many"]
            #[doc = " connections at once efficiently."]
            pub type HSteamNetPollGroup = root::uint32;
            pub use cxxbridge::SteamDatagramRelayAuthTicket;
            pub type uint16 = autocxx::c_ushort;
            #[doc = " Identifier used for a network location point of presence.  (E.g. a Valve data center.)"]
            #[doc = " Typically you won't need to directly manipulate these."]
            pub type SteamNetworkingPOPID = root::uint32;
            pub use cxxbridge::ISteamNetworkingConnectionSignaling;
            pub use cxxbridge::ISteamNetworkingSignalingRecvContext;
            pub use cxxbridge::SteamDatagramGameCoordinatorServerLogin;
            pub use cxxbridge::SteamDatagramHostedAddress;
            impl ISteamNetworkingSockets {
                #[doc = "autocxx bindings couldn't be generated: Pointer pointed to something unsupported"]
                fn SendMessages(_uhoh: autocxx::BindingGenerationFailure) {}
                #[doc = "autocxx bindings couldn't be generated: Pointer pointed to something unsupported"]
                fn ReceiveMessagesOnConnection(_uhoh: autocxx::BindingGenerationFailure) {}
                #[doc = "autocxx bindings couldn't be generated: Pointer pointed to something unsupported"]
                fn ReceiveMessagesOnPollGroup(_uhoh: autocxx::BindingGenerationFailure) {}
                #[doc = " Creates a \"server\" socket that listens for clients to connect to by"]
                pub unsafe fn CreateListenSocketIP(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    localAddress: &root::SteamNetworkingIPAddr,
                    nOptions: autocxx::c_int,
                    pOptions: *const root::SteamNetworkingConfigValue_t,
                ) -> autocxx::c_uint {
                    cxxbridge::CreateListenSocketIP_autocxx_wrapper(
                        self,
                        localAddress,
                        nOptions,
                        pOptions,
                    )
                }
                #[doc = " Creates a connection and begins talking to a \"server\" over UDP at the"]
                pub unsafe fn ConnectByIPAddress(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    address: &root::SteamNetworkingIPAddr,
                    nOptions: autocxx::c_int,
                    pOptions: *const root::SteamNetworkingConfigValue_t,
                ) -> autocxx::c_uint {
                    cxxbridge::ConnectByIPAddress_autocxx_wrapper(self, address, nOptions, pOptions)
                }
                #[doc = " Like CreateListenSocketIP, but clients will connect using ConnectP2P."]
                pub unsafe fn CreateListenSocketP2P(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    nLocalVirtualPort: autocxx::c_int,
                    nOptions: autocxx::c_int,
                    pOptions: *const root::SteamNetworkingConfigValue_t,
                ) -> autocxx::c_uint {
                    cxxbridge::CreateListenSocketP2P_autocxx_wrapper(
                        self,
                        nLocalVirtualPort,
                        nOptions,
                        pOptions,
                    )
                }
                #[doc = " Begin connecting to a peer that is identified using a platform-specific identifier."]
                pub unsafe fn ConnectP2P(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    identityRemote: &root::SteamNetworkingIdentity,
                    nRemoteVirtualPort: autocxx::c_int,
                    nOptions: autocxx::c_int,
                    pOptions: *const root::SteamNetworkingConfigValue_t,
                ) -> autocxx::c_uint {
                    cxxbridge::ConnectP2P_autocxx_wrapper(
                        self,
                        identityRemote,
                        nRemoteVirtualPort,
                        nOptions,
                        pOptions,
                    )
                }
                #[doc = " Accept an incoming connection that has been received on a listen socket."]
                pub fn AcceptConnection(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hConn: autocxx::c_uint,
                ) -> root::EResult {
                    cxxbridge::AcceptConnection_autocxx_wrapper(self, hConn)
                }
                #[doc = " Disconnects from the remote host and invalidates the connection handle."]
                pub unsafe fn CloseConnection(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hPeer: autocxx::c_uint,
                    nReason: autocxx::c_int,
                    pszDebug: *const ::std::os::raw::c_char,
                    bEnableLinger: bool,
                ) -> bool {
                    cxxbridge::CloseConnection_autocxx_wrapper(
                        self,
                        hPeer,
                        nReason,
                        pszDebug,
                        bEnableLinger,
                    )
                }
                #[doc = " Destroy a listen socket.  All the connections that were accepting on the listen"]
                pub fn CloseListenSocket(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hSocket: autocxx::c_uint,
                ) -> bool {
                    cxxbridge::CloseListenSocket_autocxx_wrapper(self, hSocket)
                }
                #[doc = " Set connection user data.  the data is returned in the following places"]
                pub fn SetConnectionUserData(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hPeer: autocxx::c_uint,
                    nUserData: autocxx::c_longlong,
                ) -> bool {
                    cxxbridge::SetConnectionUserData_autocxx_wrapper(self, hPeer, nUserData)
                }
                #[doc = " Fetch connection user data.  Returns -1 if handle is invalid"]
                pub fn GetConnectionUserData(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hPeer: autocxx::c_uint,
                ) -> autocxx::c_longlong {
                    cxxbridge::GetConnectionUserData_autocxx_wrapper(self, hPeer)
                }
                #[doc = " Set a name for the connection, used mostly for debugging"]
                pub unsafe fn SetConnectionName(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hPeer: autocxx::c_uint,
                    pszName: *const ::std::os::raw::c_char,
                ) {
                    cxxbridge::SetConnectionName_autocxx_wrapper(self, hPeer, pszName)
                }
                #[doc = " Fetch connection name.  Returns false if handle is invalid"]
                pub unsafe fn GetConnectionName(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hPeer: autocxx::c_uint,
                    pszName: *mut ::std::os::raw::c_char,
                    nMaxLen: autocxx::c_int,
                ) -> bool {
                    cxxbridge::GetConnectionName_autocxx_wrapper(self, hPeer, pszName, nMaxLen)
                }
                #[doc = " Send a message to the remote host on the specified connection."]
                pub unsafe fn SendMessageToConnection(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hConn: autocxx::c_uint,
                    pData: *const autocxx::c_void,
                    cbData: autocxx::c_uint,
                    nSendFlags: autocxx::c_int,
                    pOutMessageNumber: *mut autocxx::c_longlong,
                ) -> root::EResult {
                    cxxbridge::SendMessageToConnection_autocxx_wrapper(
                        self,
                        hConn,
                        pData,
                        cbData,
                        nSendFlags,
                        pOutMessageNumber,
                    )
                }
                #[doc = " Flush any messages waiting on the Nagle timer and send them"]
                pub fn FlushMessagesOnConnection(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hConn: autocxx::c_uint,
                ) -> root::EResult {
                    cxxbridge::FlushMessagesOnConnection_autocxx_wrapper(self, hConn)
                }
                #[doc = " Returns basic information about the high-level state of the connection."]
                pub unsafe fn GetConnectionInfo(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hConn: autocxx::c_uint,
                    pInfo: *mut root::SteamNetConnectionInfo_t,
                ) -> bool {
                    cxxbridge::GetConnectionInfo_autocxx_wrapper(self, hConn, pInfo)
                }
                #[doc = " Returns a small set of information about the real-time state of the connection"]
                pub unsafe fn GetQuickConnectionStatus(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hConn: autocxx::c_uint,
                    pStats: *mut root::SteamNetworkingQuickConnectionStatus,
                ) -> bool {
                    cxxbridge::GetQuickConnectionStatus_autocxx_wrapper(self, hConn, pStats)
                }
                #[doc = " Returns detailed connection stats in text format.  Useful"]
                pub unsafe fn GetDetailedConnectionStatus(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hConn: autocxx::c_uint,
                    pszBuf: *mut ::std::os::raw::c_char,
                    cbBuf: autocxx::c_int,
                ) -> autocxx::c_int {
                    cxxbridge::GetDetailedConnectionStatus_autocxx_wrapper(
                        self, hConn, pszBuf, cbBuf,
                    )
                }
                #[doc = " Returns local IP and port that a listen socket created using CreateListenSocketIP is bound to."]
                pub unsafe fn GetListenSocketAddress(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hSocket: autocxx::c_uint,
                    address: *mut root::SteamNetworkingIPAddr,
                ) -> bool {
                    cxxbridge::GetListenSocketAddress_autocxx_wrapper(self, hSocket, address)
                }
                #[doc = " Create a pair of connections that are talking to each other, e.g. a loopback connection."]
                pub unsafe fn CreateSocketPair(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pOutConnection1: *mut autocxx::c_uint,
                    pOutConnection2: *mut autocxx::c_uint,
                    bUseNetworkLoopback: bool,
                    pIdentity1: *const root::SteamNetworkingIdentity,
                    pIdentity2: *const root::SteamNetworkingIdentity,
                ) -> bool {
                    cxxbridge::CreateSocketPair_autocxx_wrapper(
                        self,
                        pOutConnection1,
                        pOutConnection2,
                        bUseNetworkLoopback,
                        pIdentity1,
                        pIdentity2,
                    )
                }
                #[doc = " Get the identity assigned to this interface."]
                pub unsafe fn GetIdentity(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pIdentity: *mut root::SteamNetworkingIdentity,
                ) -> bool {
                    cxxbridge::GetIdentity_autocxx_wrapper(self, pIdentity)
                }
                #[doc = " Indicate our desire to be ready participate in authenticated communications."]
                pub fn InitAuthentication(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                ) -> root::ESteamNetworkingAvailability {
                    cxxbridge::InitAuthentication_autocxx_wrapper(self)
                }
                #[doc = " Query our readiness to participate in authenticated communications.  A"]
                pub unsafe fn GetAuthenticationStatus(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pDetails: *mut root::SteamNetAuthenticationStatus_t,
                ) -> root::ESteamNetworkingAvailability {
                    cxxbridge::GetAuthenticationStatus_autocxx_wrapper(self, pDetails)
                }
                #[doc = " Create a new poll group."]
                pub fn CreatePollGroup(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                ) -> autocxx::c_uint {
                    cxxbridge::CreatePollGroup_autocxx_wrapper(self)
                }
                #[doc = " Destroy a poll group created with CreatePollGroup()."]
                pub fn DestroyPollGroup(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hPollGroup: autocxx::c_uint,
                ) -> bool {
                    cxxbridge::DestroyPollGroup_autocxx_wrapper(self, hPollGroup)
                }
                #[doc = " Assign a connection to a poll group.  Note that a connection may only belong to a"]
                pub fn SetConnectionPollGroup(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    hConn: autocxx::c_uint,
                    hPollGroup: autocxx::c_uint,
                ) -> bool {
                    cxxbridge::SetConnectionPollGroup_autocxx_wrapper(self, hConn, hPollGroup)
                }
                #[doc = " Call this when you receive a ticket from your backend / matchmaking system.  Puts the"]
                pub unsafe fn ReceivedRelayAuthTicket(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pvTicket: *const autocxx::c_void,
                    cbTicket: autocxx::c_int,
                    pOutParsedTicket: *mut root::SteamDatagramRelayAuthTicket,
                ) -> bool {
                    cxxbridge::ReceivedRelayAuthTicket_autocxx_wrapper(
                        self,
                        pvTicket,
                        cbTicket,
                        pOutParsedTicket,
                    )
                }
                #[doc = " Search cache for a ticket to talk to the server on the specified virtual port."]
                pub unsafe fn FindRelayAuthTicketForServer(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    identityGameServer: &root::SteamNetworkingIdentity,
                    nRemoteVirtualPort: autocxx::c_int,
                    pOutParsedTicket: *mut root::SteamDatagramRelayAuthTicket,
                ) -> autocxx::c_int {
                    cxxbridge::FindRelayAuthTicketForServer_autocxx_wrapper(
                        self,
                        identityGameServer,
                        nRemoteVirtualPort,
                        pOutParsedTicket,
                    )
                }
                #[doc = " Client call to connect to a server hosted in a Valve data center, on the specified virtual"]
                pub unsafe fn ConnectToHostedDedicatedServer(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    identityTarget: &root::SteamNetworkingIdentity,
                    nRemoteVirtualPort: autocxx::c_int,
                    nOptions: autocxx::c_int,
                    pOptions: *const root::SteamNetworkingConfigValue_t,
                ) -> autocxx::c_uint {
                    cxxbridge::ConnectToHostedDedicatedServer_autocxx_wrapper(
                        self,
                        identityTarget,
                        nRemoteVirtualPort,
                        nOptions,
                        pOptions,
                    )
                }
                #[doc = " Returns the value of the SDR_LISTEN_PORT environment variable.  This"]
                pub fn GetHostedDedicatedServerPort(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                ) -> autocxx::c_ushort {
                    cxxbridge::GetHostedDedicatedServerPort_autocxx_wrapper(self)
                }
                #[doc = " Returns 0 if SDR_LISTEN_PORT is not set.  Otherwise, returns the data center the server"]
                pub fn GetHostedDedicatedServerPOPID(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                ) -> autocxx::c_uint {
                    cxxbridge::GetHostedDedicatedServerPOPID_autocxx_wrapper(self)
                }
                #[doc = " Return info about the hosted server.  This contains the PoPID of the server,"]
                pub unsafe fn GetHostedDedicatedServerAddress(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pRouting: *mut root::SteamDatagramHostedAddress,
                ) -> root::EResult {
                    cxxbridge::GetHostedDedicatedServerAddress_autocxx_wrapper(self, pRouting)
                }
                #[doc = " Create a listen socket on the specified virtual port.  The physical UDP port to use"]
                pub unsafe fn CreateHostedDedicatedServerListenSocket(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    nLocalVirtualPort: autocxx::c_int,
                    nOptions: autocxx::c_int,
                    pOptions: *const root::SteamNetworkingConfigValue_t,
                ) -> autocxx::c_uint {
                    cxxbridge::CreateHostedDedicatedServerListenSocket_autocxx_wrapper(
                        self,
                        nLocalVirtualPort,
                        nOptions,
                        pOptions,
                    )
                }
                #[doc = " Generate an authentication blob that can be used to securely login with"]
                pub unsafe fn GetGameCoordinatorServerLogin(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pLoginInfo: *mut root::SteamDatagramGameCoordinatorServerLogin,
                    pcbSignedBlob: *mut autocxx::c_int,
                    pBlob: *mut autocxx::c_void,
                ) -> root::EResult {
                    cxxbridge::GetGameCoordinatorServerLogin_autocxx_wrapper(
                        self,
                        pLoginInfo,
                        pcbSignedBlob,
                        pBlob,
                    )
                }
                #[doc = " Create a P2P \"client\" connection that does signaling over a custom"]
                pub unsafe fn ConnectP2PCustomSignaling(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pSignaling: *mut root::ISteamNetworkingConnectionSignaling,
                    pPeerIdentity: *const root::SteamNetworkingIdentity,
                    nRemoteVirtualPort: autocxx::c_int,
                    nOptions: autocxx::c_int,
                    pOptions: *const root::SteamNetworkingConfigValue_t,
                ) -> autocxx::c_uint {
                    cxxbridge::ConnectP2PCustomSignaling_autocxx_wrapper(
                        self,
                        pSignaling,
                        pPeerIdentity,
                        nRemoteVirtualPort,
                        nOptions,
                        pOptions,
                    )
                }
                #[doc = " Called when custom signaling has received a message.  When your"]
                pub unsafe fn ReceivedP2PCustomSignal(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pMsg: *const autocxx::c_void,
                    cbMsg: autocxx::c_int,
                    pContext: *mut root::ISteamNetworkingSignalingRecvContext,
                ) -> bool {
                    cxxbridge::ReceivedP2PCustomSignal_autocxx_wrapper(self, pMsg, cbMsg, pContext)
                }
                #[doc = " Invoke all callback functions queued for this interface."]
                pub fn RunCallbacks(self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>) {
                    cxxbridge::RunCallbacks_autocxx_wrapper(self)
                }
                #[doc = " Reset the identity associated with this instance."]
                pub unsafe fn ResetIdentity(
                    self: ::std::pin::Pin<&mut root::ISteamNetworkingSockets>,
                    pIdentity: *const root::SteamNetworkingIdentity,
                ) {
                    cxxbridge::ResetIdentity_autocxx_wrapper(self, pIdentity)
                }
            }
            #[allow(unused_imports)]
            use self::super::super::cxxbridge;
            #[allow(unused_imports)]
            use self::super::super::ToCppString;
            #[allow(unused_imports)]
            use self::super::root;
        }
    }
    #[cxx::bridge]
    mod cxxbridge {
        impl UniquePtr<EResult> {}
        impl SharedPtr<EResult> {}
        impl WeakPtr<EResult> {}
        impl CxxVector<EResult> {}
        impl UniquePtr<ESteamNetworkingAvailability> {}
        impl SharedPtr<ESteamNetworkingAvailability> {}
        impl WeakPtr<ESteamNetworkingAvailability> {}
        impl CxxVector<ESteamNetworkingAvailability> {}
        unsafe extern "C++" {
            fn autocxx_make_string_default(str_: &str) -> UniquePtr<CxxString>;
            #[doc = " Lower level networking API."]
            type ISteamNetworkingSockets;
            #[doc = " Creates a \"server\" socket that listens for clients to connect to by"]
            pub unsafe fn CreateListenSocketIP_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                localAddress: &SteamNetworkingIPAddr,
                nOptions: c_int,
                pOptions: *const SteamNetworkingConfigValue_t,
            ) -> c_uint;
            #[doc = " Creates a connection and begins talking to a \"server\" over UDP at the"]
            pub unsafe fn ConnectByIPAddress_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                address: &SteamNetworkingIPAddr,
                nOptions: c_int,
                pOptions: *const SteamNetworkingConfigValue_t,
            ) -> c_uint;
            #[doc = " Like CreateListenSocketIP, but clients will connect using ConnectP2P."]
            pub unsafe fn CreateListenSocketP2P_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                nLocalVirtualPort: c_int,
                nOptions: c_int,
                pOptions: *const SteamNetworkingConfigValue_t,
            ) -> c_uint;
            #[doc = " Begin connecting to a peer that is identified using a platform-specific identifier."]
            pub unsafe fn ConnectP2P_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                identityRemote: &SteamNetworkingIdentity,
                nRemoteVirtualPort: c_int,
                nOptions: c_int,
                pOptions: *const SteamNetworkingConfigValue_t,
            ) -> c_uint;
            #[doc = " Accept an incoming connection that has been received on a listen socket."]
            pub fn AcceptConnection_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hConn: c_uint,
            ) -> EResult;
            #[doc = " Disconnects from the remote host and invalidates the connection handle."]
            pub unsafe fn CloseConnection_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hPeer: c_uint,
                nReason: c_int,
                pszDebug: *const c_char,
                bEnableLinger: bool,
            ) -> bool;
            #[doc = " Destroy a listen socket.  All the connections that were accepting on the listen"]
            pub fn CloseListenSocket_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hSocket: c_uint,
            ) -> bool;
            #[doc = " Set connection user data.  the data is returned in the following places"]
            pub fn SetConnectionUserData_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hPeer: c_uint,
                nUserData: c_longlong,
            ) -> bool;
            #[doc = " Fetch connection user data.  Returns -1 if handle is invalid"]
            pub fn GetConnectionUserData_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hPeer: c_uint,
            ) -> c_longlong;
            #[doc = " Set a name for the connection, used mostly for debugging"]
            pub unsafe fn SetConnectionName_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hPeer: c_uint,
                pszName: *const c_char,
            );
            #[doc = " Fetch connection name.  Returns false if handle is invalid"]
            pub unsafe fn GetConnectionName_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hPeer: c_uint,
                pszName: *mut c_char,
                nMaxLen: c_int,
            ) -> bool;
            #[doc = " Send a message to the remote host on the specified connection."]
            pub unsafe fn SendMessageToConnection_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hConn: c_uint,
                pData: *const c_void,
                cbData: c_uint,
                nSendFlags: c_int,
                pOutMessageNumber: *mut c_longlong,
            ) -> EResult;
            #[doc = " Flush any messages waiting on the Nagle timer and send them"]
            pub fn FlushMessagesOnConnection_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hConn: c_uint,
            ) -> EResult;
            #[doc = " Returns basic information about the high-level state of the connection."]
            pub unsafe fn GetConnectionInfo_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hConn: c_uint,
                pInfo: *mut SteamNetConnectionInfo_t,
            ) -> bool;
            #[doc = " Returns a small set of information about the real-time state of the connection"]
            pub unsafe fn GetQuickConnectionStatus_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hConn: c_uint,
                pStats: *mut SteamNetworkingQuickConnectionStatus,
            ) -> bool;
            #[doc = " Returns detailed connection stats in text format.  Useful"]
            pub unsafe fn GetDetailedConnectionStatus_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hConn: c_uint,
                pszBuf: *mut c_char,
                cbBuf: c_int,
            ) -> c_int;
            #[doc = " Returns local IP and port that a listen socket created using CreateListenSocketIP is bound to."]
            pub unsafe fn GetListenSocketAddress_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hSocket: c_uint,
                address: *mut SteamNetworkingIPAddr,
            ) -> bool;
            #[doc = " Create a pair of connections that are talking to each other, e.g. a loopback connection."]
            pub unsafe fn CreateSocketPair_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pOutConnection1: *mut c_uint,
                pOutConnection2: *mut c_uint,
                bUseNetworkLoopback: bool,
                pIdentity1: *const SteamNetworkingIdentity,
                pIdentity2: *const SteamNetworkingIdentity,
            ) -> bool;
            #[doc = " Get the identity assigned to this interface."]
            pub unsafe fn GetIdentity_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pIdentity: *mut SteamNetworkingIdentity,
            ) -> bool;
            #[doc = " Indicate our desire to be ready participate in authenticated communications."]
            pub fn InitAuthentication_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
            ) -> ESteamNetworkingAvailability;
            #[doc = " Query our readiness to participate in authenticated communications.  A"]
            pub unsafe fn GetAuthenticationStatus_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pDetails: *mut SteamNetAuthenticationStatus_t,
            ) -> ESteamNetworkingAvailability;
            #[doc = " Create a new poll group."]
            pub fn CreatePollGroup_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
            ) -> c_uint;
            #[doc = " Destroy a poll group created with CreatePollGroup()."]
            pub fn DestroyPollGroup_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hPollGroup: c_uint,
            ) -> bool;
            #[doc = " Assign a connection to a poll group.  Note that a connection may only belong to a"]
            pub fn SetConnectionPollGroup_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                hConn: c_uint,
                hPollGroup: c_uint,
            ) -> bool;
            #[doc = " Call this when you receive a ticket from your backend / matchmaking system.  Puts the"]
            pub unsafe fn ReceivedRelayAuthTicket_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pvTicket: *const c_void,
                cbTicket: c_int,
                pOutParsedTicket: *mut SteamDatagramRelayAuthTicket,
            ) -> bool;
            #[doc = " Search cache for a ticket to talk to the server on the specified virtual port."]
            pub unsafe fn FindRelayAuthTicketForServer_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                identityGameServer: &SteamNetworkingIdentity,
                nRemoteVirtualPort: c_int,
                pOutParsedTicket: *mut SteamDatagramRelayAuthTicket,
            ) -> c_int;
            #[doc = " Client call to connect to a server hosted in a Valve data center, on the specified virtual"]
            pub unsafe fn ConnectToHostedDedicatedServer_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                identityTarget: &SteamNetworkingIdentity,
                nRemoteVirtualPort: c_int,
                nOptions: c_int,
                pOptions: *const SteamNetworkingConfigValue_t,
            ) -> c_uint;
            #[doc = " Returns the value of the SDR_LISTEN_PORT environment variable.  This"]
            pub fn GetHostedDedicatedServerPort_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
            ) -> c_ushort;
            #[doc = " Returns 0 if SDR_LISTEN_PORT is not set.  Otherwise, returns the data center the server"]
            pub fn GetHostedDedicatedServerPOPID_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
            ) -> c_uint;
            #[doc = " Return info about the hosted server.  This contains the PoPID of the server,"]
            pub unsafe fn GetHostedDedicatedServerAddress_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pRouting: *mut SteamDatagramHostedAddress,
            ) -> EResult;
            #[doc = " Create a listen socket on the specified virtual port.  The physical UDP port to use"]
            pub unsafe fn CreateHostedDedicatedServerListenSocket_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                nLocalVirtualPort: c_int,
                nOptions: c_int,
                pOptions: *const SteamNetworkingConfigValue_t,
            ) -> c_uint;
            #[doc = " Generate an authentication blob that can be used to securely login with"]
            pub unsafe fn GetGameCoordinatorServerLogin_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pLoginInfo: *mut SteamDatagramGameCoordinatorServerLogin,
                pcbSignedBlob: *mut c_int,
                pBlob: *mut c_void,
            ) -> EResult;
            #[doc = " Create a P2P \"client\" connection that does signaling over a custom"]
            pub unsafe fn ConnectP2PCustomSignaling_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pSignaling: *mut ISteamNetworkingConnectionSignaling,
                pPeerIdentity: *const SteamNetworkingIdentity,
                nRemoteVirtualPort: c_int,
                nOptions: c_int,
                pOptions: *const SteamNetworkingConfigValue_t,
            ) -> c_uint;
            #[doc = " Called when custom signaling has received a message.  When your"]
            pub unsafe fn ReceivedP2PCustomSignal_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pMsg: *const c_void,
                cbMsg: c_int,
                pContext: *mut ISteamNetworkingSignalingRecvContext,
            ) -> bool;
            #[doc = " Invoke all callback functions queued for this interface."]
            pub fn RunCallbacks_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
            );
            #[doc = " Reset the identity associated with this instance."]
            pub unsafe fn ResetIdentity_autocxx_wrapper(
                autocxx_gen_this: Pin<&mut ISteamNetworkingSockets>,
                pIdentity: *const SteamNetworkingIdentity,
            );
            pub fn SteamNetworkingSockets() -> *mut ISteamNetworkingSockets;
            pub fn GameNetworkingSockets_Kill();
            #[doc = " Store an IP and port.  IPv6 is always used; IPv4 is represented using"]
            type SteamNetworkingIPAddr;
            #[doc = " In a few places we need to set configuration options on listen sockets and connections, and"]
            type SteamNetworkingConfigValue_t;
            #[doc = " An abstract way to represent the identity of a network host.  All identities can"]
            type SteamNetworkingIdentity;
            type EResult = super::bindgen::root::EResult;
            #[doc = " Describe the state of a connection."]
            type SteamNetConnectionInfo_t;
            #[doc = " Quick connection state, pared down to something you could call"]
            type SteamNetworkingQuickConnectionStatus;
            type ESteamNetworkingAvailability = super::bindgen::root::ESteamNetworkingAvailability;
            #[doc = " A struct used to describe our readiness to participate in authenticated,"]
            type SteamNetAuthenticationStatus_t;
            type SteamDatagramRelayAuthTicket;
            type SteamDatagramHostedAddress;
            type SteamDatagramGameCoordinatorServerLogin;
            type ISteamNetworkingConnectionSignaling;
            type ISteamNetworkingSignalingRecvContext;
            type c_int = autocxx::c_int;
            type c_uint = autocxx::c_uint;
            type c_void = autocxx::c_void;
            type c_longlong = autocxx::c_longlong;
            type c_ushort = autocxx::c_ushort;
            include!("steamnetworkingsockets.h");
            include!("isteamnetworkingsockets.h");
            include!("steamnetworkingtypes.h");
            include!("autocxxgen_ffi.h");
        }
        extern "Rust" {}
    }
    #[allow(unused_imports)]
    use bindgen::root;
    pub use cxxbridge::autocxx_make_string_default as make_string;
    #[doc = "autocxx bindings couldn't be generated: Encountered type not yet known by autocxx: [:: std :: os :: raw :: c_char ; 1024usize]"]
    pub struct SteamNetworkingErrMsg;
    #[doc = "autocxx bindings couldn't be generated: This item depends on some other type which autocxx could not generate."]
    pub struct SteamDatagramErrMsg;
    pub use cxxbridge::ISteamNetworkingSockets;
    pub use cxxbridge::SteamNetworkingSockets;
    #[doc = "autocxx bindings couldn't be generated: This item depends on some other type which autocxx could not generate."]
    pub struct GameNetworkingSockets_Init;
    pub use bindgen::root::int64;
    pub use bindgen::root::uint16;
    pub use bindgen::root::uint32;
    pub use bindgen::root::HSteamListenSocket;
    pub use bindgen::root::HSteamNetConnection;
    pub use bindgen::root::HSteamNetPollGroup;
    pub use bindgen::root::SteamNetworkingPOPID;
    pub use cxxbridge::EResult;
    pub use cxxbridge::ESteamNetworkingAvailability;
    pub use cxxbridge::GameNetworkingSockets_Kill;
    pub use cxxbridge::ISteamNetworkingConnectionSignaling;
    pub use cxxbridge::ISteamNetworkingSignalingRecvContext;
    pub use cxxbridge::SteamDatagramGameCoordinatorServerLogin;
    pub use cxxbridge::SteamDatagramHostedAddress;
    pub use cxxbridge::SteamDatagramRelayAuthTicket;
    pub use cxxbridge::SteamNetAuthenticationStatus_t;
    pub use cxxbridge::SteamNetConnectionInfo_t;
    pub use cxxbridge::SteamNetworkingConfigValue_t;
    pub use cxxbridge::SteamNetworkingIPAddr;
    pub use cxxbridge::SteamNetworkingIdentity;
    pub use cxxbridge::SteamNetworkingQuickConnectionStatus;
}
