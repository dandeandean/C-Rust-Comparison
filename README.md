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


## Question 1: How does Rust prevent common memory errors, like buffer overflow and use-after-free? 


### Ownership System: 
Rust employs the concept of ownership, where each value in memory has a single "owner" that is responsible for deallocating the memory when the value goes out of scope. This prevents multiple pointers from accessing or modifying the same memory concurrently, thus mitigating the risk of data races and concurrent modification issues. 

In our rust code:  

pub struct Maze { 

    pub map: [[i32; COLS]; ROWS], 

    pub start: (usize, usize), 

    pub fin: (usize, usize), 

} 

In the Maze struct, the ownership of the map field is clearly defined. The map owns the 2D array of integers representing the maze. This ownership ensures that the memory associated with the maze data is deallocated when the Maze struct goes out of scope. 

### Borrowing and References: 
Borrowing and References: Instead of relying solely on ownership, Rust introduces the concept of borrowing, allowing multiple immutable references or one mutable reference to a value. These references are checked at a program’s compile time to ensure that they don’t outlive the data they refer to or conflict with other references [1]. This prevents both dangling pointers and data races. 

From out rust code:  

fn get_walkable_neighbors(&self, point: VisitedCell) -> Vec<VisitedCell> { 

    // Function taking immutable reference to self (borrowing) 

    // and returning a vector of VisitedCell references 

} 

The get_walkable_neighbors function takes an immutable reference to self, allowing it to borrow the Maze instance. This ensures that get_walkable_neighbors cannot modify the Maze instance, preventing concurrent modifications. Additionally, the function returns a vector of VisitedCell references, which borrows data from the Maze instance rather than taking ownership. 

### Lifetime Annotations:  
Rust's borrow checker analyzes the lifetimes of references to determine their validity. Developers can use lifetime annotations to explicitly specify the relationships between references, helping the compiler verify that references do not outlive the data they point to or reference data after it has been deallocated. 

### No Null Pointers:  
Rust does not have null pointers or null references by default. Instead, it uses an optional type called Option<T> to represent optional values. This eliminates the possibility of null pointer dereferencing errors, as the compiler ensures that values are properly initialized before use. 

In our code, we can see rust avoids null pointers by using the Option type. In the load_grid function, the file is of type Option<File>. Instead of checking for null pointers, Rust uses the Option enum to represent the presence or absence of a value. Here, file.is_none() checks if the file variable contains a valid file handle. 

### Bounds Checking: 
Rust performs bounds checking on array accesses at runtime to prevent buffer overflow errors. This ensures that accesses to array elements are within the bounds of the array, preventing memory corruption due to invalid memory accesses. 

Rust performs bounds checking to prevent buffer overflow errors. In the get_walkable_neighbors function, nx is checked against the bounds of the maze rows (ROWS) to ensure that array accesses stay within valid bounds. This prevents accessing memory outside the allocated array.  

For example: 

if nx < 0 || nx >= ROWS.try_into().unwrap() { 

    continue; 

} 

### Unsafe Rust: 
While Rust emphasizes memory safety, it also provides an unsafe keyword for opting into low-level operations that bypass the safety checks of the compiler. However, the use of unsafe blocks is confined to specific portions of code where safety invariants can be manually upheld by the developer, and such code is subject to rigorous review and testing.  

Unsafe Rust allows bypassing some of the safety checks of the compiler. In this example, unsafe is used to push a value into the been_to vector without performing bounds checking or lifetime validation. However, the use of unsafe should be minimized and contained within well-audited blocks. 

Example: 

unsafe { 

    been_to.push(child.coord); 

} 


## Question 2: What are the remaining security issues in rust programs? 


### Concurrency and Data Races 
In Rust: let mut been_to: Vec<(usize, usize)> = Vec::new(); 

Although Rust's ownership system prevents data races at compile time, concurrent access to mutable data can still lead to logic errors. In this example, been_to is a mutable vector that could be accessed concurrently by multiple threads. To mitigate this, we should use Rust's concurrency primitives like Mutex or Arc to enforce thread safety [2]. 


### Unsafe Code:  
Rust allows bypassing safety checks using the unsafe keyword. While necessary for interfacing with low-level system APIs or optimizing performance-critical code, misuse of unsafe can introduce vulnerabilities like buffer overflows or null pointer dereferences. It's crucial to use unsafe judiciously and carefully audit such code.  

unsafe { 

    been_to.push(child.coord); 

} 


### Fuzzing and Input Validation: 
For example: let file = fs::read_to_string(file_name).unwrap(); 

Reading input from external sources like files or network streams without proper validation can lead to security vulnerabilities like buffer overflows or injection attacks. In this example, read_to_string assumes that the file exists and can be read, potentially leading to unexpected behavior or vulnerabilities. Input should be thoroughly validated and sanitized to prevent exploits. 

### Memory Safety Bugs in External Dependencies: 
For example: let lines = file.lines(); 

Rust programs often rely on external dependencies, which may contain memory safety bugs. While Rust's safety guarantees apply to safe Rust code, unsafe code within dependencies could introduce vulnerabilities. It's essential to vet dependencies for security issues and keep them updated to minimize risks. 

### Memory Leaks and Resource Exhaustion: 
For example: return Some(child); 

Improper resource management can lead to memory leaks or resource exhaustion. While Rust's ownership system mitigates many memory-related issues, incorrect handling of resources like file handles or network connections could still occur. Proper resource management, such as using RAII patterns or Rust's Drop trait, is essential to prevent these vulnerabilities. [3] 


## Question 3: What are the main challenges of programming in rust? 


### Ownership and Borrowing 
In Rust: let mut been_to: Vec<(usize, usize)> = Vec::new(); 

Compared to C: std::vector<Coord *> been_to; 

In Rust, managing ownership and borrowing can be challenging, especially when dealing with complex data structures like vectors or structs. Rust's ownership system ensures memory safety by enforcing strict rules on ownership transfer and borrowing, which may require careful consideration and restructuring of code compared to the more relaxed memory management in C. 


### Concurrency and Mutable State 
In Rust: let mut queue: VecDeque<VisitedCell> = VecDeque::new(); 

In C: std::deque<Coord *> q; 

Rust encourages safe concurrency through its ownership and borrowing system, which helps prevent data races and mutable state issues at compile time. However, managing concurrent data structures and ensuring thread safety can be challenging compared to C, where developers have more flexibility but must manually synchronize access to shared data. 

### Safety vs. Performance Trade-offs 
In Rust: unsafe { 

    been_to.push(child.coord); 

} 

In C: been_to.push_back(child); 

Rust's safety guarantees come with a performance cost, particularly when using unsafe code blocks to bypass safety checks. While Rust aims to provide safe abstractions without sacrificing performance, developers may need to carefully balance safety and performance considerations, especially in performance-critical sections of code, compared to C, where performance optimizations are more manual but potentially more straightforward.

### Learning Curve: 
Rust introduces new concepts like ownership, borrowing, lifetimes, and pattern matching, which may require developers to invest time in understanding and mastering these concepts compared to the more straightforward syntax and semantics of C. While Rust's safety features provide benefits in terms of reliability and maintainability, they also require a learning curve for developers transitioning from languages like C. 

For Example:
In Rust: let mut fin: (usize, usize) = (0, 0); 
In C: int fin[2] = {0, 0}; [4]


##Question 4: How can we make rust easy to use?


### Improved Documentation and Learning Resources 
Providing clear and comprehensive documentation is essential for reducing the learning curve associated with Rust. Alongside documentation, tutorials, guides, and examples play a crucial role in helping developers understand the language's concepts and best practices. Detailed explanations, code samples, and real-world use cases contribute significantly to enhancing understanding and facilitating usage, especially for complex concepts that may be challenging to grasp initially. 

### Ergonomic Syntax and Abstractions 
Rust's syntax prioritizes clarity and expressiveness, aiming to enhance readability and ease of use compared to more verbose languages. The language offers ergonomic abstractions and data structures that reduce boilerplate code and simplify common tasks. These abstractions not only make code more concise but also improve developer productivity by streamlining the development process. 

### Tooling and IDE Support 
Rust's build system, Cargo, along with tools like Rust Analyzer and Rustfmt, provides a seamless development experience for programmers. Robust tooling and IDE support, including features such as code completion, linting, and refactoring, significantly enhance developer productivity and ease of use. These tools not only aid in writing high-quality code but also assist developers in navigating and understanding Rust's features and libraries.

### Community Engagement and Support 
Rust boasts an active and welcoming community that fosters support and engagement among developers of all skill levels. Forums, chat rooms, and community events provide valuable platforms for developers to seek help, share knowledge, and collaborate on projects. The supportive nature of the Rust community encourages learning and exploration, making Rust more accessible and inclusive to a wide range of developers.

### Incremental Learning and Adoption 
Encouraging incremental learning and adoption of Rust is essential for developers to grasp the language's concepts gradually. Providing small, manageable projects and tutorials allows developers to build confidence and familiarity with Rust over time. Starting with simple examples and gradually introducing more advanced features helps developers gain practical experience and solidify their understanding of Rust's capabilities. [5] 


## Question 5: What is the main idea of rust-lancet? Do you think it is useful for rust developers and why? 


### Main Idea Overview: 
Rust-lancet is a tool that focusses on ownership-rule-violations, as the way it works it takes a program with safety-rule violations as an input and proceeds to either produce a modified program that does not contain any violations and preservers the semantics of the original program, or it will report that it cannot successfully patch the program following several attempts [6]. 

### Usefulness Discussion 
In essence, the idea of Rust-lancet is to automate the process of fixing the safety violations that exist within a programmer's code, as the safety rules are as strict as they are hefty. This tool can indeed be useful, so long as it is not solely relied upon, as to put blind faith in this tool and expect a given program to truly be both violation free and fully functional as intended by the programmer 100% of the time would be absurd. 

### Additional Consideration 
Another point to consider is that the tool is used to assist the optimization not only the work, but also the workflow for the programmer, so should the tool continuously fail after constant tweaking, it may have been more efficient for the programmer to have manually fixed the safety-rule violations than not in the essence of time and peace of mind. 

### Closing Thoughts 
In summation, as with most tools that are meant to automate tasks, they are best used and tweaked within the means of their use cases, and even regardless of the use case, they should always be monitored and checked for correctness to ensure optimal results.
  
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

In the process of rewriting the C++ program to Rust, developers likely encountered and addressed memory-related issues such as buffer overflows and dangling pointers. Rust's ownership system ensures memory safety by enforcing strict rules at compile-time, preventing common memory errors that are prevalent in languages like C or C++ [7]. This enhanced memory safety reduces the risk of security vulnerabilities, making Rust a favorable choice for future development projects, especially in security-critical domains.

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

Rust’s ability to interoperate with C libraries is a significant advantage when working with existing codebases or libraries. Rust’s foreign function interface (FFI) allows it to call C functions directly from Rust code [8]. This means you can leverage existing C libraries without having to rewrite them in Rust. Additionally, this feature allows for a gradual migration of a C or C++ codebase to Rust, which can be beneficial in large, complex projects.

#### Performance:
Rust is designed to provide control over low-level details such as memory layout, like C and C++. This means that Rust programs can be as fast as their C or C++ counterparts. Rust also allows fine-grained control over memory management, which can lead to more efficient programs. Rust achieves this through zero-cost abstractions, where higher-level constructs compile into as efficient a form as if they were written in lower-level code [9]. For example, Rust’s ownership model allows for automatic memory management without the need for a garbage collector, which can lead to performance improvements in certain scenarios.
## Reasons Not to Use Rust

### Speed of Development 
There is only one main reason against the use of Rust in development. That is the speed of development due to the Borrow Checker refusing to compile the code. This was a large factor in the speed of development on this project. It is worth mentioning that the speed of development will definitely increase as the programmer becomes more skilled in Rust. 

## Conclusion
Our analysis on the Rust code from our project provided keen insights on secure code practices, developers can gain insights into how Rust's features and ecosystem align with the requirements of future development projects. The practical experience of working with Rust's memory safety features and speed reinforces the decision to use Rust in security-sensitive domains. Developers can confidently adopt Rust knowing that it offers practical solutions to common challenges in software development, especially in terms of security and reliability.

The experience from rewriting the program with Rust, its memory safety, productivity gains, and compiler suggestions makes Rust's potential for future projects increase. Rust's focus on safety, performance, and concurrency is ideal for developing secure software, especially in security-sensitive fields. Adopting Rust can reduce memory-related risks and ensure high-standard, secure, and reliable software.


## References

[1] S. Zhu, Z. Zhang, B. Qin, A. Xiong, and L. Song, “Learning and programming challenges of rust,” Proceedings of the 44th International Conference on Software Engineering, May 2022, doi: https://doi.org/10.1145/3510003.3510164. 


[2] S. Mergendahl, N. Burow, and Hamed Okhravi, “Cross-Language Attacks,” Jan. 2022, doi: https://doi.org/10.14722/ndss.2022.24078. 


‌[3] Bugden, William, and Ayman Alahmar. "Rust: The programming language for safety and performance." arXiv preprint arXiv:2206.05503 (2022). 


[4] Zhang, Yuchen, Yunhang Zhang, Georgios Portokalidis, and Jun Xu. "Towards understanding the runtime performance of rust." In Proceedings of the 37th IEEE/ACM International Conference on Automated Software Engineering, pp. 1-6.[1] 2022. 


[5] Fulton, Kelsey R., Anna Chan, Daniel Votipka, Michael Hicks, and Michelle L. Mazurek. "Benefits and drawbacks of adopting a secure programming language: Rust as a case study." In Seventeenth Symposium on Usable Privacy and Security (SOUPS 2021), pp. 597-616. 2021. 


[6] W. Yang, L. Song, and Y. Xue, “Rust-lancet: Automated Ownership-Rule-Violation Fixing with Behavior Preservation,” Proceedings of the IEEE/ACM 46th International Conference on Software Engineering, Apr. 2024, doi: https://doi.org/10.1145/3597503.3639103. 

[7].  Xu, H., Chen, Z., Sun, M., Zhou, Y., & Lyu, M. R. (2021). Memory-safety challenge considered solved? An in-depth study with all Rust CVEs. ACM Transactions on Software Engineering and Methodology (TOSEM), 31(1), 1-25.

[8].  Maiga, A., Artho, C., Gilcher, F., & Moy, Y. (2023, October). Does Rust SPARK Joy? Safe Bindings from Rust to SPARK, Applied to the BBQueue Library. In _Proceedings of the 9th ACM SIGPLAN International Workshop on Formal Techniques for Safety-Critical Systems_ (pp. 37-47).

[9].  Holk, E., Pathirage, M., Chauhan, A., Lumsdaine, A., & Matsakis, N. D. (2013, May). GPU programming in rust: Implementing high-level abstractions in a systems-level language. In 2013 IEEE International Symposium on Parallel & Distributed Processing, Workshops and Phd Forum (pp. 315-324). IEEE.
