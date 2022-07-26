use sc_cli::{
	BuildSpecCmd, CheckBlockCmd, ExportBlocksCmd, ExportStateCmd, ImportBlocksCmd, KeySubcommand,
	PurgeChainCmd, RevertCmd, RunCmd,
};

use frame_benchmarking_cli::BenchmarkCmd;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: RunCmd,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[clap(subcommand)]
	Key(KeySubcommand),

	/// Build a chain specification.
	BuildSpec(BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(ExportStateCmd),

	/// Import blocks.
	ImportBlocks(ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(RevertCmd),

	/// The custom benchmark subcommand benchmarking runtime pallets.
	#[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,
}
