/// A structure to hold information about a package.
///
/// # Fields
/// - `name`: The name of the package.
/// - `version`: The version of the package.
/// - `authors`: The authors of the package.
/// - `description`: The description of the package.
/// - `dependencies`: A list of dependencies for the package.
pub struct PackageInfos {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: &'static str,
    pub description: &'static str,
    pub dependencies: Vec<PackageInfos>,
}

#[macro_export]
macro_rules! pkg_infos {
    ( $( $x:expr ),* ) => {
            /// Constructs a `PackageInfos` structure with metadata and dependencies.
            ///
            /// This macro gathers information about the package from the environment variables
            /// set by Cargo, and also includes the provided dependencies.
            ///
            /// # Arguments
            ///
            /// - `$( $x:expr ),*`: A comma-separated list of expressions that each return a `PackageInfos` structure,
            /// representing the dependencies of the package.
            ///
            /// # Returns
            ///
            /// A `PackageInfos` structure containing metadata about the package and its dependencies.
            pub fn get_package_infos() -> PackageInfos {
                let name = env!("CARGO_PKG_NAME");
                let version = env!("CARGO_PKG_VERSION");
                let authors = env!("CARGO_PKG_AUTHORS");
                let description = env!("CARGO_PKG_DESCRIPTION");
                let mut dependencies = Vec::new();
                $(
                    dependencies.push($x.get_package_infos());
                )*

                PackageInfos {
                    name,
                    version,
                    authors,
                    description,
                    dependencies
                }
            }
    };
}
