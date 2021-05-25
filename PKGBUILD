# Maintainer: Devin Pohl <pohl.devin@gmail.com>

depends=()

makedepends=('rust')

pkgname=rocket_fs
pkgver=0.0.2
pkgrel=1
pkgdesc="The thing that serves my files"
arch=('x86_64')
url="https://github.com/Shizcow/rocket_fs"
license=('GPL')
provides=($pkgname)
conflicts=($pkgname)
source=("${pkgname}::git+${url}.git")
md5sums=('SKIP')

build() (
    cd $pkgname
    cargo build
)

package() (
  cd $pkgname
  install -Dm644 Rocket.toml "$pkgdir"/opt/rocket_fs/Rocket.toml
  install -Dm644 templates "$pkgdir"/opt/rocket_fs/templates
  install -Dm644 target/release/rocket_fs "$pkgdir"/usr/bin/rocket_fs
)
