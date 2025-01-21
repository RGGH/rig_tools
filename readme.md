# Agent + tools with OpenAI and Rig (Rust 🦀) [![Rust](https://github.com/RGGH/rig_tools/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/rig_tools/actions/workflows/rust.yml)

![image](https://github.com/user-attachments/assets/24713bc3-5512-43eb-9eb7-b9922a4c67b1)


Q. What does it do?
A. It demonstrates how to create a simple IP network validator tool for an agent using OpenAI's GPT-4 model and the Rig library. 
The calculator agent can perform basic network operations using predefined *tools*. 
The aim is to understand tools so I can progress to automate some cool sh*t related to smart contracts.

## Features
- A custom `IPtool` tool to perform check on whether an ip address is in the supernet.
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

- **IpTool Tool**: A simple tool that checks ip address and supernet passed as arguments.
- **OpenAI Client**: Connects to OpenAI's GPT-4, with the IpTool agent prompting the model to calculate sums using the `ip_in_subnet` tool.

### Example Output
```text
is 192.168.1.0 255.255.255.0 in 192.168.0.0?
network Agent: {"in_subnet":true,"ip":"192.168.1.1","valid":true}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
