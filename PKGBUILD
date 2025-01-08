pkgname=mullvad_socks
pkgver=1.0.0
pkgrel=1
pkgdesc="Filter Mullvad's SOCKS5 proxies by country, city, datacenter (weight and online status are not yet supported)"
arch=('any')
url="https://git.nospy.in/Rust/$pkgname"
depends=()
source=()
sha256sums=()

package() {
  install -Dm755 "$startdir/target/release/mvs_cli" "$pkgdir/usr/bin/mvs_cli"
  install -Dm755 "$startdir/target/release/mvs_web" "$pkgdir/usr/bin/mvs_web"
  install -Dm755 "$startdir/$pkgname.openrc" "$pkgdir/etc/init.d/$pkgname"
  }
