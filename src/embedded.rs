pub const CORE_SCHEMA: &str = include_str!("embedded/core_schema.json");
pub const BASE_CONFIG_TEMPLATE: &str = include_str!("embedded/templates/init/initial_config.json");

// Pack templates
pub const PACK_DOCKERFILE_TEMPLATE: &str = include_str!("embedded/templates/pack/Dockerfile");
pub const PACK_FRONTEND_VALUES_TEMPLATE: &str = include_str!("embedded/templates/pack/values.frontend.yaml");
pub const PACK_BACKEND_VALUES_TEMPLATE: &str = include_str!("embedded/templates/pack/values.backend.yaml");
pub const PACK_FULLSTACK_VALUES_TEMPLATE: &str = include_str!("embedded/templates/pack/values.fullstack.yaml");

// Base Build Templates
pub const BIND9_DB: &str = include_str!("embedded/templates/base_build/bind9.db.j2");
pub const NAMED_CONF_LOCAL: &str = include_str!("embedded/templates/base_build/named.conf.local.j2");
pub const NAMED_CONF_OPTIONS: &str = include_str!("embedded/templates/base_build/named.conf.options.j2");
pub const REGISTRIES: &str = include_str!("embedded/templates/base_build/registries.yaml.j2");

// Helm charts
pub const NYA_CHART: &str = include_str!("embedded/helm/Chart.yaml");
pub const NYA_DEPLOYMENT_TEMPLATE: &str = include_str!("embedded/helm/deployment.yaml");
pub const NYA_BACKEND_TEMPLATE: &str = include_str!("embedded/helm/backend.yaml");
pub const NYA_FRONTEND_TEMPLATE: &str = include_str!("embedded/helm/frontend.yaml");