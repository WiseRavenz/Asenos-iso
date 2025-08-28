## Asenos ISO

Asenos ISO contains tools and scripts to build bootable ISO images for Asenos Linux. It is based on the upstream Arch Linux [`archiso`](https://gitlab.archlinux.org/archlinux/archiso/) project and aims to produce a guided, mostly automated live/installer image.

### Key goals:
- Produce reproducible, bootable ISO images for Asenos Linux.
- Provide simple test targets for BIOS and UEFI in QEMU.

### Prerequisites
 - Linux host (this repository was developed for Linux environments)
 - sudo or root access for building and cleaning
 - GNU Make
 - qemu (for running tests)

### Quick start

1. Clone the repository and enter it:

```zsh
git clone https://github.com/WiseRavenz/Asenos-iso.git
cd Asenos-iso
```

2. Build the ISO (requires sudo because files are created with root-owned paths):

```zsh
sudo make build
```

### Building

- The `make build` target drives the build pipeline that prepares a working directory, customizes an airootfs, and assembles the final ISO images.
- Build artifacts are placed in the `out/` directory and intermediate data in `work/`.

### Testing

You can run the generated images in QEMU. Ensure `qemu` and related packages are installed on your host.

Run the UEFI test (QEMU will try to boot the generated UEFI image):

```zsh
make test-uefi
```

Run the BIOS test:

```zsh
make test-bios
```

Cleanup

To remove build artifacts and working directories (this will delete `work/` and `out/`):

```zsh
sudo make clean
```

### Contributing

Contributions are welcome. Please read the contribution guide before opening issues or pull requests. The build system relies on the scripts under `scripts/` and iso files under `iso/`.

Notes & troubleshooting
- If a build step fails due to missing packages, install the packages listed in `configs/bootstrap_packages.x86_64` and `configs/packages.x86_64` as appropriate.
- For test failures in QEMU, confirm `qemu-system-x86_64` is available and that virtualization support is enabled on your host.
- This repository intentionally follows `archiso` conventions; if you need to adapt to another environment, review `configs/airootfs/` and `scripts/`.

### License

Asenos-iso is released under the GNU General Public License v3.0 (GPL-3.0). See the `LICENSE` file for details.

### Acknowledgements

This project takes heavy inspiration from and reuses approaches from the upstream Arch Linux [`archiso`](https://github.com/archlinux/archiso)
