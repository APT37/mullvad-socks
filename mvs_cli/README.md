## Mullvad Socks Proxy Filter (CLI)

`mvs-cli [OPTIONS]`

*While you can use multiple types of location identifiers in a single query, it's not advised to do so, as proxies are filtered level by level - `country` > `city` - this means only proxies matching all given levels will be left in the list.*

| Option          | Value                            | Effect                                |
| --------------- | -------------------------------- | ------------------------------------- |
| -C, --countries | countries                        | see '-l countries'                    |
| -c, --cities    | cities                           | see '-l cities'                       |
| -w, --weight    | 0-65535 [default: 100]           | Weight limit (inclusive)              |
| -o, --offline   | hide, show, only [default: hide] | Print offline proxies                 |
| -s, --style     | v4, v6, hostname [default: v4]   | Output type                           |
| -u, --scheme    |                                  | Prepend `socks5://`                   |
| -p, --port      |                                  | Append `:1080`                        |
| -l, --locations | countries, cities, datacenters   | List available locations by type      |
| -j, --json      |                                  | Format output as JSON                 |
| -r, --random    |                                  | Print a single, randomly chosen proxy |
| -h, --help      |                                  | Print help                            |
| -V, --version   |                                  | Print version                         |
