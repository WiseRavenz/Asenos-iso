#!/usr/bin/env bash
# System installation module for the setup wizard

install_base_system() {
	echo "Installing base packages to /mnt"
	read -r -p "Enter any extra packages to install with pacstrap (space separated), or press ENTER for defaults: " extra
	local pkgs=(base linux linux-firmware vim networkmanager fzf)
	if [[ -n "$extra" ]]; then
		pkgs+=( $extra )
	fi
	pacstrap /mnt "${pkgs[@]}"
	genfstab -U /mnt >> /mnt/etc/fstab

	echo "Setting up basic system configuration inside chroot..."
	read -r -p "Enter hostname for the new system: " newhost
	arch-chroot /mnt /bin/bash -c "echo '$newhost' > /etc/hostname"

	# set locale to en_US.UTF-8 by default
	arch-chroot /mnt /bin/bash -c 'echo -e "en_US.UTF-8 UTF-8" > /etc/locale.gen && locale-gen'
	arch-chroot /mnt /bin/bash -c 'ln -sf /usr/share/zoneinfo/UTC /etc/localtime && hwclock --systohc'

	# enable NetworkManager
	arch-chroot /mnt /bin/bash -c 'systemctl enable NetworkManager'

	echo "Base installation complete. You can arch-chroot /mnt for further configuration (users, bootloader, etc.)."
}

configure_timezone() {
	local timezone
	timezone=$(timedatectl list-timezones | fzf --height 40% --border --prompt='Timezone> ')
	if [[ -n "$timezone" ]]; then
		echo "Setting timezone to $timezone"
		arch-chroot /mnt /bin/bash -c "ln -sf /usr/share/zoneinfo/$timezone /etc/localtime && hwclock --systohc"
	else
		echo "No timezone selected, keeping UTC"
	fi
}

configure_locale() {
	local locale
	locale=$(grep -E "^#.*UTF-8" /etc/locale.gen | sed 's/^#//' | fzf --height 40% --border --prompt='Locale> ')
	if [[ -n "$locale" ]]; then
		echo "Setting locale to $locale"
		arch-chroot /mnt /bin/bash -c "echo '$locale' > /etc/locale.gen && locale-gen"
		arch-chroot /mnt /bin/bash -c "echo 'LANG=${locale%% *}' > /etc/locale.conf"
	else
		echo "No locale selected, keeping en_US.UTF-8"
	fi
}

create_user() {
	read -r -p "Enter username for new user: " username
	if [[ -n "$username" ]]; then
		arch-chroot /mnt /bin/bash -c "useradd -m -G wheel '$username'"
		echo "Setting password for user $username:"
		arch-chroot /mnt /bin/bash -c "passwd '$username'"
		
		# Enable sudo for wheel group
		arch-chroot /mnt /bin/bash -c "sed -i 's/^# %wheel ALL=(ALL:ALL) ALL$/%wheel ALL=(ALL:ALL) ALL/' /etc/sudoers"
	fi
}

install_bootloader() {
	echo "Installing systemd-boot bootloader..."
	arch-chroot /mnt /bin/bash -c 'bootctl install'
	
	# Create boot entry
	cat > /mnt/boot/loader/entries/arch.conf << EOF
title   Arch Linux
linux   /vmlinuz-linux
initrd  /initramfs-linux.img
options root=PARTUUID=$(blkid -s PARTUUID -o value ${ROOT_PART:-$(findmnt -n -o SOURCE /mnt)}) rw
EOF

	# Configure loader
	cat > /mnt/boot/loader/loader.conf << EOF
default arch.conf
timeout 4
console-mode max
editor  no
EOF
	
	echo "Bootloader installed successfully."
}

# If script is run directly, provide installation options
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
	echo "System Installation Menu"
	echo "1. Install base system"
	echo "2. Configure timezone"
	echo "3. Configure locale" 
	echo "4. Create user"
	echo "5. Install bootloader"
	read -r -p "Choice: " choice
	case $choice in
		1) install_base_system ;;
		2) configure_timezone ;;
		3) configure_locale ;;
		4) create_user ;;
		5) install_bootloader ;;
		*) echo "Invalid choice" ;;
	esac
fi