# IDS706 Python Script interacting with SQL Database

## Continuous Integration with GitHub Actions
[![Format](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/format.yml/badge.svg)](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/format.yml)
[![Lint](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/lint.yml/badge.svg)](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/lint.yml)
[![Build](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/build.yml/badge.svg)](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/build.yml)
[![Tests](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/test.yml/badge.svg)](https://github.com/Reby0217/ids706-indvidual2/actions/workflows/test.yml)



This project focuses on building a Rust CLI Binary that interacts with an SQLite database, demonstrating CRUD operations (Create, Read, Update, Delete). The binary is optimized and automatically built using GitHub Actions, and artifacts can be downloaded after the CI/CD pipeline completes successfully.

---

## Deliverables
- **Rust CLI Binary**: The main binary (`ids706-indvidual2`) performs CRUD operations on a database of the wealthiest individuals.
- **SQLite Database**: Includes an SQLite database (`wealth_db.db`) for demonstration. Below is a snapshot of the table after the CRUD operations have been executed, showing the remaining records:
![DB](screenshots/db.png)

- **Screenshot of successful database operations**: 
![Log](screenshots/log.png)

- **Use of LLM and Use of Gitlab Copilot**: For this project, we chose not to utilize an LLM (such as ChatGPT) or GitLab Copilot during the development process. After consulting with the TA, we confirmed that the use of these tools is entirely optional, and not using them will not result in any loss of points. We decided to approach the project by learning and implementing Rust independently, relying on personal research and development.

![Chat](screenshots/chat.png)

- **Optimized Rust Binary**:
The project includes an optimized Rust binary (`ids706-indvidual2`) built using the `cargo build --release` command. This binary is designed for efficient performance and demonstrates CRUD (Create, Read, Update, Delete) operations on an SQLite database. The binary is built and packaged automatically through GitHub Actions as an artifact that can be downloaded directly from the repository. 

![Artifact](screenshots/artifact.png)

---

## Project Structure
```bash
.
├── src
│   ├── lib.rs                         # Contains the core logic for CRUD operations on the SQLite database
│   ├── main.rs                        # Entry point for the Rust CLI binary
├── tests
│   ├── test.rs                        # Unit tests for CRUD operations in Rust
├── Makefile                           # Commands for build, test, lint, and format
├── wealth_db.db                       # SQLite database used for the project
├── Cargo.toml                         # Project dependencies and metadata
└── .github/workflows                  # CI/CD workflows for GitHub Actions
```

## Getting Started

### Prerequisites

- **Rust and Cargo**: Both can be installed with the following command

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Verify Installation**: After installation, you can check that Rust and Cargo have been successfully installed with the following commands

  ```bash
  rustc --version   # Rust compiler version
  cargo --version   # Cargo package manager version
  ```

### Installation

1. **Clone the repository**:

   First, clone the repository to your local machine and navigate into the project directory:

   ```bash
   git clone https://github.com/Reby0217/ids706-indvidual2.git
   cd ids706-indvidual2
   ```

2. **Install Rust and required toolchain components**:

   The `make install` command will install Rust and its required components (`rustfmt` and `clippy`) if they are not already installed:

   ```bash
   make install
   ```

3. **Format, lint, and build the binary**:

   Format and lint your code using `rustfmt` and `clippy`, then build the binary:

   - For development (debug mode):
     ```bash
     make format   # Format the Rust code
     make lint     # Lint the code for issues
     make build    # Build the binary in debug mode
     ```

   - For release (optimized binary):
     ```bash
     make release  # Build the optimized binary for release
     ```

4. **Run the binary**:

   After building the project, you can run the binary:

   - For development (debug mode):
     ```bash
     make run      # Build and run the binary in debug mode
     ```

   - For release:
     ```bash
     target/release/ids706-indvidual2
     ```

## Requirements

### SQLite Database

This project connects to a SQLite database (`wealth_db.db`) and performs various operations on it, including creating tables, inserting, updating, reading, and deleting records. The connection is established using the `rusqlite` crate.


### CRUD Operations

- **Create**: Inserts records for the wealthiest individuals into the database.
- **Read**: Retrieves records from the database, sorted by `net_worth` or filtered by `industry`.
- **Update**: Updates the net worth of specific individuals.
- **Delete**: Deletes specific records from the database.

1. **Create**:
   ```rust
   conn.execute(
       "INSERT INTO wealthiest_people (id, name, country, industry, net_worth, company) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
       params![person.0, person.1, person.2, person.3, person.4, person.5],
   )?;
   ```

2. **Read**:
   - **All Records**:
     ```sql
     SELECT * FROM wealthiest_people
     ```
   - **Sort by Net Worth**:
     ```sql
     SELECT * FROM wealthiest_people ORDER BY net_worth DESC
     ```
   - **Filter by Industry**:
     ```sql
     SELECT * FROM wealthiest_people WHERE industry = ?
     ```

3. **Update**:
   ```rust
   conn.execute(
       "UPDATE wealthiest_people SET net_worth = 180 WHERE name = 'Charlie'",
       params![],
   )?;
   ```

4. **Delete**:
   ```rust
   conn.execute("DELETE FROM wealthiest_people WHERE name = 'Bob'", params![])?;
   ```


---

## Makefile

The project uses a `Makefile` to streamline development tasks, including version checks, testing, formatting, linting, and building. Below are the key commands available in the `Makefile`:

- **Rust Version**: Display the versions of the Rust toolchain components (compiler, package manager, formatter, linter).
  ```bash
  make rust-version
  ```
  This will print the versions of `rustc`, `cargo`, `rustfmt`, `rustup`, and `clippy-driver`.

- **Format**: Format all Rust files using `rustfmt` to ensure consistent code style.
  ```bash
  make format
  ```

- **Lint**: Run `clippy` to analyze the Rust code for common mistakes and best practices.
  ```bash
  make lint
  ```

- **Test**: Run unit tests for the project using `cargo test`. It specifically targets tests located in the `tests/test.rs` file.
  ```bash
  make test
  ```

- **Build**: Compile the project in debug mode.
  ```bash
  make build
  ```

- **Run**: Build the project and then execute the compiled binary.
  ```bash
  make run
  ```

- **Release**: Build an optimized release version of the binary.
  ```bash
  make release
  ```

- **All**: Perform a full suite of tasks—format, lint, build, test, and run the project.
  ```bash
  make all
  ```