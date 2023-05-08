mod infra;

// Your tests go here!
success_tests! {
    {
        name: fact,
        file: "fact.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: even_odd_1,
        file: "even_odd.snek",
        input: "10",
        expected: "10\ntrue\ntrue",
    },
    {
        name: even_odd_2,
        file: "even_odd.snek",
        input: "9",
        expected: "9\nfalse\nfalse",
    },
    {
        name: my1_1,
        file: "my1.snek",
        input: "10",
        expected: "55",
    },
    {
        name: my1_2,
        file: "my1.snek",
        input: "100",
        expected: "5050",
    },
    {
        name: my2_1,
        file: "my2.snek",
        input: "10",
        expected: "89",
    },
    {
        name: my2_2,
        file: "my2.snek",
        input: "40",
        expected: "165580141",
    },
    {
        name: my3,
        file: "my3.snek",
        input: "10",
        expected: "1\n10\n1\n9\n2\n8\n3\n7\n5\n6\n8\n5\n13\n4\n21\n3\n34\n2\n55\n1\n89\n0\n89\n",
    },
}

runtime_error_tests! {}

static_error_tests! {
    {
        name: duplicate_params,
        file: "duplicate_params.snek",
        expected: "",
    },
    {
        name: err1_duplicate_definition,
        file: "err1.snek",
        expected: "",
    },
    {
        name: err2_invalid_arg_name,
        file: "err2.snek",
        expected: "",
    },
    {
        name: err3_wrong_number_of_args,
        file: "err2.snek",
        expected: "",
    },
}
