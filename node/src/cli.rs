use sc_cli::{
	BuildSpecCmd, CheckBlockCmd, ExportBlocksCmd, ExportStateCmd, ImportBlocksCmd, KeySubcommand,
	PurgeChainCmd, RevertCmd, RunCmd,
};

use frame_benchmarking_cli::BenchmarkCmd;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Key management cli utilities
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
	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(BenchmarkCmd),
}
