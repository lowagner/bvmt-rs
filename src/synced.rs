#![allow(dead_code)]

// TODO: rename to an `options.rs` file and include `NeedIt` enum.

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Synced {
    /// Not synced, the data lives only in the CPU.
    CpuOnly,
    /// Not synced, the data lives only in the GPU.
    GpuOnly,
    /// Not synced, the CPU data is ahead.
    CpuAhead,
    /// Not synced, the GPU data is ahead.
    GpuAhead,
    /// Synced, but prefer making updates to the CPU version of the data.
    CpuPreferred,
    /// Synced, but prefer making updates to the GPU version of the data.
    GpuPreferred,
}

impl Synced {
    pub(crate) fn on_cpu(&self) -> bool {
        *self != Synced::GpuOnly
    }

    pub(crate) fn on_gpu(&self) -> bool {
        *self != Synced::CpuOnly
    }

    pub(crate) fn needs_gpu_update(&self) -> bool {
        matches!(self, Synced::CpuOnly | Synced::CpuAhead)
    }

    pub(crate) fn needs_cpu_update(&self) -> bool {
        matches!(self, Synced::GpuOnly | Synced::GpuAhead)
    }

    pub(crate) fn prefers_writing_to_cpu(&self) -> bool {
        // If already ahead on the CPU, then just keep writing there.
        matches!(
            self,
            Synced::CpuOnly | Synced::CpuAhead | Synced::CpuPreferred
        )
    }

    pub(crate) fn prefers_writing_to_gpu(&self) -> bool {
        // If already ahead on the GPU, then just keep writing there.
        matches!(
            self,
            Synced::GpuOnly | Synced::GpuAhead | Synced::GpuPreferred
        )
    }

    pub(crate) fn cpu_was_updated(&mut self) {
        match self {
            Synced::CpuOnly | Synced::CpuAhead => {}
            Synced::GpuAhead | Synced::GpuOnly => {
                panic!("expected a CPU update, but initial synced state was GPU focused");
            }
            Synced::CpuPreferred | Synced::GpuPreferred => {
                *self = Synced::CpuAhead;
            }
        }
    }

    pub(crate) fn gpu_was_updated(&mut self) {
        match self {
            Synced::GpuOnly | Synced::GpuAhead => {}
            Synced::CpuAhead | Synced::CpuOnly => {
                panic!("expected a GPU update, but initial synced state was CPU focused");
            }
            Synced::CpuPreferred | Synced::GpuPreferred => {
                *self = Synced::GpuAhead;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_synced_on_cpu() {
        assert_eq!(Synced::CpuOnly.on_cpu(), true);
        assert_eq!(Synced::GpuOnly.on_cpu(), false);
        assert_eq!(Synced::CpuAhead.on_cpu(), true);
        assert_eq!(Synced::GpuAhead.on_cpu(), true);
        assert_eq!(Synced::CpuPreferred.on_cpu(), true);
        assert_eq!(Synced::GpuPreferred.on_cpu(), true);
    }

    #[test]
    fn test_synced_on_gpu() {
        assert_eq!(Synced::CpuOnly.on_gpu(), false);
        assert_eq!(Synced::GpuOnly.on_gpu(), true);
        assert_eq!(Synced::CpuAhead.on_gpu(), true);
        assert_eq!(Synced::GpuAhead.on_gpu(), true);
        assert_eq!(Synced::CpuPreferred.on_gpu(), true);
        assert_eq!(Synced::GpuPreferred.on_gpu(), true);
    }

    #[test]
    fn test_synced_needs_gpu_update() {
        assert_eq!(Synced::CpuOnly.needs_gpu_update(), true);
        assert_eq!(Synced::GpuOnly.needs_gpu_update(), false);
        assert_eq!(Synced::CpuAhead.needs_gpu_update(), true);
        assert_eq!(Synced::GpuAhead.needs_gpu_update(), false);
        assert_eq!(Synced::CpuPreferred.needs_gpu_update(), false);
        assert_eq!(Synced::GpuPreferred.needs_gpu_update(), false);
    }

    #[test]
    fn test_synced_needs_cpu_update() {
        assert_eq!(Synced::CpuOnly.needs_cpu_update(), false);
        assert_eq!(Synced::GpuOnly.needs_cpu_update(), true);
        assert_eq!(Synced::CpuAhead.needs_cpu_update(), false);
        assert_eq!(Synced::GpuAhead.needs_cpu_update(), true);
        assert_eq!(Synced::CpuPreferred.needs_cpu_update(), false);
        assert_eq!(Synced::GpuPreferred.needs_cpu_update(), false);
    }

    #[test]
    fn test_synced_prefers_writing_to_cpu() {
        assert_eq!(Synced::CpuOnly.prefers_writing_to_cpu(), true);
        assert_eq!(Synced::GpuOnly.prefers_writing_to_cpu(), false);
        assert_eq!(Synced::CpuAhead.prefers_writing_to_cpu(), true);
        assert_eq!(Synced::GpuAhead.prefers_writing_to_cpu(), false);
        assert_eq!(Synced::CpuPreferred.prefers_writing_to_cpu(), true);
        assert_eq!(Synced::GpuPreferred.prefers_writing_to_cpu(), false);
    }

    #[test]
    fn test_synced_prefers_writing_to_gpu() {
        assert_eq!(Synced::CpuOnly.prefers_writing_to_gpu(), false);
        assert_eq!(Synced::GpuOnly.prefers_writing_to_gpu(), true);
        assert_eq!(Synced::CpuAhead.prefers_writing_to_gpu(), false);
        assert_eq!(Synced::GpuAhead.prefers_writing_to_gpu(), true);
        assert_eq!(Synced::CpuPreferred.prefers_writing_to_gpu(), false);
        assert_eq!(Synced::GpuPreferred.prefers_writing_to_gpu(), true);
    }

    #[test]
    fn test_synced_can_update_cpu() {
        let mut synced = Synced::CpuOnly;
        synced.cpu_was_updated();
        assert_eq!(synced, Synced::CpuOnly);

        synced = Synced::CpuAhead;
        synced.cpu_was_updated();
        assert_eq!(synced, Synced::CpuAhead);

        synced = Synced::CpuPreferred;
        synced.cpu_was_updated();
        assert_eq!(synced, Synced::CpuAhead);

        synced = Synced::GpuPreferred;
        synced.cpu_was_updated();
        assert_eq!(synced, Synced::CpuAhead);
    }

    #[test]
    #[should_panic]
    fn test_synced_cannot_update_cpu_from_gpu_ahead() {
        let mut synced = Synced::GpuAhead;
        synced.cpu_was_updated();
    }

    #[test]
    #[should_panic]
    fn test_synced_cannot_update_cpu_from_gpu_only() {
        let mut synced = Synced::GpuOnly;
        synced.cpu_was_updated();
    }

    #[test]
    fn test_synced_can_update_gpu() {
        let mut synced = Synced::GpuOnly;
        synced.gpu_was_updated();
        assert_eq!(synced, Synced::GpuOnly);

        synced = Synced::GpuAhead;
        synced.gpu_was_updated();
        assert_eq!(synced, Synced::GpuAhead);

        synced = Synced::GpuPreferred;
        synced.gpu_was_updated();
        assert_eq!(synced, Synced::GpuAhead);

        synced = Synced::CpuPreferred;
        synced.gpu_was_updated();
        assert_eq!(synced, Synced::GpuAhead);
    }

    #[test]
    #[should_panic]
    fn test_synced_cannot_update_gpu_from_cpu_ahead() {
        let mut synced = Synced::CpuAhead;
        synced.gpu_was_updated();
    }

    #[test]
    #[should_panic]
    fn test_synced_cannot_update_gpu_from_cpu_only() {
        let mut synced = Synced::CpuOnly;
        synced.gpu_was_updated();
    }
}
