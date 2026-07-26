use crate::catalog::classifier::{ClassificationDb, Classifier};
use crate::catalog::tool::Tool;
use crate::catalog::Repository;
use crate::error::{AppError, Result};
use alpm::Alpm;
use alpm_utils::alpm_with_conf;
use hashbrown::HashMap;

pub struct AlpmBackend {
    alpm: Alpm,
    has_blackarch: bool,
    classification_db: ClassificationDb,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub installed_version: Option<String>,
    pub installed: bool,
    pub repository: Repository,
    pub groups: Vec<String>,
    pub depends_on: Vec<String>,
    pub optdepends: Vec<String>,
    pub licenses: Vec<String>,
    pub homepage: Option<String>,
    pub base: Option<String>,
    pub packager: Option<String>,
    pub arch: Option<String>,
    pub build_date: Option<i64>,
    pub install_date: Option<i64>,
    pub download_size: Option<i64>,
    pub installed_size: Option<i64>,
    pub filename: Option<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub replaces: Vec<String>,
}

impl AlpmBackend {
    pub fn new() -> Result<Self> {
        let conf = pacmanconf::Config::new()
            .map_err(|e| AppError::Alpm(format!("Failed to parse pacman.conf: {}", e)))?;

        let alpm = alpm_with_conf(&conf)
            .map_err(|e| AppError::Alpm(format!("Failed to initialize ALPM: {}", e)))?;

        let has_blackarch = alpm.syncdbs().iter().any(|db| db.name() == "blackarch");

        if has_blackarch {
            tracing::info!("BlackArch repository detected");
        } else {
            tracing::info!("BlackArch repository not detected");
        }

        Ok(Self {
            alpm,
            has_blackarch,
            classification_db: ClassificationDb::default(),
        })
    }

    pub fn has_blackarch(&self) -> bool {
        self.has_blackarch
    }

    pub fn all_packages(&self) -> Result<Vec<PackageInfo>> {
        let mut packages = Vec::new();
        let local_db = self.alpm.localdb();

        let installed_names: HashMap<String, String> = local_db
            .pkgs()
            .iter()
            .map(|pkg| {
                let name = pkg.name().to_string();
                let ver = pkg.version().to_string();
                (name, ver)
            })
            .collect();

        for db in self.alpm.syncdbs() {
            let repo_name = db.name().to_string();
            let repository = match repo_name.as_str() {
                "core" => Repository::Core,
                "extra" => Repository::Extra,
                "community" => Repository::Community,
                "blackarch" => Repository::BlackArch,
                other => Repository::Custom(other.to_string()),
            };

            for pkg in db.pkgs() {
                let name = pkg.name().to_string();
                let installed_version = installed_names.get(&name).cloned();
                let installed = installed_version.is_some();

                let groups: Vec<String> = pkg.groups().iter().map(|g| g.to_string()).collect();

                let depends: Vec<String> = pkg
                    .depends()
                    .iter()
                    .map(|d| {
                        let s = d.to_string();
                        s.split(['>', '<', '='])
                            .next()
                            .unwrap_or(&s)
                            .trim()
                            .to_string()
                    })
                    .collect();

                let optdepends: Vec<String> = pkg
                    .optdepends()
                    .iter()
                    .map(|d| {
                        let s = d.to_string();
                        s.split(['>', '<', '='])
                            .next()
                            .unwrap_or(&s)
                            .trim()
                            .to_string()
                    })
                    .collect();

                let licenses: Vec<String> = pkg.licenses().iter().map(|l| l.to_string()).collect();

                let conflicts: Vec<String> = pkg
                    .conflicts()
                    .iter()
                    .map(|d| {
                        let s = d.to_string();
                        s.split(['>', '<', '='])
                            .next()
                            .unwrap_or(&s)
                            .trim()
                            .to_string()
                    })
                    .collect();

                let provides: Vec<String> = pkg
                    .provides()
                    .iter()
                    .map(|d| {
                        let s = d.to_string();
                        s.split(['>', '<', '='])
                            .next()
                            .unwrap_or(&s)
                            .trim()
                            .to_string()
                    })
                    .collect();

                let replaces: Vec<String> = pkg
                    .replaces()
                    .iter()
                    .map(|d| {
                        let s = d.to_string();
                        s.split(['>', '<', '='])
                            .next()
                            .unwrap_or(&s)
                            .trim()
                            .to_string()
                    })
                    .collect();

                packages.push(PackageInfo {
                    name,
                    description: pkg.desc().unwrap_or("").to_string(),
                    version: pkg.version().to_string(),
                    installed_version,
                    installed,
                    repository: repository.clone(),
                    groups,
                    depends_on: depends,
                    optdepends,
                    licenses,
                    homepage: pkg.url().map(|u| u.to_string()),
                    base: pkg.base().map(|b| b.to_string()),
                    packager: pkg.packager().map(|p| p.to_string()),
                    arch: pkg.arch().map(|a| a.to_string()),
                    build_date: Some(pkg.build_date()),
                    install_date: pkg.install_date(),
                    download_size: Some(pkg.size()),
                    installed_size: Some(pkg.isize()),
                    filename: pkg.filename().map(|f| f.to_string()),
                    conflicts,
                    provides,
                    replaces,
                });
            }
        }

        Ok(packages)
    }

    pub fn build_tools(&self) -> Result<Vec<Tool>> {
        let packages = self.all_packages()?;
        let installed_map: HashMap<String, String> = packages
            .iter()
            .filter(|p| p.installed)
            .map(|p| {
                (
                    p.name.clone(),
                    p.installed_version.clone().unwrap_or_default(),
                )
            })
            .collect();

        let classifier = Classifier::new(self.classification_db.clone());

        let tools: Vec<Tool> = packages
            .iter()
            .filter_map(|pkg| {
                let (categories, tags, detailed_desc) =
                    classifier.classify(&pkg.name, &pkg.description, &pkg.groups, &pkg.repository);

                let is_blackarch = pkg.repository == Repository::BlackArch;

                if !is_blackarch && categories.is_empty() {
                    return None;
                }

                let executables = crate::package::executable::discover_from_name(&pkg.name);

                Some(Tool {
                    id: pkg.name.clone(),
                    name: pkg.name.clone(),
                    short_description: pkg.description.clone(),
                    detailed_description: detailed_desc,
                    categories,
                    tags,
                    repository: pkg.repository.clone(),
                    available_version: pkg.version.clone(),
                    installed_version: installed_map.get(&pkg.name).cloned(),
                    installed: pkg.installed,
                    licenses: pkg.licenses.clone(),
                    homepage: pkg.homepage.clone(),
                    dependencies: pkg.depends_on.clone(),
                    optional_dependencies: pkg.optdepends.clone(),
                    groups: pkg.groups.clone(),
                    executables,
                    related: Vec::new(),
                    metadata_source: if pkg.repository == Repository::BlackArch {
                        crate::catalog::tool::MetadataSource::BlackArch
                    } else {
                        crate::catalog::tool::MetadataSource::ArchRepo
                    },
                    packager: pkg.packager.clone(),
                    arch: pkg.arch.clone(),
                    build_date: pkg.build_date,
                    install_date: pkg.install_date,
                    download_size: pkg.download_size,
                    installed_size: pkg.installed_size,
                    filename: pkg.filename.clone(),
                    conflicts: pkg.conflicts.clone(),
                    provides: pkg.provides.clone(),
                    replaces: pkg.replaces.clone(),
                    required_by: Vec::new(),
                })
            })
            .collect();

        Ok(tools)
    }
}
