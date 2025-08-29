#!/usr/bin/env bash
# Disk partitioning module for the setup wizard

# Source common utilities for confirm function
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

# Global variable to store selected disk
DISK=""

select_disk() {
	echo "Select target disk for installation (this will wipe partitions if you choose guided partitioning)."
	local disks
	disks=$(lsblk -dpno NAME,SIZE,MODEL | sed 's/  */\t/g' | fzf --height 40% --border --prompt='Disk> ')
	if [[ -z "$disks" ]]; then
		echo "No disk selected."; return 1
	fi
	# extract device path
	DISK=$(echo "$disks" | awk '{print $1}')
	echo "Selected disk: $DISK"
	return 0
}

partition_guided() {
	if [[ -z "$DISK" ]]; then
		echo "No disk selected. Please select a disk first."
		return 1
	fi
	
	# Guided partitioning: GPT, EFI 1024MiB, swap (size asked), rest root
	echo "Guided partitioning will wipe $DISK."
	confirm "Are you sure you want to continue? This will destroy all data on $DISK" || return 1
	read -r -p "Enter swap size (eg 2G) or leave empty to create no swap: " swapsize
	parted -s "$DISK" mklabel gpt
	parted -s "$DISK" mkpart primary fat32 1MiB 1025MiB
	parted -s "$DISK" set 1 boot on
	if [[ -n "$swapsize" ]]; then
		start=1025               # MiB, where swap should start
        swapsize=${swapsize:-1024}  # MiB, default to 1024 if not set
        end=$((start + swapsize))
        parted -s "$DISK" mkpart primary linux-swap "${start}MiB" "${end}MiB" || true
		# fallback: place swap then root
		parted -s "$DISK" mkpart primary ext4 "${end}MiB" 100% || true
	else
		parted -s "$DISK" mkpart primary ext4 1025MiB 100%
	fi
	sleep 1
}

partition_manual() {
	if [[ -z "$DISK" ]]; then
		echo "No disk selected. Please select a disk first."
		return 1
	fi
	
	echo "Launching parted for manual partitioning. Create partitions and then exit parted to continue."
	parted "$DISK"
}

format_and_mount() {
	if [[ -z "$DISK" ]]; then
		echo "No disk selected. Please select a disk first."
		return 1
	fi
	
	echo "Detecting partitions for $DISK..."
	# detect partitions: look for partition number 1,2,3
	PART1=${DISK}1
	PART2=${DISK}2
	PART3=${DISK}3
	echo "Formatting $PART1 as FAT32 (EFI)"
	mkfs.fat -F32 "$PART1"
	if [[ -b "$PART2" ]]; then
		# heuristics: if size small treat as swap
		echo "Creating swap on $PART2 (if intended)"
		mkswap "$PART2" || true
		swapon "$PART2" || true
	fi
	if [[ -b "$PART3" ]]; then
		echo "Formatting $PART3 as ext4 for root"
		mkfs.ext4 -F "$PART3"
		ROOT_PART="$PART3"
	else
		# if only 2 partitions exist, assume PART2 is root
		echo "Formatting $PART2 as ext4 for root"
		mkfs.ext4 -F "$PART2"
		ROOT_PART="$PART2"
	fi

	echo "Mounting root to /mnt"
	mount "$ROOT_PART" /mnt
	mkdir -p /mnt/boot/efi
	mount "$PART1" /mnt/boot/efi
}

# Export the DISK variable so other modules can use it
export DISK

# If script is run directly, provide a simple menu
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
	echo "Partitioning Menu"
	echo "1. Select disk"
	echo "2. Guided partitioning"
	echo "3. Manual partitioning"
	echo "4. Format and mount"
	read -r -p "Choice: " choice
	case $choice in
		1) select_disk ;;
		2) partition_guided ;;
		3) partition_manual ;;
		4) format_and_mount ;;
		*) echo "Invalid choice" ;;
	esac
fi