use crate::BunLanguage;
use proto_core::{
    async_trait, Describable, Executable, Installable, ProtoError, Resolvable, ShimBuilder,
    Shimable,
};

#[async_trait]
impl Shimable<'_> for BunLanguage {
    async fn create_shims(&mut self, _find_only: bool) -> Result<(), ProtoError> {
        let mut shimmer = ShimBuilder::new(self.get_id(), self.get_bin_path()?)?;

        shimmer
            .dir(self.get_install_dir()?)
            .version(self.get_resolved_version());

        shimmer.create_global_shim()?;

        // No tool shim

        Ok(())
    }
}
