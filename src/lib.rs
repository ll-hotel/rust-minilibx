mod xvars;
use xvars::MlxVars;

mod window;
use window::MlxWindow;

mod events;
use events::*;

pub struct Mlx {
    vars: Box<MlxVars>,
}
impl Mlx {
    pub fn new() -> Option<Self> {
        Some(Mlx { vars: MlxVars::new()? })
    }
    pub fn new_window(&self, width: u32, height: u32, title: &str) -> Option<Box<MlxWindow>> {
        MlxWindow::new(&self.vars, width, height, title)
    }
}
