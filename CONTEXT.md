# Git manager

The language used to discuss desktop Git workflows and parity with the reference product.

## Language

**Reference release**:
GitKraken Desktop 12.6.1, the fixed release against which feature coverage and user experience are evaluated. Later releases form separate follow-up work.

**Feature parity**:
Coverage of every feature, tab, function, and observable workflow in the reference release, including capabilities that depend on connected services.
_Avoid_: MVP parity, basic Git parity.

**Action history**:
The record of application actions, including actual Git commands and their results, and activity for actions performed through provider APIs. Collapsing its panel does not stop recording.
_Avoid_: Fake terminal output.

**Interactive terminal**:
A repository- or worktree-associated shell session in which the user can enter commands.

**Self-hosted service**:
A service deployable on infrastructure controlled by the user, supporting capabilities shared across machines or team members.
