# Audit connected services, AI and agent feature coverage

Parent: ../map.md
Type: research
Label: wayfinder:research
Status: resolved
Assignee: /root/connected_audit
Blocked by: none

## Question

Which reference capabilities require provider APIs, account/team services, AI providers or coding-agent integrations, and what equivalent self-hosted service boundaries are needed to preserve their observable behavior?

## Research contract

Read the installed research skill. Use primary reference documentation and first-party API documentation where needed. Produce stable feature identifiers with evidence, observable behavior, provider/edition constraints, service dependencies, and validation needs. Cover integrations and provider variants, pull requests including stacks/review, issues, Launchpad, workspaces/Insights, sharing/patches, organization/profile/team workflows, AI, agents/status/permissions and hosted/on-premise differences. Distinguish known facts from proposed equivalents, inaccessible vendor service details and unresolved decisions. Do not drop a feature because it needs a service or paid reference account. Avoid claiming a public drop-in GitKraken cloud API exists without evidence.

Deliver docs/research/connected-parity.md on research/connected-parity. Do not modify shared tracker files or resolve this ticket in the research worktree. Report the commit and important remaining gaps to the parent agent.

## Answer

Resolved October 30, 2026. [Connected feature parity research](../../../docs/research/connected-parity.md) contains 133 stable documented entries spanning provider integrations, PRs/stacks, issues, workspaces, Launchpad, sharing, teams, AI, agents and deployment. Research commit: f415ad19587c3703d3913ac91230f08165f64cc9 on research/connected-parity; integrated into main.

Release notes establish the browser Code Review handoff and cross-device account/integration changes. Equivalent self-hosted services must supply cloud-connected behaviors beyond GitKraken's reduced on-premise feature set. Undocumented stack/agent protocols, exact reference controls, external web destinations and product-level account/licensing choices remain explicit gaps. All entries remain unimplemented. This closes the bounded source audit, not the service-boundary or acceptance decisions.
