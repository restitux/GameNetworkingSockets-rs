use eyre::{eyre, Result};
use std::pin::Pin;

macro_rules! ctype_wrapper {
    ($t:ident, $r:ident, $c:expr, $d:expr) => {
        #[doc=$d]
        #[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct $t(pub ::std::os::raw::$r);

        unsafe impl cxx::ExternType for $t {
            type Id = cxx::type_id!($c);
            type Kind = cxx::kind::Trivial;
        }
    };
}

// C types
ctype_wrapper!(
    c_longlong,
    c_longlong,
    "c_longlong",
    "Newtype wrapper for a long long"
);
ctype_wrapper!(c_uint, c_uint, "c_uint", "Newtype wrapper for a uint");
ctype_wrapper!(c_char, c_char, "c_char", "Newytpe wrapper for a char");
ctype_wrapper!(int64, c_longlong, "int64", "Newtype wrapper for int64");

// Steam Types
ctype_wrapper!(
    HSteamListenSocket,
    c_uint,
    "HSteamListenSocket",
    "Newtype wrapper for HSteamListenSocket"
);

ctype_wrapper!(
    HSteamNetConnection,
    c_uint,
    "HSteamNetConnection",
    "Newtype wrapper for HSeamNetConnection"
);

ctype_wrapper!(
    HSteamNetPollGroup,
    c_uint,
    "HSteamNetPollGroup",
    "Newtype wrapper for HSteamNetPollGroup"
);

// Newtype wrapper for a C void. Only useful as a `*c_void`
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct c_void(pub ::std::os::raw::c_void);

unsafe impl cxx::ExternType for c_void {
    type Id = cxx::type_id!(c_void);
    type Kind = cxx::kind::Trivial;
}

//#[allow(non_camel_case_types)]
//#[repr(transparent)]
//pub struct HSteamListenSocket(pub ::std::os::raw::c_uint);
//
//unsafe impl cxx::ExternType for HSteamListenSocket {
//    type Id = cxx::type_id!(HSteamListenSocket);
//    type Kind = cxx::kind::Trivial;
//}

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type int64 = crate::int64;
        type c_void = crate::c_void;
        type c_char = crate::c_char;
        type HSteamListenSocket = crate::HSteamListenSocket;
        type HSteamNetConnection = crate::HSteamNetConnection;
        type HSteamNetPollGroup = crate::HSteamNetPollGroup;
    }
    struct InitReturn {
        ret: bool,
        err_msg: String,
    }
    unsafe extern "C++" {
        include!("gns.h");
        fn GameNetworkingSockets_Init_rs() -> InitReturn;
        fn GameNetworkingSockets_Kill();
    }
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
    unsafe extern "C++" {
        include!("gns.h");
        type EResult;
    }
    #[repr(u32)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum ESteamNetworkingConfigValue {
        k_ESteamNetworkingConfig_Invalid = 0,
        k_ESteamNetworkingConfig_TimeoutInitial = 24,
        k_ESteamNetworkingConfig_TimeoutConnected = 25,
        k_ESteamNetworkingConfig_SendBufferSize = 9,
        k_ESteamNetworkingConfig_ConnectionUserData = 40,
        k_ESteamNetworkingConfig_SendRateMin = 10,
        k_ESteamNetworkingConfig_SendRateMax = 11,
        k_ESteamNetworkingConfig_NagleTime = 12,
        k_ESteamNetworkingConfig_IP_AllowWithoutAuth = 23,
        k_ESteamNetworkingConfig_MTU_PacketSize = 32,
        k_ESteamNetworkingConfig_MTU_DataSize = 33,
        k_ESteamNetworkingConfig_Unencrypted = 34,
        k_ESteamNetworkingConfig_SymmetricConnect = 37,
        k_ESteamNetworkingConfig_LocalVirtualPort = 38,
        k_ESteamNetworkingConfig_FakePacketLoss_Send = 2,
        k_ESteamNetworkingConfig_FakePacketLoss_Recv = 3,
        k_ESteamNetworkingConfig_FakePacketLag_Send = 4,
        k_ESteamNetworkingConfig_FakePacketLag_Recv = 5,
        k_ESteamNetworkingConfig_FakePacketReorder_Send = 6,
        k_ESteamNetworkingConfig_FakePacketReorder_Recv = 7,
        k_ESteamNetworkingConfig_FakePacketReorder_Time = 8,
        k_ESteamNetworkingConfig_FakePacketDup_Send = 26,
        k_ESteamNetworkingConfig_FakePacketDup_Recv = 27,
        k_ESteamNetworkingConfig_FakePacketDup_TimeMax = 28,
        k_ESteamNetworkingConfig_PacketTraceMaxBytes = 41,
        k_ESteamNetworkingConfig_FakeRateLimit_Send_Rate = 42,
        k_ESteamNetworkingConfig_FakeRateLimit_Send_Burst = 43,
        k_ESteamNetworkingConfig_FakeRateLimit_Recv_Rate = 44,
        k_ESteamNetworkingConfig_FakeRateLimit_Recv_Burst = 45,
        k_ESteamNetworkingConfig_Callback_ConnectionStatusChanged = 201,
        k_ESteamNetworkingConfig_Callback_AuthStatusChanged = 202,
        k_ESteamNetworkingConfig_Callback_RelayNetworkStatusChanged = 203,
        k_ESteamNetworkingConfig_Callback_MessagesSessionRequest = 204,
        k_ESteamNetworkingConfig_Callback_MessagesSessionFailed = 205,
        k_ESteamNetworkingConfig_Callback_CreateConnectionSignaling = 206,
        k_ESteamNetworkingConfig_P2P_STUN_ServerList = 103,
        k_ESteamNetworkingConfig_P2P_Transport_ICE_Enable = 104,
        k_ESteamNetworkingConfig_P2P_Transport_ICE_Penalty = 105,
        k_ESteamNetworkingConfig_P2P_Transport_SDR_Penalty = 106,
        k_ESteamNetworkingConfig_SDRClient_ConsecutitivePingTimeoutsFailInitial = 19,
        k_ESteamNetworkingConfig_SDRClient_ConsecutitivePingTimeoutsFail = 20,
        k_ESteamNetworkingConfig_SDRClient_MinPingsBeforePingAccurate = 21,
        k_ESteamNetworkingConfig_SDRClient_SingleSocket = 22,
        k_ESteamNetworkingConfig_SDRClient_ForceRelayCluster = 29,
        k_ESteamNetworkingConfig_SDRClient_DebugTicketAddress = 30,
        k_ESteamNetworkingConfig_SDRClient_ForceProxyAddr = 31,
        k_ESteamNetworkingConfig_SDRClient_FakeClusterPing = 36,
        k_ESteamNetworkingConfig_EnumerateDevVars = 35,
    }

    unsafe extern "C++" {
        include!("gns.h");
        type FnSteamNetConnectionStatusChanged;
        type SteamNetConnectionStatusChangedCallback_t;
        type ESteamNetworkingConfigValue;
    }

    unsafe extern "C++" {
        include!("gns.h");
        type SteamNetworkingConfigValue_t;
        fn new_SteamNetworkingConfigValue_t() -> UniquePtr<SteamNetworkingConfigValue_t>;
        fn new_SteamNetworkingConfigValue_t_Vector(
        ) -> UniquePtr<CxxVector<SteamNetworkingConfigValue_t>>;
        fn SteamNetworkingConfigValue_t_Vector_push(
            vec: &UniquePtr<CxxVector<SteamNetworkingConfigValue_t>>,
            val: UniquePtr<SteamNetworkingConfigValue_t>,
        );
        fn SetInt32(
            self: Pin<&mut SteamNetworkingConfigValue_t>,
            eVal: ESteamNetworkingConfigValue,
            data: i32,
        );
        fn SetInt64(
            self: Pin<&mut SteamNetworkingConfigValue_t>,
            eVal: ESteamNetworkingConfigValue,
            data: i64,
        );
        fn SetFloat(
            self: Pin<&mut SteamNetworkingConfigValue_t>,
            eVal: ESteamNetworkingConfigValue,
            data: f32,
        );
        unsafe fn SetPtr(
            self: Pin<&mut SteamNetworkingConfigValue_t>,
            eVal: ESteamNetworkingConfigValue,
            data: *mut c_void,
        );
        /// WARNING - Just saves your pointer.  Does NOT make a copy of the string
        unsafe fn SetString(
            self: Pin<&mut SteamNetworkingConfigValue_t>,
            eVal: ESteamNetworkingConfigValue,
            data: *const c_char,
        );
    }

    unsafe extern "C++" {
        include!("gns.h");
        type SteamNetworkingMessage_t;
        fn GetSize(&self) -> u32;
        fn GetData(&self) -> *const c_void;
        fn GetChannel(&self) -> i32;
        fn GetConnection(&self) -> u32;
        fn GetConnectionUserData(&self) -> int64;
        fn GetTimeReceived(&self) -> int64;
        fn GetMessageNumber(&self) -> int64;
    }
    unsafe extern "C++" {
        include!("gns.h");
        fn SteamNetworkingSockets() -> *mut ISteamNetworkingSockets;
    }
    unsafe extern "C++" {
        include!("gns.h");
        type SteamNetworkingIPAddr;
        fn new_SteamNetworkingIPAddr() -> UniquePtr<SteamNetworkingIPAddr>;
        //fn Clear(&mut self);
        fn Clear(self: Pin<&mut SteamNetworkingIPAddr>);
        fn IsIPv6AllZeros(&self) -> bool;
        fn SetIPv4(self: Pin<&mut SteamNetworkingIPAddr>, nIP: u32, nPort: u16);
        fn IsIPv4(&self) -> bool;
        fn GetIPv4(&self) -> u32;
        fn SetIPv6LocalHost(self: Pin<&mut SteamNetworkingIPAddr>, nPort: u16);
        fn IsLocalHost(&self) -> bool;
    }
    unsafe extern "C++" {
        include!("gns.h");
        type ISteamNetworkingSockets;
        unsafe fn CreateListenSocketIP(
            self: Pin<&'static mut ISteamNetworkingSockets>,
            localAddress: &SteamNetworkingIPAddr,
            nOptions: i32,
            pOptions: *const SteamNetworkingConfigValue_t,
        ) -> HSteamListenSocket;
        unsafe fn CreateListenSocketIP_Vector(
            sns: *mut ISteamNetworkingSockets,
            localAddress: &SteamNetworkingIPAddr,
            Options: &UniquePtr<CxxVector<SteamNetworkingConfigValue_t>>,
        ) -> HSteamListenSocket;
         unsafe fn ConnectByIPAddress(
            self: Pin<&'static mut ISteamNetworkingSockets>,
            localAddress: &SteamNetworkingIPAddr,
            nOptions: i32,
            pOptions: *const SteamNetworkingConfigValue_t,
        ) -> HSteamNetConnection;
        unsafe fn ConnectByIPAddress_Vector(
            sns: *mut ISteamNetworkingSockets,
            localAddress: &SteamNetworkingIPAddr,
            Options: &UniquePtr<CxxVector<SteamNetworkingConfigValue_t>>,
        ) -> HSteamNetConnection;
        fn CloseListenSocket(
            self: Pin<&'static mut ISteamNetworkingSockets>,
            hSocket: HSteamListenSocket,
        ) -> bool;
        fn CreatePollGroup(self: Pin<&'static mut ISteamNetworkingSockets>) -> HSteamNetPollGroup;
        fn DestroyPollGroup(
            self: Pin<&'static mut ISteamNetworkingSockets>,
            hPollGroup: HSteamNetPollGroup,
        ) -> bool;
        unsafe fn ReceiveMessagesOnPollGroup(
            self: Pin<&'static mut ISteamNetworkingSockets>,
            hPollGroup: HSteamNetPollGroup,
            ppOutMessages: *mut *mut SteamNetworkingMessage_t,
            nMaxMessages: i32,
        ) -> i32;
        fn RunCallbacks(self: Pin<&'static mut ISteamNetworkingSockets>);
    }
}

extern "C" {
    pub static HSteamListenSocket_Invalid: HSteamListenSocket;
    pub static HSteamNetConnection_Invalid: HSteamNetConnection;
    pub static HSteamNetPollGroup_Invalid: HSteamNetPollGroup;
}

impl ffi::SteamNetworkingConfigValue_t {
    pub fn new() -> cxx::UniquePtr<SteamNetworkingConfigValue_t> {
        new_SteamNetworkingConfigValue_t()
    }
    pub fn new_vec() -> cxx::UniquePtr<cxx::CxxVector<SteamNetworkingConfigValue_t>> {
        new_SteamNetworkingConfigValue_t_Vector()
    }
}

impl ffi::SteamNetworkingIPAddr {
    pub fn new() -> cxx::UniquePtr<ffi::SteamNetworkingIPAddr> {
        let mut val = new_SteamNetworkingIPAddr();
        val.pin_mut().Clear();
        val
    }
}

pub use ffi::*;
