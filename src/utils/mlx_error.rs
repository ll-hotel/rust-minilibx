#[derive(Clone, Copy, Debug)]
pub enum MlxError {
    MlxCreationError,
    WindowCreationError,
    ImageCreationError,
}

impl std::fmt::Display for MlxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MlxCreationError => write!(f, "Failed to create mlx"),
            Self::WindowCreationError => write!(f, "Faild to create new mlx window"),
            Self::ImageCreationError => write!(f, "Failed to create new mlx image"),
        }
    }
}
