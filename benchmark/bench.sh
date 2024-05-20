#!/bin/bash

# Apply memory limit.
# The tested limit was 2^25kB 33_554_432kB ~ 33GB
if [[ -z "${MEMORY_LIMIT}" ]]; then
  echo "Memory limit not set. Defaulting to 32GB."
  MEMORY_LIMIT=33554432
fi

ulimit -v $MEMORY_LIMIT

# The script assumes that you have 
# biodivine-boolean-functions and PyEDA
# already installed.

pip list > result_pip.txt

python3 bench_expr_parser.py &> result_expr_parser.tsv
python3 bench_expr_cnf.py &> result_expr_cnf.tsv

python3 bench_bdd_adder.py &> result_bdd_adder.tsv
python3 bench_bdd_monotonicity.py &> result_bdd_monotonicity.tsv

python3 bench_table_subst.py &> result_table_subst.tsv 

zip -r results.zip result_*