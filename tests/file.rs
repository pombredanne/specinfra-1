extern crate specinfra;

use specinfra::backend;

#[test]
#[cfg(any(target_os="macos", target_os="linux"))]
fn file_resource() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/etc/passwd");

    assert_eq!(file.mode().unwrap(), 0o644);
    assert_eq!(file.is_file().unwrap(), true);
    assert_eq!(file.exist().unwrap(), true);
    assert_eq!(file.is_directory().unwrap(), false);
    assert_eq!(file.is_block_device().unwrap(), false);
    assert_eq!(file.is_character_device().unwrap(), false);
    assert_eq!(file.is_pipe().unwrap(), false);
    assert_eq!(file.is_socket().unwrap(), false);
    assert_eq!(file.is_symlink().unwrap(), false);
    assert_eq!(file.contents().unwrap().contains("root"), true);
    assert_eq!(file.owner().unwrap(), "root");

    let group = file.group().unwrap();
    assert!(group == "root" || group == "wheel");

    assert_eq!(file.is_readable().unwrap(), true);
    assert_eq!(file.is_readable_by_owner().unwrap(), true);
    assert_eq!(file.is_readable_by_group().unwrap(), true);
    assert_eq!(file.is_readable_by_others().unwrap(), true);
    assert_eq!(file.is_readable_by_user("root").unwrap(), true);

    assert_eq!(file.is_writable().unwrap(), true);
    assert_eq!(file.is_writable_by_owner().unwrap(), true);
    assert_eq!(file.is_writable_by_group().unwrap(), false);
    assert_eq!(file.is_writable_by_others().unwrap(), false);
    assert_eq!(file.is_writable_by_user("root").unwrap(), true);

    assert_eq!(file.md5sum().unwrap().len(), 32);
    assert_eq!(file.sha256sum().unwrap().len(), 64);

    assert!(file.size().unwrap() > 0);
}

#[test]
fn file_not_exist() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("file_does_not_exist");

    assert_eq!(file.exist().unwrap(), false);
}

#[test]
#[cfg(target_os="macos")]
fn file_link() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/etc");

    assert_eq!(file.linked_to().unwrap(), "private/etc");
}

#[test]
#[cfg(target_os="linux")]
fn file_link() {
    let b = backend::direct::Direct::new();
    let s = specinfra::new(&b).unwrap();
    let file = s.file("/var/lock");

    assert_eq!(file.linked_to().unwrap(), "/run/lock");
}
