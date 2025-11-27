// Copyright 2020 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod abandon;
mod absorb;
#[cfg(feature = "bench")]
mod bench;
mod bisect;
mod bookmark;
mod commit;
mod config;
mod debug;
mod describe;
mod diff;
mod diffedit;
mod duplicate;
mod edit;
mod evolog;
mod file;
mod fix;
#[cfg(feature = "git")]
mod gerrit;
#[cfg(feature = "git")]
mod git;
mod help;
mod interdiff;
mod log;
mod metaedit;
mod new;
mod next;
mod operation;
mod parallelize;
mod prev;
mod rebase;
mod redo;
mod resolve;
mod restore;
mod revert;
mod root;
mod run;
mod show;
mod sign;
mod simplify_parents;
mod sparse;
mod split;
mod squash;
mod status;
mod tag;
mod undo;
mod unsign;
mod util;
mod version;
mod workspace;

use std::fmt::Debug;

use clap::CommandFactory as _;
use clap::FromArgMatches as _;
use clap::Subcommand as _;
use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap_complete::engine::SubcommandCandidates;
use tracing::instrument;

use crate::cli_util::Args;
use crate::cli_util::CommandHelper;
use crate::command_error::CommandError;
use crate::complete;
use crate::ui::Ui;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default().bold())
    .usage(AnsiColor::Yellow.on_default().bold())
    .literal(AnsiColor::Green.on_default().bold())
    .placeholder(AnsiColor::Green.on_default());

#[allow(dead_code)]
const HELP_HEADING_COMMON_COMMANDS: &str = "Common Commands";
#[allow(dead_code)]
const HELP_HEADING_EDITING_CHANGES: &str = "Editing Changes";
#[allow(dead_code)]
const HELP_HEADING_REWRITING_CHANGES: &str = "Rewriting Changes";
#[allow(dead_code)]
const HELP_HEADING_WORKING_COPY: &str = "Working Copy";
#[allow(dead_code)]
const HELP_HEADING_FILE_OPERATIONS: &str = "File Operations";
#[allow(dead_code)]
const HELP_HEADING_REVIEW_COMMANDS: &str = "Review Commands";
#[allow(dead_code)]
const HELP_HEADING_REFERENCES: &str = "References (Bookmarks & Tags)";
#[allow(dead_code)]
const HELP_HEADING_OPERATION_LOG: &str = "Operation Log";
#[allow(dead_code)]
const HELP_HEADING_WORKSPACE: &str = "Workspace";
#[allow(dead_code)]
const HELP_HEADING_ADVANCED: &str = "Advanced";
#[allow(dead_code)]
const HELP_HEADING_CONFIGURATION_HELP: &str = "Configuration & Help";
#[allow(dead_code)]
const HELP_HEADING_GIT_INTEGRATION: &str = "Git Integration";
#[allow(dead_code)]
const HELP_HEADING_DEVELOPMENT: &str = "Development";

#[derive(clap::Parser, Clone, Debug)]
#[command(styles = STYLES)]
#[command(disable_help_subcommand = true)]
#[command(after_long_help = help::show_keyword_hint_after_help())]
#[command(add = SubcommandCandidates::new(complete::aliases))]
enum Command {
    Abandon(abandon::AbandonArgs),
    Absorb(absorb::AbsorbArgs),
    #[cfg(feature = "bench")]
    #[command(subcommand)]
    Bench(bench::BenchCommand),
    #[command(subcommand)]
    Bisect(bisect::BisectCommand),
    #[command(subcommand)]
    Bookmark(bookmark::BookmarkCommand),
    Commit(commit::CommitArgs),
    #[command(subcommand)]
    Config(config::ConfigCommand),
    #[command(subcommand)]
    Debug(debug::DebugCommand),
    Describe(describe::DescribeArgs),
    Diff(diff::DiffArgs),
    Diffedit(diffedit::DiffeditArgs),
    Duplicate(duplicate::DuplicateArgs),
    Edit(edit::EditArgs),
    #[command(alias = "obslog", visible_alias = "evolution-log")]
    Evolog(evolog::EvologArgs),
    #[command(subcommand)]
    File(file::FileCommand),
    Fix(fix::FixArgs),
    #[cfg(feature = "git")]
    #[command(subcommand)]
    Gerrit(gerrit::GerritCommand),
    #[cfg(feature = "git")]
    #[command(subcommand)]
    Git(git::GitCommand),
    Help(help::HelpArgs),
    Interdiff(interdiff::InterdiffArgs),
    Log(log::LogArgs),
    Metaedit(metaedit::MetaeditArgs),
    New(new::NewArgs),
    Next(next::NextArgs),
    #[command(subcommand)]
    #[command(visible_alias = "op")]
    Operation(operation::OperationCommand),
    Parallelize(parallelize::ParallelizeArgs),
    Prev(prev::PrevArgs),
    Rebase(rebase::RebaseArgs),
    Redo(redo::RedoArgs),
    Resolve(resolve::ResolveArgs),
    Restore(restore::RestoreArgs),
    Revert(revert::RevertArgs),
    Root(root::RootArgs),
    #[command(hide = true)]
    // TODO: Flesh out.
    Run(run::RunArgs),
    Show(show::ShowArgs),
    Sign(sign::SignArgs),
    SimplifyParents(simplify_parents::SimplifyParentsArgs),
    #[command(subcommand)]
    Sparse(sparse::SparseCommand),
    Split(split::SplitArgs),
    Squash(squash::SquashArgs),
    Status(status::StatusArgs),
    #[command(subcommand)]
    Tag(tag::TagCommand),
    Undo(undo::UndoArgs),
    Unsign(unsign::UnsignArgs),
    #[command(subcommand)]
    Util(util::UtilCommand),
    Version(version::VersionArgs),
    #[command(subcommand)]
    Workspace(workspace::WorkspaceCommand),
}

pub fn default_app() -> clap::Command {
    let mut app = Command::augment_subcommands(Args::command())
        // Common/Basic commands - most frequently used
        .mut_subcommand("log", |cmd| cmd.help_heading(HELP_HEADING_COMMON_COMMANDS))
        .mut_subcommand("show", |cmd| cmd.help_heading(HELP_HEADING_COMMON_COMMANDS))
        .mut_subcommand("status", |cmd| cmd.help_heading(HELP_HEADING_COMMON_COMMANDS))
        .mut_subcommand("diff", |cmd| cmd.help_heading(HELP_HEADING_COMMON_COMMANDS))
        .mut_subcommand("commit", |cmd| cmd.help_heading(HELP_HEADING_COMMON_COMMANDS))
        // Creating and editing changes
        .mut_subcommand("new", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        .mut_subcommand("describe", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        .mut_subcommand("edit", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        .mut_subcommand("abandon", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        .mut_subcommand("duplicate", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        .mut_subcommand("restore", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        .mut_subcommand("revert", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        .mut_subcommand("metaedit", |cmd| cmd.help_heading(HELP_HEADING_EDITING_CHANGES))
        // Rewriting history
        .mut_subcommand("split", |cmd| cmd.help_heading(HELP_HEADING_REWRITING_CHANGES))
        .mut_subcommand("squash", |cmd| cmd.help_heading(HELP_HEADING_REWRITING_CHANGES))
        .mut_subcommand("absorb", |cmd| cmd.help_heading(HELP_HEADING_REWRITING_CHANGES))
        .mut_subcommand("rebase", |cmd| cmd.help_heading(HELP_HEADING_REWRITING_CHANGES))
        .mut_subcommand("parallelize", |cmd| cmd.help_heading(HELP_HEADING_REWRITING_CHANGES))
        .mut_subcommand("simplify-parents", |cmd| cmd.help_heading(HELP_HEADING_REWRITING_CHANGES))
        // Working copy navigation
        .mut_subcommand("next", |cmd| cmd.help_heading(HELP_HEADING_WORKING_COPY))
        .mut_subcommand("prev", |cmd| cmd.help_heading(HELP_HEADING_WORKING_COPY))
        // Conflict resolution and file operations
        .mut_subcommand("diffedit", |cmd| cmd.help_heading(HELP_HEADING_REWRITING_CHANGES))
        .mut_subcommand("resolve", |cmd| cmd.help_heading(HELP_HEADING_FILE_OPERATIONS))
        .mut_subcommand("file", |cmd| cmd.help_heading(HELP_HEADING_FILE_OPERATIONS))
        .mut_subcommand("sparse", |cmd| cmd.help_heading(HELP_HEADING_FILE_OPERATIONS))
        // Reviewing changes
        .mut_subcommand("interdiff", |cmd| cmd.help_heading(HELP_HEADING_REVIEW_COMMANDS))
        .mut_subcommand("evolog", |cmd| cmd.help_heading(HELP_HEADING_REVIEW_COMMANDS))
        // Bookmarks and tags - version control references
        .mut_subcommand("bookmark", |cmd| cmd.help_heading(HELP_HEADING_REFERENCES))
        .mut_subcommand("tag", |cmd| cmd.help_heading(HELP_HEADING_REFERENCES))
        // Operation log and history
        .mut_subcommand("operation", |cmd| cmd.help_heading(HELP_HEADING_OPERATION_LOG))
        .mut_subcommand("undo", |cmd| cmd.help_heading(HELP_HEADING_OPERATION_LOG))
        .mut_subcommand("redo", |cmd| cmd.help_heading(HELP_HEADING_OPERATION_LOG))
        // Workspace management
        .mut_subcommand("workspace", |cmd| cmd.help_heading(HELP_HEADING_WORKSPACE))
        .mut_subcommand("root", |cmd| cmd.help_heading(HELP_HEADING_WORKSPACE))
        // Advanced workflows
        .mut_subcommand("bisect", |cmd| cmd.help_heading(HELP_HEADING_ADVANCED))
        .mut_subcommand("fix", |cmd| cmd.help_heading(HELP_HEADING_ADVANCED))
        .mut_subcommand("run", |cmd| cmd.help_heading(HELP_HEADING_ADVANCED))
        .mut_subcommand("sign", |cmd| cmd.help_heading(HELP_HEADING_ADVANCED))
        .mut_subcommand("unsign", |cmd| cmd.help_heading(HELP_HEADING_ADVANCED))
        // Configuration and utility
        .mut_subcommand("config", |cmd| cmd.help_heading(HELP_HEADING_CONFIGURATION_HELP))
        .mut_subcommand("version", |cmd| cmd.help_heading(HELP_HEADING_CONFIGURATION_HELP))
        .mut_subcommand("debug", |cmd| cmd.help_heading(HELP_HEADING_CONFIGURATION_HELP))
        .mut_subcommand("util", |cmd| cmd.help_heading(HELP_HEADING_CONFIGURATION_HELP));

    // Git integration (conditional)
    #[cfg(feature = "git")]
    {
        app = app
            .mut_subcommand("git", |cmd| cmd.help_heading(HELP_HEADING_GIT_INTEGRATION))
            .mut_subcommand("gerrit", |cmd| cmd.help_heading(HELP_HEADING_GIT_INTEGRATION));
    }

    // Development tools (conditional)
    #[cfg(feature = "bench")]
    {
        app = app.mut_subcommand("bench", |cmd| cmd.help_heading(HELP_HEADING_DEVELOPMENT));
    }

    app
}

#[instrument(skip_all)]
pub fn run_command(ui: &mut Ui, command_helper: &CommandHelper) -> Result<(), CommandError> {
    let subcommand = Command::from_arg_matches(command_helper.matches()).unwrap();
    match &subcommand {
        Command::Abandon(args) => abandon::cmd_abandon(ui, command_helper, args),
        Command::Absorb(args) => absorb::cmd_absorb(ui, command_helper, args),
        #[cfg(feature = "bench")]
        Command::Bench(args) => bench::cmd_bench(ui, command_helper, args),
        Command::Bisect(args) => bisect::cmd_bisect(ui, command_helper, args),
        Command::Bookmark(args) => bookmark::cmd_bookmark(ui, command_helper, args),
        Command::Commit(args) => commit::cmd_commit(ui, command_helper, args),
        Command::Config(args) => config::cmd_config(ui, command_helper, args),
        Command::Debug(args) => debug::cmd_debug(ui, command_helper, args),
        Command::Describe(args) => describe::cmd_describe(ui, command_helper, args),
        Command::Diff(args) => diff::cmd_diff(ui, command_helper, args),
        Command::Diffedit(args) => diffedit::cmd_diffedit(ui, command_helper, args),
        Command::Duplicate(args) => duplicate::cmd_duplicate(ui, command_helper, args),
        Command::Edit(args) => edit::cmd_edit(ui, command_helper, args),
        Command::Evolog(args) => evolog::cmd_evolog(ui, command_helper, args),
        Command::File(args) => file::cmd_file(ui, command_helper, args),
        Command::Fix(args) => fix::cmd_fix(ui, command_helper, args),
        #[cfg(feature = "git")]
        Command::Gerrit(sub_args) => gerrit::cmd_gerrit(ui, command_helper, sub_args),
        #[cfg(feature = "git")]
        Command::Git(args) => git::cmd_git(ui, command_helper, args),
        Command::Help(args) => help::cmd_help(ui, command_helper, args),
        Command::Interdiff(args) => interdiff::cmd_interdiff(ui, command_helper, args),
        Command::Log(args) => log::cmd_log(ui, command_helper, args),
        Command::Metaedit(args) => metaedit::cmd_metaedit(ui, command_helper, args),
        Command::New(args) => new::cmd_new(ui, command_helper, args),
        Command::Next(args) => next::cmd_next(ui, command_helper, args),
        Command::Operation(args) => operation::cmd_operation(ui, command_helper, args),
        Command::Parallelize(args) => parallelize::cmd_parallelize(ui, command_helper, args),
        Command::Prev(args) => prev::cmd_prev(ui, command_helper, args),
        Command::Rebase(args) => rebase::cmd_rebase(ui, command_helper, args),
        Command::Redo(args) => redo::cmd_redo(ui, command_helper, args),
        Command::Resolve(args) => resolve::cmd_resolve(ui, command_helper, args),
        Command::Restore(args) => restore::cmd_restore(ui, command_helper, args),
        Command::Revert(args) => revert::cmd_revert(ui, command_helper, args),
        Command::Root(args) => root::cmd_root(ui, command_helper, args),
        Command::Run(args) => run::cmd_run(ui, command_helper, args),
        Command::SimplifyParents(args) => {
            simplify_parents::cmd_simplify_parents(ui, command_helper, args)
        }
        Command::Show(args) => show::cmd_show(ui, command_helper, args),
        Command::Sign(args) => sign::cmd_sign(ui, command_helper, args),
        Command::Sparse(args) => sparse::cmd_sparse(ui, command_helper, args),
        Command::Split(args) => split::cmd_split(ui, command_helper, args),
        Command::Squash(args) => squash::cmd_squash(ui, command_helper, args),
        Command::Status(args) => status::cmd_status(ui, command_helper, args),
        Command::Tag(args) => tag::cmd_tag(ui, command_helper, args),
        Command::Undo(args) => undo::cmd_undo(ui, command_helper, args),
        Command::Unsign(args) => unsign::cmd_unsign(ui, command_helper, args),
        Command::Util(args) => util::cmd_util(ui, command_helper, args),
        Command::Version(args) => version::cmd_version(ui, command_helper, args),
        Command::Workspace(args) => workspace::cmd_workspace(ui, command_helper, args),
    }
}

/// Wraps deprecated command of `old_name` which has been renamed to `new_name`.
pub(crate) fn renamed_cmd<Args>(
    old_name: &'static str,
    new_name: &'static str,
    cmd: impl Fn(&mut Ui, &CommandHelper, &Args) -> Result<(), CommandError>,
) -> impl Fn(&mut Ui, &CommandHelper, &Args) -> Result<(), CommandError> {
    move |ui: &mut Ui, command: &CommandHelper, args: &Args| -> Result<(), CommandError> {
        writeln!(
            ui.warning_default(),
            "`jj {old_name}` is deprecated; use `jj {new_name}` instead, which is equivalent"
        )?;
        writeln!(
            ui.warning_default(),
            "`jj {old_name}` will be removed in a future version, and this will be a hard error"
        )?;
        cmd(ui, command, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        default_app().debug_assert();
    }
}
