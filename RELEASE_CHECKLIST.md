# Release checklist

- [ ] Every SPEC.md requirement has implementation and acceptance evidence
- [ ] QUALITY.md engineering, tests, docs and CI complete
- [ ] Correct full MIT and Apache-2.0 license texts and owner metadata
- [ ] Real repository and documentation metadata verified
- [ ] Crate names available or owned by publisher
- [ ] Supported platforms, MSRV, feature combinations verified
- [ ] Security and performance claims match measured evidence
- [ ] Publication deliberately enabled after completion
- [ ] Versioned workspace dependencies and release order reviewed
- [ ] cargo package --list audited for secrets/unnecessary files
- [ ] cargo package and cargo publish --dry-run passed
- [ ] Cargo authentication available without exposing secrets
- [ ] Published versions confirmed on crates.io
