#!/usr/bin/env bash
# Common utilities and functions for the setup wizard

REQUIRED_CMDS=(fzf lsblk pacstrap genfstab arch-chroot)

check_requirements() {
	for cmd in "${REQUIRED_CMDS[@]}"; do
		if ! command -v "$cmd" >/dev/null 2>&1; then
			echo "Required command '$cmd' not found. Please install it in the live environment and re-run." >&2
			return 1
		fi
	done
	return 0
}

confirm() {
	local prompt="$1"
	read -r -p "$prompt [y/N]: " ans
	case "$ans" in
		[Yy]|[Yy][Ee][Ss]) return 0 ;;
		*) return 1 ;;
	esac
}

check_root() {
	if [[ $EUID -ne 0 ]]; then
		echo "This script must be run as root." >&2
		return 1
	fi
	return 0
}
