# Known Limitations (Self-host)

This list tracks intentionally deferred or optional behaviors in the current FreeFlowy self-host flow.

## Product/feature limitations

- Guest-sharing entitlement behavior is not a primary target of this fork and may differ from managed cloud behavior.
- Hosted plan upgrade flows are intentionally de-emphasized for self-host usage.

## AI service optionality

- AI features depend on external model/provider configuration.
- If AI provider keys are not configured, core workspace functionality still works and AI-specific features may be unavailable.

## Identifier and compatibility scope

- Internal identifiers and compatibility keys with `appflowy` naming (env prefixes, URL scheme, package IDs) are still present where required for compatibility.
- The first-pass branding effort prioritizes user-visible text and assets, not all internal identifiers.

## Packaging and platform metadata

- Some platform packaging identifiers (for example Linux desktop IDs and deep link schemes) remain legacy-compatible and are not fully renamed yet.

## Documentation status

- Documentation now covers the current local and VPS deployment flow in this fork.
- Continue treating this file as the source of truth for deferred items while Phase 3/4 work completes.
