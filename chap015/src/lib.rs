use std::mem;

pub mod ipv4 {
    #[derive(Debug)]
    pub struct IPv4 {
        pub address: [u8; 4],
    }

    impl IPv4 {
        pub fn new(address_str: &str) -> Result<IPv4, &str> {
            let v: Vec<u8> = address_str
                .split('.')
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            if v.len() != 4 {
                return Err("address must contain 4 segment");
            }
            Ok(IPv4 {
                address: [v[0], v[1], v[2], v[3]],
            })
        }
        pub fn calc_dist(mut ip_from: IPv4, mut ip_to: IPv4) {
            if ip_from.address[0] > ip_to.address[0] {
                std::mem::swap(&mut ip_from, &mut ip_to);
            }
        }
    }
}
