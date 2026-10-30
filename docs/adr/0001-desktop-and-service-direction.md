# Desktop and service direction

Use Tauri with a Rust application core and a React/TypeScript interface. The user selected this approach to combine Rust application logic with the interface flexibility needed for GitKraken Desktop parity; an Electron application and a fully Rust interface were considered earlier in the conversation.

Design connected capabilities for self-hosted services first. This preserves the requested cloud and team workflows while allowing deployment on infrastructure controlled by the user, rather than making a vendor-operated service the initial deployment requirement. Validate Windows first, then macOS and Linux; all three remain in scope.
