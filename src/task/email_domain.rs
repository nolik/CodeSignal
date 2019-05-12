pub fn findEmailDomain(address: String) -> String {
    address.split('@').last().unwrap().to_string()
}
