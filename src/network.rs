use nix::net::if_::InterfaceFlags;
use std::collections::HashMap;

pub fn description() -> String {
    let mut interfaces = HashMap::new();
    let addrs = nix::ifaddrs::getifaddrs().unwrap();
    for ifaddr in addrs {
        let interface = interfaces
            .entry(ifaddr.interface_name.clone())
            .or_insert_with(Vec::new);
        if let Some(address) = ifaddr.address {
            interface.push((address, ifaddr.flags));
        }
    }

    // Create list of running interfaces, that isn't loopback, and sort it
    let mut interfaces_sorted = interfaces
        .iter()
        .filter(|(name, addreses)| {
            name.as_str() != "lo"
                && addreses
                    .iter()
                    .any(|(_, flags)| flags.contains(InterfaceFlags::IFF_RUNNING))
        })
        .collect::<Vec<_>>();
    interfaces_sorted.sort_unstable_by_key(|(name, _)| *name);

    let mut net_string = String::new();
    for (iface, addrs) in interfaces_sorted {
        net_string += &format!("{}:", iface);
        for (addr, _) in addrs {
            net_string += &format!(" {}", addr);
        }
    }

    net_string
}
