#!/usr/bin/env bash
# Test framework for setup wizard modules

# Set PWD to project root (one level above this scripts directory)
PWD="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRIPT_DIR="$PWD/iso/airootfs/usr/bin"
TEST_RESULTS=()

# Mock functions for testing
mock_fzf() {
    # Return first line of input for testing
    head -n 1
}

mock_confirm() {
    # Always return yes for automated testing
    return 0
}

mock_command() {
    # Mock command -v to always succeed
    return 0
}

run_test() {
    local test_name="$1"
    local test_function="$2"
    
    echo "Running test: $test_name"
    
    # Override functions for testing
    export -f mock_fzf mock_confirm mock_command
    alias fzf='mock_fzf'
    alias confirm='mock_confirm'
    
    if $test_function; then
        echo "✓ PASS: $test_name"
        TEST_RESULTS+=("PASS: $test_name")
    else
        echo "✗ FAIL: $test_name"
        TEST_RESULTS+=("FAIL: $test_name")
    fi
    
    unalias fzf confirm 2>/dev/null || true
}

test_common_module() {
    source "$SCRIPT_DIR/common.sh"
    
    # Test that functions exist
    for func in check_requirements confirm check_root; do
        if ! declare -F "$func" >/dev/null; then
            return 1
        fi
    done
    
    return 0
}

test_keymap_module() {
    # Test that the module can be sourced without errors
    source "$SCRIPT_DIR/select_keymap.sh" || return 1
    
    # Test that the function exists
    if ! declare -F select_keymap >/dev/null; then
        return 1
    fi
    
    return 0
}

test_wifi_module() {
    # Test that the module can be sourced without errors
    source "$SCRIPT_DIR/setup_wifi.sh" || return 1
    
    # Test that the function exists
    if ! declare -F setup_wifi >/dev/null; then
        return 1
    fi
    
    return 0
}

test_partition_module() {
    # Test that the module can be sourced without errors
    source "$SCRIPT_DIR/partition.sh" || return 1
    
    # Test that functions exist
    for func in select_disk partition_guided partition_manual format_and_mount; do
        if ! declare -F "$func" >/dev/null; then
            return 1
        fi
    done
    
    return 0
}

test_install_module() {
    # Test that the module can be sourced without errors
    source "$SCRIPT_DIR/install_system.sh" || return 1
    
    # Test that functions exist
    for func in install_base_system configure_timezone configure_locale create_user install_bootloader; do
        if ! declare -F "$func" >/dev/null; then
            return 1
        fi
    done
    
    return 0
}

test_main_wizard() {
    # Test individual functions exist by sourcing modules directly
    source "$SCRIPT_DIR/common.sh" || return 1
    source "$SCRIPT_DIR/select_keymap.sh" || return 1
    source "$SCRIPT_DIR/setup_wifi.sh" || return 1
    source "$SCRIPT_DIR/partition.sh" || return 1
    source "$SCRIPT_DIR/install_system.sh" || return 1
    
    # Now test the main menu function from setupwizard.sh
    if ! grep -q "main_menu()" "$SCRIPT_DIR/setupwizard.sh"; then
        return 1
    fi
    
    return 0
}

# Run all tests
echo "Starting Setup Wizard Module Tests"
echo "================================="

run_test "Common module" test_common_module
run_test "Keymap module" test_keymap_module
run_test "WiFi module" test_wifi_module
run_test "Partition module" test_partition_module
run_test "Install module" test_install_module
run_test "Main wizard" test_main_wizard

echo
echo "Test Results:"
echo "============="
for result in "${TEST_RESULTS[@]}"; do
    echo "$result"
done

# Count results
PASS_COUNT=$(printf '%s\n' "${TEST_RESULTS[@]}" | grep -c "PASS" || true)
FAIL_COUNT=$(printf '%s\n' "${TEST_RESULTS[@]}" | grep -c "FAIL" || true)

echo
echo "Summary: $PASS_COUNT passed, $FAIL_COUNT failed"

if [[ $FAIL_COUNT -eq 0 ]]; then
    echo "All tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
