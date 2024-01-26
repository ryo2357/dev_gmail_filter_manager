use crate::interface::Interface;

pub fn sync() -> anyhow::Result<()> {
    let interface = Interface::create_from_env()?;
    interface.sync_from_yaml()?;

    Ok(())
}
