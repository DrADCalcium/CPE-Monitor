# 第三方组件与许可证

本清单依据 `Cargo.lock` 和 Windows x64 默认功能的 Cargo 解析图生成，包含运行依赖、构建依赖及其传递依赖。它记录上游声明，不会把第三方代码重新授权为本项目的 GPL。

## 授权选择与发布

- 项目自有代码为 `GPL-2.0-or-later`。当前组合程序包含 Apache-2.0 组件，组合发行版选择 **GPL v3**；完整正文见 [LICENSES/GPL-3.0.txt](LICENSES/GPL-3.0.txt)。
- 对 `MIT OR Apache-2.0` 等任选授权，可选择 MIT 分支；`Apache-2.0 AND MIT` 和含 `AND` 的字体授权要求同时满足各项。不能把 `AND` 当成任选。
- `self_cell` 声明 `Apache-2.0 OR GPL-2.0-only`，当前 GPL v3 组合选择其 Apache-2.0 分支。
- `winit`、`sync_wrapper`、`unicode-general-category` 为 Apache-2.0；`dpi` 同时包含 Apache-2.0 与 MIT 要求。保留各自的版权和通知。
- egui 默认字体中的 OFL、Ubuntu Font Licence 和字体图标 MIT 授权继续适用；代码授权不会替换字体授权。系统字体不由仓库打包或重新分发。
- 供应源码中的许可证及下列离线文本来自依赖包。个别 crate 未在下载包中附独立许可文本时，请查阅对应上游包，发布前应核对该版本的上游许可与通知。
- 对外发布二进制时附上适用的 GPL 和第三方通知、字体许可，并按 GPL 要求提供对应源码。用户提供的 `CTM.ico` 不在本项目代码授权变更范围内。

本地修改的光栅器与 epaint 见 [vendor/README.md](vendor/README.md)，其上游许可文件保持原样。依赖更新后应重新检查清单和许可证兼容性。

参考：[GNU Apache-2.0 兼容性说明](https://www.gnu.org/licenses/license-list.html#apache2)、[GNU GPL 常见问题](https://www.gnu.org/licenses/gpl-faq.html#v2v3Compatibility)。

## 锁定依赖清单

| 组件 | 版本 | 上游许可证表达式 | 包内许可/通知文本 |
| --- | --- | --- | --- |
| [accesskit](https://github.com/AccessKit/accesskit) | 0.24.1 | `MIT OR Apache-2.0` | 见上游 crate 源码包 |
| [adler2](https://github.com/oyvindln/adler2) | 2.0.1 | `0BSD OR MIT OR Apache-2.0` | [1](LICENSES/third-party/adler2-2.0.1/LICENSE-0BSD)、[2](LICENSES/third-party/adler2-2.0.1/LICENSE-APACHE)、[3](LICENSES/third-party/adler2-2.0.1/LICENSE-MIT) |
| [ahash](https://github.com/tkaitchuck/ahash) | 0.8.12 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/ahash-0.8.12/LICENSE-APACHE)、[2](LICENSES/third-party/ahash-0.8.12/LICENSE-MIT) |
| [anyhow](https://github.com/dtolnay/anyhow) | 1.0.104 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/anyhow-1.0.104/LICENSE-APACHE)、[2](LICENSES/third-party/anyhow-1.0.104/LICENSE-MIT) |
| [arboard](https://github.com/1Password/arboard) | 3.7.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/arboard-3.7.0/LICENSE-APACHE.txt)、[2](LICENSES/third-party/arboard-3.7.0/LICENSE-MIT.txt) |
| [arrayvec](https://github.com/bluss/arrayvec) | 0.7.8 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/arrayvec-0.7.8/LICENSE-APACHE)、[2](LICENSES/third-party/arrayvec-0.7.8/LICENSE-MIT) |
| [atomic-waker](https://github.com/smol-rs/atomic-waker) | 1.1.2 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/atomic-waker-1.1.2/LICENSE-APACHE)、[2](LICENSES/third-party/atomic-waker-1.1.2/LICENSE-MIT)、[3](LICENSES/third-party/atomic-waker-1.1.2/LICENSE-THIRD-PARTY) |
| [autocfg](https://github.com/cuviper/autocfg) | 1.5.1 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/autocfg-1.5.1/LICENSE-APACHE)、[2](LICENSES/third-party/autocfg-1.5.1/LICENSE-MIT) |
| [base64](https://github.com/marshallpierce/rust-base64) | 0.22.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/base64-0.22.1/LICENSE-APACHE)、[2](LICENSES/third-party/base64-0.22.1/LICENSE-MIT) |
| [base64](https://github.com/marshallpierce/rust-base64) | 0.23.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/base64-0.23.1/LICENSE-APACHE)、[2](LICENSES/third-party/base64-0.23.1/LICENSE-MIT) |
| [bitflags](https://github.com/bitflags/bitflags) | 2.13.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/bitflags-2.13.2/LICENSE-APACHE)、[2](LICENSES/third-party/bitflags-2.13.2/LICENSE-MIT) |
| [block-buffer](https://github.com/RustCrypto/utils) | 0.12.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/block-buffer-0.12.1/LICENSE-APACHE)、[2](LICENSES/third-party/block-buffer-0.12.1/LICENSE-MIT) |
| [bytemuck](https://github.com/Lokathor/bytemuck) | 1.25.2 | `Zlib OR Apache-2.0 OR MIT` | [1](LICENSES/third-party/bytemuck-1.25.2/LICENSE-APACHE)、[2](LICENSES/third-party/bytemuck-1.25.2/LICENSE-MIT)、[3](LICENSES/third-party/bytemuck-1.25.2/LICENSE-ZLIB) |
| [bytemuck_derive](https://github.com/Lokathor/bytemuck) | 1.12.1 | `Zlib OR Apache-2.0 OR MIT` | [1](LICENSES/third-party/bytemuck_derive-1.12.1/LICENSE-APACHE)、[2](LICENSES/third-party/bytemuck_derive-1.12.1/LICENSE-MIT)、[3](LICENSES/third-party/bytemuck_derive-1.12.1/LICENSE-ZLIB) |
| [byteorder-lite](https://github.com/image-rs/byteorder-lite) | 0.1.0 | `Unlicense OR MIT` | [1](LICENSES/third-party/byteorder-lite-0.1.0/LICENSE-MIT)、[2](LICENSES/third-party/byteorder-lite-0.1.0/UNLICENSE) |
| [bytes](https://github.com/tokio-rs/bytes) | 1.12.1 | `MIT` | [1](LICENSES/third-party/bytes-1.12.1/LICENSE) |
| [cfg-if](https://github.com/rust-lang/cfg-if) | 1.0.5 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/cfg-if-1.0.5/LICENSE-APACHE)、[2](LICENSES/third-party/cfg-if-1.0.5/LICENSE-MIT) |
| [cfg_aliases](https://github.com/katharostech/cfg_aliases) | 0.2.2 | `MIT` | [1](LICENSES/third-party/cfg_aliases-0.2.2/LICENSE)、[2](LICENSES/third-party/cfg_aliases-0.2.2/NOTICES.md) |
| [chrono](https://github.com/chronotope/chrono) | 0.4.45 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/chrono-0.4.45/LICENSE.txt) |
| [clipboard-win](https://github.com/DoumanAsh/clipboard-win) | 5.4.1 | `BSL-1.0` | 见上游 crate 源码包 |
| [cmov](https://github.com/RustCrypto/utils) | 0.5.4 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/cmov-0.5.4/LICENSE-APACHE)、[2](LICENSES/third-party/cmov-0.5.4/LICENSE-MIT) |
| [color](https://github.com/linebender/color) | 0.3.3 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/color-0.3.3/LICENSE-APACHE)、[2](LICENSES/third-party/color-0.3.3/LICENSE-MIT) |
| [const-oid](https://github.com/RustCrypto/formats) | 0.10.2 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/const-oid-0.10.2/LICENSE-APACHE)、[2](LICENSES/third-party/const-oid-0.10.2/LICENSE-MIT) |
| [constify](https://github.com/DGriffin91/egui_software_backend) | 0.0.1 | `MIT OR Apache-2.0` | 见上游 crate 源码包 |
| [cookie](https://github.com/SergioBenitez/cookie-rs) | 0.18.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/cookie-0.18.2/LICENSE-APACHE)、[2](LICENSES/third-party/cookie-0.18.2/LICENSE-MIT) |
| [cookie_store](https://github.com/pfernie/cookie_store) | 0.22.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/cookie_store-0.22.1/LICENSE-APACHE)、[2](LICENSES/third-party/cookie_store-0.22.1/LICENSE-MIT) |
| [core_maths](https://github.com/robertbastian/core_maths) | 0.1.1 | `MIT` | [1](LICENSES/third-party/core_maths-0.1.1/LICENSE) |
| [cpufeatures](https://github.com/RustCrypto/utils) | 0.3.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/cpufeatures-0.3.1/LICENSE-APACHE)、[2](LICENSES/third-party/cpufeatures-0.3.1/LICENSE-MIT) |
| [crc32fast](https://github.com/srijs/rust-crc32fast) | 1.5.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/crc32fast-1.5.2/LICENSE-APACHE)、[2](LICENSES/third-party/crc32fast-1.5.2/LICENSE-MIT) |
| [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam) | 0.5.17 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/crossbeam-channel-0.5.17/LICENSE-APACHE)、[2](LICENSES/third-party/crossbeam-channel-0.5.17/LICENSE-MIT)、[3](LICENSES/third-party/crossbeam-channel-0.5.17/LICENSE-THIRD-PARTY) |
| [crossbeam-utils](https://github.com/crossbeam-rs/crossbeam) | 0.8.23 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/crossbeam-utils-0.8.23/LICENSE-APACHE)、[2](LICENSES/third-party/crossbeam-utils-0.8.23/LICENSE-MIT) |
| [crypto-common](https://github.com/RustCrypto/traits) | 0.2.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/crypto-common-0.2.2/LICENSE-APACHE)、[2](LICENSES/third-party/crypto-common-0.2.2/LICENSE-MIT) |
| [ctutils](https://github.com/RustCrypto/utils) | 0.4.3 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/ctutils-0.4.3/LICENSE-APACHE)、[2](LICENSES/third-party/ctutils-0.4.3/LICENSE-MIT) |
| [cursor-icon](https://github.com/rust-windowing/cursor-icon) | 1.2.0 | `MIT OR Apache-2.0 OR Zlib` | [1](LICENSES/third-party/cursor-icon-1.2.0/LICENSE-APACHE)、[2](LICENSES/third-party/cursor-icon-1.2.0/LICENSE-MIT)、[3](LICENSES/third-party/cursor-icon-1.2.0/LICENSE-ZLIB) |
| [deranged](https://github.com/jhpratt/deranged) | 0.5.9 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/deranged-0.5.9/LICENSE-Apache)、[2](LICENSES/third-party/deranged-0.5.9/LICENSE-MIT) |
| [digest](https://github.com/RustCrypto/traits) | 0.11.3 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/digest-0.11.3/LICENSE-APACHE)、[2](LICENSES/third-party/digest-0.11.3/LICENSE-MIT) |
| [displaydoc](https://github.com/yaahc/displaydoc) | 0.2.7 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/displaydoc-0.2.7/LICENSE-APACHE)、[2](LICENSES/third-party/displaydoc-0.2.7/LICENSE-MIT) |
| [document-features](https://github.com/slint-ui/document-features) | 0.2.12 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/document-features-0.2.12/LICENSE-APACHE)、[2](LICENSES/third-party/document-features-0.2.12/LICENSE-MIT) |
| [dpi](https://github.com/rust-windowing/winit) | 0.1.2 | `Apache-2.0 AND MIT` | [1](LICENSES/third-party/dpi-0.1.2/LICENSE)、[2](LICENSES/third-party/dpi-0.1.2/LICENSE-LIBM-MIT) |
| [ecolor](https://github.com/emilk/egui) | 0.36.2 | `MIT OR Apache-2.0` | 见上游 crate 源码包 |
| [egui](https://github.com/emilk/egui) | 0.36.2 | `MIT OR Apache-2.0` | 见上游 crate 源码包 |
| [egui-winit](https://github.com/emilk/egui/tree/main/crates/egui-winit) | 0.36.2 | `MIT OR Apache-2.0` | 见上游 crate 源码包 |
| [egui_software_backend](https://crates.io/crates/egui_software_backend/0.0.3) | 0.0.3 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/egui_software_backend-0.0.3/LICENSE-APACHE)、[2](LICENSES/third-party/egui_software_backend-0.0.3/LICENSE-MIT) |
| [either](https://github.com/rayon-rs/either) | 1.19.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/either-1.19.0/LICENSE-APACHE)、[2](LICENSES/third-party/either-1.19.0/LICENSE-MIT) |
| [emath](https://github.com/emilk/egui/tree/main/crates/emath) | 0.36.2 | `MIT OR Apache-2.0` | 见上游 crate 源码包 |
| [epaint](https://github.com/emilk/egui/tree/main/crates/epaint) | 0.36.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/epaint-0.36.2/LICENSE-APACHE)、[2](LICENSES/third-party/epaint-0.36.2/LICENSE-MIT) |
| [epaint_default_fonts](https://github.com/emilk/egui/tree/main/crates/epaint_default_fonts) | 0.36.2 | `(MIT OR Apache-2.0) AND OFL-1.1 AND Ubuntu-font-1.0` | [1](LICENSES/third-party/epaint_default_fonts-0.36.2/fonts/emoji-icon-font-mit-license.txt)、[2](LICENSES/third-party/epaint_default_fonts-0.36.2/fonts/OFL.txt)、[3](LICENSES/third-party/epaint_default_fonts-0.36.2/fonts/UFL.txt) |
| [equivalent](https://github.com/indexmap-rs/equivalent) | 1.0.3 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/equivalent-1.0.3/LICENSE-APACHE)、[2](LICENSES/third-party/equivalent-1.0.3/LICENSE-MIT) |
| [error-code](https://github.com/DoumanAsh/error-code) | 3.4.0 | `BSL-1.0` | [1](LICENSES/third-party/error-code-3.4.0/LICENSE) |
| [euclid](https://github.com/servo/euclid) | 0.22.14 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/euclid-0.22.14/LICENSE-APACHE)、[2](LICENSES/third-party/euclid-0.22.14/LICENSE-MIT) |
| [fax](https://github.com/pdf-rs/fax) | 0.2.7 | `MIT` | [1](LICENSES/third-party/fax-0.2.7/LICENSE) |
| [fdeflate](https://github.com/image-rs/fdeflate) | 0.3.7 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/fdeflate-0.3.7/LICENSE-APACHE)、[2](LICENSES/third-party/fdeflate-0.3.7/LICENSE-MIT) |
| [fearless_simd](https://github.com/linebender/fearless_simd) | 0.4.1 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/fearless_simd-0.4.1/LICENSE-APACHE)、[2](LICENSES/third-party/fearless_simd-0.4.1/LICENSE-MIT) |
| [flate2](https://github.com/rust-lang/flate2-rs) | 1.1.10 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/flate2-1.1.10/LICENSE-APACHE)、[2](LICENSES/third-party/flate2-1.1.10/LICENSE-MIT) |
| [foldhash](https://github.com/orlp/foldhash) | 0.2.0 | `Zlib` | [1](LICENSES/third-party/foldhash-0.2.0/LICENSE) |
| [font-types](https://github.com/googlefonts/fontations) | 0.12.7 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/font-types-0.12.7/LICENSE-APACHE)、[2](LICENSES/third-party/font-types-0.12.7/LICENSE-MIT) |
| [fontdb](https://github.com/RazrFalcon/fontdb) | 0.23.0 | `MIT` | [1](LICENSES/third-party/fontdb-0.23.0/LICENSE)、[2](LICENSES/third-party/fontdb-0.23.0/tests/fonts/LICENSE.txt) |
| [form_urlencoded](https://github.com/servo/rust-url) | 1.2.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/form_urlencoded-1.2.2/LICENSE-APACHE)、[2](LICENSES/third-party/form_urlencoded-1.2.2/LICENSE-MIT) |
| [futures-channel](https://github.com/rust-lang/futures-rs) | 0.3.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/futures-channel-0.3.34/LICENSE-APACHE)、[2](LICENSES/third-party/futures-channel-0.3.34/LICENSE-MIT) |
| [futures-core](https://github.com/rust-lang/futures-rs) | 0.3.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/futures-core-0.3.34/LICENSE-APACHE)、[2](LICENSES/third-party/futures-core-0.3.34/LICENSE-MIT) |
| [futures-io](https://github.com/rust-lang/futures-rs) | 0.3.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/futures-io-0.3.34/LICENSE-APACHE)、[2](LICENSES/third-party/futures-io-0.3.34/LICENSE-MIT) |
| [futures-macro](https://github.com/rust-lang/futures-rs) | 0.3.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/futures-macro-0.3.34/LICENSE-APACHE)、[2](LICENSES/third-party/futures-macro-0.3.34/LICENSE-MIT) |
| [futures-sink](https://github.com/rust-lang/futures-rs) | 0.3.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/futures-sink-0.3.34/LICENSE-APACHE)、[2](LICENSES/third-party/futures-sink-0.3.34/LICENSE-MIT) |
| [futures-task](https://github.com/rust-lang/futures-rs) | 0.3.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/futures-task-0.3.34/LICENSE-APACHE)、[2](LICENSES/third-party/futures-task-0.3.34/LICENSE-MIT) |
| [futures-util](https://github.com/rust-lang/futures-rs) | 0.3.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/futures-util-0.3.34/LICENSE-APACHE)、[2](LICENSES/third-party/futures-util-0.3.34/LICENSE-MIT) |
| [glifo](https://github.com/linebender/vello) | 0.2.0 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/glifo-0.2.0/LICENSE-APACHE)、[2](LICENSES/third-party/glifo-0.2.0/LICENSE-MIT) |
| [guillotiere](https://github.com/nical/guillotiere) | 0.7.0 | `MIT/Apache-2.0` | [1](LICENSES/third-party/guillotiere-0.7.0/LICENSE-APACHE)、[2](LICENSES/third-party/guillotiere-0.7.0/LICENSE-MIT) |
| [half](https://github.com/VoidStarKat/half-rs) | 2.7.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/half-2.7.1/LICENSE-APACHE)、[2](LICENSES/third-party/half-2.7.1/LICENSE-MIT) |
| [harfrust](https://github.com/harfbuzz/harfrust) | 0.12.0 | `MIT` | [1](LICENSES/third-party/harfrust-0.12.0/LICENSE) |
| [hashbrown](https://github.com/rust-lang/hashbrown) | 0.17.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/hashbrown-0.17.1/LICENSE-APACHE)、[2](LICENSES/third-party/hashbrown-0.17.1/LICENSE-MIT) |
| [hex](https://github.com/KokaKiwi/rust-hex) | 0.4.3 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/hex-0.4.3/LICENSE-APACHE)、[2](LICENSES/third-party/hex-0.4.3/LICENSE-MIT) |
| [hmac](https://github.com/RustCrypto/MACs) | 0.13.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/hmac-0.13.0/LICENSE-APACHE)、[2](LICENSES/third-party/hmac-0.13.0/LICENSE-MIT) |
| [http](https://github.com/hyperium/http) | 1.5.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/http-1.5.0/LICENSE-APACHE)、[2](LICENSES/third-party/http-1.5.0/LICENSE-MIT) |
| [http-body](https://github.com/hyperium/http-body) | 1.1.0 | `MIT` | [1](LICENSES/third-party/http-body-1.1.0/LICENSE) |
| [http-body-util](https://github.com/hyperium/http-body) | 0.1.5 | `MIT` | [1](LICENSES/third-party/http-body-util-0.1.5/LICENSE) |
| [httparse](https://github.com/seanmonstar/httparse) | 1.10.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/httparse-1.10.1/LICENSE-APACHE)、[2](LICENSES/third-party/httparse-1.10.1/LICENSE-MIT) |
| [hybrid-array](https://github.com/RustCrypto/hybrid-array) | 0.4.15 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/hybrid-array-0.4.15/LICENSE-APACHE)、[2](LICENSES/third-party/hybrid-array-0.4.15/LICENSE-MIT) |
| [hyper](https://github.com/hyperium/hyper) | 1.12.0 | `MIT` | [1](LICENSES/third-party/hyper-1.12.0/LICENSE) |
| [hyper-tls](https://github.com/hyperium/hyper-tls) | 0.6.0 | `MIT/Apache-2.0` | [1](LICENSES/third-party/hyper-tls-0.6.0/LICENSE-APACHE)、[2](LICENSES/third-party/hyper-tls-0.6.0/LICENSE-MIT) |
| [hyper-util](https://github.com/hyperium/hyper-util) | 0.1.21 | `MIT` | [1](LICENSES/third-party/hyper-util-0.1.21/LICENSE) |
| [icu_collections](https://github.com/unicode-org/icu4x) | 2.3.0 | `Unicode-3.0` | [1](LICENSES/third-party/icu_collections-2.3.0/LICENSE) |
| [icu_locale_core](https://github.com/unicode-org/icu4x) | 2.3.0 | `Unicode-3.0` | [1](LICENSES/third-party/icu_locale_core-2.3.0/LICENSE) |
| [icu_normalizer](https://github.com/unicode-org/icu4x) | 2.3.0 | `Unicode-3.0` | [1](LICENSES/third-party/icu_normalizer-2.3.0/LICENSE) |
| [icu_normalizer_data](https://github.com/unicode-org/icu4x) | 2.3.0 | `Unicode-3.0` | [1](LICENSES/third-party/icu_normalizer_data-2.3.0/LICENSE) |
| [icu_properties](https://github.com/unicode-org/icu4x) | 2.3.0 | `Unicode-3.0` | [1](LICENSES/third-party/icu_properties-2.3.0/LICENSE) |
| [icu_properties_data](https://github.com/unicode-org/icu4x) | 2.3.0 | `Unicode-3.0` | [1](LICENSES/third-party/icu_properties_data-2.3.0/LICENSE) |
| [icu_provider](https://github.com/unicode-org/icu4x) | 2.3.1 | `Unicode-3.0` | [1](LICENSES/third-party/icu_provider-2.3.1/LICENSE) |
| [idna](https://github.com/servo/rust-url/) | 1.1.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/idna-1.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/idna-1.1.0/LICENSE-MIT) |
| [idna_adapter](https://github.com/hsivonen/idna_adapter) | 1.2.2 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/idna_adapter-1.2.2/LICENSE-APACHE)、[2](LICENSES/third-party/idna_adapter-1.2.2/LICENSE-MIT) |
| [image](https://github.com/image-rs/image) | 0.25.10 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/image-0.25.10/LICENSE-APACHE)、[2](LICENSES/third-party/image-0.25.10/LICENSE-MIT) |
| [indexmap](https://github.com/indexmap-rs/indexmap) | 2.14.2 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/indexmap-2.14.2/LICENSE-APACHE)、[2](LICENSES/third-party/indexmap-2.14.2/LICENSE-MIT) |
| [ipconfig](https://github.com/liranringel/ipconfig) | 0.3.4 | `MIT/Apache-2.0` | [1](LICENSES/third-party/ipconfig-0.3.4/LICENSE-APACHE)、[2](LICENSES/third-party/ipconfig-0.3.4/LICENSE-MIT) |
| [ipnet](https://github.com/krisprice/ipnet) | 2.12.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/ipnet-2.12.2/LICENSE-APACHE)、[2](LICENSES/third-party/ipnet-2.12.2/LICENSE-MIT) |
| [itertools](https://github.com/rust-itertools/itertools) | 0.15.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/itertools-0.15.0/LICENSE-APACHE)、[2](LICENSES/third-party/itertools-0.15.0/LICENSE-MIT) |
| [itoa](https://github.com/dtolnay/itoa) | 1.0.18 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/itoa-1.0.18/LICENSE-APACHE)、[2](LICENSES/third-party/itoa-1.0.18/LICENSE-MIT) |
| [keyboard-types](https://github.com/rust-windowing/keyboard-types) | 0.8.3 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/keyboard-types-0.8.3/LICENSE-APACHE)、[2](LICENSES/third-party/keyboard-types-0.8.3/LICENSE-MIT) |
| [kurbo](https://github.com/linebender/kurbo) | 0.13.1 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/kurbo-0.13.1/LICENSE-APACHE)、[2](LICENSES/third-party/kurbo-0.13.1/LICENSE-MIT) |
| [libc](https://github.com/rust-lang/libc) | 0.2.191 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/libc-0.2.191/LICENSE-APACHE)、[2](LICENSES/third-party/libc-0.2.191/LICENSE-MIT) |
| [libm](https://github.com/rust-lang/compiler-builtins) | 0.2.16 | `MIT` | [1](LICENSES/third-party/libm-0.2.16/LICENSE.txt) |
| [linebender_resource_handle](https://github.com/linebender/raw_resource_handle) | 0.1.1 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/linebender_resource_handle-0.1.1/LICENSE-APACHE)、[2](LICENSES/third-party/linebender_resource_handle-0.1.1/LICENSE-MIT) |
| [litemap](https://github.com/unicode-org/icu4x) | 0.8.3 | `Unicode-3.0` | [1](LICENSES/third-party/litemap-0.8.3/LICENSE) |
| [litrs](https://github.com/LukasKalbertodt/litrs) | 1.0.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/litrs-1.0.1/LICENSE-APACHE)、[2](LICENSES/third-party/litrs-1.0.1/LICENSE-MIT) |
| [lock_api](https://github.com/Amanieu/parking_lot) | 0.4.14 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/lock_api-0.4.14/LICENSE-APACHE)、[2](LICENSES/third-party/lock_api-0.4.14/LICENSE-MIT) |
| [log](https://github.com/rust-lang/log) | 0.4.34 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/log-0.4.34/LICENSE-APACHE)、[2](LICENSES/third-party/log-0.4.34/LICENSE-MIT) |
| [memchr](https://github.com/BurntSushi/memchr) | 2.8.3 | `Unlicense OR MIT` | [1](LICENSES/third-party/memchr-2.8.3/COPYING)、[2](LICENSES/third-party/memchr-2.8.3/LICENSE-MIT)、[3](LICENSES/third-party/memchr-2.8.3/UNLICENSE) |
| [memmap2](https://github.com/RazrFalcon/memmap2-rs) | 0.9.11 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/memmap2-0.9.11/LICENSE-APACHE)、[2](LICENSES/third-party/memmap2-0.9.11/LICENSE-MIT) |
| [miniz_oxide](https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide) | 0.8.9 | `MIT OR Zlib OR Apache-2.0` | [1](LICENSES/third-party/miniz_oxide-0.8.9/LICENSE)、[2](LICENSES/third-party/miniz_oxide-0.8.9/LICENSE-APACHE.md)、[3](LICENSES/third-party/miniz_oxide-0.8.9/LICENSE-MIT.md)、[4](LICENSES/third-party/miniz_oxide-0.8.9/LICENSE-ZLIB.md) |
| [miniz_oxide](https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide) | 0.9.1 | `MIT OR Zlib OR Apache-2.0` | [1](LICENSES/third-party/miniz_oxide-0.9.1/LICENSE)、[2](LICENSES/third-party/miniz_oxide-0.9.1/LICENSE-APACHE.md)、[3](LICENSES/third-party/miniz_oxide-0.9.1/LICENSE-MIT.md)、[4](LICENSES/third-party/miniz_oxide-0.9.1/LICENSE-ZLIB.md) |
| [mio](https://github.com/tokio-rs/mio) | 1.2.4 | `MIT` | [1](LICENSES/third-party/mio-1.2.4/LICENSE) |
| [moxcms](https://github.com/awxkee/moxcms.git) | 0.8.1 | `BSD-3-Clause OR Apache-2.0` | [1](LICENSES/third-party/moxcms-0.8.1/LICENSE-APACHE.md)、[2](LICENSES/third-party/moxcms-0.8.1/LICENSE.md) |
| [muda](https://github.com/tauri-apps/muda) | 0.21.2 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/muda-0.21.2/LICENSE-APACHE)、[2](LICENSES/third-party/muda-0.21.2/LICENSE-MIT)、[3](LICENSES/third-party/muda-0.21.2/LICENSE.spdx) |
| [native-tls](https://github.com/rust-native-tls/rust-native-tls) | 0.2.18 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/native-tls-0.2.18/LICENSE-APACHE)、[2](LICENSES/third-party/native-tls-0.2.18/LICENSE-MIT) |
| [nohash-hasher](https://github.com/paritytech/nohash-hasher) | 0.2.0 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/nohash-hasher-0.2.0/LICENSE-APACHE)、[2](LICENSES/third-party/nohash-hasher-0.2.0/LICENSE-MIT) |
| [num-conv](https://github.com/jhpratt/num-conv) | 0.2.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/num-conv-0.2.2/LICENSE-Apache)、[2](LICENSES/third-party/num-conv-0.2.2/LICENSE-MIT) |
| [num-traits](https://github.com/rust-num/num-traits) | 0.2.19 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/num-traits-0.2.19/LICENSE-APACHE)、[2](LICENSES/third-party/num-traits-0.2.19/LICENSE-MIT) |
| [once_cell](https://github.com/matklad/once_cell) | 1.21.4 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/once_cell-1.21.4/LICENSE-APACHE)、[2](LICENSES/third-party/once_cell-1.21.4/LICENSE-MIT) |
| [parking_lot](https://github.com/Amanieu/parking_lot) | 0.12.5 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/parking_lot-0.12.5/LICENSE-APACHE)、[2](LICENSES/third-party/parking_lot-0.12.5/LICENSE-MIT) |
| [parking_lot_core](https://github.com/Amanieu/parking_lot) | 0.9.12 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/parking_lot_core-0.9.12/LICENSE-APACHE)、[2](LICENSES/third-party/parking_lot_core-0.9.12/LICENSE-MIT) |
| [pbkdf2](https://github.com/RustCrypto/password-hashes) | 0.13.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/pbkdf2-0.13.0/LICENSE-APACHE)、[2](LICENSES/third-party/pbkdf2-0.13.0/LICENSE-MIT) |
| [peniko](https://github.com/linebender/peniko) | 0.6.1 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/peniko-0.6.1/LICENSE-APACHE)、[2](LICENSES/third-party/peniko-0.6.1/LICENSE-MIT) |
| [percent-encoding](https://github.com/servo/rust-url/) | 2.3.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/percent-encoding-2.3.2/LICENSE-APACHE)、[2](LICENSES/third-party/percent-encoding-2.3.2/LICENSE-MIT) |
| [pin-project-lite](https://github.com/taiki-e/pin-project-lite) | 0.2.17 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/pin-project-lite-0.2.17/LICENSE-APACHE)、[2](LICENSES/third-party/pin-project-lite-0.2.17/LICENSE-MIT) |
| [png](https://github.com/image-rs/image-png) | 0.18.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/png-0.18.1/LICENSE-APACHE)、[2](LICENSES/third-party/png-0.18.1/LICENSE-MIT) |
| [polycool](https://github.com/linebender/kurbo) | 0.4.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/polycool-0.4.0/LICENSE-APACHE)、[2](LICENSES/third-party/polycool-0.4.0/LICENSE-MIT) |
| [potential_utf](https://github.com/unicode-org/icu4x) | 0.1.6 | `Unicode-3.0` | [1](LICENSES/third-party/potential_utf-0.1.6/LICENSE) |
| [powerfmt](https://github.com/jhpratt/powerfmt) | 0.2.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/powerfmt-0.2.1/LICENSE-Apache)、[2](LICENSES/third-party/powerfmt-0.2.1/LICENSE-MIT) |
| [proc-macro2](https://github.com/dtolnay/proc-macro2) | 1.0.107 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/proc-macro2-1.0.107/LICENSE-APACHE)、[2](LICENSES/third-party/proc-macro2-1.0.107/LICENSE-MIT) |
| [profiling](https://github.com/aclysma/profiling) | 1.0.18 | `MIT OR Apache-2.0` | 见上游 crate 源码包 |
| [psl-types](https://github.com/addr-rs/psl-types) | 2.0.11 | `MIT/Apache-2.0` | [1](LICENSES/third-party/psl-types-2.0.11/LICENSE)、[2](LICENSES/third-party/psl-types-2.0.11/LICENSE-APACHE) |
| [publicsuffix](https://github.com/rushmorem/publicsuffix) | 2.3.0 | `MIT/Apache-2.0` | [1](LICENSES/third-party/publicsuffix-2.3.0/LICENSE)、[2](LICENSES/third-party/publicsuffix-2.3.0/LICENSE-APACHE) |
| [pxfm](https://github.com/awxkee/pxfm) | 0.1.30 | `BSD-3-Clause OR Apache-2.0` | [1](LICENSES/third-party/pxfm-0.1.30/LICENSE-APACHE.md)、[2](LICENSES/third-party/pxfm-0.1.30/LICENSE.md) |
| [quick-error](http://github.com/tailhook/quick-error) | 2.0.1 | `MIT/Apache-2.0` | [1](LICENSES/third-party/quick-error-2.0.1/LICENSE-APACHE)、[2](LICENSES/third-party/quick-error-2.0.1/LICENSE-MIT) |
| [quote](https://github.com/dtolnay/quote) | 1.0.47 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/quote-1.0.47/LICENSE-APACHE)、[2](LICENSES/third-party/quote-1.0.47/LICENSE-MIT) |
| [raw-window-handle](https://github.com/rust-windowing/raw-window-handle) | 0.6.2 | `MIT OR Apache-2.0 OR Zlib` | [1](LICENSES/third-party/raw-window-handle-0.6.2/LICENSE-APACHE.md)、[2](LICENSES/third-party/raw-window-handle-0.6.2/LICENSE-MIT.md)、[3](LICENSES/third-party/raw-window-handle-0.6.2/LICENSE-ZLIB.md) |
| [read-fonts](https://github.com/googlefonts/fontations) | 0.41.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/read-fonts-0.41.0/LICENSE-APACHE)、[2](LICENSES/third-party/read-fonts-0.41.0/LICENSE-MIT) |
| [reqwest](https://github.com/seanmonstar/reqwest) | 0.13.5 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/reqwest-0.13.5/LICENSE-APACHE)、[2](LICENSES/third-party/reqwest-0.13.5/LICENSE-MIT) |
| [roxmltree](https://github.com/RazrFalcon/roxmltree) | 0.21.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/roxmltree-0.21.1/LICENSE-APACHE)、[2](LICENSES/third-party/roxmltree-0.21.1/LICENSE-MIT) |
| [rustls-pki-types](https://github.com/rustls/pki-types) | 1.15.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/rustls-pki-types-1.15.1/LICENSE-APACHE)、[2](LICENSES/third-party/rustls-pki-types-1.15.1/LICENSE-MIT) |
| [schannel](https://github.com/steffengy/schannel-rs) | 0.1.30 | `MIT` | [1](LICENSES/third-party/schannel-0.1.30/LICENSE.md) |
| [scopeguard](https://github.com/bluss/scopeguard) | 1.2.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/scopeguard-1.2.0/LICENSE-APACHE)、[2](LICENSES/third-party/scopeguard-1.2.0/LICENSE-MIT) |
| [self_cell](https://github.com/Voultapher/self_cell) | 1.3.0 | `Apache-2.0 OR GPL-2.0-only` | [1](LICENSES/third-party/self_cell-1.3.0/LICENSE-APACHE)、[2](LICENSES/third-party/self_cell-1.3.0/LICENSE-GPLv2) |
| [serde](https://github.com/serde-rs/serde) | 1.0.229 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/serde-1.0.229/LICENSE-APACHE)、[2](LICENSES/third-party/serde-1.0.229/LICENSE-MIT) |
| [serde_core](https://github.com/serde-rs/serde) | 1.0.229 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/serde_core-1.0.229/LICENSE-APACHE)、[2](LICENSES/third-party/serde_core-1.0.229/LICENSE-MIT) |
| [serde_derive](https://github.com/serde-rs/serde) | 1.0.229 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/serde_derive-1.0.229/LICENSE-APACHE)、[2](LICENSES/third-party/serde_derive-1.0.229/LICENSE-MIT) |
| [serde_json](https://github.com/serde-rs/json) | 1.0.154 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/serde_json-1.0.154/LICENSE-APACHE)、[2](LICENSES/third-party/serde_json-1.0.154/LICENSE-MIT) |
| [serde_spanned](https://github.com/toml-rs/toml) | 1.1.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/serde_spanned-1.1.2/LICENSE-APACHE)、[2](LICENSES/third-party/serde_spanned-1.1.2/LICENSE-MIT) |
| [sha2](https://github.com/RustCrypto/hashes) | 0.11.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/sha2-0.11.1/LICENSE-APACHE)、[2](LICENSES/third-party/sha2-0.11.1/LICENSE-MIT) |
| [simd-adler32](https://github.com/mcountryman/simd-adler32) | 0.3.10 | `MIT` | [1](LICENSES/third-party/simd-adler32-0.3.10/LICENSE.md) |
| [skrifa](https://github.com/googlefonts/fontations) | 0.44.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/skrifa-0.44.0/LICENSE-APACHE)、[2](LICENSES/third-party/skrifa-0.44.0/LICENSE-MIT) |
| [slab](https://github.com/tokio-rs/slab) | 0.4.12 | `MIT` | [1](LICENSES/third-party/slab-0.4.12/LICENSE) |
| [slotmap](https://github.com/orlp/slotmap) | 1.1.1 | `Zlib` | [1](LICENSES/third-party/slotmap-1.1.1/LICENSE) |
| [smallvec](https://github.com/servo/rust-smallvec) | 1.16.3 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/smallvec-1.16.3/LICENSE-APACHE)、[2](LICENSES/third-party/smallvec-1.16.3/LICENSE-MIT) |
| [smol_str](https://github.com/rust-analyzer/smol_str) | 0.2.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/smol_str-0.2.2/LICENSE-APACHE)、[2](LICENSES/third-party/smol_str-0.2.2/LICENSE-MIT) |
| [socket2](https://github.com/rust-lang/socket2) | 0.6.5 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/socket2-0.6.5/LICENSE-APACHE)、[2](LICENSES/third-party/socket2-0.6.5/LICENSE-MIT) |
| [stable_deref_trait](https://github.com/storyyeller/stable_deref_trait) | 1.2.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/stable_deref_trait-1.2.1/LICENSE-APACHE)、[2](LICENSES/third-party/stable_deref_trait-1.2.1/LICENSE-MIT) |
| [strength_reduce](http://github.com/ejmahler/strength_reduce) | 0.2.4 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/strength_reduce-0.2.4/LICENSE-APACHE)、[2](LICENSES/third-party/strength_reduce-0.2.4/LICENSE-MIT) |
| [syn](https://github.com/dtolnay/syn) | 2.0.119 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/syn-2.0.119/LICENSE-APACHE)、[2](LICENSES/third-party/syn-2.0.119/LICENSE-MIT) |
| [syn](https://github.com/dtolnay/syn) | 3.0.9 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/syn-3.0.9/LICENSE-APACHE)、[2](LICENSES/third-party/syn-3.0.9/LICENSE-MIT) |
| [sync_wrapper](https://github.com/Actyx/sync_wrapper) | 1.0.2 | `Apache-2.0` | [1](LICENSES/third-party/sync_wrapper-1.0.2/LICENSE) |
| [synstructure](https://github.com/mystor/synstructure) | 0.14.0 | `MIT` | [1](LICENSES/third-party/synstructure-0.14.0/LICENSE) |
| [thiserror](https://github.com/dtolnay/thiserror) | 2.0.21 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/thiserror-2.0.21/LICENSE-APACHE)、[2](LICENSES/third-party/thiserror-2.0.21/LICENSE-MIT) |
| [thiserror-impl](https://github.com/dtolnay/thiserror) | 2.0.21 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/thiserror-impl-2.0.21/LICENSE-APACHE)、[2](LICENSES/third-party/thiserror-impl-2.0.21/LICENSE-MIT) |
| [tiff](https://github.com/image-rs/image-tiff) | 0.11.3 | `MIT` | [1](LICENSES/third-party/tiff-0.11.3/LICENSE) |
| [time](https://github.com/time-rs/time) | 0.3.55 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/time-0.3.55/LICENSE-Apache)、[2](LICENSES/third-party/time-0.3.55/LICENSE-MIT) |
| [time-core](https://github.com/time-rs/time) | 0.1.9 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/time-core-0.1.9/LICENSE-Apache)、[2](LICENSES/third-party/time-core-0.1.9/LICENSE-MIT) |
| [time-macros](https://github.com/time-rs/time) | 0.2.32 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/time-macros-0.2.32/LICENSE-Apache)、[2](LICENSES/third-party/time-macros-0.2.32/LICENSE-MIT) |
| [tinystr](https://github.com/unicode-org/icu4x) | 0.8.4 | `Unicode-3.0` | [1](LICENSES/third-party/tinystr-0.8.4/LICENSE) |
| [tinyvec](https://github.com/Lokathor/tinyvec) | 1.13.3 | `Zlib OR Apache-2.0 OR MIT` | [1](LICENSES/third-party/tinyvec-1.13.3/LICENSE-APACHE.md)、[2](LICENSES/third-party/tinyvec-1.13.3/LICENSE-MIT.md)、[3](LICENSES/third-party/tinyvec-1.13.3/LICENSE-ZLIB.md) |
| [tokio](https://github.com/tokio-rs/tokio) | 1.53.3 | `MIT` | [1](LICENSES/third-party/tokio-1.53.3/LICENSE) |
| [tokio-native-tls](https://github.com/tokio-rs/tls) | 0.3.1 | `MIT` | [1](LICENSES/third-party/tokio-native-tls-0.3.1/LICENSE) |
| [toml](https://github.com/toml-rs/toml) | 1.1.8+spec-1.1.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/toml-1.1.8+spec-1.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/toml-1.1.8+spec-1.1.0/LICENSE-MIT) |
| [toml_datetime](https://github.com/toml-rs/toml) | 1.1.2+spec-1.1.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/toml_datetime-1.1.2+spec-1.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/toml_datetime-1.1.2+spec-1.1.0/LICENSE-MIT) |
| [toml_parser](https://github.com/toml-rs/toml) | 1.1.5+spec-1.1.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/toml_parser-1.1.5+spec-1.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/toml_parser-1.1.5+spec-1.1.0/LICENSE-MIT) |
| [toml_writer](https://github.com/toml-rs/toml) | 1.1.3+spec-1.1.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/toml_writer-1.1.3+spec-1.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/toml_writer-1.1.3+spec-1.1.0/LICENSE-MIT) |
| [tower](https://github.com/tower-rs/tower) | 0.5.3 | `MIT` | [1](LICENSES/third-party/tower-0.5.3/LICENSE) |
| [tower-http](https://github.com/tower-rs/tower-http) | 0.6.11 | `MIT` | [1](LICENSES/third-party/tower-http-0.6.11/LICENSE) |
| [tower-layer](https://github.com/tower-rs/tower) | 0.3.3 | `MIT` | [1](LICENSES/third-party/tower-layer-0.3.3/LICENSE) |
| [tower-service](https://github.com/tower-rs/tower) | 0.3.3 | `MIT` | [1](LICENSES/third-party/tower-service-0.3.3/LICENSE) |
| [tracing](https://github.com/tokio-rs/tracing) | 0.1.44 | `MIT` | [1](LICENSES/third-party/tracing-0.1.44/LICENSE) |
| [tracing-core](https://github.com/tokio-rs/tracing) | 0.1.36 | `MIT` | [1](LICENSES/third-party/tracing-core-0.1.36/LICENSE)、[2](LICENSES/third-party/tracing-core-0.1.36/src/spin/LICENSE) |
| [tray-icon](https://github.com/tauri-apps/tray-icon) | 0.26.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/tray-icon-0.26.1/LICENSE-APACHE)、[2](LICENSES/third-party/tray-icon-0.26.1/LICENSE-MIT)、[3](LICENSES/third-party/tray-icon-0.26.1/LICENSE.spdx) |
| [try-lock](https://github.com/seanmonstar/try-lock) | 0.2.5 | `MIT` | [1](LICENSES/third-party/try-lock-0.2.5/LICENSE) |
| [ttf-parser](https://github.com/harfbuzz/ttf-parser) | 0.25.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/ttf-parser-0.25.1/LICENSE-APACHE)、[2](LICENSES/third-party/ttf-parser-0.25.1/LICENSE-MIT) |
| [typenum](https://github.com/paholg/typenum) | 1.20.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/typenum-1.20.1/LICENSE)、[2](LICENSES/third-party/typenum-1.20.1/LICENSE-APACHE)、[3](LICENSES/third-party/typenum-1.20.1/LICENSE-MIT) |
| [unicode-general-category](https://github.com/yeslogic/unicode-general-category) | 1.1.0 | `Apache-2.0` | [1](LICENSES/third-party/unicode-general-category-1.1.0/LICENSE) |
| [unicode-ident](https://github.com/dtolnay/unicode-ident) | 1.0.27 | `(MIT OR Apache-2.0) AND Unicode-3.0` | [1](LICENSES/third-party/unicode-ident-1.0.27/LICENSE-APACHE)、[2](LICENSES/third-party/unicode-ident-1.0.27/LICENSE-MIT)、[3](LICENSES/third-party/unicode-ident-1.0.27/LICENSE-UNICODE) |
| [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation) | 1.13.3 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/unicode-segmentation-1.13.3/LICENSE-APACHE)、[2](LICENSES/third-party/unicode-segmentation-1.13.3/LICENSE-MIT) |
| [url](https://github.com/servo/rust-url) | 2.5.8 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/url-2.5.8/LICENSE-APACHE)、[2](LICENSES/third-party/url-2.5.8/LICENSE-MIT) |
| [utf8_iter](https://github.com/hsivonen/utf8_iter) | 1.0.4 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/utf8_iter-1.0.4/LICENSE-APACHE)、[2](LICENSES/third-party/utf8_iter-1.0.4/LICENSE-MIT) |
| [uuid](https://github.com/uuid-rs/uuid) | 1.28.0 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/uuid-1.28.0/LICENSE-APACHE)、[2](LICENSES/third-party/uuid-1.28.0/LICENSE-MIT) |
| [vello_common](https://github.com/linebender/vello) | 0.1.0 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/vello_common-0.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/vello_common-0.1.0/LICENSE-MIT) |
| [vello_cpu](https://github.com/linebender/vello) | 0.1.0 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/vello_cpu-0.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/vello_cpu-0.1.0/LICENSE-MIT) |
| [version_check](https://github.com/SergioBenitez/version_check) | 0.9.5 | `MIT/Apache-2.0` | [1](LICENSES/third-party/version_check-0.9.5/LICENSE-APACHE)、[2](LICENSES/third-party/version_check-0.9.5/LICENSE-MIT) |
| [want](https://github.com/seanmonstar/want) | 0.3.2 | `MIT` | [1](LICENSES/third-party/want-0.3.2/LICENSE) |
| [web-time](https://github.com/daxpedda/web-time) | 1.1.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/web-time-1.1.0/LICENSE-APACHE)、[2](LICENSES/third-party/web-time-1.1.0/LICENSE-MIT) |
| [weezl](https://github.com/image-rs/weezl) | 0.1.12 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/weezl-0.1.12/LICENSE-APACHE)、[2](LICENSES/third-party/weezl-0.1.12/LICENSE-MIT) |
| [widestring](https://github.com/VoidStarKat/widestring-rs) | 1.2.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/widestring-1.2.1/LICENSE-APACHE)、[2](LICENSES/third-party/widestring-1.2.1/LICENSE-MIT) |
| [windows-link](https://github.com/microsoft/windows-rs) | 0.2.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows-link-0.2.1/license-apache-2.0)、[2](LICENSES/third-party/windows-link-0.2.1/license-mit) |
| [windows-registry](https://github.com/microsoft/windows-rs) | 0.6.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows-registry-0.6.1/license-apache-2.0)、[2](LICENSES/third-party/windows-registry-0.6.1/license-mit) |
| [windows-result](https://github.com/microsoft/windows-rs) | 0.4.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows-result-0.4.1/license-apache-2.0)、[2](LICENSES/third-party/windows-result-0.4.1/license-mit) |
| [windows-strings](https://github.com/microsoft/windows-rs) | 0.5.1 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows-strings-0.5.1/license-apache-2.0)、[2](LICENSES/third-party/windows-strings-0.5.1/license-mit) |
| [windows-sys](https://github.com/microsoft/windows-rs) | 0.52.0 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows-sys-0.52.0/license-apache-2.0)、[2](LICENSES/third-party/windows-sys-0.52.0/license-mit) |
| [windows-sys](https://github.com/microsoft/windows-rs) | 0.61.2 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows-sys-0.61.2/license-apache-2.0)、[2](LICENSES/third-party/windows-sys-0.61.2/license-mit) |
| [windows-targets](https://github.com/microsoft/windows-rs) | 0.52.6 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows-targets-0.52.6/license-apache-2.0)、[2](LICENSES/third-party/windows-targets-0.52.6/license-mit) |
| [windows_x86_64_msvc](https://github.com/microsoft/windows-rs) | 0.52.6 | `MIT OR Apache-2.0` | [1](LICENSES/third-party/windows_x86_64_msvc-0.52.6/license-apache-2.0)、[2](LICENSES/third-party/windows_x86_64_msvc-0.52.6/license-mit) |
| [winit](https://github.com/rust-windowing/winit) | 0.30.13 | `Apache-2.0` | [1](LICENSES/third-party/winit-0.30.13/LICENSE) |
| [winnow](https://github.com/winnow-rs/winnow) | 1.0.4 | `MIT` | [1](LICENSES/third-party/winnow-1.0.4/LICENSE-MIT) |
| [winreg](https://github.com/gentoo90/winreg-rs) | 0.56.0 | `MIT` | [1](LICENSES/third-party/winreg-0.56.0/LICENSE) |
| [winresource](https://github.com/BenjaminRi/winresource) | 0.1.34 | `MIT` | [1](LICENSES/third-party/winresource-0.1.34/LICENSE) |
| [writeable](https://github.com/unicode-org/icu4x) | 0.6.4 | `Unicode-3.0` | [1](LICENSES/third-party/writeable-0.6.4/LICENSE) |
| [yoke](https://github.com/unicode-org/icu4x) | 0.8.3 | `Unicode-3.0` | [1](LICENSES/third-party/yoke-0.8.3/LICENSE) |
| [yoke-derive](https://github.com/unicode-org/icu4x) | 0.8.4 | `Unicode-3.0` | [1](LICENSES/third-party/yoke-derive-0.8.4/LICENSE) |
| [zerocopy](https://github.com/google/zerocopy) | 0.8.63 | `BSD-2-Clause OR Apache-2.0 OR MIT` | [1](LICENSES/third-party/zerocopy-0.8.63/LICENSE-APACHE)、[2](LICENSES/third-party/zerocopy-0.8.63/LICENSE-BSD)、[3](LICENSES/third-party/zerocopy-0.8.63/LICENSE-MIT) |
| [zerocopy-derive](https://github.com/google/zerocopy) | 0.8.63 | `BSD-2-Clause OR Apache-2.0 OR MIT` | [1](LICENSES/third-party/zerocopy-derive-0.8.63/LICENSE-APACHE)、[2](LICENSES/third-party/zerocopy-derive-0.8.63/LICENSE-BSD)、[3](LICENSES/third-party/zerocopy-derive-0.8.63/LICENSE-MIT) |
| [zerofrom](https://github.com/unicode-org/icu4x) | 0.1.8 | `Unicode-3.0` | [1](LICENSES/third-party/zerofrom-0.1.8/LICENSE) |
| [zerofrom-derive](https://github.com/unicode-org/icu4x) | 0.1.8 | `Unicode-3.0` | [1](LICENSES/third-party/zerofrom-derive-0.1.8/LICENSE) |
| [zeroize](https://github.com/RustCrypto/utils) | 1.9.1 | `Apache-2.0 OR MIT` | [1](LICENSES/third-party/zeroize-1.9.1/LICENSE-APACHE)、[2](LICENSES/third-party/zeroize-1.9.1/LICENSE-MIT) |
| [zerotrie](https://github.com/unicode-org/icu4x) | 0.2.5 | `Unicode-3.0` | [1](LICENSES/third-party/zerotrie-0.2.5/LICENSE) |
| [zerovec](https://github.com/unicode-org/icu4x) | 0.11.8 | `Unicode-3.0` | [1](LICENSES/third-party/zerovec-0.11.8/LICENSE) |
| [zerovec-derive](https://github.com/unicode-org/icu4x) | 0.11.6 | `Unicode-3.0` | [1](LICENSES/third-party/zerovec-derive-0.11.6/LICENSE) |
| [zlib-rs](https://github.com/trifectatechfoundation/zlib-rs) | 0.6.8 | `Zlib` | [1](LICENSES/third-party/zlib-rs-0.6.8/LICENSE) |
| [zmij](https://github.com/dtolnay/zmij) | 1.0.23 | `MIT` | [1](LICENSES/third-party/zmij-1.0.23/LICENSE-MIT) |
| [zune-core](https://github.com/etemesi254/zune-image) | 0.5.3 | `MIT OR Apache-2.0 OR Zlib` | [1](LICENSES/third-party/zune-core-0.5.3/LICENSE-APACHE)、[2](LICENSES/third-party/zune-core-0.5.3/LICENSE-MIT)、[3](LICENSES/third-party/zune-core-0.5.3/LICENSE-ZLIB) |
| [zune-jpeg](https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg) | 0.5.15 | `MIT OR Apache-2.0 OR Zlib` | [1](LICENSES/third-party/zune-jpeg-0.5.15/LICENSE-APACHE)、[2](LICENSES/third-party/zune-jpeg-0.5.15/LICENSE-MIT)、[3](LICENSES/third-party/zune-jpeg-0.5.15/LICENSE-ZLIB) |
