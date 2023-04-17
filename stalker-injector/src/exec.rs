use crate::{Change, InjectionConfig};
use anyhow::Result;
use log::{debug, info, warn};
use stalker_mutator::FaultModel;
use stalker_utils::config::LibConfig;
use stalker_utils::context::Context;
use stalker_utils::fmt::hex;
use stalker_utils::tag::Tag;
use std::fs::{copy, create_dir_all, remove_dir, remove_file, File};
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::process::Command;

impl Change {
    pub fn id(&self) -> String {
        format!("{:x}_{}_{:02x}", self.0.offset, hex(&self.0.bytes), self.2)
    }

    pub fn perform<M: FaultModel>(
        &self,
        ctx: &Context,
        lib_config: &LibConfig,
        inj_config: &InjectionConfig,
    ) -> Result<()> {
        if inj_config.dry_run {
            Ok(())
        } else {
            // Output texts files
            let f_bdr = |name: &str| {
                format!(
                    "data/stalker/output/{}/{}/{}/{}.{}",
                    M::tag(),
                    ctx.id(),
                    &inj_config.name,
                    self.id(),
                    name
                )
            };
            let stdout_fs = f_bdr("stdout");
            let stderr_fs = f_bdr("stderr");
            let fail_fs = f_bdr("fail");
            let stdout_f = Path::new(&stdout_fs);
            let stderr_f = Path::new(&stderr_fs);
            let fail_f = Path::new(&fail_fs);

            if stdout_f.exists() || stderr_f.exists() || fail_f.exists() {
                warn!("Result file exists, skipping.");
                return Ok(());
            }
            // Binary files
            let origin = Path::new(&ctx.config.binary_info.core.file);
            let load = format!("/tmp/stalker-cache/{}", &self.id());
            let new = format!("{}/{}", &load, &lib_config.link_name);

            create_dir_all(&load)?;
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
                .env("LD_LIBRARY_PATH", &load)
                .output();
            match res {
                Ok(res) => {
                    if !res.stdout.is_empty() {
                        let mut of = File::create(stdout_f)?;
                        of.write_all(&res.stdout)?;
                        of.sync_all()?;
                    }
                    if !res.stderr.is_empty() {
                        let mut ef = File::create(stderr_f)?;
                        ef.write_all(&res.stderr)?;
                        ef.sync_all()?;
                    }
                }
                Err(e) => {
                    let mut f = File::create(fail_f)?;
                    f.write_all(e.to_string().as_bytes())?;
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
        let size = self.0.size as usize;
        let mut bytes = vec![0u8; size];
        f.read_at(&mut bytes, self.0.offset as u64)?;
        debug!(
            "Change from 0x{} to 0x{} at {}",
            hex(&bytes),
            hex(&self.1.bytes),
            self.0.offset
        );
        f.seek(SeekFrom::Start(self.0.offset as u64))?;
        let written = f.write(&self.1.bytes)?;
        assert_eq!(written, size);
        Ok(())
    }
}
