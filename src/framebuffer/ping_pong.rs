use crate::framebuffer::targets::RenderTargetManager;

pub struct PingPongController;

impl PingPongController {
    /// Inverte i buffer primario e ping-pong per un target specifico (es. colortex0)
    pub fn swap(manager: &mut RenderTargetManager, target_name: &str) {
        if let (Some(primary), Some(pingpong)) = (
            manager.color_targets.remove(target_name),
            manager.ping_pong_targets.remove(target_name),
        ) {
            manager.color_targets.insert(target_name.to_string(), pingpong);
            manager.ping_pong_targets.insert(target_name.to_string(), primary);
        }
    }

    /// Inverte tutti i buffer attivi prima del prossimo passo di composite
    pub fn swap_all(manager: &mut RenderTargetManager) {
        let keys: Vec<String> = manager.color_targets.keys().cloned().collect();
        for key in keys {
            Self::swap(manager, &key);
        }
    }
}
