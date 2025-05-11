## Mullvad Socks Proxy Filter (HTTP API)

The IPv4 address and port to bind to are configured via the `HOST` and `PORT` environment variables.

Default: `127.0.0.1:8080`

### Endpoints

#### `/locations` lists available locations by type [`country`, `city`]

```fish
curl 127.0.0.1:8080/locations?type=city

["adelaide","amsterdam","...","zagreb","zurich"]
```

#### `/proxies` list proxies, optionally add filter and formatting parameters

While you can use multiple types of location identifiers in a single query, it's not advised to do so, as proxies are filtered by level, as in *country > city*. This means only queries matching all levels will be left in the list.

| Parameter   | Value             | Default |
| ----------- | ----------------- | ------- |
| `countries` | list of countries | -       |
| `cities`    | list of cities    | -       |
| `weight`    | 0-65535           | 100     |
| `offline`   | Hide, Show, Only  | Hide    |
| `style`     | V4, V6, Hostname  | V4      |
| `scheme`    | true, false       | false   |
| `port`      | true, false       | false   |

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