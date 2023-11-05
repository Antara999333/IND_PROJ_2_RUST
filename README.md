# Individual Project 2: Rust CLI Utility with SQLite Integration

## Project Overview
In this project, the objective is to create a Rust command-line tool that facilitates ETL (Extract, Transform, Load) and CRUD (Create, Read, Update, Delete) operations on an SQLite database. This tool simplifies interactions with the database, allowing users to execute queries, insert, update, and delete records. Additionally, the project utilizes GitHub Actions for generating a downloadable Rust binary, making code distribution easy. GitHub Copilot, an AI programming assistant, assists in translating code from Python to Rust, streamlining the conversion process while reducing the likelihood of errors.

- Data: The project includes input data files stored in the "Data" directory.
  - cereal.db: A database file that contains information on 80 cereals, including details like calorie content, protein levels, fat content, sugar, vitamin levels, and user ratings.

## Functionality
The codebase provides ETL-Query functionalities: [E] Extract data from a URL, [T] Transform data, [L] Load data into an SQLite Database, and [Q] Query the data. In the context of ETL-Query:
- [E] Data extraction from a URL in CSV format.
- [T] Data transformation, including processes like cleaning, filtering, and enrichment to prepare it for analysis.
- [L] Loading of the transformed data into an SQLite database table, using Python's sqlite3 module, with capabilities for creating and updating records.
- [Q] Acceptance and execution of general SQL queries, including CRUD (Create, Read, Update, Delete) operations on the SQLite database, facilitating data analysis and insights retrieval.

## Project Steps
1. Start by cloning the repository or running the project in GitHub Codespaces.
2. Create a `Cargo.toml` file and execute `Cargo build` to install necessary project dependencies.
3. Run the program to interact with the SQLite database, enabling data queries.
4. Perform actions like inserting new data, updating existing records, or removing records from the database as needed.
5. Build an optimized release binary in Rust and download it for easy distribution.

## Results of CRUD Operations
![image](https://github.com/nogibjj/IDS706_Individual2_PJT/assets/141780408/6bf6cdc9-541d-4c87-9649-29949adebae8)

## Optimized Rust Binary
![image](https://github.com/nogibjj/IDS706_Individual2_PJT/assets/141780408/a18ce787-2d4b-452f-ac7a-8452287ce9f5)

## Utilizing GitHub Copilot
Throughout the project's development, GitHub Copilot proved invaluable by offering essential code recommendations, particularly during the conversion from the original Python codebase to Rust. Copilot also assisted in creating entries for the Makefile, simplifying build and testing processes, and enhancing my understanding of optimizing Rust binary generation.
