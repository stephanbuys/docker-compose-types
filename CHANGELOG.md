Changelog
=========

## v0.9.0

(Thanks to @pbrant)

- Add `cgroup_parent` to `Service`

## v0.8.0

- Bump `indexmap` to `2.2`
- Bump `derive_builder` to `0.20`
- Add `serde_yml` as alternative to `serde_yaml` as the latter is no longer maintained
  (Looking for feedback on this, currently behind `yml` and `yaml` feature flags, `serde_yaml` is still the default)

## v0.7.2

(Thanks to @dustins)

- Make `driver' optional for LoggingParameters

## v0.7.1

(Thanks to @agavrilov)

- Add pull_policy field (and relevant possible values) to Service
- Also: Update dependencies

## v0.7.0

(Thanks to @trijpstra-fourlights and @k9withabone for the contributions)

- Add: cap_drop and userns_mode fields to Service
- Add: group_add to Service

## v0.6.1

(Thanks to @cyqsimon for reporting the issue)

- Resolve issue #31: Cannot mix long syntax and short syntax volume declaration

## v0.6.0

(Thanks to Andrew Thorburn)

- Introduce secrets
- Bump IndexMap dependency to 2.1.0

## v0.5.2

- Bump IndexMap dependency to 2.0.0

## v0.5.1

(Thanks to Paul Nettleton)

- Fix top-level volumes visibility

## v0.5.0

(Thanks to Paul Nettleton)

- Add `tmpfs: Option<Tmpfs>` to `Service`
- Add `bind: Option<Bind>` and `tmpfs: Option<TmpfsSettings>` to `AdvancedVolumes` for bind and tmpfs volume options
- Add enums `SysCtls` and `SysCtlValue` to support `sysctls` option
- Add `Ulimit` enum for single value ulimit and soft/hard mapping
- Makes field `options` on `LoggingParameters` a map
- Add `driver_opts`, `enable_ipv6`, `labels`, and `name` to `NetworkSettings`
- Change `internal` to bool
- Add `driver` to `Ipam`
  ..
  and many more fixes and changes, thanks Paul!

## v0.4.1

(Thanks to Tom Harper)

- Add support for extra hosts in services
- Add tty support

## v0.4.0

(Thanks to Thomas Da Rocha)

- Add defaults to some types
- Avoid serializing some additional empty fields
- Bump minor version as changes are breaking

## v0.3.1

(Thanks to Julian Scheid)

- Add support for additional types of environment variables for a service

## v0.3.0

(Thanks to Julian Scheid)

- Support for latest build settings

## v0.2.5

- Update outdated dependencies

## v0.2.4

(Thanks to Thomas Da Rocha)

- Handle healthcheck start_period field

## v0.2.2

(Thanks to Thomas Da Rocha)

- Make the indexmap dependency optional

## v0.2.1

(Thanks to Thomas Da Rocha)

- Make remaining fields of note visible

## v0.2.0

(Thanks to Atk)

- Remove anyhow, provide own error type for TryFrom
- Simplify is_zero
- Derive more
- Fix clippy lints
- Simplify Service::image and Service::network_mode
- Move test out of lib.rs

## v0.1.7

- Allow 'entrypoint' to be a list
- Add 'init', 'stdin_open' and 'shm_size' properties
- Implement Clone for ComposeFile, SingleService and Compose

## v0.1.6

- Support Extensions (fields starting with "x-")

## v0.1.5

- Allow dns servers to be specified, improve network parsing

## v0.1.4

- Allow external networks to be explicitly disabled

## v0.1.3

- Support the `privileged` flag for a service

## v0.1.2

- Support multiple `env_file`'s

## v0.1.1

- Initial Open Source release of the code
- Added and tweaked the test files from the main [docker-compose](https://github.com/docker/compose/tests/fixtures)
  project