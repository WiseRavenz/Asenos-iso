## Asenos ISO

Asenos ISO provides scripts and configuration to build bootable ISO images for Asenos Linux. It is based on the upstream Arch Linux project `archiso` and aims to produce a guided, mostly-automated live/install image.

### Key goals
- Produce reproducible, bootable ISO images for Asenos Linux.
- Make the Arch installation process simpler and more structured.
- Provide a proper infrastructure to install Asenos dotfiles (separate repo) or let users install a vanilla Arch system.
- Offer simple test targets for BIOS and UEFI using QEMU.

### How it works

The ISO contains a small live system that launches a terminal-based setup wizard (see `docs/installation_flowchart.svg`). The wizard guides users through installation steps and, at the end, offers to install Asenos dotfiles (a separate repository) or to continue with a standard Arch installation.

### Installation steps

**Prerequisites**:
- Familiarity with entering firmware/UEFI settings.
- A target disk (USB stick or similar) to write the ISO.

1. Download the latest ISO from the repository releases.
2. Write the ISO to a disk with balenaEtcher or a similar tool.
3. Reboot and enter firmware/UEFI settings.
4. If present, disable Secure Boot.
5. (Optional, recommended) Use UEFI mode by disabling CSM / Legacy boot.
6. Ensure the live-ISO disk is the first boot option.
7. Boot into the live ISO and follow the on-screen wizard.
8. When installation finishes, remove the live disk and reboot into the installed system.

## Development

### Prerequisites

- A Linux host (development and build scripts are written for Linux; Arch Linux is recommended).
- sudo or root access (required for some build steps and cleanup).
- GNU Make.
- QEMU (optional, for running test images).

*The build process uses pacman and various Arch-specific packages. Other distributions may work but will likely require manual package and configuration adjustments.*
*Containerazition support can be added in future...*

### Quick start

Clone the repository and enter it:

```zsh
git clone https://github.com/WiseRavenz/Asenos-iso.git
cd Asenos-iso
```

Build the ISO (requires sudo because some files are created with root-owned paths):

```zsh
sudo make build
```

### Building details

- `make build` runs the pipeline that prepares a working directory, customizes an airootfs, and assembles the final ISO image(s).
- Artifacts are written to `out/` and build intermediates are stored in `work/`.

### Testing

Run the generated images in QEMU. Make sure `qemu-system-x86_64` (and related packages) are installed on the host.

UEFI test (QEMU will boot the UEFI image):

```zsh
make test-uefi
```

BIOS test:

```zsh
make test-bios
```

### Cleanup

To remove build artifacts and working directories (this deletes `work/` and `out/`):

```zsh
sudo make clean
```

### Contributing

Contributions are welcome. Read `CONTRIBUTING.md` before opening issues or pull requests. The build system is driven by scripts under `scripts/` and configuration files under `iso/`.

Notes & troubleshooting
- If a build step fails due to missing packages, install the packages listed in `iso/bootstrap_packages.x86_64` and `iso/packages.x86_64`.
- If you need to inspect or customize the live filesystem, review `iso/airootfs/` and `scripts/`.
- For QEMU test failures, confirm `qemu-system-x86_64` is available and that virtualization is enabled on the host.

### License

Asenos-iso is released under the GNU General Public License v3.0 (GPL-3.0). See `LICENSE` for details.

### Acknowledgements

This project takes inspiration from the upstream Arch Linux `archiso` project.
