use crate::{Change, InjectionConfig};
use anyhow::Result;
use log::{debug, info};
use stalker_utils::config::LibConfig;
use stalker_utils::context::Context;
use stalker_utils::fmt::hex;
use std::fs::{copy, create_dir, remove_dir, remove_file, File};
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::process::Command;

impl Change {
    pub fn id(&self) -> String {
        format!("{:x}_{}_{:02x}", self.0.offset, hex(&self.0.bytes), self.2)
    }

    pub fn perform(
        &self,
        ctx: &Context,
        lib_config: &LibConfig,
        inj_config: &InjectionConfig,
    ) -> Result<()> {
        if inj_config.dry_run {
            Ok(())
        } else {
            let origin = Path::new(&ctx.binary_info.core.file);
            let load = format!("data/stalker/load/{}/", &self.id());
            let new = format!("{}{}", &load, &lib_config.link_name);

            create_dir(&load)?;
            info!(
                "Performing injection `{}` for {}.",
                &self.id(),
                &lib_config.link_name
            );
            // Create
            copy(origin, &new)?;
            self.edit(&new)?;
            // Run
            let res = Command::new(&inj_config.exec_command)
                .args(&inj_config.exec_args)
                .current_dir(&inj_config.work_dir)
                .env("LD_LIBRARY_PATH", &format!("../../../{}", &load))
                .output();
            match res {
                Ok(res) => {
                    let stdout_f = format!(
                        "data/stalker/output/{}/{}/{}/{}.stdout",
                        ctx.id(),
                        lib_config.link_name,
                        &inj_config.name,
                        &self.id()
                    );
                    let stderr_f = format!(
                        "data/stalker/output/{}/{}/{}/{}.stderr",
                        ctx.id(),
                        lib_config.link_name,
                        &inj_config.name,
                        &self.id()
                    );
                    if !res.stdout.is_empty() {
                        let mut of = File::create(&stdout_f)?;
                        of.write_all(&res.stdout)?;
                        of.sync_all()?;
                    }
                    if !res.stderr.is_empty() {
                        let mut ef = File::create(&stderr_f)?;
                        ef.write_all(&res.stderr)?;
                        ef.sync_all()?;
                    }
                }
                Err(e) => {
                    let fail_f = format!(
                        "data/stalker/output/{}/{}/{}/{}.fail",
                        ctx.id(),
                        lib_config.link_name,
                        &inj_config.name,
                        &self.id()
                    );
                    let mut f = File::create(&fail_f)?;
                    f.write_all(&e.to_string().as_bytes())?;
                }
            };

            // Clean
            remove_file(&new)?;
            remove_dir(&load)?;
            Ok(())
        }
    }

    fn edit(&self, file: &str) -> Result<()> {
        let mut f = File::options().write(true).read(true).open(file)?;
        let mut bytes = [0u8; 4];
        f.read_at(&mut bytes, self.0.offset as u64)?;
        debug!(
            "Change from 0x{} to 0x{} at {}",
            hex(&bytes),
            hex(&self.1.bytes),
            self.0.offset
        );
        f.seek(SeekFrom::Start(self.0.offset as u64))?;
        f.write(&self.1.bytes)?;
        Ok(())
    }
}
