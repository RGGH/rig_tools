use dotenv::dotenv;
use rig::{completion::Prompt, providers::openai};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(serde::Deserialize)]
struct AddArgs {
    x: i32,
    y: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
struct MathError;

#[derive(serde::Deserialize, serde::Serialize)]
struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "add";

    type Error = MathError;
    type Args = AddArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to add"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to add"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x + args.y;
        Ok(result)
    }
}


#[derive(Deserialize,Debug)]
struct IpToolArgs {
    ip: String,
    subnet: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum IpToolError {
    #[error("Invalid IP address")]
    InvalidIp,
    #[error("Invalid subnet format")]
    InvalidSubnet,
}

#[derive(Serialize, Deserialize)]
struct IpTool;

impl Tool for IpTool {
    const NAME: &'static str = "ip_tool";

    type Error = IpToolError;
    type Args = IpToolArgs;
    type Output = serde_json::Value;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "ip_tool".to_string(),
            description: "Performs IP address-related tasks like validation, subnetting, etc.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "ip": {
                        "type": "string",
                        "description": "The IP address to validate or analyze"
                    },
                    "subnet": {
                        "type": "string",
                        "description": "Optional subnet in CIDR format for additional checks"
                    }
                },
                "required": ["ip"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("IpTool was called with args: {:?}", args);
        // Parse the IP address
        let ip: IpAddr = args.ip.parse().map_err(|_| IpToolError::InvalidIp)?;

        // Optional subnet validation
        if let Some(subnet) = args.subnet {
            if !subnet.contains('/') {
                return Err(IpToolError::InvalidSubnet);
            }
            // Example: Check if IP belongs to subnet
            if ip_in_subnet(&ip, &subnet) {
                return Ok(serde_json::json!({
                    "ip": ip.to_string(),
                    "valid": true,
                    "in_subnet": true
                }));
            } else {
                return Ok(serde_json::json!({
                    "ip": ip.to_string(),
                    "valid": true,
                    "in_subnet": false
                }));
            }
        }

        // If no subnet is provided, just validate the IP
        Ok(serde_json::json!({
            "ip": ip.to_string(),
            "valid": true
        }))
    }
}

fn ip_in_subnet(ip: &IpAddr, subnet: &str) -> bool {
    use ipnetwork::IpNetwork;

    if let Ok(network) = subnet.parse::<IpNetwork>() {
        network.contains(*ip)
    } else {
        false
    }
}



#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok(); // Load .env file into the environment
    let openai_client = openai::Client::from_env();

    // Create agent with a single context prompt and two tools
    let calculator_agent = openai_client
    .agent(openai::GPT_4O)
    .preamble("You are a network engineer to help the user do IP networks. Always use the tools provided to validate IPs and subnets.")
    .max_tokens(1024)
    .tool(Adder)
    .tool(IpTool)
    .build();


    // Prompt the agent and print the response
    println!("is 192.168.1.1 in 192.168.0.0/25?");
    let response = calculator_agent
    .prompt("is 192.168.1.1 in 192.168.0.0/25?")
    .await?;

    println!("");
    println!("\t{}", response);


    Ok(())
}

#[tokio::test]
async fn test_adder_tool() {
    // Set up the Adder tool
    let adder = Adder;

    // Define test cases
    let test_cases = vec![
        (AddArgs { x: 2, y: 5 }, 7),
        (AddArgs { x: -2, y: -3 }, -5),
        (AddArgs { x: 0, y: 0 }, 0),
    ];

    // Run the test cases
    for (args, expected) in test_cases {
        let result = adder.call(args).await;
        assert_eq!(result.unwrap(), expected);
    }
}

#[tokio::test]
async fn test_tool_definition() {
    let adder = Adder;

    // Check the tool definition to ensure it's correct
    let definition = adder.definition("What is 2 + 5?".to_string()).await;
    
    assert_eq!(definition.name, "add");
    assert_eq!(definition.description, "Add x and y together");
    assert!(definition.parameters.is_object());
}
