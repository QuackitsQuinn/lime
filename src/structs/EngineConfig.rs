use std::fmt::Display;

pub mod general {

pub enum WindowType {
    Window,
    Fullscreen,
    Borderless
}
}
pub mod LimeCfg {
    use crate::structs::EngineConfig::general::WindowType;

    const defaultConfig: LimeCfg = LimeCfg {
        window_type: WindowType::Window,
        window_width: 800,
        window_height: 600,
        lock_fps: true,
        fps: 60,
    };
pub struct LimeCfg {
    pub window_type: WindowType,
    pub window_width: u32,
    pub window_height: u32,
    pub lock_fps: bool,
    pub fps: u32,
}
impl LimeCfg {
    pub fn new() -> LimeCfg {
        defaultConfig
    }
}
}

