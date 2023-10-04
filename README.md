# Kvstore

Short Rust tutorial based on Ryan's YT video on Introduction to Rust

- Implements a key value database in Rust

## Project Goal

- Build out a composable database from open source component systems.

## Typical data system architecture
- ref: Voltron Data

- User Interface 
  - Users interact with this in order to initiate operations on data. This is typically exposed as a language frontend or API.
- Execution engine
  - The engine performs operations on the data as specified by users.
- Data storage
  - This is the layer that stores the data that is available to users.
- Intermediate representation
  - A standard way to represent query plans.
- Connectivity
  - A standard for accessing databases.
- Data memory layout
  - A standard format for representing data in memory.

- Standards ease interoperability struggles.
  - Data interoperability.
  - Query interoperability.
  - System interoperability.
- A healthy data system has an evolutionary architecture that supports constant change across every layer, ui, engines and data storage.

> Many are also realizing they don’t have the “big data” that warrants distributed compute to begin with. I believe these factors are contributing to the emergence of a new, unbundled OLAP architecture.
 In the unbundled OLAP architecture, data is stored directly in object storage like S3 or GCS. Indexing is handled by open-source formats like Hudi and Iceberg,
 which then structure and provide transactional guarantees over the data to be queried by a distributed query engine like Trino, or in-process with DuckDB.
 This allows for the right storage, indexing, and querying technologies to be applied to each use case on the basis of cost, performance, and operating requirements.

### DBMS Architecture

- Storage system (stores actual data)
- Catalog (store metadata about what is in the storage system)
- Query Engine (query and retrieve requested data)
- Access Control and Authorization (users, groups, permissions)
- Resource Management (divide resources between users)
- Administration utilities (monitor resource usage, set policies, etc)
- Clients for Network connectivity (implement JDBC, ODBC)
- Multi-node coordination and management.

### Features of a typical DBMS
- ref: Andrew Lamb
 
- In-memory storage
- Data model/Type system
- Metadata catalog + management
- In-memory filter + aggregation.
- Concurrency control.
- Query Language parser.
- Client API
- Arithmetic expressions.
- Heuristic Query Planner
- Durability/Persistence
- Optimized/Compressed storage.
- Storage rearrangement.
- Date/Time expressions.
- Execution on compressed data.
- Joins
- Distributed query execution.
- Resource management.
- Window functions.
- Cost based optimizer.
- Outer Joins
- Out of core algorithms.
- Subquery support.
- Online recovery.
- Advanced analytics.
- Additional client languages.

- Let's see how many of this features, composable systems support.

