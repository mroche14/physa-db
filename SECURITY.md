# Security policy

## Reporting a vulnerability

Please **do not** open public GitHub issues for security vulnerabilities.

Until a dedicated security contact is set up, report privately to:

- Email: `marvin@dynovant.com`
- Subject line: `physa-db security — <short description>`

We will acknowledge within 72h. Once the fix is in, we will publish a GHSA advisory and credit you (or keep it anonymous, your choice).

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
