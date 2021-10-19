///IpAddress enum which have variants for classes of ipaddress
///
/// #variant
///
/// ClassA:-ClassA is variant of enum IpAddress and it is String type
///
/// ClassB:-ClassB is variant of enum IpAddress and it is String type
///
/// ClassC:-ClassC is variant of enum IpAddress and it is String type
///
/// ClassD:-ClassD is variant of enum IpAddress and it is String type
///
/// ClassE:- ClassE is variant of enum IpAddress and it is String type
#[derive(PartialEq, Eq, Debug)]
pub enum IpAddress {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
}

/// check_ip_address which is used check the given ip_Address
///
/// #Arguments
///
///ipconfig: A ipconfig is tuple object of unsigned integer type
///
/// #Return
///
/// No return 
pub fn check_ip_address(ipconfig: (u128, u128, u128, u128)) {
    match ipconfig {
        (1..=127, 0..=255, 0..=255, 1..=254) => println!(
            "IpAddress::ClassA({}.{}.{}.{})",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ),
        (128..=191, 0..=255, 0..=255, 1..=254) => println!(
            "IpAddress::ClassB({}.{}.{}.{})",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ),
        (192..=223, 0..=255, 1..=254, 1..=254) => println!(
            "IpAddress::ClassC({}.{}.{}.{})",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ),
        (224..=239, 0..=255, 0..=255, 0..=255) => println!(
            "IpAddress::ClassD({}.{}.{}.{})",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ),
        (240..=254, 0..=255, 0..=255, 0..=254) => println!(
            "IpAddress::ClassE({}.{}.{}.{})",
            ipconfig.0, ipconfig.1, ipconfig.2, ipconfig.3
        ),
        _ => println!("Unwanted Input"),
    }
}

fn main() {
    // ip address 1 to check class type
    let ip_address_1 = (143, 143, 10, 10);
    // calling check_ip_address to check class type of ip address 1
    check_ip_address(ip_address_1);
    // ip address 2 to check class type
    let ip_address_2 = (220, 143, 3, 4);
    // calling check_ip_address to check class type of ip address 2
    check_ip_address(ip_address_2);
}
