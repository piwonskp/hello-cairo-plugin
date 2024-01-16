use std::fmt;
use anyhow::Result;
use cairo_lang_defs::plugin::PluginSuite;
use scarb::compiler::plugin::builtin::BuiltinStarkNetPlugin;
use camino::Utf8Path;
use scarb::compiler::plugin::{CairoPlugin, CairoPluginInstance};
use scarb::core::{PackageId, PackageName, SourceId};
use semver::Version;
use url::Url;

use crate::plugin::macro_plugin::HelloPlugin;
use crate::plugin::config::{PACKAGE_NAME, VERSION, URL};


// Target the plugin located on a local filesystem. Useful for development
struct LocalPlugin;
impl CairoPlugin for LocalPlugin {
    fn id(&self) -> PackageId {
        PackageId::new(
            PackageName::new(PACKAGE_NAME),
            Version::parse(VERSION).unwrap(),
            SourceId::for_path(&(Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("Scarb.toml"))).unwrap(),
        )
    }

    fn instantiate(&self) -> Result<Box<dyn CairoPluginInstance>> {
        Ok(Box::new(PluginInstance))
    }
}


// Target the plugin located in remote repository
struct GitPlugin;
impl CairoPlugin for GitPlugin {
    fn id(&self) -> PackageId {
        PackageId::new(
            PackageName::new(PACKAGE_NAME),
            Version::parse(VERSION).unwrap(),
            SourceId::for_git(
                        &Url::parse(URL).unwrap(),
                        &scarb::core::GitReference::DefaultBranch,
                    )
                    .unwrap(),
        )
    }

    fn instantiate(&self) -> Result<Box<dyn CairoPluginInstance>> {
        Ok(Box::new(PluginInstance))
    }
}


// Target the plugin inside the compiler, unsupported yet (v2.4.0)
// See https://github.com/software-mansion/scarb/issues/119, https://github.com/software-mansion/scarb/discussions/227
struct InCompilerPlugin;
impl CairoPlugin for InCompilerPlugin {
    fn id(&self) -> PackageId {
        PackageId::new(
            PackageName::new(PACKAGE_NAME),
            Version::parse(VERSION).unwrap(),
            SourceId::for_std(),
        )
    }

    fn instantiate(&self) -> Result<Box<dyn CairoPluginInstance>> {
        Ok(Box::new(PluginInstance))
    }
}

struct PluginInstance;
impl CairoPluginInstance for PluginInstance {
    fn plugin_suite(&self) -> PluginSuite {
        let mut suite = PluginSuite::default();
        suite.add_plugin::<HelloPlugin>();
        suite
    }
}

pub struct CairoPluginRepository(pub scarb::compiler::plugin::CairoPluginRepository);

impl CairoPluginRepository {
    pub fn new() -> Self {
        let mut repo = scarb::compiler::plugin::CairoPluginRepository::empty();

        repo.add(Box::new(BuiltinStarkNetPlugin)).unwrap();
        repo.add(Box::new(LocalPlugin)).unwrap();
        repo.add(Box::new(GitPlugin)).unwrap();
        repo.add(Box::new(InCompilerPlugin)).unwrap();

        Self(repo)
    }
}

impl Default for CairoPluginRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CairoPluginRepository> for scarb::compiler::plugin::CairoPluginRepository {
    fn from(val: CairoPluginRepository) -> Self {
        val.0
    }
}

impl fmt::Debug for CairoPluginRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
