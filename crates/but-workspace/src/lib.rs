#![deny(missing_docs, rust_2018_idioms)]
#![deny(clippy::indexing_slicing)]

//! ### Terminology
//!
//! * **Workspace**
//!     - A GitButler concept of the combination of one or more branches into one worktree. This allows
//!       multiple branches to be perceived in one worktree, by merging multiple branches together.
//!     - Currently, there is only one workspace per repository, but this is something we intend to change
//!       in the future to facilitate new use cases.
//!
//! * **Stack**
//!   - GitButler implements the concept of a branch stack. This is essentially a collection of "heads"
//!     (pseudo branches) that contain each other.
//!   - Always contains at least one branch.
//!   - High level documentation here: https://docs.gitbutler.com/features/stacked-branches
//!

use anyhow::Result;
use bstr::BString;
use gitbutler_id::id::Id;
use gitbutler_stack::VirtualBranchesHandle;
use std::path::Path;

/// Represents a lightweight version of a [`gitbutler_stack::Stack`] for listing.
#[derive(Debug, Clone)]
pub struct StackEntry {
    /// The ID of the stack.
    pub id: Id<gitbutler_stack::Stack>,
    /// The list of the branch names that are part of the stack.
    /// The list is never empty.
    /// The first entry in the list is always the most recent branch on top the stack.
    pub branch_names: Vec<BString>,
}

/// Returns the list of stacks that are currently part of the workspace.
/// If there are no applied stacks, the returned Vec is empty.
/// If the GitButler state file in the provided path is missing or invalid, an error is returned.
///
/// - `gb_dir`: The path to the GitButler state for the project. Normally this is `.git/gitbutler` in the project's repository.
pub fn stacks(gb_dir: &Path) -> Result<Vec<StackEntry>> {
    let state = state_handle(gb_dir);
    Ok(state
        .list_stacks_in_workspace()?
        .into_iter()
        .map(|stack| StackEntry {
            id: stack.id,
            branch_names: stack.heads().into_iter().map(Into::into).collect(),
        })
        .collect())
}

/// Represents the state a commit could be in for the purposes of rendering the UI.
#[derive(Debug, Clone)]
pub enum CommitState {
    /// The commit is only local
    LocalOnly,
    /// The commit is also present at the remote [`StackBranch::upstream_refrence`].
    /// This is the commit state if:
    ///  - The commit has been pushed to the remote
    ///  - The commit has been copied from a remote commit (when applying a remote branch)
    ///
    /// This variant carries the remote commit id.
    /// The `remote_commit_id` may be the same as the `id` or it may be different if the local commit has been rebased or updated in another way.
    LocalAndRemote(gix::ObjectId),
    /// The commit is considered integrated.
    /// This should happen when this commit or the contents of this commit is already part of the base.
    Integrated,
}

/// Commit that is a part of a [`StackBranch`] and, as such, containing state derived in relation to the specific branch.
#[derive(Debug, Clone)]
pub struct StackBranchCommit {
    /// The OID of the commit.
    pub id: gix::ObjectId,
    /// The message of the commit.
    pub message: BString,
    /// Whether the commit is in a conflicted state.
    /// Conflicted state of a commit is a GitButler concept.
    /// GitButler will perform rebasing/reordering etc without interruptions and flag commits as conflicted if needed.
    /// Conflicts are resolved via the Edit Mode mechanism.
    pub has_conflicts: bool,
    /// Represents wether the the commit is considered integrated, local only,
    /// or local and remote with respect to the branch it belongs to.
    /// Note that remote only commits in the context of a branch are expressed with the [`UpstreamCommit`] struct instead of this.
    pub state: CommitState,
}

/// Commit that is only at the remote.
/// Unlike the `Commit` struct, there is no knowledge of GitButler concepts like conflicted state etc.
#[derive(Debug, Clone)]
pub struct UpstreamCommit {
    /// The OID of the commit.
    pub id: gix::ObjectId,
    /// The message of the commit.
    pub message: BString,
}

/// Replesents a branch in a [`gitbutler_stack::Stack`]. It contains commits derived from the local pseudo branch and it's respective remote
#[derive(Debug, Clone)]
pub struct StackBranch {
    /// The name of the branch.
    pub name: BString,
    /// Upstream reference, e.g. `refs/remotes/origin/base-branch-improvements`
    pub upstream_refrence: Option<BString>,
    /// List of commits beloning to this branch. Ordered from newest to oldest.
    /// Created from the local pseudo branch (head currently stored in the TOML file)
    pub commits: Vec<StackBranchCommit>,
    /// List of commits that exist **only** on the upstream branch. Ordered from newest to oldest.
    /// Created from the tip of the local tracking branch eg. refs/remotes/origin/my-branch -> refs/heads/my-branch
    pub upstream_commits: Vec<UpstreamCommit>,
    /// Description of the branch.
    /// This is provided by the UI and can include arbitrary utf8 data, eg. markdown etc.
    pub description: Option<String>,
    /// The pull(merge) request associated with the branch, or None if no such entity has not been created.
    /// This is provided by the UI which is also responsible for creating Pull Requests.
    pub pr_number: Option<usize>,
}

/// Provides the relevant details of a particular [`gitbutler_stack::Stack`]
pub fn stack_branches() -> Result<()> {
    Ok(())
}

fn state_handle(gb_state_path: &Path) -> VirtualBranchesHandle {
    VirtualBranchesHandle::new(gb_state_path)
}
