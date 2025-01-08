## Mullvad Socks Proxy Filter (HTTP API)

Based on [this library](https://crates.io/crates/mullvad_socks).

There is also a [CLI](https://crates.io/crates/mvs_cli).

### Configuration

The IPv4 address and the port to bind to are configured by using the `HOST` and `PORT` variables, respectively.

Default: `127.0.0.1:8080`

### Endpoints

#### `/locations` lists available locations by type [`country`, `city`, `datacenter`]

```fish
curl 127.0.0.1:8080/locations?type=country

["adelaide","amsterdam","...","zagreb","zurich"]
```

#### `/proxies` list proxies, optionally add filter and formatting parameters

While you can use multiple types of location identifiers in a single query, it's not advised to do so, as proxies are filtered by level, as in *country > city > datacenter*. This means only queries matching all levels will be left in the list.

| Parameter | Value | Default |
| --- | --- | --- |
| `countries` | list of countries | - |
| `cities` | list of cities | - |
| `datacenters` | list of datacenters | - |
| `weight` | 0-65535 | 100 |
| `offline` | Hide, Show, Only | Hide |
| `style` | V4, V6, Hostname | V4 |
| `scheme` | true, false | false |
| `port` | true, false | false |

Default filters and formatting:

```fish
curl 127.0.0.1:8080/proxies

["10.124.0.155","10.124.0.212","...","10.124.2.59","10.124.2.60"]
```

Custom filters and formatting:

```fish
curl 127.0.0.1:8080/proxies?countries=germany,austria&weight=200&offline=Show&style=V6&scheme=true&port=true

["socks5://fd00:aaaa::543:1080","...","socks5://fd00:aaaa::496:1080"]
```

#### `/version` prints crate version

```fish
curl 127.0.0.1:8080/version

{"version":"mvs_web v1.0.0"}
```