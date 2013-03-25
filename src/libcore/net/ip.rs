#[cfg(test)]
mod test {
    use core::prelude::*;
    use core::net::ip::*;

    #[test]
    fn test_ip_ipv4_create_and_format_ip() {
        let localhost_str = ~"127.0.0.1";
        let port = 8080;
        let addr_str = fmt!("%s:%s", localhost_str, port);
        let format_result = format_addr(&ipv4_addr(localhost_str, port));
        debug!("results: expected: '%s' actual: '%s'",
            addr_str, format_result);
        fail_unless!(format_result == addr_str)
    }

    #[test]
    fn test_ip_ipv6_create_and_format_ip() {
        let localhost_str = ~"::1";
        let port = 8080;
        let addr_str = fmt!("%s:%s", localhost_str, port);
        let format_result = format_addr(&ipv6_addr(localhost_str, port));
        debug!("results: expected: '%s' actual: '%s'",
            addr_str, format_result);
        fail_unless!(format_result == addr_str);
    }
}
