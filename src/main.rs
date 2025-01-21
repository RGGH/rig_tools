use dotenv::dotenv;
use rig::{completion::Prompt, providers::openai};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolSet},
};

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
//        .tool(Subtract)
        .build();

    // Prompt the agent and print the response
    println!("Calculate 2 + 5");
    println!(
        "Calculator Agent: {}",
        calculator_agent.prompt("Calculate 2 + 5").await?
    );

    Ok(())
}
