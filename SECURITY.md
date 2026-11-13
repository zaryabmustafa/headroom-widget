# Security policy

## Supported versions

Only the [latest release](https://github.com/zaryabmustafa/headroom-widget/releases/latest)
gets security fixes.

## Reporting a vulnerability

Please report it privately through
[GitHub's private vulnerability reporting](https://github.com/zaryabmustafa/headroom-widget/security/advisories/new),
not in a public issue. Include what you found, how to reproduce it, and which version
you used.

You should hear back within a week. Once a fix is released, the advisory is made
public and you're credited unless you'd rather not be.

## What Headroom handles

Headroom reads the login tokens that Claude Code and Codex save on your PC and sends
them only to `api.anthropic.com`, `platform.claude.com` and `chatgpt.com`. It writes
Claude Code's credentials only when it refreshes that login, and it never writes
Codex's `auth.json`. Problems in how it reads, stores or sends those tokens are in scope.
