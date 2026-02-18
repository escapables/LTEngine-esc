---
summary: 'Release checklist for changelog, test gate, build verification, tagging, and GitHub release quality.'
read_when:
  - Preparing a release.
  - Tagging and publishing a release.
  - Curating release notes.
---

# Releasing

## Versioning

Use Semantic Versioning:
- `MAJOR`: breaking API changes.
- `MINOR`: new features and improvements.
- `PATCH`: bug fixes, docs/build fixes.

Tag format: `vX.Y.Z`.

## Changelog

Release notes source is `CHANGELOG.md`.

Before release:
1. Curate `## Unreleased` with user-visible changes only.
2. Move those bullets into `## [X.Y.Z] - YYYY-MM-DD`.
3. Keep a fresh empty `## Unreleased` section at top.

## Release Policy

- Distribution channel: GitHub Releases only.
- Release gate: local build verification completed before tag; CI runs on published release.

## Release Checklist

1. Confirm local build is green:
```bash
cargo build --release
```

2. Run tests:
```bash
cargo test
```

3. Smoke check binary:
```bash
./target/release/ltengine --help
```

4. Commit release metadata:
```bash
git add CHANGELOG.md docs/RELEASING.md docs/HANDOFF.md
git commit -m "chore: release vX.Y.Z"
```

5. Tag:
```bash
git tag vX.Y.Z
```

6. Push branch + tag (when approved):
```bash
git push origin main
git push origin vX.Y.Z
```

7. Publish GitHub release (triggers CI release asset upload):
```bash
gh release create vX.Y.Z --verify-tag --title "LTEngine vX.Y.Z" --notes-file /tmp/release-notes.md
```

## Post-Release

- Update `docs/HANDOFF.md` with release SHA/tag plus build outcomes.
- Verify CI release run completed and attached artifacts:
```bash
gh run list --workflow release --limit 5
gh release view vX.Y.Z
```
- Confirm tag points to intended commit:
```bash
git show --no-patch --decorate vX.Y.Z
```

## GitHub Release Guardrails

- Title format: `LTEngine vX.Y.Z`.
- Release body: curated changelog bullets for that version, verbatim.
- Attach all shipping artifacts for target platforms.
- Verify tag, title, body, and assets after publish; fix mismatches immediately.
