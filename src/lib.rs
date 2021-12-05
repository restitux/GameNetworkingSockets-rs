use std::net::SocketAddr;
use std::pin::Pin;

use cxx::{CxxVector, UniquePtr};

use eyre::{eyre, Result};

use gamenetworkingsockets_sys as gns_sys;

macro_rules! unsafe_bool_functions_to_result {
    ($func_call:expr, $error:literal) => {
        match unsafe { $func_call } {
            true => Ok(()),
            false => Err(eyre!($error)),
        }
    };
}

macro_rules! to_pin_raw {
    ($ptr:expr) => {
        Pin::new_unchecked($ptr.as_mut().unwrap())
    };
}

pub struct SteamNetworkingSockets {
    sns: *mut gns_sys::ISteamNetworkingSockets,
}

pub struct SteamListenSocket {
    sock: gns_sys::HSteamListenSocket,
}

pub struct SteamNetConnection {
    conn: gns_sys::HSteamNetConnection,
}

pub struct SteamPollGroup {
    pgroup: gns_sys::HSteamNetPollGroup,
}

pub struct SteamNetworkingMessages {
    pub msgs: Vec<SteamNetworkingMessage>,
    root: *mut gns_sys::SteamNetworkingMessage_t,
}

pub struct SteamNetworkingConfigValue {
    val: cxx::UniquePtr<gns_sys::SteamNetworkingConfigValue_t>,
}

impl SteamNetworkingConfigValue {
    fn new() -> SteamNetworkingConfigValue {
        SteamNetworkingConfigValue {
            val: gns_sys::new_SteamNetworkingConfigValue_t(),
        }
    }
    //fn set_i32(&mut self, val: ESteamNetworkingConfigValue, data: i32) {
    //    self.val.pin_mut().Seti32()
    //}
    //fn set_i64(&mut self, val: i64)
    //fn set_f32(&mut self, val: f32)
    fn set_ptr(&mut self, val: gns_sys::ESteamNetworkingConfigValue, data: *mut gns_sys::c_void) {
        unsafe { self.val.pin_mut().SetPtr(val, data) };
    }
    //fn set_string(&mut self, val: String)
}

pub struct SteamNetworkingConfigValues {
    vals: cxx::UniquePtr<cxx::CxxVector<gns_sys::SteamNetworkingConfigValue_t>>,
}

impl SteamNetworkingConfigValues {
    fn new() -> SteamNetworkingConfigValues {
        SteamNetworkingConfigValues {
            vals: gns_sys::new_SteamNetworkingConfigValue_t_Vector(),
        }
    }
    fn push(&mut self, val: SteamNetworkingConfigValue) {
        gns_sys::SteamNetworkingConfigValue_t_Vector_push(&mut self.vals, val.val);
    }
}

struct SteamNetworkingIPAddr {
    ip_addr: cxx::UniquePtr<gns_sys::SteamNetworkingIPAddr>,
}

impl SteamNetworkingIPAddr {
    fn new() -> SteamNetworkingIPAddr {
        SteamNetworkingIPAddr {
            ip_addr: gns_sys::SteamNetworkingIPAddr::new(),
        }
    }
    fn from_sock_addr(addr: std::net::SocketAddr) -> SteamNetworkingIPAddr {
        let mut steam_addr = SteamNetworkingIPAddr::new();
        match addr {
            SocketAddr::V4(v) => steam_addr.set_ipv4(v),
            SocketAddr::V6(_v) => panic!("IPV6 is unimplemented"),
        }

        steam_addr
    }
    pub fn set_ipv4(&mut self, addr: std::net::SocketAddrV4) {
        // swap to host order for passing to library
        let ip = u32::from_be_bytes(addr.ip().octets());
        self.ip_addr.pin_mut().SetIPv4(ip, addr.port());
    }
}

pub struct SteamNetworkingMessage {
    msg: *mut gns_sys::SteamNetworkingMessage_t,
}

impl SteamNetworkingMessage {
    fn size(&self) -> u32 {
        unsafe { to_pin_raw!(self.msg).GetSize() }
    }
    fn data(&self) -> *const u8 {
        (unsafe { to_pin_raw!(self.msg).GetData() }) as *const u8
    }
    fn channel(&self) -> i32 {
        unsafe { to_pin_raw!(self.msg).GetChannel() }
    }
    fn connection(&self) -> u32 {
        unsafe { to_pin_raw!(self.msg).GetConnection() }
    }
    fn connection_user_data(&self) -> i64 {
        (unsafe { to_pin_raw!(self.msg).GetConnectionUserData() }).0
    }
    fn time_received(&self) -> i64 {
        (unsafe { to_pin_raw!(self.msg).GetTimeReceived() }).0
    }
    fn message_number(&self) -> i64 {
        (unsafe { to_pin_raw!(self.msg).GetMessageNumber() }).0
    }
}

fn connection_status_changed_callback(info: gns_sys::SteamNetConnectionStatusChangedCallback_t) {
    println!("CALLBACK CALLED");
}

impl SteamNetworkingSockets {
    fn new(isns: &'static mut gns_sys::ISteamNetworkingSockets) -> SteamNetworkingSockets {
        SteamNetworkingSockets { sns: isns }
    }
    pub fn connect_by_ip_address(
        &mut self,
        address: std::net::SocketAddr,
        _options: Option<SteamNetworkingConfigValues>,
    ) -> Result<SteamNetConnection> {
        let mut steam_addr = SteamNetworkingIPAddr::from_sock_addr(address);

        // TODO: This should append options onto included options (and filter out duplicate options)
        match _options {
            Some(_o) => panic!("Options are unimplemented"),
            None => (),
        }

        // Hardcoded default options
        let mut hardcoded_options = SteamNetworkingConfigValues::new();

        // Hardcode callback function that forwards ConnectionStatusChanged events to queue
        let mut connection_status_changed_callback_option = SteamNetworkingConfigValue::new();
        connection_status_changed_callback_option.set_ptr(gns_sys::ESteamNetworkingConfigValue::k_ESteamNetworkingConfig_Callback_ConnectionStatusChanged, connection_status_changed_callback as *mut gns_sys::c_void);

        hardcoded_options.push(connection_status_changed_callback_option);

        let conn = unsafe {
            gns_sys::ConnectByIPAddress_Vector(
                self.sns,
                &steam_addr.ip_addr,
                &hardcoded_options.vals,
            )
        };

        if unsafe { conn == gns_sys::HSteamNetConnection_Invalid } {
            Err(eyre!("Failed to listen on address: {}", address))
        } else {
            Ok(SteamNetConnection { conn })
        }
    }

    pub fn create_listen_socket_ip(
        &mut self,
        local_address: std::net::SocketAddr,
        _options: Option<SteamNetworkingConfigValues>,
    ) -> Result<SteamListenSocket> {
        let mut steam_addr = SteamNetworkingIPAddr::from_sock_addr(local_address);

        // TODO: This should append options onto included options (and filter out duplicate options)
        match _options {
            Some(_o) => panic!("Options are unimplemented"),
            None => (),
        }

        // Hardcoded default options
        let mut hardcoded_options = SteamNetworkingConfigValues::new();

        // Hardcode callback function that forwards ConnectionStatusChanged events to queue
        let mut connection_status_changed_callback_option = SteamNetworkingConfigValue::new();
        connection_status_changed_callback_option.set_ptr(gns_sys::ESteamNetworkingConfigValue::k_ESteamNetworkingConfig_Callback_ConnectionStatusChanged, connection_status_changed_callback as *mut gns_sys::c_void);

        hardcoded_options.push(connection_status_changed_callback_option);

        let sock = unsafe {
            gns_sys::CreateListenSocketIP_Vector(
                self.sns,
                &steam_addr.ip_addr,
                &hardcoded_options.vals,
            )
        };

        if unsafe { sock == gns_sys::HSteamListenSocket_Invalid } {
            Err(eyre!("Failed to listen on address: {}", local_address))
        } else {
            Ok(SteamListenSocket { sock })
        }
    }
    pub fn close_listen_socket(&mut self, listen_socket: SteamListenSocket) -> Result<()> {
        unsafe_bool_functions_to_result!(
            to_pin_raw!(self.sns).CloseListenSocket(listen_socket.sock),
            "Listen socket was not valid"
        )
    }
    pub fn create_poll_group(&mut self) -> Result<SteamPollGroup> {
        let pgroup = unsafe { to_pin_raw!(self.sns).CreatePollGroup() };
        if unsafe { pgroup == gns_sys::HSteamNetPollGroup_Invalid } {
            Err(eyre!("Failed to create poll group."))
        } else {
            Ok(SteamPollGroup { pgroup })
        }
    }
    pub fn receive_messages_on_poll_group(
        &mut self,
        poll_group: &SteamPollGroup,
    ) -> Result<Option<SteamNetworkingMessages>> {
        let max_messages = 1024;
        let mut out_messages: *mut gns_sys::SteamNetworkingMessage_t = std::ptr::null_mut();
        let num_msgs = unsafe {
            to_pin_raw!(self.sns).ReceiveMessagesOnPollGroup(
                poll_group.pgroup,
                &mut out_messages,
                max_messages,
            )
        };
        match num_msgs {
            std::i32::MIN..=-1 => Err(eyre!("Could not check for messages: {}", num_msgs)),
            0 => Ok(None),
            _ => {
                let mut msg_vec = Vec::with_capacity(num_msgs as usize);
                for i in 0..num_msgs {
                    msg_vec.push(SteamNetworkingMessage {
                        msg: unsafe { out_messages.offset(i as isize) },
                    });
                }
                Ok(Some(SteamNetworkingMessages {
                    msgs: msg_vec,
                    root: out_messages,
                }))
            }
        }
    }
    pub fn destroy_poll_group(&mut self, poll_group: SteamPollGroup) -> Result<()> {
        unsafe_bool_functions_to_result!(
            to_pin_raw!(self.sns).DestroyPollGroup(poll_group.pgroup),
            "Poll group was not valid"
        )
    }
    pub fn run_callbacks(&mut self) {
        unsafe { to_pin_raw!(self.sns).RunCallbacks() };
    }
}

pub fn init() -> Result<SteamNetworkingSockets> {
    // create C++ string
    let return_val = gns_sys::GameNetworkingSockets_Init_rs();
    if return_val.ret == false {
        Err(eyre!(return_val.err_msg))
    } else {
        let handle = gns_sys::SteamNetworkingSockets();
        let handle = unsafe { handle.as_mut().ok_or(eyre!("Could not get handle"))? };
        Ok(SteamNetworkingSockets::new(handle))
    }
}
