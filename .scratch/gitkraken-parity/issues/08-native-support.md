# Define Git distribution and native platform support

Parent: ../map.md
Type: grilling
Label: wayfinder:grilling
Status: open
Assignee: unassigned
Blocked by: 01-core-parity-audit.md, 03-runtime-investigation.md

## Question

Which Git distribution/version, operating-system versions and CPU architectures must the application support, and how will native builds be validated in the confirmed Windows-first sequence?

## Decision inputs

Core reference coverage and runtime research. Decide bundled versus discovered system Git, compatibility with user configuration/credential helpers/hooks/signing/LFS, native toolchain setup, required test hosts and packaging scope. The user already confirmed Windows, macOS and Linux remain in scope. This decision makes their support requirements explicit rather than removing platforms. Toolchain presence is not build validation.
