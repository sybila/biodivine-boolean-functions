#!/bin/bash

## Resolve folder of this script, following all symlinks, cd to parent
SCRIPT_SOURCE="${BASH_SOURCE[0]}"
while [ -h "$SCRIPT_SOURCE" ]; do # resolve $SOURCE until the file is no longer a symlink
  SCRIPT_DIR="$( cd -P "$( dirname "$SCRIPT_SOURCE" )" && pwd )"
  SCRIPT_SOURCE="$(readlink "$SCRIPT_SOURCE")"
  # if $SOURCE was a relative symlink, we need to resolve it relative to the path where the symlink file was located
  [[ $SCRIPT_SOURCE != /* ]] && SCRIPT_SOURCE="$SCRIPT_DIR/$SCRIPT_SOURCE"
done
cd -P "$( dirname "$SCRIPT_SOURCE" )/.." || exit 1

# If VIRTUAL_ENV is not set, check if either venv, .venv or .env directory
# exists and use that as the Python environment.

# Activating a virtual environment should set VIRTUAL_ENV, so this script
# should respect an active virtual environment (if any), and default to
# venv / .venv / .env if no environment is active.

if [ -z "$VIRTUAL_ENV" ]; then  # True if VIRTUAL_ENV is empty.
  if [ -d "venv" ]; then  # True if venv is a directory.
      VIRTUAL_ENV="`pwd`/venv"   # Absolute path is preferred here.
  elif [ -d ".venv" ]; then
      VIRTUAL_ENV="`pwd`/.venv"
  elif [ -d ".env" ]; then
        VIRTUAL_ENV="`pwd`/.env"
  else
      echo "Error: No 'venv' / '.venv' / '.env' directory found. Please activate a virtual environment or specify one in VIRTUAL_ENV."
      exit 1
  fi
fi

echo "Virtual environment: $VIRTUAL_ENV"

# We *have* to activate the environment for maturin to work (we can't just call
# it with an absolute path like any other executable Python package).
source $VIRTUAL_ENV/bin/activate; maturin develop -F python