pkgbase=mullvad-socks
pkgname=('mvs-cli' 'mvs-web')
pkgver=1.1.0
pkgrel=6
pkgdesc="Filter Mullvad's SOCKS5 proxies by country, city, datacenter, weight and online status"
arch=('x86_64')
url="https://git.nospy.in/Rust/$pkgname"
backup=("etc/init.d/$pkgname")

package_cli() {
  install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}

package_web() {
  install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm755 "$startdir/mvs_web/service.rc" "$pkgdir/etc/init.d/$pkgname"
}
