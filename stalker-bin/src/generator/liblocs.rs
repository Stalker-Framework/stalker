use anyhow::Result;
use log::{debug, info};
use stalker_utils::context::Context;
use stalker_utils::fmt::hex;

pub fn gen(ctx: &mut Context) -> Result<()> {
    let locs = ctx.lib.locs.to_vec();
    let cnt = locs.len();
    let db = ctx
        .db
        .as_ref()
        .expect("Context Db should be initialized first.");
    let locs_tree = db.instruction.open_tree(&"locs")?;
    info!("Openning db.");
    for (i, loc) in locs.iter().enumerate() {
        info!("{:2}/{:2} {}", i, cnt, &loc.name);
        locs_tree.insert(i.to_be_bytes(), loc.name.as_str())?;
        let locinfo = ctx.lib.get_locinfo(&mut ctx.rz, &loc.name);
        let tree = db.instruction.open_tree(&loc.name)?;
        if locinfo.is_err() {
            continue;
        }
        for asm in locinfo.unwrap().ops.iter() {
            if let Ok(Some(_)) = tree.get(format!("{:08x}", asm.offset)) {
                continue;
            } else {
                let m = asm;
                let key = format!("{:08x}", asm.offset);
                let val = format!(
                    "{}_{}",
                    hex(&m.bytes),
                    match &m.disasm {
                        None => "invalid",
                        Some(asm) => asm,
                    }
                );
                debug!("{} {}", &key, &val);
                tree.insert(key, val.as_str())?;
            }
        }
    }
    Ok(())
}
