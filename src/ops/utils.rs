use openssh::{Session, SessionBuilder};
use super::types::BaseNodeConfig;

pub async fn create_ssh_session(node: &BaseNodeConfig) -> Session {
    let mut session_builder = SessionBuilder::default();
    session_builder.user(node.user.clone());
    session_builder.keyfile(node.ssh_key_path.clone());

    match session_builder.connect(node.host.clone()).await {
        Ok(session) => session,
        Err(e) =>  { 
          println!("Failed to connect to node at {}: {:?}", node.host, e);
          panic!("Failed to connect to node");
        },
    }
}