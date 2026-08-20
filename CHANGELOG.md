# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
<!-- and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).-->

<!-- Use this as a template
## [X.Y.Z] - YYYY-MM-DD
### Added
- for new features.

### Changed
- for changes in existing functionality.

### Deprecated
- for soon-to-be removed features.

### Removed
- for now removed features.

### Fixed
- for any bug fixes.

### Security
- in case of vulnerabilities.
-->
## [0.3.7] - 2026-08-20

### Changed

- `minreq` replaced with a custom [tiny_http_client](https://github.com/ygguser/tiny_http_client). This eliminates the vulnerable `minreq` 2.6 dependency without significantly increasing the binary size compared to `minreq` 3.
- `regex` replaced with `regex-lite`. Since not all of regex's functionality was required, this change slightly reduces the binary size.

## [0.3.6] - 2026-08-07

### Added
- Added `--dedup`: peer deduplication by address/IP (ygguser/peers_updater#40)

### Contributors

- [@Liniya](https://github.com/Liniya) — Added `--dedup`: peer deduplication by address/IP

## [0.3.5] - 2026-08-02

### Changed
- `--api` command line argument accepts an optional admin API endpoint string (ygguser/peers_updater#35)

### Fixed
- Previously, the node URI was truncated to `proto://host:port`, now the full URI will always be used.
  For example: it used to be `quic://host.eu:8888`, now it will be: `quic://host.eu:8888?key=ec56fa43b8c9e71c1fe31832b87653f23166f2bac12d52c755d9ac68f55dbc46`
- Fixed various Clippy warnings.

### Contributors
- [@Liniya](https://github.com/Liniya) — `--api` command line argument accepts an optional admin API endpoint string
- [@Ygguser](https://github.com/ygguser) — using the full URI

## [0.3.4] - 2024-08-06

### Added

- Added WebSocket-peers support (ws:// | wss://)

## [0.3.3] - 2023-11-07

### Changed

- Сhanged default paths: `/etc/yggdrasil.conf` -> `/etc/yggdrasil/yggdrasil.conf`; `/var/run/yggdrasil.sock` -> `/var/run/yggdrasil/yggdrasil.sock`

## [0.3.2] - 2023-10-28

### Added

- Adaptation for Yggdrasil v. 0.5 (quic:// support)

## [0.3.1] - 2023-03-01

### Fixed

- Error while loading shared libraries (libssl.so) (#10).

## [0.3.0] - 2023-03-01

### Added

- Added the ability to simply disable some functionality during assembly. For more information, see [README.md](README.md#build-from-source) 

### Changed

- The number of dependencies has been reduced and some of them have been replaced with "lighter" alternatives. As a result, smaller binaries are obtained.
- Added a couple of notes about the [build](README.md#build-from-source) from source

## [0.2.0] - 2023-02-23

### Added

- Added the `-S` (`--self_update`) option. 

Starting from this version, it will be possible to update the utility by running it with the `-S` option. This is done for the simplicity and convenience of updating. You will no longer need to manually download the release from GitHub, unpack and manually replace the executable file - the utility will do all this automatically.

**Usage example:**

Checking the version:
```
./peers_updater -V
Yggdrasil peers updater 0.2.0
```
```
./peers_updater -S
New release found: 0.2.0 --> 0.2.1
Downloading...
Extracting...
Replacing binary file...
Done.
```
Checking the version again:
```
./peers_updater -V
Yggdrasil peers updater 0.2.1
```
Here we see that the program has been successfully updated.

The utility with the `-S` option can be run on a schedule (cron, windows scheduler) or manually.

## [0.1.0] - 2023-02-18

### Added

- Added the `-I` (`--ignore_country`) option. Related to #6

### Changed

- Updated versions of dependencies (clap, nu-json, regex, attohttpc)

Some antiviruses may falsely trigger on compressed UPX binary files, so I added uncompressed files as well.

## [0.0.9] - 2023-01-26

### Fixed

- adding one peer twice with different protocols (#5)

Some antiviruses may falsely trigger on compressed UPX binary files, so I added uncompressed files as well.

## [0.0.8] - 2023-01-07

### Changed

- optimizing the size of executable files.

## [0.0.7] - 2022-12-17

### Fixed

- fix incorrect behavior without the `-i` option.

## [0.0.6] - 2022-12-15

### Changed

- If the peer URI contains at least one of the strings that are passed with the `-i` option, it will be ignored (see the example in [README.md](README.md)).

## [0.0.5] - 2022-12-14

### Fixed

- Fix pinging ipv6 nodes (by @parnikkapore)
- Checking conf access only if necessary

## [0.0.4] - 2022-12-13

### Changed

- Comments will no longer be deleted from the configuration file.

## [0.0.3] - 2022-12-10

### Changed

- The work with the admin api has been redesigned, the messages about problems will be more informative.
- Added the `-u` (`--update_cfg`) option. Changes to the configuration file will be made only if this parameter is specified.

## [0.0.2] - 2022-12-02

### Added

- parameter `-i` (`--ignore`) , where value is a space-separated string, in which you can specify the URI of peers that should be ignored

### Changed

- Additional peers (extra) will be added not only in the config, but also using the API
- Minor improvements, optimization

## [0.0.1] - 2022-11-29

### Added

- First commit.
- Initial public release.
