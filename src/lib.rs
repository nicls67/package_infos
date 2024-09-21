

pub struct PackageInfos {
    name: String,
    version: String,
    author: String,
    description: String,
    dependencies: Vec<PackageInfos>
}