# Execution prototype validation

Observed on Windows x64, October 30, 2026. Source: `prototype/execution-history`; executable source checkpoint: `8ff5faf`. This evidence covers the narrow prototype, not GitKraken parity or the production acceptance criteria.

## Question and result

Can Rust own actual Git execution and its command record while React hides and reopens the activity panel?

Yes, for the implemented noninteractive reads. The command count advanced from 21 to 29 while the panel was collapsed. Reopening it exposed all eight actual status commands and their successful exits. The journal and child ownership live outside the panel component. This supports that interface boundary; it does not settle durable replay, interactive processes, cancellation or repository coordination.

## Build evidence

- TypeScript checking and the Vite production frontend build passed.
- `npm run tauri -- build --debug --no-bundle` passed and produced a Windows executable with embedded frontend assets.
- The built executable launched as a native Tauri window and was exercised directly through Windows UI automation.
- Build toolchain: Rust/Cargo 1.99.1, MSVC 14.29 x64 from Visual Studio 2019 Build Tools, and the installed Windows SDK. The local isolated Rust toolchain required no global PATH changes.
- Dependency resolution is captured in `package-lock.json` and `src-tauri/Cargo.lock`. The frontend pins React 19.3.1, Tauri API/CLI 2.12.4, dialog 2.8.4, Vite 8.3.5 and TypeScript 7.0.2; the Rust lock resolves Tauri 2.12.5.
- An incomplete newer Visual Studio installation initially caused a missing `msvcrt.lib` link failure. Selecting the complete VS2019 environment and linker fixed the build. Downloads for two required Windows crates used a mirror with archive SHA-256 values checked against `Cargo.lock` after direct downloads failed.
- Runtime prerequisites for this executable are Git and Microsoft WebView2. This debug executable is a local preview, not an installer or validated distribution package.

## Actual workflow checks

| Workflow | Observed result |
| --- | --- |
| Inspect the source repository | Actual local branches, clean working state and 18 history commits appeared; six Git reads were recorded. |
| Create disposable demo | Four actual commits, two local branches, a merge, modified `src/theme.css` and untracked `NOTES.md` appeared. Independent Git CLI checks confirmed the commit count and working state. |
| Select a modified file | Inspector showed the actual palette changes. Two selected-file diff commands, including the cached diff, were recorded with literal pathspec handling. |
| Collapse during live checks | Baseline 21 recorded commands; count advanced while collapsed and ended at 29. All eight status reads were inspectable after reopening. |
| Show a real failure | Missing-reference `rev-parse --verify` produced exit 128 and `fatal: Needed a single revision`; the journal retained the failure. |
| Select a commit | Inspector displayed actual `git show --format=fuller` output and the matching theme diff; recorded command count reached 31. |
| Cancel native folder picker | The native Open a Git repository dialog appeared. Cancel preserved the selected demo and commit inspector. |
| Literal filename fixture | A manual CLI fixture with `a[1].txt` and `a1.txt` confirmed `--literal-pathspecs` selected only the requested filename. This was a command-level check, not a full native filename acceptance test. |

The app was left running on the demo repository for user inspection. Its activity history is process-local and will be lost after closing it.

## Review and remaining decisions

Separate Standards and Spec reviews identified two material issues: inherited Git environment could redirect child commands, and selected filenames could be interpreted as pathspec patterns. Both were corrected and reviewed again. Child commands now clear inherited Git routing/config override context, and selected-file diffs use literal pathspecs.

The verdict is limited to a useful native proof of noninteractive execution visibility. G5 remains claimed pending user review and a production contract; G8 and the first production slice remain blocked. Interactive stdin/PTY behavior, shutdown/process-tree cancellation, concurrent repository mutations, bounded/redacted output, durable history, non-Unicode path identity and provider activity are still open. No automated test suite was added to this throwaway prototype.
