//! Private, executable-relative data with current-user/SYSTEM ACLs and atomic writes.
use anyhow::{Context, Result, bail};
use std::{fs, io::Write, os::windows::fs::MetadataExt, path::Path, sync::OnceLock};
use windows_sys::Win32::{
    Foundation::*,
    Security::{Authorization::*, *},
    System::Threading::*,
};
fn wide(s: &std::ffi::OsStr) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    s.encode_wide().chain(Some(0)).collect()
}
fn no_link(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(m) if m.file_attributes() & 0x400 != 0 => bail!("数据路径不允许符号链接或目录联接"),
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}
fn secure(path: &Path) -> Result<()> {
    no_link(path)?;
    unsafe {
        let mut token = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            bail!("无法读取当前用户身份");
        }
        let mut size = 0;
        GetTokenInformation(token, TokenUser, std::ptr::null_mut(), 0, &mut size);
        let mut data = vec![0u8; size as usize];
        let ok = GetTokenInformation(token, TokenUser, data.as_mut_ptr().cast(), size, &mut size);
        CloseHandle(token);
        if ok == 0 {
            bail!("无法读取当前用户 SID");
        }
        let user = &*data.as_ptr().cast::<TOKEN_USER>();
        let mut sid = std::ptr::null_mut();
        if ConvertSidToStringSidW(user.User.Sid, &mut sid) == 0 {
            bail!("无法转换用户 SID");
        }
        let mut len = 0;
        while *sid.add(len) != 0 {
            len += 1;
        }
        let sid_string = String::from_utf16_lossy(std::slice::from_raw_parts(sid, len));
        LocalFree(sid.cast());
        let sddl = wide(std::ffi::OsStr::new(&format!(
            "D:P(A;OICI;FA;;;{sid_string})(A;OICI;FA;;;SY)"
        )));
        let mut descriptor = std::ptr::null_mut();
        if ConvertStringSecurityDescriptorToSecurityDescriptorW(
            sddl.as_ptr(),
            SDDL_REVISION_1,
            &mut descriptor,
            std::ptr::null_mut(),
        ) == 0
        {
            bail!("无法创建数据访问权限");
        }
        let ok = SetFileSecurityW(
            wide(path.as_os_str()).as_ptr(),
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            descriptor,
        );
        LocalFree(descriptor);
        if ok == 0 {
            bail!("无法限制数据文件权限：{}", std::io::Error::last_os_error());
        }
    }
    Ok(())
}
pub fn init() -> Result<()> {
    static READY: OnceLock<std::result::Result<(), String>> = OnceLock::new();
    READY
        .get_or_init(|| {
            (|| -> Result<()> {
                let dir = crate::model::data_dir();
                for p in dir.ancestors() {
                    no_link(p)?;
                }
                fs::create_dir_all(&dir)?;
                secure(&dir)?;
                for e in fs::read_dir(&dir)? {
                    let p = e?.path();
                    no_link(&p)?;
                    if p.is_file() {
                        secure(&p)?;
                    }
                }
                Ok(())
            })()
            .map_err(|e| e.to_string())
        })
        .as_ref()
        .map_err(|e| anyhow::anyhow!(e.clone()))
        .copied()
}
pub fn read(name: &str) -> Result<Vec<u8>> {
    init()?;
    let p = crate::model::data_dir().join(name);
    no_link(&p)?;
    let m = fs::metadata(&p)?;
    anyhow::ensure!(m.len() <= 1024 * 1024, "数据文件超过大小限制");
    fs::read(p).context("无法读取本地数据")
}
pub fn write(name: &str, bytes: &[u8]) -> Result<()> {
    init()?;
    anyhow::ensure!(
        !name.contains(['/', '\\']) && name != "..",
        "无效数据文件名"
    );
    let dir = crate::model::data_dir();
    let path = dir.join(name);
    no_link(&dir)?;
    no_link(&path)?;
    let mut random = [0u8; 16];
    unsafe {
        use windows_sys::Win32::Security::Cryptography::*;
        if BCryptGenRandom(
            std::ptr::null_mut(),
            random.as_mut_ptr(),
            16,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        ) != 0
        {
            bail!("随机数生成失败");
        }
    }
    let temp = dir.join(format!(".{}.tmp", hex::encode(random)));
    let result = (|| -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        secure(&temp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        no_link(&path)?;
        fs::rename(&temp, &path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn atomic_replace_and_private_acl() {
        let name = "storage-test.json";
        write(name, b"old").unwrap();
        write(name, b"new").unwrap();
        assert_eq!(read(name).unwrap(), b"new");
        unsafe {
            let path = crate::model::data_dir().join(name);
            let mut needed = 0;
            GetFileSecurityW(
                wide(path.as_os_str()).as_ptr(),
                DACL_SECURITY_INFORMATION,
                std::ptr::null_mut(),
                0,
                &mut needed,
            );
            let mut bytes = vec![0u8; needed as usize];
            assert_ne!(
                GetFileSecurityW(
                    wide(path.as_os_str()).as_ptr(),
                    DACL_SECURITY_INFORMATION,
                    bytes.as_mut_ptr().cast(),
                    needed,
                    &mut needed
                ),
                0
            );
            let mut control = 0;
            let mut revision = 0;
            assert_ne!(
                GetSecurityDescriptorControl(
                    bytes.as_mut_ptr().cast(),
                    &mut control,
                    &mut revision
                ),
                0
            );
            assert_ne!(control & SE_DACL_PROTECTED, 0);
            let mut present = 0;
            let mut defaulted = 0;
            let mut acl = std::ptr::null_mut();
            assert_ne!(
                GetSecurityDescriptorDacl(
                    bytes.as_mut_ptr().cast(),
                    &mut present,
                    &mut acl,
                    &mut defaulted
                ),
                0
            );
            assert_ne!(present, 0);
            assert!(!acl.is_null());
            assert_eq!((*acl).AceCount, 2);
        }
        fs::remove_file(crate::model::data_dir().join(name)).unwrap();
        assert!(write("../invalid", b"x").is_err());
    }
}
