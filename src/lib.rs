use std::fmt;

/// A structure to hold information about a package.
///
/// # Functionality
/// Holds metadata about a rust package including name, version, authors, description,
/// and a list of dependencies.
///
/// # Fields
/// - `name`: The name of the package.
/// - `version`: The version of the package.
/// - `authors`: The authors of the package.
/// - `description`: The description of the package.
/// - `dependencies`: A list of dependencies for the package.
///
/// # Parameters
/// None.
///
/// # Returns
/// None.
///
/// # Error Handling
/// None.
///
/// # Panicking
/// None.
#[derive(PartialEq, PartialOrd, Debug)]
pub struct PackageInfos {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: &'static str,
    pub description: &'static str,
    pub dependencies: Vec<PackageInfos>,
}

impl fmt::Display for PackageInfos {
    /// # Functionality
    /// Formats the package information for display.
    ///
    /// # Parameters
    /// - `p_f`: The formatter to write to.
    ///
    /// # Returns
    /// - `fmt::Result`: Result of the formatting operation.
    ///
    /// # Error Handling
    /// Returns an error if writing to the formatter fails.
    ///
    /// # Panicking
    /// This function does not panic.
    fn fmt(&self, p_f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            p_f,
            "{} crate version {}\n   {}\n   Authors : {}",
            self.name, self.version, self.description, self.authors
        )?;
        if !self.dependencies.is_empty() {
            write!(p_f, "\n\n   Using librairies :")?;
        }
        for l_dependency in self.dependencies.iter() {
            let l_binding = l_dependency.to_string();
            let mut l_split = l_binding.lines();
            if let Some(l_first) = l_split.next() {
                write!(p_f, "\n      * {}", l_first)?;
            }
            for l_line in l_split {
                write!(p_f, "\n        {}", l_line)?;
            }
        }
        writeln!(p_f)
    }
}

/// Constructs a `PackageInfos` structure with metadata and dependencies.
///
/// # Functionality
/// This macro gathers information about the package from the environment variables
/// set by Cargo, and also includes the provided dependencies.
///
/// # Parameters
/// - `args`: A comma-separated list of expressions that each return a `PackageInfos` structure,
///   representing the dependencies of the package.
///
/// # Returns
/// - A `PackageInfos` structure containing metadata about the package and its dependencies.
///
/// # Error Handling
/// This macro does not handle errors. It relies on Cargo environment variables being present.
///
/// # Panicking
/// This macro will panic at compile time if the required Cargo environment variables
/// (`CARGO_PKG_NAME`, `CARGO_PKG_VERSION`, `CARGO_PKG_AUTHORS`, `CARGO_PKG_DESCRIPTION`) are not set.
#[macro_export]
macro_rules! pkg_infos {
    ( $( $x:ident ),* ) => {

            pub fn get_package_infos() -> PackageInfos {
                let l_name = env!("CARGO_PKG_NAME");
                let l_version = env!("CARGO_PKG_VERSION");
                let l_authors = env!("CARGO_PKG_AUTHORS");
                let l_description = env!("CARGO_PKG_DESCRIPTION");
                let l_dependencies = vec![
                $(
                    $x::get_package_infos(),
                )*
                ];

                PackageInfos {
                    name: l_name,
                    version: l_version,
                    authors: l_authors,
                    description: l_description,
                    dependencies: l_dependencies
                }
            }
    };
}

pkg_infos!();
