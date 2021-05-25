# Maintainer: Devin Pohl <pohl.devin@gmail.com>

depends=()

makedepends=('rust')

pkgname=rocket_fs
pkgver=0.0.2
pkgrel=1
pkgdesc="The thing that serves my files"
arch=('x86_64')
url="https://github.com/Shizcow/bad_rocket_fileserver"
license=('GPL')
provides=($pkgname)
conflicts=($pkgname)
source=("${pkgname}::git+${url}.git")
md5sums=('SKIP')

build() (
    cd $pkgname
    cargo build --release
)

package() (
  install -Dm644 "$srcdir"/"$pkgname"/Rocket.toml "$pkgdir"/usr/share/rocket_fs/Rocket.toml
  install -Dm644 -d "$srcdir"/"$pkgname"/templates "$pkgdir"/usr/share/rocket_fs/templates
  install -Dm755 "$srcdir"/"$pkgname"/target/release/rocket_fs "$pkgdir"/usr/bin/rocket_fs
)
