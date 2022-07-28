pub(crate) fn ownership() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home2 = IpAddr2::V4(127, 0, 0, 1);

    let loopback2 = IpAddr2::V6(String::from("::1"));



    fn ip_addr() {
        struct Ipv4Addr {
            // --snip--
        }

        struct Ipv6Addr {
            // --snip--
        }

        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main() {}


}
