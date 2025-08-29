#!/usr/bin/env bash
# Wi-Fi setup module for the setup wizard

setup_wifi() {
	echo "Configuring Wi-Fi..."
	if command -v nmcli >/dev/null 2>&1; then
		nmcli device wifi rescan >/dev/null 2>&1 || true
		local ssid
		ssid=$(nmcli -t -f SSID,SIGNAL device wifi list | sed '/^$/d' | awk -F: '{print $1 "\t" $2}' | fzf --height 40% --border --prompt='Wi-Fi SSID> ' | awk -F'\t' '{print $1}')
		if [[ -n "$ssid" ]]; then
			echo "Connecting to $ssid using NetworkManager (you may be prompted for a password)..."
			nmcli device wifi connect "$ssid"
		else
			echo "No network selected; skipping Wi-Fi setup.";
		fi
	elif command -v iwctl >/dev/null 2>&1; then
		local dev
		dev=$(iwctl device list | awk 'NR>1{print $1}' | fzf --height 20% --border --prompt='wireless dev> ')
		if [[ -z "$dev" ]]; then
			echo "No wireless device selected; skipping Wi-Fi."; return
		fi
		echo "Scanning for networks on $dev..."
		iwctl station "$dev" scan
		sleep 1
		local nets
		nets=$(iwctl station "$dev" get-networks | sed '1,2d' | awk '{$1=""; print substr($0,2)}' | fzf --height 40% --border --prompt='Wi-Fi SSID> ')
		if [[ -n "$nets" ]]; then
			echo "Connecting to '$nets' (iwctl will prompt for password if needed)"
			iwctl station "$dev" connect "$nets"
		else
			echo "No network selected.";
		fi
	else
		echo "No supported Wi-Fi client (nmcli or iwctl) found; please connect manually and press ENTER to continue.";
		read -r
	fi
}

# If script is run directly, execute the function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
	setup_wifi
fi