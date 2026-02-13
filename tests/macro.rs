use package_infos::PackageInfos;
use rusttests::{check_value, CheckType};

#[test]
fn test_macro() -> Result<(), String> {
    let l_infos = package_infos::get_package_infos();

    let l_expected = PackageInfos {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        authors: env!("CARGO_PKG_AUTHORS"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        dependencies: Vec::new(),
    };

    check_value((1, 1), &l_infos, &l_expected, CheckType::Equal)
}
