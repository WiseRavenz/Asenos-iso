#!/usr/bin/env bash
set -euo pipefail

# Wrapper script to run the Asenos setup wizard in a terminal
# This script is called by the asenos-setupwizard@.service

TTY_DEVICE="$1"

# Ensure we have full control of the TTY
exec 0< "$TTY_DEVICE"
exec 1> "$TTY_DEVICE"
exec 2> "$TTY_DEVICE"

# Set up the terminal environment
export TERM=linux
stty sane

# Clear the screen and reset the terminal
clear
reset

# Make sure we're in the root directory
cd /

# Display welcome message
echo 
echo "========================================"
echo "       Welcome to Asenos Linux!        "
echo "========================================"
echo
echo "Starting Asenos Setup Wizard..."
echo "The wizard will guide you through the installation process."
echo "It will continue running until all steps are completed."
echo
sleep 2

echo "Running Asenos Setup Wizard..."
echo

# Run the Asenos setup wizard
if [ -x /usr/bin/setupwizard ]; then
  /usr/bin/setupwizard
  echo
  echo "Setup wizard has finished."
else
  echo
  echo "Setup wizard not found or not executable."
fi

echo "Press any key to continue..."
read -n 1 -s < "$TTY_DEVICE"
echo
