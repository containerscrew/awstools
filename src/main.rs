use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, Region};
use aws_config::retry::RetryConfig;
use aws_sdk_iam::Client;
use aws_sdk_iam::error::SdkError;
use aws_sdk_iam::operation::list_attached_role_policies::{ListAttachedRolePoliciesError, ListAttachedRolePoliciesOutput};
use crate::logger::setup_logger;


mod logger;

async fn list_attached_role_policies(
    client: Client,
    role_name: String,
    path_prefix: Option<String>,
    marker: Option<String>,
    max_items: Option<i32>,
) -> Result<ListAttachedRolePoliciesOutput, SdkError<ListAttachedRolePoliciesError>> {
    let response = client
        .list_attached_role_policies()
        .role_name(role_name)
        .set_path_prefix(path_prefix)
        .set_marker(marker)
        .set_max_items(max_items)
        .send()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable some logging
    setup_logger("info".to_string());

    // AWS config
    let region_provider = RegionProviderChain::first_try(Region::new("eu-west-1".to_string()))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));


    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .retry_config(RetryConfig::disabled())
        .load()
        .await;

    let client = Client::new(&shared_config);

    let policies : ListAttachedRolePoliciesOutput = list_attached_role_policies(client, "eks-admin-role".to_string(), None, None, None).await?;

    let pretty = serde_json::to_string_pretty(&policies.attached_policies.unwrap())?;

    println!("{}", pretty);

    Ok(())
}
