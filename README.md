# Individual Project 2: Rust CLS Binary with SQLite

## Overview
This project aims to develop a command line tool in Rust that enables users to perform ETL (Extract, Transform, Load) and CRUD (Create, Read, Update, Delete) operations on an SQLite database. The tool facilitates interactions with the database, allowing users to query, insert, update, and delete records. Additionally, the project leverages GitHub Actions to generate a downloadable Rust binary, making it easy to distribute the code system. Furthermore, this project harnesses the power of GitHub Copilot, an AI programming assistant, to assist in translating code from Python to Rust. This helps expedite the translation process and reduces the likelihood of errors.

- Data: The project includes input data files stored in the "Data" directory.
  - cereal.db: A database file containing information on 80 cereals, such as their calorie content, protein, fat, sugar, vitamin levels, and ratings.

## Functionality
The code does: ETL-Query: [E] Extract a dataset from URL, [T] Transform, [L] Load into SQLite Database and [Q] Query For the ETL-Query lab:
- [E] Extract a dataset from a URL with CSV format.
- [T] Transform the data by cleaning, filtering, enriching, etc to get it ready for analysis.
- [L] Load the transformed data into a SQLite database table using Python's sqlite3 module with create and update operations.
- [Q] Accept and execute general SQL queries including in CRUD (Create, Read, Update, Delete) operations on the SQLite database to analyze and retrieve insights from the data.

## Project Steps
1. Clone the repository or run the project in GitHub Codespaces.
2. Create a `Cargo.toml` file and execute `Cargo build` to install project dependencies.
3. Run the program to interact with the SQLite database, allowing you to query the data.
4. Insert new data, update existing records, or remove records from the database as necessary.
5. Build an optimized release binary in Rust, and download it for easy distribution.

## CRUD Operation Results
![image](https://github.com/nogibjj/IDS706_Individual2_PJT/assets/141780408/6bf6cdc9-541d-4c87-9649-29949adebae8)

## Optimized Rust Binary
![image](https://github.com/nogibjj/IDS706_Individual2_PJT/assets/141780408/a18ce787-2d4b-452f-ac7a-8452287ce9f5)

## Use of Github Copilot
Throughout the project's evolution, GitHub Copilot proved indispensable, offering essential code recommendations, especially during the conversion from the original Python codebase to Rust. Furthermore, Copilot assisted in crafting entries for the Makefile, simplifying the build and testing processes, and enhancing my understanding of optimizing Rust binary generation.

## Visualization of Process
![process (3)](https://github.com/nogibjj/Fall2023_IDS706_MiniProject5_JiayiZhou/assets/143651921/f0480b87-bc09-49f4-9d9a-4f483343284c)

## Demo Video
