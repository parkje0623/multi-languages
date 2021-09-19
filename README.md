# Multi-Language Project
Multi-language Calculator project is a website with many different types of calculators created by using multiple languages.  
The Virtual Machine (Vagrant) is used to run code in multiple languages.  
Langauges include: HTML/CSS, Python, C, and Go  
Types of Calculators are:  
 - CGPA and Target GPA Calculator
 - Prime Number Calculator
 - Factorial Calculator
 - Fibonacci Calculator
More details explained in the following document: [Multi-language Calculator Document](Multi-language_Calculator/Calculator.md)  
  
# Haskell
Simple Haskell exercises completed before creating the Rainbow Table in Haskell.  
**Rainbow Table**
Simple Rainbow Table (insecure) created to get an idea of how pre-computed tables work.  
 - Password and key spaces are much larger in reality for security reason
Rainbow Table is a compromise between hashing every possible password until we find a match and storing every possible password/hash pair.
 - Instead of storing the whole table, only first and last columns are stored since entire table will take long time and require big file
 - Require hash function that system uses
 - Require reduce function that maps hash value to an arbitrary password

# Scala
Simple Scala exercises and samples completed before writing the report to summarize the features of Scala.  
The report is to learn how to approach to a new language by finidng and understanding the basic concepts of the language.  
The Scala Report includes following information:
 - Type System: Built-in Types, Objects, and Type System Features
 - Memrmoy Management: Garbage collection
 - Interesting Features: Immutability, Lazy Evaluation, Type Inference, and Pattern Matching
 - Comparison with other languages: Java and Haskell

# Rust
Simple Rust exercises completed before creating the Blockchain system in Rust.  
**Blockchain System**
Simple blockchain system is created using Rust language to understand the core idea (not a safe blockchain - bad cryptography).  
Each block is a data structure that contains some information and the data is hashed with cryptographic hash function.
 - If the hash function matches the contents, it will be defined as a valid hash
 - Proof-of-Work required to verify a sequence of a block to define a valide block
 - Proof-of-Work function is computed in parallel using the Work Queue

