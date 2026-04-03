# Security Policy

## Supported Versions

Currently supported versions of Docker Project Manager:

| Version | Supported          |
| ------- | ------------------ |
| 1.6.x   | :white_check_mark: |
| 1.5.x   | :white_check_mark: |
| 1.4.x   | :white_check_mark: |
| < 1.4   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability, please send an email to:

**Security Contact**: Please open a GitHub Security Advisory

We take security vulnerabilities seriously. Please include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Any fixes (if known)

We aim to respond within 48 hours and will keep you updated on progress.

## Security Best Practices

When contributing to Docker Project Manager:

1. Never commit credentials or secrets
2. Use environment variables for sensitive configuration
3. Follow the principle of least privilege
4. Sanitize all user inputs
5. Keep dependencies updated

## Dependencies

We use `cargo-deny` to monitor dependencies for vulnerabilities.
Run security checks locally:

```bash
cargo deny check
```

## Third-Party Libraries

All third-party libraries are reviewed for:

- License compatibility (GPLv3 compatible)
- Security vulnerabilities
- Maintenance status
- Active development

## Security Updates

Security updates are released as patch versions and are prioritized over
feature development.

## Acknowledgments

Thank you to all contributors who help identify and fix security issues.