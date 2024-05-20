import biodivine_boolean_functions as bbf
from pyeda.inter import *
from utils import run_bench

"""
This benchmark constructs a binary "ripple carry adder" BDD.

This function has a "good" and "bad" variable ordering, resulting
in BDDs of vastly different sizes. Here, we are using the "bad"
ordering to force the library to use significant resources.
"""


def var_name(var_id):
    # These weird names ensure that (at least initially),
    # the variables are sorted based on their ID.
    return "v_" + ("x" * var_id)


def bench_adder_bbf(num_vars):
    assert num_vars % 2 == 0  # Only even numbers are allowed.

    variables = [var_name(i) for i in range(num_vars)]
    literals = [bbf.Bdd.mk_literal(var, True) for var in variables]
    result = bbf.Bdd.mk_const(False)

    for x in range(int(num_vars / 2)):
        x1 = literals[x]
        x2 = literals[x + int(num_vars / 2)]
        result = bbf.Bdd.mk_or(result, bbf.Bdd.mk_and(x1, x2))
    return result


def bench_adder_pyeda(num_vars):
    assert num_vars % 2 == 0  # Only even numbers are allowed.

    variables = [var_name(i) for i in range(num_vars)]
    literals = [bddvar(var) for var in variables]
    result = 0

    for x in range(int(num_vars / 2)):
        x1 = literals[x]
        x2 = literals[x + int(num_vars / 2)]
        result = result | (x1 & x2)

    return result


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

for num_vars in range(1, 20):
    num_vars = num_vars * 2  # Benchmark requires even variable count.

    (bbf_avg, bbf_dev, bbf_times) = run_bench(lambda: bench_adder_bbf(num_vars))
    (pyeda_avg, pyeda_dev, pyeda_times) = run_bench(lambda: bench_adder_pyeda(num_vars))

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
