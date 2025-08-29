#!/usr/bin/env bash
set -euo pipefail

# Interactive Arch setup wizard for live environment
# Uses fzf for selections (keymap, wifi, disk, etc.)
# Designed to be run from an Arch live ISO as root.

# Get the directory of this script
SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

# Source all modules
source "$SCRIPT_DIR/common.sh"
source "$SCRIPT_DIR/select_keymap.sh"
source "$SCRIPT_DIR/setup_wifi.sh"
source "$SCRIPT_DIR/partition.sh"
source "$SCRIPT_DIR/install_system.sh"

main_menu() {
	PS3="Choose an action> "
	options=(
		"Set keymap" 
		"Configure Wi-Fi" 
		"Select disk" 
		"Partition (guided)" 
		"Partition (manual)" 
		"Format & mount" 
		"Install base system"
		"Configure timezone"
		"Configure locale"
		"Create user"
		"Install bootloader"
		"Exit"
	)
	while true; do
		echo
		select opt in "${options[@]}"; do
			case $REPLY in
				1) select_keymap; break;;
				2) setup_wifi; break;;
				3) select_disk; break;;
				4) partition_guided; break;;
				5) partition_manual; break;;
				6) format_and_mount; break;;
				7) install_base_system; break;;
				8) configure_timezone; break;;
				9) configure_locale; break;;
				10) create_user; break;;
				11) install_bootloader; break;;
				12) echo "Exiting."; return 0;;
				*) echo "Invalid option."; break;;
			esac
		done
	done
}

# Main execution
if ! check_requirements; then
	exit 1
fi

if ! check_root; then
	exit 1
fi

echo "Arch Setup Wizard"
main_menu
