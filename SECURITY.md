# Security policy

The 0.1 release line receives security fixes. Report vulnerabilities privately
using the contact information published by the repository owner:
https://github.com/faizanali2k05. If no private channel is listed, open an issue at
https://github.com/faizanali2k05/app-context/issues requesting a private reporting
channel, without disclosing the vulnerability or sensitive details publicly.

Include affected versions/features, a minimal reproducer and impact. Never send
credentials or stored application values. Response and resolution times depend on
maintainer availability; no fixed response SLA is promised.

The crate provides type-safe sharing, not authorization or secret isolation. Review
docs/security.md for the threat model and adapter error policy. Optional frameworks
and their transitive dependencies are checked in security/dependency CI.
