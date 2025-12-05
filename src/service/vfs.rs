use rust_embed::Embed;
use super::lua::FilesystemWrapper;

#[derive(Embed, Debug)]
#[folder = "$CARGO_MANIFEST_DIR/src/luau"]
#[prefix = ""]
pub struct LuauBase;

pub fn get_luau_vfs() -> FilesystemWrapper {
    FilesystemWrapper::new(vfs::EmbeddedFS::<LuauBase>::new())
}