#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="open-erase-client"
iso_label="OPEN_ERASE_CLIENT"
iso_publisher="open-erase"
iso_application="open-erase client for securely erasing x86 devices"
iso_version="0.1.0"
install_dir="arch"
buildmodes=('iso')
bootmodes=('bios.syslinux' 'uefi.grub')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="erofs"
airootfs_image_tool_options=('-zlzma,109' -E 'ztailpacking')
bootstrap_tarball_compression=(zstd -c -T0 --long -19)
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/usr/bin/client-x86_64"]="0:0:755"
)
