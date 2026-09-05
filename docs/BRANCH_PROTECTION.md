**English** | [简体中文](BRANCH_PROTECTION.zh-CN.md)

> English is normative. If the translations differ, this document takes precedence.

# Protecting the `main` Branch

Repository documentation asks contributors and AI agents not to work directly on `main`, but documentation cannot technically prevent a push. A repository administrator must configure a GitHub ruleset or branch protection rule.

## Recommended GitHub Ruleset

In the repository's **Settings → Rules → Rulesets**, create an active branch ruleset that targets the default branch, `main`.

Enable the following protections:

- Restrict deletions;
- Block force pushes;
- Require a pull request before merging;
- Require at least 1 approval;
- Dismiss stale pull request approvals when new commits are pushed;
- Require conversation resolution before merging;
- Require status checks to pass;
- Require branches to be up to date before merging;
- Select the current CI job named `Check` as a required status check;
- Do not grant bypass permission to administrators, integrations, or broad teams unless an emergency process is separately documented.

If the repository UI displays the workflow and job together, select the check produced by the `CI` workflow's `Check` job after it has run at least once.

## Merge Settings

In **Settings → General → Pull Requests**:

- Enable squash merging;
- Disable merge commits and rebase merging unless maintainers intentionally support them;
- Require an English Conventional Commit style pull request title, because the squash commit derives from it;
- Automatically delete merged branches if that matches the team's retention needs.

Recommended title examples:

```text
feat(board): render playable vertices
fix(rules): preserve state after illegal moves
docs(contributing): clarify AI disclosure
```

## Security Reporting

In **Settings → Security → Private vulnerability reporting**, enable private vulnerability reports so contributors can follow `SECURITY.md` without exposing a vulnerability publicly.

## Verification

After saving the ruleset, use a temporary branch and pull request to confirm that:

1. A direct push to `main` is rejected;
2. A pull request cannot merge without one approval;
3. A new commit dismisses an existing approval;
4. An unresolved review conversation blocks merging;
5. A failing or pending `Check` status blocks merging;
6. Force pushes and branch deletion are rejected;
7. An administrator cannot bypass the rule through the normal merge UI.

Record any intentional exception in this document and in the repository ruleset description.

## Emergency Changes

Urgent fixes still use a short-lived `fix/` branch and pull request. If GitHub availability or a critical incident makes the normal workflow impossible, a maintainer may temporarily adjust protection only with another maintainer's approval and must restore it immediately afterward. Document the exception and perform retrospective review.
