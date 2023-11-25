# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2023-11-25

### Added

- Added prototype library page.
- Added serving of static files.
- Redirect http to https.
- Added favicons.
- Add robots.txt.
- Added healthchecks.io monitoring.

### Fixed
- Broken asset links.

### Changed
- Use systemd for service management.
- Switching to Leptos and unstable rust.
- Switch to nightly-2023-10-01.

### Removed
- Dockerfile and fly.toml.
- Postgres

## [0.1.0] - 2023-10-08

### Added

- Simple Landing page with links.
- fly.toml file for deployment to fly.io.
- Dockerfile for building the using musl.
