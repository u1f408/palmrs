use std::path::PathBuf;

use palmrs_sync::{conduit::ConduitHandler, SyncMode};
use stable_eyre::eyre::{Report, WrapErr};
use structopt::StructOpt;
use subprocess::Redirection;

/// Call a sync conduit in debugging mode
#[derive(Debug, StructOpt)]
#[structopt(name = "palmrs-sync-dbgconduit")]
struct Opt {
	/// Path to sync configuration
	#[allow(unused)]
	#[structopt(short, long)]
	config: Option<PathBuf>,

	/// Name of the conduit to call
	#[structopt(name = "CONDUIT")]
	conduit: String,

	/// Sync mode
	#[structopt(name = "MODE", parse(try_from_str))]
	sync_mode: SyncMode,
}

fn main() -> Result<(), Report> {
	env_logger::init();
	stable_eyre::install()?;
	let opt = Opt::from_args();
	log::trace!("opt = {:#?}", &opt);

	// Construct a ConduitHandler
	let conduit = ConduitHandler::new(&opt.conduit, opt.sync_mode);
	log::trace!("conduit = {:#?}", &conduit);

	// Do the popen
	let mut popen = conduit
		.popen(Redirection::None, Redirection::None)
		.wrap_err("popen() on conduit object failed")?;
	log::trace!("popen = {:#?}", &popen);

	// And wait for the child process to exit
	let exit_status = popen
		.wait()
		.wrap_err("wait() on conduit popen object failed")?;
	log::trace!("exit_status = {:#?}", &exit_status);

	Ok(())
}
