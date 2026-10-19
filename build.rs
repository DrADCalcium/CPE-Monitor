fn main() {
    winresource::WindowsResource::new()
        .set_icon("CTM.ico")
        .set("ProductName", "CPE 流量监控")
        .set("FileDescription", "CPE 流量监控")
        .compile()
        .expect("Windows icon resource");
}
