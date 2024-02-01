Table of Contents

- [Code Style Guide](#code-style-guide)
- [Committing](#Committing)
- [Forking and Branches](#forking-and-branches)

# Code Style Guide

## Rust

`cargo fmt` should be run before every commit that touches code.

# Committing

It is preferred that commits are signed with GPG.

Commits should follow the following format:

```
subsystem: active action taken

This patch does this thing. This is why it was necessary.
This is how it improves ACC.

Signed-off-by: Committer Name <commiter@email.tld>
```

If your patches were co-developed, add at the end a `Co-developed-by` line.
If they were reviewed by someone, add a `Reviewed-by` line.
If they fix a GitHub issue, add a `Fixes: #NUM` line.

When reasonably possible, patches should be rebased. Failing that, make a squash commit.

# Forking and Branches

When forking, do NOT edit your main branch!
This needs to remain unchanged to sync with the main repo.

Instead, after syncing, branch off your main for each patch.
You can send PRs directly from the branch into repo-main.
