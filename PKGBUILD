pkgbase=mullvad-socks
pkgname=('mvs-cli' 'mvs-web')
pkgver=1.2.1
pkgrel=2
pkgdesc="Filter Mullvad's SOCKS5 proxies by country, city, datacenter, weight and online status"
arch=('x86_64')
url="https://git.nospy.in/Rust/$pkgbase"

package_mvs-cli() {
  install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}

package_mvs-web() {
  backup=("etc/init.d/$pkgname")

  install -Dm755 "$startdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm755 "$startdir/mvs_web/$pkgname.rc" "$pkgdir/etc/init.d/$pkgname"
}
