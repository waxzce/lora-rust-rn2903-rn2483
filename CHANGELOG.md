# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased - v0.3.0
### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## v0.2.0
### Added
- `Rn2903::system_version_bytes()`
- `Rn2903::system_factory_reset()`
- `Rn2903::system_module_reset()`
- `Rn2903::mac_pause()` and `::mac_resume()`
- `Rn2903::system_{get, set}_nvm()`
- `Rn2903::radio_set_modulation_mode()`
- `Rn2903::radio_rx()`
- `BadResponse`, `TransceiverBusy`, and `CannotPause` error variants
- `NvmAddress` newtype for representing values that can be passed to NVM functions
- `ModulationMode` enum listing available modulation modes
- Examples directory:
    - LED blink
    - NVM read/write
    - LoRa packet RX

### Changed
- README.md example (now showing packet RX)

### Deprecated

### Removed
- `main.rs` (moved to an example)

### Fixed

### Security

## v0.1.0
### Added

- GNU GPL v3 license
- Cargo metadata
- Basic functionality
- "Blinky" main program

### Changed

### Deprecated

### Removed

### Fixed

### Security
