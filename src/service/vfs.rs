use mluau_require::rust_embed;
use mluau_require::Embed;

#[derive(Embed, Debug)]
#[folder = "$CARGO_MANIFEST_DIR/src/luau"]
#[prefix = ""]
pub struct LuauBase;

pub fn get_luau_vfs() -> mluau_require::vfs::EmbeddedFS<LuauBase> {
    mluau_require::vfs::EmbeddedFS::<LuauBase>::new()
}