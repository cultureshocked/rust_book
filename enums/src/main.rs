enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let lo = IpAddr::V6(String::from("::1"));

    route(localhost);
    route(lo);
}

fn route(ip_type: IpAddr) {
    ()
}
