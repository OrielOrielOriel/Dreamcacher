use nix::sys::reboot::reboot;
use nix::sys::reboot;

fn main() -> Result<(), nix::Error> {
	reboot(reboot::RebootMode::RB_POWER_OFF)?;
	Ok(())
}
