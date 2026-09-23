# Maintainer: LuYishan-4
pkgname=cosmic-files-niri
pkgver=1.9.0.r1
pkgrel=1
pkgdesc='COSMIC Files customized for Niri and NyxNiri'
arch=('x86_64')
url='https://github.com/LuYishan-4/cosmic-files-niri'
license=('GPL-3.0-only')
depends=(
  'cosmic-icon-theme'
  'glib2'
  'glibc'
  'gvfs'
  'libgcc'
  'libxkbcommon'
  'xdg-utils'
  'zstd'
)
makedepends=(
  'cargo'
  'clang'
  'git'
  'just'
  'lld'
)
provides=('cosmic-files')
conflicts=('cosmic-files')
source=("git+${url}.git")
b2sums=('SKIP')

pkgver() {
  cd "$srcdir/cosmic-files-niri"
  printf '1.9.0.r%s.%s' "$(git rev-list --count HEAD)" "$(git rev-parse --short=8 HEAD)"
}

build() {
  cd "$srcdir/cosmic-files-niri"
  export CARGO_TARGET_DIR=target
  export CARGO_PROFILE_RELEASE_LTO=thin
  export RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-fuse-ld=lld"
  just build-release --locked
}

check() {
  cd "$srcdir/cosmic-files-niri"
  cargo test --locked --no-default-features
}

package() {
  cd "$srcdir/cosmic-files-niri"

  install -Dm0755 target/release/cosmic-files     "$pkgdir/usr/bin/cosmic-files"
  install -Dm0755 target/release/cosmic-files-applet     "$pkgdir/usr/bin/cosmic-files-applet"
  install -Dm0755 target/release/cosmic-files-thumbnailer     "$pkgdir/usr/bin/cosmic-files-thumbnailer"

  install -Dm0644 target/xdgen/com.system76.CosmicFiles.desktop     "$pkgdir/usr/share/applications/com.system76.CosmicFiles.desktop"
  install -Dm0644 target/xdgen/com.system76.CosmicFiles.metainfo.xml     "$pkgdir/usr/share/metainfo/com.system76.CosmicFiles.metainfo.xml"
  install -Dm0644 res/com.system76.CosmicFiles.thumbnailer     "$pkgdir/usr/share/thumbnailers/com.system76.CosmicFiles.thumbnailer"

  local icon
  for icon in res/icons/hicolor/*/apps/com.system76.CosmicFiles.svg; do
    local size
    size="$(basename "$(dirname "$(dirname "$icon")")")"
    install -Dm0644 "$icon"       "$pkgdir/usr/share/icons/hicolor/$size/apps/com.system76.CosmicFiles.svg"
  done

  install -Dm0755 scripts/cosmic-files-niri-setup     "$pkgdir/usr/bin/cosmic-files-niri-setup"

  install -Dm0644 LICENSE     "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm0644 NYXNIRI.md     "$pkgdir/usr/share/doc/$pkgname/NYXNIRI.md"
}
