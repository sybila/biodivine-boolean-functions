import biodivine_boolean_functions as bbf
from pyeda.inter import *
import time

def ripple_carry_adder_bbf(num_vars):
    assert num_vars % 2 == 0 # Only even numbers are allowed.
    
    variables = [ f"x_{i}" for i in range(num_vars) ]
    literals = [ bbf.Bdd.mk_literal(var, True) for var in variables ]
    result = bbf.Bdd.mk_const(False)
    
    for x in range(int(num_vars / 2)):
        x1 = literals[x]
        x2 = literals[x + int(num_vars / 2)]
        result = bbf.Bdd.mk_or(result, bbf.Bdd.mk_and(x1, x2))

    return result

def ripple_carry_adder_pyeda(num_vars):
    assert num_vars % 2 == 0 # Only even numbers are allowed.
    
    variables = [ f"x_{i}" for i in range(num_vars) ]
    literals = [ bddvar(var) for var in variables ]
    result = 0
    
    for x in range(int(num_vars / 2)):
        x1 = literals[x]
        x2 = literals[x + int(num_vars / 2)]
        result = result | (x1 & x2)

    return result

REPETITIONS = 10

print("size\tBBF\tPyEDA")

for x in range(20):
	num_vars = 2 * x

	total_bbf = 0	
	for _ in range(REPETITIONS):
		start = time.perf_counter_ns()
		ripple_carry_adder_bbf(num_vars)
		total_bbf += time.perf_counter_ns() - start

	total_pyeda = 0
	for _ in range(REPETITIONS):
		start = time.perf_counter_ns()
		ripple_carry_adder_pyeda(num_vars)
		total_pyeda += time.perf_counter_ns() - start

	print(f"{num_vars}\t{int(total_bbf / REPETITIONS)}\t{int(total_pyeda / REPETITIONS)}")