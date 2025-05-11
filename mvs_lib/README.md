## Mullvad Socks Proxy Filter (Library)

Fetch [SOCKS5 proxies](https://mullvad.net/en/help/socks5-proxy/) from [Mullvad's API](https://api-relays.mullvad.net/network/v1-beta1/socks-proxies) and optionally filter by `country`, `city`, `weight` and `online` status. Print as IPv4/IPv6 or hostname, optionally add scheme and port.

#### By default, proxies will be printed as IPv4, with the following filters applied:
- `online == true`
- `weight <= 100`