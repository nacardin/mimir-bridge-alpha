use mimir_types::Address;
use mimir_util::hex;
use common::Role;
use std::str::FromStr;
use std::fmt;


/// channel value.
///
/// ```
/// #
/// extern crate mimir_transport;
/// use mimir_transport::common::Channel;
/// # fn main() {
/// 
/// // shared channels contain messages/work that is
/// // competitively consumed.
/// let shared: Channel = "oracle::work".parse().unwrap();
///
/// assert_eq!("oracle",shared.get_role().to_string());
/// 
/// // direct channels conain messages directed at a specific
/// // entity (identified by address).
/// let direct: Channel = "notary::00a329c0648769a73afac7f9381e08fb43dbea72"
///     .parse().unwrap();
/// 
/// assert!(direct.get_addr().is_some());
/// # }
/// ```
pub enum Channel {
    /// competitive consumption channel
    Shared {
        /// channel consumer role
        role: Role 
    },
    /// entity-specific channel
    Direct {
        /// channel consumer role
        role: Role,
        /// channel consumer identity
        addr: Address
    }
}


impl Channel {

    /// get channel's role value
    #[inline]
    pub fn get_role(&self) -> Role {
        match *self {
            Channel::Shared { role, .. } => role,
            Channel::Direct { role, .. } => role,
        }
    }

    /// get channel's address value if exists
    #[inline]
    pub fn get_addr(&self) -> Option<Address> {
        if let Channel::Direct { addr, .. } = *self {
            Some(addr)
        } else {
            None
        }
    }
}


simple_error!(
    ParseChannelError => "error parsing channel name"
);


impl FromStr for Channel {

    type Err = ParseChannelError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let mut split = s.splitn(2,"::");
        if let Some(Ok(role)) = split.next().map(|r| r.parse()) {
            match split.next() {
                Some("work") => Ok(Channel::Shared { role }),
                Some(other) => {
                    let addr: Address = other.parse()
                        .map_err(|_|ParseChannelError)?;
                    Ok(Channel::Direct { role, addr })
                },
                None => Err(ParseChannelError),
            }
        } else {
            Err(ParseChannelError)
        }
    }
}


impl fmt::Display for Channel {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.get_role().as_ref())?;
        f.write_str("::")?;
        if let Some(addr) = self.get_addr() {
            let mut buff = [0u8;40];
            let hex_str = hex::as_str(&addr,&mut buff);
            f.write_str(hex_str)
        } else {
            f.write_str("work")
        }
    }
}

