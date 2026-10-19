Software rasterizer: https://github.com/DGriffin91/egui_software_backend
Revision: c70e02d3d95c75a82733deb97e516b13a6c86dc6
License: MIT OR Apache-2.0; license texts are included alongside the source.

Local changes: update egui to 0.36.2; apply each ImageDelta in the batched
TexturesDelta.set API; keep the CPU renderer and constify macro dependencies.
Only the default std feature is used/supported by CPE Monitor. Upstream winit/softbuffer integration, test-render harness and standalone
macro tests were removed from this minimal source distribution.
GDI window/surface/event integration is implemented in src/runtime.rs.

epaint 0.36.2: https://github.com/emilk/egui
Revision: 49682f8baa058bf49e011035cfbd6e825f88a5ef
License: MIT OR Apache-2.0; both license texts are included in epaint/.
Local change in src/text/fonts.rs: parsed font blobs retain the existing
Arc<FontData> through an AsRef<[u8]> wrapper instead of cloning the whole owned
font byte buffer. This retains safe ownership and releases data when unused.
