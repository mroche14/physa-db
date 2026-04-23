# Security policy

## Reporting a vulnerability

Please **do not** open public GitHub issues for security vulnerabilities.

Use GitHub's private vulnerability reporting — it keeps the report confidential until a fix ships:

> **Report a vulnerability →** https://github.com/mroche14/physa-db/security/advisories/new

We will acknowledge within 72h. Once the fix is in, we publish a GHSA advisory and credit you (or keep it anonymous, your choice).

## Supported versions

Pre-1.0: only the `main` branch is supported. Post-1.0, a security-support matrix will be published here.

## Scope

In scope:
- The core `physa-*` crates.
- The `physa` daemon and CLI.
- The official Rust client (`physa-client`).

Out of scope for reports (but fixes welcome via PR):
- Dependencies (please report upstream first).
- The dashboard SPA (no sensitive data; it reads a public JSON snapshot).
- Typos in docs.
