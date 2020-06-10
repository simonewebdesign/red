#!/usr/bin/env bash

# Exit on error
set -e

echo Running end-to-end test...
# https://superuser.com/questions/1307732/how-to-send-binary-data-in-netcat-to-an-already-established-connection
# Create a temporary fifo:
tmpd=`mktemp -d`
tmpf="$tmpd"/fifo
mkfifo "$tmpf"
printf "%s\n" "$tmpf"  # just to know the path to the fifo, it may be useful later

# Create a background process that will:
# * read the fifo
# * pass data to the server
# * pass the stdout to a local file named "result"
nc 127.0.0.1 7878 < "$tmpf" > result &
ncpid=$!  # PID may be useful later

# Assuming there's no problem with the connection itself nor the server,
# the above background command will stay connected until you finish
# sending the first bunch of data through the fifo.
# But you want it to stay open and accept multiple writes,
# so open the fifo and don't close it (yet):
exec 3> "$tmpf"

# Now you can send whatever you like through the fifo and the background connection persists:
# echo 'set'     >&3     # sends text
# echo -e '\x80' >&3     # sends "binary"
# cat /etc/issue >&3     # sends file
# cat            >&3     # type whatever you want, terminate with Ctrl+D
echo 'set 1 a'   >&3     # outputs OK
echo 'set 2 b'   >&3     # outputs OK

echo 'get 1'     >&3     # outputs a
echo 'get 2'     >&3     # outputs b
echo 'get 3'     >&3     # outputs nil

echo 'sadd x'    >&3
echo 'sadd x'    >&3
echo 'sadd y'    >&3
echo 'sadd z'    >&3

echo 'srem z'    >&3

# echo 'smembers'  >&3     # outputs x\ny\n, but can't be tested because the order is arbitrary

echo 'debug'     >&3     # outputs entire state: 1a, 2b, x, y


EXPECTED="$(echo -n "OK
OK
a
b
nil
")"

if test -r result; then
  # Close the descriptor
  exec 3>&-
  # This is needed before reading the file contents
  sleep 0.1

  if [ "$(cat result)" = "$EXPECTED" ]; then
    echo -e 'TEST PASSED \033[32m✓\033[0m'
  else
    echo -e 'TEST FAILED \033[31m✗\033[0m'
    exit_status=1
  fi
else
  echo file was not readable or present
  exit_status=2
fi

# Remove the temporary directory and its content (i.e. the fifo):
# rm -r "$tmpd"
rm result

exit $exit_status
