use rimrs::helpers::*;

#[test]
fn get_local_mods_location() -> anyhow::Result<()> {
    let conf = RimPyConfig::from_file()?;
    assert!(conf.folders.local_mods.is_some());
    Ok(())
}

