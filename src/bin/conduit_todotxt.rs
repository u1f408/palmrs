use palmrs_sync::conduit::WithinConduit;
use stable_eyre::eyre::{Report, WrapErr};

fn main() -> Result<(), Report> {
	env_logger::init();
	stable_eyre::install()?;

	let conduit = WithinConduit::new("palmrs-conduit-todotxt")
		.from_env()
		.wrap_err("WithinConduit construction failed, are we running standalone?")?;
	log::trace!("conduit = {:#?}", &conduit);

	Ok(())
}
