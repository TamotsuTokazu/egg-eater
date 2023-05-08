# Yizhao's Diamondback Compiler

- Local variables are stored between `[RSP]` and `[RBP]` (of course). They are indexed relatively to `RBP` with a negative offset. That is, a new variable will be at `[RBP - 8 * si]`, where `si >= 1`.
- `env` contains the mapping from the name of the variables to their memory locations. The value is the number of words relative to `RBP`. As a exception, `i32::MAX` represent the special value `input`.
- For functions defined in Diamondback, the calling convention is like **cdecl**:
    - All arguments are passed using the stack, and are pushed from right to left.
    - Caller cleans up the arguments on stack.
    - The stack is aligned to a 16-byte boundary when calling a function. That is, before a `call` instruction, `RSP` should be a multiple of `16`.
    - Registers `RAX`, `RCX`, and `RDX` are caller-saved, and the rest are callee-saved.
- For external functions (in Diamondback, there's only `snek_print`), it seems they use the **System V AMD64 ABI**, so the first argument is passed through `RDI`.