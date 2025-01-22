pub enum MlxEventHook {
    General(MlxGeneralHook),
    Mouse(MlxMouseHook),
}

pub type MlxGeneralHook = Box<dyn GeneralHook>;
pub type MlxMouseHook = Box<dyn MouseHook>;

pub trait GeneralHook {
    fn call(&self, sym: i32);
}
pub trait MouseHook {
    fn call(&self, x: i32, y: i32, sym: i32);
}
