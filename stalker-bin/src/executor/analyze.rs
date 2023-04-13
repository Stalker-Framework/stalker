use anyhow::Result;
use stalker_classifier::analyze::AnalyzeConfig;

pub fn exec(analyze_config: &AnalyzeConfig) -> Result<()> {
    analyze_config.model.analyze_fn()(&analyze_config);
    Ok(())
}
