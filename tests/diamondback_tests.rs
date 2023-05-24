mod infra;

// Your tests go here!
success_tests! {
    {
        name: fact,
        file: "diamondback/fact.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: even_odd_1,
        file: "diamondback/even_odd.snek",
        input: "10",
        expected: "10\ntrue\ntrue",
    },
    {
        name: even_odd_2,
        file: "diamondback/even_odd.snek",
        input: "9",
        expected: "9\nfalse\nfalse",
    },
    {
        name: my1_1,
        file: "diamondback/my1.snek",
        input: "10",
        expected: "55",
    },
    {
        name: my1_2,
        file: "diamondback/my1.snek",
        input: "100",
        expected: "5050",
    },
    {
        name: my2_1,
        file: "diamondback/my2.snek",
        input: "10",
        expected: "89",
    },
    {
        name: my2_2,
        file: "diamondback/my2.snek",
        input: "40",
        expected: "165580141",
    },
    {
        name: my3,
        file: "diamondback/my3.snek",
        input: "10",
        expected: "1\n10\n1\n9\n2\n8\n3\n7\n5\n6\n8\n5\n13\n4\n21\n3\n34\n2\n55\n1\n89\n0\n89\n",
    },
    {
        name: my4_normal,
        file: "diamondback/my4.snek",
        input: "2",
        expected: "16\n16",
    },
    {
        name: my5_normal,
        file: "diamondback/my5.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: my7,
        file: "diamondback/my7.snek",
        input: "10",
        expected: "3628800",
    },
}

runtime_error_tests! {
    {
        name: my4_overflow,
        file: "diamondback/my4.snek",
        input: "2305843009213693952",
        expected: "overflow",
    },
    {
        name: my4_argument,
        file: "diamondback/my4.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: my5_overflow,
        file: "diamondback/my5.snek",
        input: "100",
        expected: "overflow",
    },
    {
        name: my5_argument,
        file: "diamondback/my5.snek",
        input: "true",
        expected: "invalid argument",
    },
}

static_error_tests! {
    {
        name: duplicate_params,
        file: "diamondback/duplicate_params.snek",
        expected: "",
    },
    {
        name: err1_duplicate_definition,
        file: "diamondback/err1.snek",
        expected: "",
    },
    {
        name: err2_invalid_arg_name,
        file: "diamondback/err2.snek",
        expected: "",
    },
    {
        name: err3_wrong_number_of_args,
        file: "diamondback/err3.snek",
        expected: "",
    },
    {
        name: err4_input_in_fun,
        file: "diamondback/err4.snek",
        expected: "",
    },
    {
        name: err5_function_undefined,
        file: "diamondback/err5.snek",
        expected: "",
    },
}
