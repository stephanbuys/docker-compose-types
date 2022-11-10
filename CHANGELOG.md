Changelog
=========

## v0.3.1
(Thanks to Julian Scheid)
- Add support for additional types of environment variables for a service

## v0.3.0
(Thanks to Julian Scheid)
- support for latest build settings

## v0.2.5
- Update outdated dependencies

## v0.2.4
(Thanks to Thomas Da Rocha)
- Handle healthcheck start_period field

## v0.2.2
(Thanks to Thomas Da Rocha)
- make the indexmap dependency optional

## v0.2.1
(Thanks to Thomas Da Rocha)
- make remaining fields of note visible

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
- Added and tweaked the test files from the main [docker-compose](https://github.com/docker/compose/tests/fixtures) project