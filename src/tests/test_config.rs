#[cfg(test)]
mod config_test {
    use crate::structs::EngineConfig::{general::WindowType,LimeCfg::LimeCfg};

    #[test]
    pub fn test_defaut_conf() {
        let default_config = LimeCfg::new();
    }
}