const IS_PYTHON3_INSTALLED: &str = "command -v python3 > /dev/null 2>&1";
const IS_DOCKER_INSTALLED: &str = "command -v docker > /dev/null 2>&1";
const IS_K3S_INSTALLED: &str = "command -v k3s > /dev/null 2>&1";
const IS_K3S_RUNNING: &str = "systemctl is-active --quiet k3s";
const IS_K3S_AGENT_RUNNING: &str = "systemctl is-active --quiet k3s-agent";
const IS_REGISTRY_RUNNING: &str = "docker ps --filter name=registry --filter status=running | grep -q registry";
const IS_MKCERT_INSTALLED: &str = "command -v mkcert > /dev/null 2>&1";
const IS_HELM_INSTALLED: &str = "command -v helm > /dev/null 2>&1";