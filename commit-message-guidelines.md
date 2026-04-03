# Commit Message Guidelines

This project follows the Conventional Commits specification for all commit messages.

## Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Types

| Type     | Description                                                  |
|----------|--------------------------------------------------------------|
| `feat`   | A new feature                                                |
| `fix`    | A bug fix                                                    |
| `docs`   | Documentation only changes                                  |
| `style`  | Changes that do not affect the meaning of the code        |
| `refactor` | A code change that neither fixes a bug nor adds a feature |
| `test`   | Adding missing tests or correcting existing tests           |
| `chore`  | Changes to the build process or auxiliary tools            |
| `perf`   | A code change that improves performance                     |
| `ci`     | Changes to CI configuration files and scripts               |
| `build`  | Changes that affect the build system or external dependencies|
| `revert` | Reverts a previous commit                                    |

## Examples

```bash
# Feature
git commit -m "feat: add Docker container management commands"

# Bug fix
git commit -m "fix: resolve path resolution issue on Windows"

# Documentation
git commit -m "docs: update installation instructions"

# Breaking change
git commit -m "feat!: remove deprecated CLI interface"
git commit -m "fix: correct API endpoint URL

BREAKING CHANGE: The API endpoint has changed from /v1 to /v2"
```

## Rules

1. **Lowercase** type (feat, fix, docs)
2. **No trailing period** in subject line
3. **50 character max** for subject line
4. **72 character max** for body lines
5. Use imperative mood ("add" not "added")

## Local Validation

Install commitlint and validate locally:

```bash
npm install
npm run commitlint       # Validate from file
npm run commitlint-ci    # Validate last commit
```

## Why Conventional Commits?

- **Semantic Versioning**: Automated releases based on commit types
- **Searchable History**: Easy to find specific changes
- **Automated Changelog**: Generate changelogs automatically
- **Clear Intent**: Instantly understand what a commit does

For more details, see [conventionalcommits.org](https://www.conventionalcommits.org/).