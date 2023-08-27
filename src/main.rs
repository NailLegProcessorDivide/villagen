use joes_gdmc_http_rs;
use tokio;

#[tokio::main]
async fn main() {
    //let res = joes_gdmc_http_rs::get_block(0, -10, 0).await.unwrap();
    //println!("Hello, world! {}!", joes_gdmc_http_rs::blocks::block_to_string(&res));
    let mut res = vec![];
    let chunks = match joes_gdmc_http_rs::get_chunk_nbt(0, 0, 4, 4).await {
        Ok(c) => c,
        Err(s) => panic!("{}", s),
    };
    for chunk in chunks {
        for y in 1..=384 {
            let sy = (384 - y) % 16;
            let by = (384 - y) / 16;
            if chunk.sub_chunks[sy].blocks[4][by][4] != joes_gdmc_http_rs::blocks::Blocks::Air {
                println!(
                    "{} {} {} {:?}",
                    chunk.x * 16 + 4,
                    384 - y as isize - 64,
                    chunk.z * 16 + 4,
                    chunk.sub_chunks[sy].blocks[4][by][4]
                );
                break;
            }
        }
    }

    let start = 740;
    let mut fut = joes_gdmc_http_rs::get_chunk_nbt(0, start, 1, 1);
    for x in 1..1 {
        let next = joes_gdmc_http_rs::get_chunk_nbt(0, start + x, 64, 1);
        let mut chunks = match fut.await {
            Ok(c) => c,
            Err(s) => panic!("{}", s),
        };
        fut = next;
        res.append(&mut chunks);
    }
    let mut chunks = match fut.await {
        Ok(c) => c,
        Err(s) => panic!("{}", s),
    };
    res.append(&mut chunks);
    //println!("Hello, world! {:?}!", res);
}
