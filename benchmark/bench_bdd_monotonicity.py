import biodivine_boolean_functions as bbf
from pyeda.inter import *
from utils import run_bench

"""
This benchmark constructs a BDD representing the set of all monotonic
functions of the given arity. The size of this set are the "Dedekind
numbers". Computing dedekind numbers is an open problem with no known
solution better than O((n ** 2) ** 2) (i.e. double exponential).

Regardless of variable ordering, the BDD for this problem is exponentially
large. Furthermore, this problem is biologically relevant, since the
Boolean functions that typically appear in biological models are assumed
to be monotonic.
"""


def var_name(var_id):
    # These weird names ensure that (at least initially),
    # the variables are sorted based on their ID.
    return "v_" + ("x" * var_id)


def bench_monotonicity_bbf(arity):
    num_vars = 2 ** arity

    variables = [var_name(i) for i in range(num_vars)]
    literals = [bbf.Bdd.mk_literal(var, True) for var in variables]
    result = bbf.Bdd.mk_const(True)

    for i in range(arity):
        block_size = 2 ** (i + 1)
        half_block = int(block_size / 2)
        regulator_formula = bbf.Bdd.mk_const(True)
        for block in range(int(num_vars / block_size)):
            for block_item in range(half_block):
                var1 = literals[block_size * block + block_item]
                var2 = literals[block_size * block + block_item + half_block]
                implies = bbf.Bdd.mk_or(bbf.Bdd.mk_not(var1), var2)
                regulator_formula = bbf.Bdd.mk_and(regulator_formula, implies)
        result = bbf.Bdd.mk_and(result, regulator_formula)
    return result


def bench_monotonicity_pyeda(arity):
    num_vars = 2 ** arity

    variables = [var_name(i) for i in range(num_vars)]
    literals = [bddvar(var) for var in variables]
    result = 1

    for i in range(arity):
        block_size = 2 ** (i + 1)
        half_block = int(block_size / 2)
        regulator_formula = 1
        for block in range(int(num_vars / block_size)):
            for block_item in range(half_block):
                var1 = literals[block_size * block + block_item]
                var2 = literals[block_size * block + block_item + half_block]
                implies = ~var1 | var2
                regulator_formula = regulator_formula & implies
        result = result & regulator_formula
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

for num_vars in range(1, 6):
    (bbf_avg, bbf_dev, bbf_times) = run_bench(lambda: bench_monotonicity_bbf(num_vars))
    (pyeda_avg, pyeda_dev, pyeda_times) = run_bench(lambda: bench_monotonicity_pyeda(num_vars))

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
