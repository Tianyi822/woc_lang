// This is the AST (Abstract Syntax Tree) version 2. It is a simplified version compared to AST version 1.
// The AST version 1 uses dynamic dispatch to handle different types of nodes, which leads to increased complexity and slower performance due to runtime type checking.
// In contrast, the new AST version 2 utilizes enums to represent different types of nodes, eliminating the need for frequent runtime type checks and thus enhancing efficiency.
// The new AST version 2 also provides a more straightforward and concise implementation, making it easier to understand and maintain.
pub enum Node {}