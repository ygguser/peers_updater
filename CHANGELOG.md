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
