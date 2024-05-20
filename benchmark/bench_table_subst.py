import biodivine_boolean_functions as bbf
from pyeda.inter import *
import time

def bench_subst_bbf(num_vars):
	variable_names = [ f"x_{i}" for i in range(num_vars) ]
	variables = [ bbf.Table.from_expression(bbf.Expression(f"x_{i}")) for i in range(num_vars) ]
	table = bbf.Table.mk_xor(variables[0], variables[1])
	for i in range(num_vars - 1):
		if i == 0:
			continue
		exp = bbf.Table.mk_xor(variables[i], variables[i+1])
		table = table.substitute({ variable_names[i]: exp })
	return table

def bench_subst_pyeda(num_vars):
	variables = [ ttvar(f"x_{i}") for i in range(num_vars) ]
	table = variables[0] ^ variables[1]
	for i in range(num_vars - 1):
		if i == 0:
			continue
		exp = variables[i] ^ variables[i+1]
		table = table.compose({ variables[i]: exp })
	return table

REPETITIONS = 10

print("size\tBBF\tPyEDA")

for x in range(20):
	if x < 5:
		continue
	num_vars = x

	total_bbf = 0	
	for _ in range(REPETITIONS):
		start = time.perf_counter_ns()
		bench_subst_bbf(num_vars)
		total_bbf += time.perf_counter_ns() - start

	total_pyeda = 0
	for _ in range(REPETITIONS):
		start = time.perf_counter_ns()
		bench_subst_pyeda(num_vars)
		total_pyeda += time.perf_counter_ns() - start

	print(f"{num_vars}\t{int(total_bbf / REPETITIONS)}\t{int(total_pyeda / REPETITIONS)}")