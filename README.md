# Large-Project
A Large Project comparing the memory safety of c++ to that of Rust 

# Project Guide Lines
## Rust programming
# Part A
### Read the following papers published on top-tier security conferences
* [Learning and Programming Challenges of Rust: A Mixed-Methods Study](https://songlh.github.io/paper/survey.pdf)
* [Cross-Language Attacks](https://www.ndss-symposium.org/wp-content/uploads/2022-78-paper.pdf)
* [Rust-lancet: Automated Ownership-Rule-Violation Fixing with Behavior Preservation] (to be announced)

### based on these papers, answer the following questions

* how does Rust prevent common memory errors, like buffer overflow and use-after-free?
* what is the remaining security issues in rust programs?
* what is the main challenges of programming in rust?
* how can we make rust easy to use?
* what is the main idea of rust-lancet? do you think it is useful for rust developers and why?
# Part B
### rewrite a vulnerable program with Rust
* write a buggy program with C or C++
* construct an input that will trigger the crash in the C program
* rewrite the program with rust, and show that the program will not crash under the same input
# c. based on the experience in b, would you like to use rust in your future development? why?
