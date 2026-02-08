pub const CORE_SCHEMA: &str = include_str!("embedded/core_schema.json");
pub const BASE_CONFIG_TEMPLATE: &str = include_str!("embedded/templates/init/initial_config.json");

// Templates
pub const BIND9_DB: &str = include_str!("embedded/templates/base_build/bind9.db.j2");
pub const NAMED_CONF_LOCAL: &str = include_str!("embedded/templates/base_build/named.conf.local.j2");
pub const NAMED_CONF_OPTIONS: &str = include_str!("embedded/templates/base_build/named.conf.options.j2");
pub const REGISTRIES: &str = include_str!("embedded/templates/base_build/registries.yaml.j2");

// Playbooks
pub const BUILD_CONTROL_PLANE: &str = include_str!("embedded/playbooks/base_build/build_control_plane.yml");
pub const BUILD_NODES: &str = include_str!("embedded/playbooks/base_build/build_nodes.yml");
pub const POST_BUILD: &str = include_str!("embedded/playbooks/base_build/post_build.yml");
pub const VALIDATE_CLUSTER: &str = include_str!("embedded/playbooks/base_build/validate_cluster.yml");
pub const DESTROY_CONTROL_PLANE: &str = include_str!("embedded/playbooks/base_destroy/destroy_control_plane.yml");
pub const DESTROY_NODES: &str = include_str!("embedded/playbooks/base_destroy/destroy_nodes.yml");

pub fn get_playbook(name: &str) -> Option<&'static str> {
    match name {
        "build_control_plane" => Some(BUILD_CONTROL_PLANE),
        "build_nodes" => Some(BUILD_NODES),
        "post_build" => Some(POST_BUILD),
        "validate_cluster" => Some(VALIDATE_CLUSTER),
        "destroy_control_plane" => Some(DESTROY_CONTROL_PLANE),
        "destroy_nodes" => Some(DESTROY_NODES),
        _ => None,
    }
}

pub fn get_base_template(name: &str) -> Option<(&'static str, &'static str)> {
    match name {
        "bind9_db" => Some(("bind9.db.j2", BIND9_DB)),
        "conf_local" => Some(("named.conf.local.j2", NAMED_CONF_LOCAL)),
        "conf_options" => Some(("named.conf.options.j2", NAMED_CONF_OPTIONS)),
        "registries" => Some(("registries.yaml.j2", REGISTRIES)),
        _ => None,
    }
}