#!/usr/bin/env bash
# Keymap selection module for the setup wizard

select_keymap() {
	echo "Selecting keymap..."
	local keymap
	if command -v localectl >/dev/null 2>&1; then
		keymap=$(localectl list-keymaps | fzf --height 40% --border --prompt='Keymap> ')
	else
		# fallback to kbd keymaps
		keymap=$(find /usr/share/kbd/keymaps -type f -name '*.map.gz' | sed 's|/usr/share/kbd/keymaps/||;s|\.map\.gz$||;s|/| |g' | fzf --height 40% --border --prompt='Keymap> ')
	fi
	if [[ -n "$keymap" ]]; then
		echo "Applying keymap: $keymap"
		if command -v loadkeys >/dev/null 2>&1; then
			loadkeys "$keymap" || true
		fi
		if command -v localectl >/dev/null 2>&1; then
			localectl set-keymap "$keymap" || true
		fi
	else
		echo "No keymap selected; skipping.";
	fi
}

# If script is run directly, execute the function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
	select_keymap
fi