use package_infos::PackageInfos;
use rusttests::{check_value, CheckType};

#[test]
fn test_macro() -> Result<(), String> {
    let infos = package_infos::get_package_infos();

    let expected = PackageInfos {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        authors: env!("CARGO_PKG_AUTHORS"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        dependencies: Vec::new(),
    };

    check_value((1, 1), &infos, &expected, CheckType::Equal)
}
