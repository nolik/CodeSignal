use std::net::{IpAddr, AddrParseError};

pub fn isIPv4Address(inputString: String) -> bool {
    let addr: Result<IpAddr, AddrParseError> = inputString.parse();

    match addr {
        Ok(unwrapped_adr) => unwrapped_adr.is_ipv4(),
        Err(e) => false
    }
}
