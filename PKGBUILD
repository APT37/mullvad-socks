pkgname=mullvad-socks
pkgver=1.1.0
pkgrel=4
pkgdesc="Filter Mullvad's SOCKS5 proxies by country, city, datacenter, weight and online status"
arch=('x86_64')
url="https://git.nospy.in/Rust/$pkgname"
backup=("etc/init.d/$pkgname")

package() {
  install -Dm755 "$startdir/target/release/mvs_cli" "$pkgdir/usr/bin/mvs-cli"
  install -Dm755 "$startdir/target/release/mvs_web" "$pkgdir/usr/bin/mvs-web"
  install -Dm755 "$startdir/mvs_web/service.rc" "$pkgdir/etc/init.d/$pkgname"
}
