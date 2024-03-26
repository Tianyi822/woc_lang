// Each state represents the stage to which the command has currently been parsed by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    // Lexers state
    StartState,
    EndState,

    // ========================= Data Types =========================
    IdentState,
    IntegerNumState,
    FloatNumState,

    // ========================= Single Symbols =========================
    CommaState,        // ,
    DotState,          // .
    SemiColonState,    // ;
    ColonState,        // :
    AssignmentState,   // =
    LeftParenState,    // (
    RightParenState,   // )
    LeftBraceState,    // {
    RightBraceState,   // }
    LeftBracketState,  // [
    RightBracketState, // ]
    QuoteState,        // "
    SingleQuoteState,  // '

    // ========================= Logical Symbols =========================
    NotState,                  // !
    GreaterState,              // >
    LessState,                 // <
    GreaterThanOrEqualToState, // >=
    LessThanOrEqualToState,    // <=
    EqualToState,              // ==
    NotEqualToState,           // !=
    AndState,                  // &&
    OrState,                   // ||

    // ========================= Bit Calculation =========================
    BitAndState, // &
    BitOrState,  // |
    BitNotState, // ~

    // ========================= Data calculate symbols =========================
    PlusState,        // +
    MinusState,       // -
    StarState,        // *
    SlashState,       // /
    PercentState,     // %
    PlusAssignState,  // +=
    MinusAssignState, // -=
    StarAssignState,  // *=
    SlashAssignState, // /=

    // ========================= Keywords =========================
    // ============ while ============
    WhileState1,
    WhileState2,
    WhileState3,
    WhileState4,
    WhileState,

    // ============ for ============
    ForState1,
    ForState2,
    ForState,

    // ============ if ============
    IfState1,
    IfState,

    // ============ else ============
    ElseState1,
    ElseState2,
    ElseState3,
    ElseState,

    // ============ break ============
    BreakState1,
    BreakState2,
    BreakState3,
    BreakState4,
    BreakState,

    // ============ continue ============
    ContinueState1,
    ContinueState2,
    ContinueState3,
    ContinueState4,
    ContinueState5,
    ContinueState6,
    ContinueState7,
    ContinueState,

    // ============ let ============
    LetState1,
    LetState2,
    LetState,

    // ============ function ============
    FuncState2,
    FuncState3,
    FuncState,

    // ============ return ============
    ReturnState1,
    ReturnState2,
    ReturnState3,
    ReturnState4,
    ReturnState5,
    ReturnState,

    // ============ struct ============
    StructState1,
    StructState2,
    StructState3,
    StructState4,
    StructState5,
    StructState,

    // ============ enum ============
    EnumState2,
    EnumState3,
    EnumState,

    // ============ None ============
    NoneState1,
    NoneState2,
    NoneState3,
    NoneState,

    // ============ True ============
    TrueState1,
    TrueState2,
    TrueState3,
    TrueState,

    // ============ False ============
    FalseState2,
    FalseState3,
    FalseState4,
    FalseState,

    // ========================= Others =========================
    UnderscoreState, // _
}
