#!/usr/bin/env bash
# shellcheck disable=SC2034

iso_name="asenos"
iso_label="ASENOS_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="Asenos Linux"
iso_application="Asenos Linux Live/Rescue Environment"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)"
install_dir="asenos"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito'
           'uefi-ia32.systemd-boot.esp' 'uefi-x64.systemd-boot.esp'
           'uefi-ia32.systemd-boot.eltorito' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
bootstrap_tarball_compression=('zstd' '-c' '-T0' '--auto-threads=logical' '--long' '-19')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/root"]="0:0:750"
  ["/root/.automated_script.sh"]="0:0:755"
  ["/root/.gnupg"]="0:0:700"
  ["/usr/bin/setupwizard-service.sh"]="0:0:755"
  ["/usr/bin/setupwizard.sh"]="0:0:755"
  ["/usr/bin/setup_wifi.sh"]="0:0:755"
  ["/usr/bin/select_keymap.sh"]="0:0:755"
  ["/usr/bin/partition.sh"]="0:0:755"
  ["/usr/bin/install_system.sh"]="0:0:755"
  ["/usr/bin/common.sh"]="0:0:755"
)
