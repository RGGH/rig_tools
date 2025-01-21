use dotenv::dotenv;
use rig::{completion::Prompt, providers::openai};
use rig::{completion::ToolDefinition, tool::Tool};

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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok(); // Load .env file into the environment
    let openai_client = openai::Client::from_env();

    // Create agent with a single context prompt and two tools
    let calculator_agent = openai_client
        .agent(openai::GPT_4O)
        .preamble("You are a calculator here to help the user perform arithmetic operations. Use the tools provided to answer the user's question.")
        .max_tokens(1024)
        .tool(Adder)
        // .tool(Subtract)
        .build();

    // Prompt the agent and print the response
    println!("Calculate 2 + 5");
    println!(
        "Calculator Agent: {}",
        calculator_agent.prompt("Calculate 2 + 5").await?
    );

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
