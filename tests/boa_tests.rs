mod infra;

// Your tests go here!
success_tests! {
    {
        name: add1,
        file: "boa/add1.snek",
        input: "0",
        expected: "73",
    },
    {
        name: add,
        file: "boa/add.snek",
        input: "0",
        expected: "15",
    },
    {
        name: nested_arith,
        file: "boa/nested_arith.snek",
        input: "0",
        expected: "25",
    },
    {
        name: binding,
        file: "boa/binding.snek",
        input: "0",
        expected: "5",
    },
    {
        name: expr1,
        file: "boa/expr1.snek",
        input: "0",
        expected: "41",
    },
    {
        name: expr2,
        file: "boa/expr2.snek",
        input: "0",
        expected: "40",
    },
    {
        name: expr3,
        file: "boa/expr3.snek",
        input: "0",
        expected: "126",
    },
    {
        name: expr4,
        file: "boa/expr4.snek",
        input: "0",
        expected: "1",
    },
    {
        name: auto_expr_0,
        file: "boa/auto_expr_0.snek",
        input: "0",
        expected: "48",
    },
    {
        name: auto_expr_1,
        file: "boa/auto_expr_1.snek",
        input: "0",
        expected: "131",
    },
    {
        name: auto_expr_2,
        file: "boa/auto_expr_2.snek",
        input: "0",
        expected: "27805",
    },
    {
        name: auto_expr_3,
        file: "boa/auto_expr_3.snek",
        input: "0",
        expected: "102",
    },
    {
        name: auto_expr_4,
        file: "boa/auto_expr_4.snek",
        input: "0",
        expected: "-53",
    },
    {
        name: auto_expr_5,
        file: "boa/auto_expr_5.snek",
        input: "0",
        expected: "11580",
    },
    {
        name: auto_expr_6,
        file: "boa/auto_expr_6.snek",
        input: "0",
        expected: "858",
    },
    {
        name: auto_expr_7,
        file: "boa/auto_expr_7.snek",
        input: "0",
        expected: "780",
    },
    {
        name: auto_expr_8,
        file: "boa/auto_expr_8.snek",
        input: "0",
        expected: "-41580",
    },
    {
        name: auto_expr_9,
        file: "boa/auto_expr_9.snek",
        input: "0",
        expected: "-250",
    },
    {
        name: auto_let_0,
        file: "boa/auto_let_0.snek",
        input: "0",
        expected: "-245",
    },
    {
        name: auto_let_1,
        file: "boa/auto_let_1.snek",
        input: "0",
        expected: "-20054",
    },
    {
        name: auto_let_2,
        file: "boa/auto_let_2.snek",
        input: "0",
        expected: "-714",
    },
    {
        name: auto_let_3,
        file: "boa/auto_let_3.snek",
        input: "0",
        expected: "531",
    },
    {
        name: auto_let_4,
        file: "boa/auto_let_4.snek",
        input: "0",
        expected: "20",
    },
    {
        name: auto_let_5,
        file: "boa/auto_let_5.snek",
        input: "0",
        expected: "184485",
    },
    {
        name: auto_let_6,
        file: "boa/auto_let_6.snek",
        input: "0",
        expected: "1936",
    },
    {
        name: auto_let_7,
        file: "boa/auto_let_7.snek",
        input: "0",
        expected: "4",
    },
    {
        name: auto_let_8,
        file: "boa/auto_let_8.snek",
        input: "0",
        expected: "-3380",
    },
    {
        name: auto_let_9,
        file: "boa/auto_let_9.snek",
        input: "0",
        expected: "-8905",
    },
}