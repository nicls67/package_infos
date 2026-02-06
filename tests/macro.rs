use package_infos::PackageInfos;

#[test]
fn test_macro() {
    let infos = package_infos::get_package_infos();

    let expected = PackageInfos {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        authors: env!("CARGO_PKG_AUTHORS"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        dependencies: Vec::new(),
    };

    assert_eq!(infos, expected);
}
