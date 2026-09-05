**English** | [简体中文](SECURITY.zh-CN.md)

> English is normative. If the translations differ, this document takes precedence.

# Security Policy

## Supported Versions

HexGo is in early development and has not published a stable release. Security fixes are applied only to the latest code on `main` unless a release announcement explicitly states otherwise.

| Version | Supported |
|---|---|
| Latest `main` | Yes |
| Older commits or development builds | No |

## Reporting a Vulnerability

Do not open a public issue, pull request, or discussion for a suspected vulnerability.

Use GitHub's **Report a vulnerability** option on the repository's Security page. This creates a private vulnerability report visible to the maintainers. If the option is unavailable, contact the repository owner privately through a contact method listed on their GitHub profile and include only enough initial detail to establish a secure follow-up channel.

Include, when available:

- A concise description and affected component;
- Reproduction steps or a minimal proof of concept;
- Expected and observed impact;
- Affected versions, platforms, or configurations;
- Possible mitigations or fixes;
- Whether the issue has been disclosed elsewhere.

Never include secrets, personal data, or data obtained without authorization.

## What to Expect

Maintainers will acknowledge a valid reporting channel as availability permits, assess severity and scope, coordinate a fix, and agree on disclosure timing with the reporter. Early development and volunteer availability may affect response times; no fixed service-level agreement is promised.

Please allow maintainers a reasonable opportunity to investigate and release a fix before public disclosure. Good-faith research that respects user privacy, avoids service disruption, and follows this policy is welcome.

## Security in Contributions

- Keep credentials and personal data out of the repository, logs, screenshots, and test fixtures.
- Do not add a dependency or `unsafe` code without explicit approval and documented justification.
- Report unexpected dependency behavior or exposed secrets privately.
- Never commit a real exploit for an unresolved vulnerability to a public branch.
