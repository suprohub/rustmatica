#[cfg(feature = "_mcdata-all")]
use rustmatica::{Litematic, LitematicMetadata};

#[cfg(not(feature = "_mcdata-all"))]
fn main() {
    panic!("this test requires the `_mcdata-all` feature to be enabled");
}

#[cfg(feature = "_mcdata-all")]
macro_rules! try_for {
    ($version:ident, $path:expr, $meta:expr) => {{
        let schem: rustmatica::Result<
            Litematic<
                mcdata::$version::BlockState,
                mcdata::$version::Entity,
                mcdata::$version::BlockEntity,
            >,
        > = Litematic::read_file_with_metadata($path, $meta);
        println!("\n\x1b[1m{}\x1b[0m", $path.display());

        match schem {
            Ok(schem) => {
                for region in schem.regions {
                    for block in region.block_palette() {
                        if let mcdata::$version::BlockState::Other(generic) = block {
                            println!("\x1b[1;33mcontains generic block state: {generic:?}\x1b[0m");
                        }
                    }
                    for entity in region.entities {
                        if let mcdata::$version::Entity::Other(generic) = entity {
                            println!("\x1b[1;33mcontains generic entity: {generic:?}\x1b[0m");
                        }
                    }
                    for entity in region.block_entities {
                        if let mcdata::$version::BlockEntity::Other(generic) = entity {
                            println!("\x1b[1;33mcontains generic block entity: {generic:?}\x1b[0m");
                        }
                    }
                }
            }
            Err(e) => eprintln!(
                "\x1b[1;31mcould not read '{}' as litematic: {e}\x1b[0m",
                $path.display()
            ),
        }
    }};
}

#[cfg(feature = "_mcdata-all")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for file in std::fs::read_dir("test_files/tmc_catalogue")? {
        let file = file?;
        let path = file.path();
        println!("\n\x1b[1m{}\x1b[0m", path.display());
        let meta = LitematicMetadata::read_file(&path)?;
        match meta.minecraft_data_version {
            mcdata::data_version::MC_1_20_5..=u32::MAX => try_for!(mc1_20_5, &path, meta),
            mcdata::data_version::MC_1_20_3..=u32::MAX => try_for!(mc1_20_3, &path, meta),
            mcdata::data_version::MC_1_20_2..=u32::MAX => try_for!(mc1_20_2, &path, meta),
            mcdata::data_version::MC_1_20..=u32::MAX => try_for!(mc1_20, &path, meta),
            mcdata::data_version::MC_1_19_4..=u32::MAX => try_for!(mc1_19_4, &path, meta),
            mcdata::data_version::MC_1_19_3..=u32::MAX => try_for!(mc1_19_3, &path, meta),
            mcdata::data_version::MC_1_19_1..=u32::MAX => try_for!(mc1_19_1, &path, meta),
            mcdata::data_version::MC_1_19..=u32::MAX => try_for!(mc1_19, &path, meta),
            mcdata::data_version::MC_1_18..=u32::MAX => try_for!(mc1_18, &path, meta),
            mcdata::data_version::MC_1_17..=u32::MAX => try_for!(mc1_17, &path, meta),
            mcdata::data_version::MC_1_16_2..=u32::MAX => try_for!(mc1_16_2, &path, meta),
            mcdata::data_version::MC_1_16..=u32::MAX => try_for!(mc1_16, &path, meta),
            mcdata::data_version::MC_1_15_2..=u32::MAX => try_for!(mc1_15_2, &path, meta),
            mcdata::data_version::MC_1_15..=u32::MAX => try_for!(mc1_15, &path, meta),
            mcdata::data_version::MC_1_14..=u32::MAX => try_for!(mc1_14, &path, meta),
            v => eprintln!("\x1b[1;31mMinecraft data version {v} not supported\x1b[0m"),
        }
    }

    Ok(())
}
