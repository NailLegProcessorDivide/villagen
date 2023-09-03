use std::{error::Error, path::PathBuf};

use joes_gdmc_http_rs::{self, net_loader::NetLoader, WorldLoader, world::{coords::{V2di32, Vec3D}, blocks::Blocks}, file_loader::FileLoader};

fn main() -> Result<(), Box<dyn Error>> {
    // let mut loader = NetLoader::new("http://127.0.0.1:9000".to_string());
    let mut loader = FileLoader::new_from_file(&PathBuf::from("New World"))?;
    loader.load_chunks(V2di32::new(-60, -60), 100.into())?;

    let world = loader.get_world();
    //let res = joes_gdmc_http_rs::get_block(0, -10, 0).await.unwrap();
    //println!("Hello, world! {}!", joes_gdmc_http_rs::blocks::block_to_string(&res));

    println!("0 {:?}", world.get_block(Vec3D::new(1, 0, 0))?);
    println!("-1 {:?}", world.get_block(Vec3D::new(0, -1, 0))?);
    println!("-2 {:?}", world.get_block(Vec3D::new(0, -2, 0))?);
    println!("-3 {:?}", world.get_block(Vec3D::new(0, -3, 0))?);
    println!("-4 {:?}", world.get_block(Vec3D::new(0, -4, 0))?);
    println!("-5 {:?}", world.get_block(Vec3D::new(0, -5, 0))?);
    // for x in 0..4 {
    //     for z in 0..4 {
    //         for y in (-64..320).rev() {
    //             let block = world.get_block(Vec3D::new(x, y, z))?;
    //             if block != Blocks::Air {
    //                 println!(
    //                     "{} {} {} {:?}",
    //                     x,
    //                     y,
    //                     z,
    //                     block
    //                 );
    //                 break;
    //             }
    //         }
    //     }
    // }
    Ok(())
    //println!("Hello, world! {:?}!", res);
}
