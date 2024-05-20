import biodivine_boolean_functions as bbf
from pyeda.inter import *
from utils import run_bench

"""
This benchmark compares the performance of the logic table representations
using a repeated substitution while increasing the variable count of the
formula.

The operation starts with a formula 'x_0' and it iteratively
substitutes 'x_i' for '(x_i ^ x_{i+1})'', starting with i = 0. So in the
end, we obtain a formula (x_0 ^ (x_1 ^ (x_2 ^ ... ( ^ x_n)))).

This operation was chosen because it substitutes the variable for
a formula that contains the variable itself, plus another, new variable.
As such, it should be more complicated than basic operations on tables
where the arity of the function does not change.
"""


def bench_subst_bbf(num_vars):
    variable_names = [f"x_{i}" for i in range(num_vars)]
    variables = [bbf.Table.from_expression(bbf.Expression(f"x_{i}")) for i in range(num_vars)]
    table = variables[0]
    for i in range(num_vars - 1):
        exp = bbf.Table.mk_xor(variables[i], variables[i + 1])
        table = table.substitute({variable_names[i]: exp})
    return table


def bench_subst_pyeda(num_vars):
    variables = [ttvar(f"x_{i}") for i in range(num_vars)]
    table = variables[0]
    for i in range(num_vars - 1):
        exp = variables[i] ^ variables[i + 1]
        table = table.compose({variables[i]: exp})
    return table


header = [
    "Var. count",
    "BBF[avg]",
    "PyEDA[avg]",
    "BBF[dev]",
    "PyEDA[dev]",
    "BBF[times]",
    "PyEDA[times]"
]
print("\t".join(header))

for num_vars in range(2, 20):
    (bbf_avg, bbf_dev, bbf_times) = run_bench(lambda: bench_subst_bbf(num_vars))
    (pyeda_avg, pyeda_dev, pyeda_times) = run_bench(lambda: bench_subst_pyeda(num_vars))

    row = [
        str(num_vars),
        str(bbf_avg),
        str(pyeda_avg),
        str(bbf_dev),
        str(pyeda_dev),
        str(bbf_times),
        str(pyeda_times)
    ]

    print("\t".join(row))
