# Large-Project
A Large Project comparing the memory safety of c++ to that of Rust 

# Project Guide Lines
Option 6 - Rust programming
# Part A
### Read the following papers published on top-tier security conferences
* [Learning and Programming Challenges of Rust: A Mixed-Methods Study](https://songlh.github.io/paper/survey.pdf)
* [Cross-Language Attacks](https://www.ndss-symposium.org/wp-content/uploads/2022-78-paper.pdf)
* [Rust-lancet: Automated Ownership-Rule-Violation Fixing with Behavior Preservation](https://songlh.github.io/paper/lancet.pdf)
### Based on these papers, answer the following questions

* how does Rust prevent common memory errors, like buffer overflow and use-after-free?
* what is the remaining security issues in rust programs?
* what is the main challenges of programming in rust?
* how can we make rust easy to use?
* what is the main idea of rust-lancet? do you think it is useful for rust developers and why?
  
# Part B

## What Does our Project do? 
Our program will be a maze solver. The input will be a text document that contains an 'S', 'F', blank spaces, and '#'. The (S)tart and (F)inish blocks will be the start and end of the BFS algorithm to solve the maze. The output will be text to stdout, which is the solved maze with the path taken. 


### Bewrite a vulnerable program with Rust
* write a buggy program with C or C++
* construct an input that will trigger the crash in the C program
* rewrite the program with rust, and show that the program will not crash under the same input

### Usage
  #### Rust
  Navigate to rust_src
  ```shell
    cargo r <path to maze>
  ```
  #### Cpp
  Navigate to cpp_src
  ```shell
    make && run <path to maze>
  ```

# Part C 
### Based on the experience in b, would you like to use rust in your future development? why?


## Reasons to Use Rust
### Memory Safety:
The Rust code showcases various features that ensure memory safety, such as the use of safe abstractions like `_VecDeque_` instead of raw pointers, and the absence of manual memory management. For instance, in the `_build_vc_from_parent_` function, Rust's ownership system ensures that memory is managed safely when creating a new `_VisitedCell_` struct, mitigating the risk of memory leaks or undefined behavior. This project demonstrates how Rust's memory safety features contribute to writing secure and reliable code.

In the process of rewriting the C++ program to Rust, developers likely encountered and addressed memory-related issues such as buffer overflows and dangling pointers. Rust's ownership system ensures memory safety by enforcing strict rules at compile-time, preventing common memory errors that are prevalent in languages like C or C++ [1]. This enhanced memory safety reduces the risk of security vulnerabilities, making Rust a favorable choice for future development projects, especially in security-critical domains.

``` Rust
fn build_vc_from_parent(coord: (usize, usize), parent: &VisitedCell) -> VisitedCell {
    /* HEAVY memory implications */
    let mut ancestors = parent.ancestors.clone();
    ancestors.push_front(parent.coord);
    VisitedCell { coord, ancestors }
}
```


This code from our project demonstrates how Rust's ownership system ensures memory safety by managing memory allocations and deallocations correctly. The use of _clone()_ for the _ancestors_ vector creates a deep copy, preventing unintended mutations to the original vector. This approach mitigates the risk of memory leaks or data corruption, showcasing Rust's memory safety guarantees in practice.

```Rust
pub struct Maze {
    pub map: [[i32; COLS]; ROWS],
    pub start: (usize, usize),
    pub fin: (usize, usize),
}
```

This code from _Maze.rs_ demonstrates how Rust's type system enables developers to design robust and maintainable data structures. By defining the _Maze_ struct with explicit field types and sizes, Rust ensures type safety and memory layout predictability, reducing the risk of runtime errors and optimizing performance. This practical example highlights how Rust's features support safe development considerations, such as code maintainability and scalability, making it a suitable choice for long-term software projects.

These functions were implemented differently in C++, as the raw pointer arithmetic made it more difficult to write a Maze struct. Furthermore, in C++ it was trivial to point to the parent Cells, but this was not trivial in Rust. We could have designed it using Box variable to allocated them on the stack and reference back to them, but we decided to go with the simpler method of cloning, which ensured memory safety at the cost of time and space complexity.
### Interoperability with C

Rust’s ability to interoperate with C libraries is a significant advantage when working with existing codebases or libraries. Rust’s foreign function interface (FFI) allows it to call C functions directly from Rust code [2]. This means you can leverage existing C libraries without having to rewrite them in Rust. Additionally, this feature allows for a gradual migration of a C or C++ codebase to Rust, which can be beneficial in large, complex projects.

#### Performance:
Rust is designed to provide control over low-level details such as memory layout, like C and C++. This means that Rust programs can be as fast as their C or C++ counterparts. Rust also allows fine-grained control over memory management, which can lead to more efficient programs. Rust achieves this through zero-cost abstractions, where higher-level constructs compile into as efficient a form as if they were written in lower-level code [3]. For example, Rust’s ownership model allows for automatic memory management without the need for a garbage collector, which can lead to performance improvements in certain scenarios.
## Reasons Not to Use Rust

### Speed of Development 
There is only one main reason against the use of Rust in development. That is the speed of development due to the Borrow Checker refusing to compile the code. This was a large factor in the speed of development on this project. It is worth mentioning that the speed of development will definitely increase as the programmer becomes more skilled in Rust. 

## Conclusion
Our analysis on the Rust code from our project provided keen insights on secure code practices, developers can gain insights into how Rust's features and ecosystem align with the requirements of future development projects. The practical experience of working with Rust's memory safety features and speed reinforces the decision to use Rust in security-sensitive domains. Developers can confidently adopt Rust knowing that it offers practical solutions to common challenges in software development, especially in terms of security and reliability.

The experience from rewriting the program with Rust, its memory safety, productivity gains, and compiler suggestions makes Rust's potential for future projects increase. Rust's focus on safety, performance, and concurrency is ideal for developing secure software, especially in security-sensitive fields. Adopting Rust can reduce memory-related risks and ensure high-standard, secure, and reliable software.



[1].  Xu, H., Chen, Z., Sun, M., Zhou, Y., & Lyu, M. R. (2021). Memory-safety challenge considered solved? An in-depth study with all Rust CVEs. ACM Transactions on Software Engineering and Methodology (TOSEM), 31(1), 1-25.

[2].  Maiga, A., Artho, C., Gilcher, F., & Moy, Y. (2023, October). Does Rust SPARK Joy? Safe Bindings from Rust to SPARK, Applied to the BBQueue Library. In _Proceedings of the 9th ACM SIGPLAN International Workshop on Formal Techniques for Safety-Critical Systems_ (pp. 37-47).

[3].  Holk, E., Pathirage, M., Chauhan, A., Lumsdaine, A., & Matsakis, N. D. (2013, May). GPU programming in rust: Implementing high-level abstractions in a systems-level language. In 2013 IEEE International Symposium on Parallel & Distributed Processing, Workshops and Phd Forum (pp. 315-324). IEEE.
