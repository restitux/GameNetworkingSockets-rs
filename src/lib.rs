use std::net::SocketAddr;
use std::pin::Pin;

use eyre::{eyre, Result};

use gamenetworkingsockets_sys as gns_sys;

pub struct SteamNetworkingSockets {
    sns: *mut gns_sys::ISteamNetworkingSockets,
}

pub struct SteamListenSocket {
    sock: gns_sys::HSteamListenSocket,
}

pub struct SteamPollGroup {
    pgroup: gns_sys::HSteamNetPollGroup,
}

pub struct SteamNetworkingMessages {
    pub msgs: Vec<SteamNetworkingMessage>,
    root: *mut gns_sys::SteamNetworkingMessage_t,
}

pub struct SteamNetworkingMessage {
    msg: *mut gns_sys::SteamNetworkingMessage_t,
}

impl SteamNetworkingMessage {
    fn size(&self) -> u32 {
        unsafe { Pin::new_unchecked(self.msg.as_mut().unwrap()).GetSize() }
    }
    fn data(&self) -> *const u8 {
        (unsafe { Pin::new_unchecked(self.msg.as_mut().unwrap()).GetData() }) as *const u8
    }
    fn channel(&self) -> i32 {
        unsafe { Pin::new_unchecked(self.msg.as_mut().unwrap()).GetChannel() }
    }
    fn connection(&self) -> u32 {
        unsafe { Pin::new_unchecked(self.msg.as_mut().unwrap()).GetConnection() }
    }
    fn connection_user_data(&self) -> i64 {
        (unsafe { Pin::new_unchecked(self.msg.as_mut().unwrap()).GetConnectionUserData() }).0
    }
    fn time_received(&self) -> i64 {
        (unsafe { Pin::new_unchecked(self.msg.as_mut().unwrap()).GetTimeReceived() }).0
    }
    fn message_number(&self) -> i64 {
        (unsafe { Pin::new_unchecked(self.msg.as_mut().unwrap()).GetMessageNumber() }).0
    }
}

impl SteamNetworkingSockets {
    fn new(isns: &'static mut gns_sys::ISteamNetworkingSockets) -> SteamNetworkingSockets {
        SteamNetworkingSockets { sns: isns }
    }
    pub fn create_listen_socket_ip(
        &mut self,
        local_address: std::net::SocketAddr,
        _options: Option<Vec<SteamNetworkingConfigValue>>,
    ) -> Result<SteamListenSocket> {
        let mut steam_addr = gns_sys::SteamNetworkingIPAddr::new();
        match local_address {
            SocketAddr::V4(v) => steam_addr.set_ipv4(v),
            //SocketAddrV6(v) => steam_addr.set_ipv6(v),
            SocketAddr::V6(_v) => panic!("IPV6 is unimplemented"),
        }
        match _options {
            Some(_o) => panic!("Options are unimplemented"),
            None => {
                let sock = unsafe {
                    Pin::new_unchecked(self.sns.as_mut().unwrap()).CreateListenSocketIP(
                        &steam_addr,
                        0,
                        std::ptr::null(),
                    )
                };
                if unsafe { sock == gns_sys::HSteamListenSocket_Invalid } {
                    Err(eyre!("Failed to listen on address: {}", local_address))
                } else {
                    Ok(SteamListenSocket { sock })
                }
            }
        }
    }
    pub fn create_poll_group(&mut self) -> Result<SteamPollGroup> {
        let pgroup = unsafe { Pin::new_unchecked(self.sns.as_mut().unwrap()).CreatePollGroup() };
        if unsafe { pgroup == gns_sys::HSteamNetPollGroup_Invalid } {
            Err(eyre!("Failed to create poll group."))
        } else {
            Ok(SteamPollGroup { pgroup })
        }
    }
    pub fn receive_messages_on_poll_group(&mut self, poll_group: &SteamPollGroup) -> Result<Option<SteamNetworkingMessages>> {
        let max_messages = 1024;
        let mut out_messages : *mut gns_sys::SteamNetworkingMessage_t = std::ptr::null_mut();
        let num_msgs = unsafe { Pin::new_unchecked(self.sns.as_mut().unwrap()).ReceiveMessagesOnPollGroup(poll_group.pgroup, &mut out_messages, max_messages) };
        match num_msgs {
            std::i32::MIN..=-1 => Err(eyre!("Error checking for messages: {}", num_msgs)),
            0 => Ok(None),
            _ => {
                let mut msg_vec = Vec::with_capacity(num_msgs as usize);
                for i in 0..num_msgs {
                    msg_vec.push( SteamNetworkingMessage {
                        msg: unsafe { out_messages.offset(i as isize) },
                    });
                }
                Ok(Some(SteamNetworkingMessages {
                    msgs: msg_vec,
                    root: out_messages,
                }))
            },
        }
    }
}

pub struct SteamNetworkingConfigValue {}

pub fn init() -> Result<SteamNetworkingSockets> {
    // create C++ string
    let return_val = gns_sys::GameNetworkingSockets_Init_rs();
    if return_val.ret == false {
        Err(eyre!(return_val.errMsg))
    } else {
        let handle = gns_sys::SteamNetworkingSockets();
        let handle = unsafe {
            handle
                .as_mut()
                .ok_or(eyre!("ERROR: Could not get handle"))?
        };
        Ok(SteamNetworkingSockets::new(handle))
    }
}
