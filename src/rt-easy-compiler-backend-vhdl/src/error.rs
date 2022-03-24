use thiserror::Error;

pub type Result<T> = std::result::Result<T, SynthError>;

#[derive(Debug, Error)]
#[error("VHDL backend error")]
pub struct BackendError {
    pub errors: Vec<SynthError>,
    pub signals: rtvhdl::Signals,
}

#[derive(Debug, Error)]
pub enum SynthError {
    #[error("next state depends on an unclocked item")]
    UnclockedGotoDependency,
    #[error("conditional goto in first state")]
    ConditionalGotoInFirstState,
}
