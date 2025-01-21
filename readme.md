# Calculator Agent with OpenAI and Rig

Q. What does it do?
A. It demonstrates how to create a simple calculator agent using OpenAI's GPT-4 model and the Rig library. The calculator agent can perform basic arithmetic operations, such as addition, using predefined *tools*. The aim is to understand to tools so I can progress to automate some cool sh*t related to smart contracts.

## Features
- A custom `Adder` tool to perform addition of two integers.
- Integration with OpenAI's GPT-4 via the Rig library.
- Use of `.env` file for configuration.

## Setup

### Prerequisites
- Rust
- OpenAI API Key (set via `.env`)

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/RGGH/rig_tools
   cd calculator-agent
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Create a `.env` file and add your OpenAI API key:
   ```env
   OPENAI_API_KEY=your_openai_api_key
   ```

4. Run the project:
   ```bash
   cargo run
   ```

## Code Overview

- **Adder Tool**: A simple tool that adds two numbers `x` and `y` passed as arguments.
- **OpenAI Client**: Connects to OpenAI's GPT-4, with the calculator agent prompting the model to calculate sums using the `Adder` tool.

### Example Output
```text
Calculate 2 + 5
Calculator Agent: 7
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
