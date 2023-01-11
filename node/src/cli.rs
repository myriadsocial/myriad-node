use sc_cli::{
	BuildSpecCmd, ChainInfoCmd, CheckBlockCmd, ExportBlocksCmd, ExportStateCmd, ImportBlocksCmd,
	KeySubcommand, PurgeChainCmd, RevertCmd, RunCmd, SignCmd, VanityCmd, VerifyCmd,
};

use frame_benchmarking_cli::BenchmarkCmd;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: RunCmd,

	#[clap(long)]
	pub no_hardware_benchmarks: bool,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[clap(subcommand)]
	Key(KeySubcommand),

	/// Sign a message, with a given (secret) key.
	Sign(SignCmd),

	/// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
	Verify(VerifyCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(VanityCmd),

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

	/// Db meta columns information.
	ChainInfo(ChainInfoCmd),

	/// Sub-commands concerned with benchmarking.
	#[clap(subcommand)]
	Benchmark(BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,
}
