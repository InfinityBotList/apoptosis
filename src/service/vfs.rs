use mluau_require::rust_embed;
use mluau_require::Embed;
use mluau_require::FilesystemWrapper;

#[derive(Embed, Debug)]
#[folder = "$CARGO_MANIFEST_DIR/src/luau"]
#[prefix = ""]
pub struct LuauBase;

pub fn get_luau_vfs() -> FilesystemWrapper {
    FilesystemWrapper::new(mluau_require::vfs::EmbeddedFS::<LuauBase>::new())
}