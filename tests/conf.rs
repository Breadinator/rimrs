use rimrs::serialization::*;

#[test]
fn get_local_mods_location() -> Result<(), ReadRimPyConfigError> {
    let conf = RimPyConfig::from_file()?;
    assert!(conf.folders.local_mods.is_some());
    Ok(())
}

