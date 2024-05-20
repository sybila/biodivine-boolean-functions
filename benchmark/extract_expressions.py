# pip install biodivine_aeon==1.0.0a8
from biodivine_aeon import *
import sys
import os

"""
This file takes as an argument a path to a directory with Boolean network
model files and extracts the update functions from each file. It then
prints all the functions that have length greater than
the `MIN_LEN` threshold (in the string format).

Note that this is a poor approximation of complexity since the length
of the string representation depends on the length of the variable names.
But it should be a good enough filter to prune very simple functions.

To obtain the source model files, you can download this
archive of the BBM dataset:

https://github.com/sybila/biodivine-boolean-models/releases/download/august-2022/edition-2022-aeon.zip
"""

# Directory with models
DIR = sys.argv[1]

MIN_LEN = 100

files = sorted([f for f in os.listdir(DIR) if f.endswith(".aeon")])

for f in files:
    bn = BooleanNetwork.from_file(f"{DIR}/{f}")
    for var in bn.variables():
        update = str(bn.get_update_function(var))
        if len(update) > MIN_LEN:
            print(update)
