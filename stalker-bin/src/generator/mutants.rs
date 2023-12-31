use anyhow::Result;
use log::{debug, info};
use stalker_mutator::{FaultModel, Mutatable};
use stalker_utils::asm::Asm;
use stalker_utils::config::LibConfig;
use stalker_utils::context::Context;
use stalker_utils::fmt::hex;

pub fn gen<M: FaultModel>(ctx: &mut Context, lib_config: &LibConfig) -> Result<()> {
    let locs = ctx.lib.locs.to_vec();
    let db = ctx
        .db
        .as_ref()
        .expect("Context Db should be initialized first.");
    for loc in locs
        .iter()
        .filter(|x| lib_config.syms.iter().any(|f| f.predicate()(&x.name)))
    {
        if let Ok(locinfo) = ctx.lib.get_locinfo(&mut ctx.rz, &loc.name) {
            info!("Found symbol {}", &loc.name);
            for op in locinfo.ops.iter() {
                let asm = Asm::from(op);
                let mutant_db = &db.mutant.open_tree(M::tag())?;
                if let Ok(Some(_)) = mutant_db.get(asm.key(&ctx.config.arch, asm.size * 8 - 1)) {
                    continue;
                } else {
                    for (i, mutant) in asm
                        .mutants::<M>(|bytes| ctx.config.arch.disasm(&bytes))
                        .enumerate()
                    {
                        let m = mutant.unwrap();
                        let key = asm.key(&ctx.config.arch, i as u8);
                        let val = format!(
                            "{}_{}",
                            hex(&m.bytes),
                            match &m.disasm {
                                None => "invalid",
                                Some(asm) => asm,
                            }
                        );
                        debug!("{} {}", &key, &val);
                        mutant_db.insert(key, val.as_str())?;
                    }
                }
            }
        }
    }
    Ok(())
}
