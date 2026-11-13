// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2026 CPE Monitor contributors.
fn main() {
    winresource::WindowsResource::new()
        .set_icon("CTM.ico")
        .set_manifest(include_str!("app.manifest"))
        .set("ProductName", "CPE 流量监控")
        .set("FileDescription", "CPE 流量监控")
        .compile()
        .expect("Windows icon resource");
}
