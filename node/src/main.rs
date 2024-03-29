use sc_cli::Result;

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

fn main() -> Result<()> {
	command::run()
}
